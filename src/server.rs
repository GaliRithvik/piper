// Piper HTTP Server + Deception Layer
// Only compiled for native targets (not WASM).

use std::collections::HashMap;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;
use tiny_http::{Header, Response, Server};
use crate::interpreter::{Interpreter, Value};

// ── Entry point ───────────────────────────────────────────────────────────────

pub fn start_server(interp: &mut Interpreter, host: &str, port: u16) {
    let addr = format!("{}:{}", host, port);
    let server = Server::http(&addr)
        .unwrap_or_else(|e| panic!("[piper] Cannot bind to {}: {}", addr, e));

    println!();
    println!("  ┌──────────────────────────────────────────┐");
    println!("  │       Piper Deception Server v0.9.0      │");
    println!("  ├──────────────────────────────────────────┤");
    println!("  │  Listening  :  http://{:<19}│", addr);
    println!("  │  Routes     :  {:<26}│", interp.routes.len());
    println!("  │  Honeypots  :  {:<26}│", interp.honeypots.len());
    println!("  │  Canaries   :  {:<26}│", interp.canary_callbacks.len());
    println!("  └──────────────────────────────────────────┘");
    println!();
    println!("  [ctrl+c to stop]\n");

    loop {
        match server.recv() {
            Ok(raw) => handle_request(interp, raw),
            Err(e)  => eprintln!("[piper] server error: {}", e),
        }
    }
}

// ── Request dispatch ──────────────────────────────────────────────────────────

fn handle_request(interp: &mut Interpreter, mut raw: tiny_http::Request) {
    let ip     = extract_ip(&raw);
    let method = raw.method().to_string();
    let url    = raw.url().to_string();
    let path   = url.split('?').next().unwrap_or(&url).to_string();

    let mut headers: HashMap<String, String> = HashMap::new();
    for h in raw.headers() {
        headers.insert(h.field.to_string().to_lowercase(), h.value.to_string());
    }

    let mut body = String::new();
    let _ = raw.as_reader().read_to_string(&mut body);

    // ── Blocklist check ───────────────────────────────────────────────────────
    if interp.blocked_ips.contains(&ip) {
        let _ = raw.respond(json_response(429, r#"{"error":"too many requests"}"#));
        log_line(&format!("  [BLOCKED]  {} {} {}", method, path, ip));
        return;
    }

    let req_val = build_request_dict(&ip, &method, &path, &headers, &body);

    // ── Route lookup ──────────────────────────────────────────────────────────
    // Honeypots checked first — they shadow real routes on their paths
    let handler = find_handler(interp, &path);

    let (status, resp_body) = match handler {
        Some((is_honeypot, func)) => {
            if is_honeypot {
                let entry = format!("[{}] HONEYPOT  ip={}  path={}  ua={}",
                    now(),
                    ip, path,
                    headers.get("user-agent").map(|s| s.as_str()).unwrap_or("-"));
                println!("  ⚠  THREAT   {}", entry);
                interp.threat_log.push(entry);
            } else {
                println!("  ✓  {:<6} {}  [{}]", method, path, ip);
            }

            // Call the Piper handler — catch panics so server stays alive
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                interp.call_value(&func, vec![req_val])
            }));

            match result {
                Ok(val) => extract_response(&val),
                Err(_)  => (500, r#"{"error":"internal server error"}"#.to_string()),
            }
        }
        None => {
            println!("  ✗  {:<6} {}  [{}]  404", method, path, ip);
            (404, format!(r#"{{"error":"not found","path":"{}"}}"#, esc(&path)))
        }
    };

    // ── Canary scan ───────────────────────────────────────────────────────────
    check_canaries(interp, &resp_body, &ip);

    // respond consumes raw — must be last
    drop(raw.respond(json_response(status, &resp_body)));
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn extract_ip(req: &tiny_http::Request) -> String {  // borrows only, does not consume
    req.remote_addr()
        .map(|a| {
            let s = a.to_string();
            // strip port: "1.2.3.4:5678" → "1.2.3.4"
            s.split(':').next().unwrap_or(&s).to_string()
        })
        .unwrap_or_else(|| "0.0.0.0".to_string())
}

fn json_response(status: u16, body: &str) -> Response<std::io::Cursor<Vec<u8>>> {
    Response::from_string(body)
        .with_status_code(status)
        .with_header(Header::from_bytes("Content-Type", "application/json").unwrap())
        .with_header(Header::from_bytes("X-Powered-By", "Piper/0.9").unwrap())
}

/// Find a matching handler. Honeypots take priority over real routes.
fn find_handler(interp: &Interpreter, path: &str) -> Option<(bool, Value)> {
    for (pattern, func) in &interp.honeypots {
        if path_matches(path, pattern) {
            return Some((true, func.clone()));
        }
    }
    for (pattern, func) in &interp.routes {
        if path_matches(path, pattern) {
            return Some((false, func.clone()));
        }
    }
    None
}

/// `*` wildcard suffix, exact match otherwise.
fn path_matches(req_path: &str, pattern: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix('*') {
        req_path.starts_with(prefix)
    } else {
        req_path == pattern
    }
}

/// If the value is a `response(status, body)` dict, unpack it; else 200.
fn extract_response(val: &Value) -> (u16, String) {
    if let Value::Dict(d) = val {
        let d = d.borrow();
        if let Some(Value::Number(s)) = d.get("__status__") {
            let status = *s as u16;
            let body   = d.get("__body__").map(value_to_json).unwrap_or_else(|| "{}".to_string());
            return (status, body);
        }
    }
    (200, value_to_json(val))
}

/// Build the request object that handlers receive as a Piper dict.
pub fn build_request_dict(
    ip: &str, method: &str, path: &str,
    headers: &HashMap<String, String>, body: &str,
) -> Value {
    let mut hdr: HashMap<String, Value> = HashMap::new();
    for (k, v) in headers {
        hdr.insert(k.clone(), Value::Str(v.clone()));
    }

    let mut req: HashMap<String, Value> = HashMap::new();
    req.insert("ip".into(),      Value::Str(ip.into()));
    req.insert("method".into(),  Value::Str(method.into()));
    req.insert("path".into(),    Value::Str(path.into()));
    req.insert("body".into(),    Value::Str(body.into()));
    req.insert("headers".into(), Value::Dict(Rc::new(RefCell::new(hdr))));

    Value::Dict(Rc::new(RefCell::new(req)))
}

/// Scan the outgoing response body for registered canary tokens.
/// If a token is found, fire its callback with the caller's IP.
fn check_canaries(interp: &mut Interpreter, body: &str, ip: &str) {
    let pairs: Vec<(String, Value)> = interp.canary_callbacks.clone();
    for (token, callback) in pairs {
        if body.contains(&token) {
            let entry = format!("[{}] 🔴 CANARY  token={}  ip={}", now(), token, ip);
            println!("  {}", entry);
            interp.threat_log.push(entry);
            interp.call_value(&callback, vec![Value::Str(ip.to_string())]);
        }
    }
}

// ── Bot detection ─────────────────────────────────────────────────────────────

/// Returns 0–100 indicating how likely this request is from a bot/AI.
/// >= 50 is treated as a bot by `is_bot()`.
pub fn compute_bot_score(req: &Value) -> u32 {
    let headers = match req {
        Value::Dict(d) => match d.borrow().get("headers") {
            Some(Value::Dict(h)) => h.borrow().iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect::<HashMap<String, String>>(),
            _ => HashMap::new(),
        },
        _ => return 60,  // not a request dict → assume suspicious
    };

    let mut score: u32 = 0;

    // No User-Agent
    let ua = headers.get("user-agent").map(|s| s.to_lowercase());
    match &ua {
        None => score += 45,
        Some(ua) => {
            let bot_sigs = [
                "python-requests", "python-urllib", "python-httpx",
                "curl/", "wget/", "scrapy", "playwright", "puppeteer",
                "selenium", "headlesschrome", "headless", "phantomjs",
                "httpclient", "okhttp", "apache-httpclient",
                "go-http-client", "node-fetch", "axios", "got/",
                "libwww-perl", "java/", "jakarta", "pycurl",
                "claudebot", "gptbot", "anthropic", "openai",
                "bingbot", "googlebot", "crawl",
            ];
            for sig in &bot_sigs {
                if ua.contains(sig) { score += 40; break; }
            }
        }
    }

    // No Accept-Language header
    if !headers.contains_key("accept-language") { score += 25; }

    // No Accept header
    if !headers.contains_key("accept") { score += 15; }

    // No Referer
    if !headers.contains_key("referer") { score += 10; }

    // No Connection header
    if !headers.contains_key("connection") { score += 5; }

    score.min(100)
}

// ── JSON serialisation ────────────────────────────────────────────────────────

pub fn value_to_json(v: &Value) -> String {
    match v {
        Value::Number(n) => {
            if *n == n.floor() && n.abs() < 1e15 {
                format!("{}", *n as i64)
            } else {
                // trim trailing zeros
                let s = format!("{:.6}", n);
                s.trim_end_matches('0').trim_end_matches('.').to_string()
            }
        }
        Value::Str(s)  => format!("\"{}\"", esc(s)),
        Value::Bool(b) => b.to_string(),
        Value::Nil     => "null".to_string(),
        Value::List(l) => {
            let items: Vec<String> = l.borrow().iter().map(value_to_json).collect();
            format!("[{}]", items.join(","))
        }
        Value::Dict(d) => {
            let mut pairs: Vec<String> = d.borrow().iter()
                .filter(|(k, _)| !k.starts_with("__"))  // hide internal keys
                .map(|(k, v)| format!("\"{}\":{}", esc(k), value_to_json(v)))
                .collect();
            pairs.sort();
            format!("{{{}}}", pairs.join(","))
        }
        Value::Instance { fields, .. } => {
            let mut pairs: Vec<String> = fields.borrow().iter()
                .map(|(k, v)| format!("\"{}\":{}", esc(k), value_to_json(v)))
                .collect();
            pairs.sort();
            format!("{{{}}}", pairs.join(","))
        }
        _ => "null".to_string(),
    }
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\")
     .replace('"',  "\\\"")
     .replace('\n', "\\n")
     .replace('\r', "\\r")
     .replace('\t', "\\t")
}

// ── Timestamp ─────────────────────────────────────────────────────────────────

pub fn now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let total = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Rough UTC date (±1 day for leap years; good enough for threat logs)
    let secs_in_day = total % 86400;
    let h = secs_in_day / 3600;
    let m = (secs_in_day % 3600) / 60;
    let s = secs_in_day % 60;
    let days = total / 86400;
    // Simplified Gregorian (ignores leap seconds, close enough)
    let y400 = days / 146097;
    let d1   = days % 146097;
    let y100 = (d1 / 36524).min(3);
    let d2   = d1 - y100 * 36524;
    let y4   = d2 / 1461;
    let d3   = d2 % 1461;
    let y1   = (d3 / 365).min(3);
    let year = y400 * 400 + y100 * 100 + y4 * 4 + y1 + 1970;
    let doy  = d3 - y1 * 365;
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let mdays: [u64; 12] = if is_leap {
        [31,29,31,30,31,30,31,31,30,31,30,31]
    } else {
        [31,28,31,30,31,30,31,31,30,31,30,31]
    };
    let mut month = 1u64;
    let mut rem   = doy;
    for (i, &md) in mdays.iter().enumerate() {
        if rem < md { month = i as u64 + 1; break; }
        rem -= md;
    }
    let day = rem + 1;
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", year, month, day, h, m, s)
}

fn log_line(s: &str) { println!("{}", s); }
