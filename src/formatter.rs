// Piper source code formatter — `piper fmt <file>`
//
// Safe transformations only (no indentation recomputation):
//   • Trailing whitespace stripped
//   • Leading tabs → 4 spaces per tab
//   • Consecutive blank lines capped at 2
//   • Blank line inserted before top-level `fn` / `class` declarations
//   • Space after `#` in comments (e.g. `#foo` → `# foo`)
//   • Space after `,` when missing (respects string literals)

pub fn format_source(source: &str) -> String {
    let mut out: Vec<String> = Vec::with_capacity(source.lines().count() + 16);
    let mut blank_run: usize = 0;
    let mut in_triple = false; // inside a triple-quoted string block

    for line in source.lines() {
        // ── Triple-string tracking ────────────────────────────────────────
        // Count """ occurrences on this line to toggle in_triple.
        let toggles = triple_toggles(line);

        if in_triple {
            // Inside a multi-line string — emit verbatim, no formatting
            out.push(line.to_string());
            if toggles % 2 == 1 {
                in_triple = false;
            }
            blank_run = 0;
            continue;
        }

        if toggles % 2 == 1 {
            in_triple = true;
        }

        // ── Trailing whitespace ───────────────────────────────────────────
        let content = line.trim_end();

        // ── Blank line cap ────────────────────────────────────────────────
        if content.is_empty() {
            blank_run += 1;
            if blank_run <= 2 {
                out.push(String::new());
            }
            continue;
        }
        blank_run = 0;

        // ── Split leading whitespace from body ────────────────────────────
        let leading_len = content.len() - content.trim_start().len();
        let raw_indent  = &content[..leading_len];
        let body        = &content[leading_len..];

        // ── Normalize tabs in indent ──────────────────────────────────────
        let indent = raw_indent.replace('\t', "    ");

        // ── Blank line before top-level fn / class ────────────────────────
        let is_top_def = indent.is_empty()
            && (body.starts_with("fn ")
                || body.starts_with("class ")
                || body.starts_with("static fn "));

        if is_top_def
            && !out.is_empty()
            && out.last().map(|l: &String| !l.is_empty()).unwrap_or(false)
        {
            out.push(String::new());
        }

        // ── Format body ───────────────────────────────────────────────────
        let formatted_body = if toggles > 0 {
            // Line that opens a triple string — format only the part before """
            format_before_triple(body)
        } else {
            format_body(body)
        };

        out.push(format!("{}{}", indent, formatted_body));
    }

    // ── Strip trailing blank lines, ensure single final newline ───────────
    while out.last().map(|s: &String| s.is_empty()).unwrap_or(false) {
        out.pop();
    }
    out.push(String::new());
    out.join("\n")
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Count how many times `"""` appears (non-overlapping) in `line`.
fn triple_toggles(line: &str) -> usize {
    let mut count = 0;
    let mut i = 0;
    let bytes = line.as_bytes();
    while i + 2 < bytes.len() {
        if bytes[i] == b'"' && bytes[i + 1] == b'"' && bytes[i + 2] == b'"' {
            count += 1;
            i += 3;
        } else {
            i += 1;
        }
    }
    count
}

/// Format a normal line body (no triple-string involvement).
fn format_body(body: &str) -> String {
    // Pure comment line
    if body.starts_with('#') {
        return fix_comment(body);
    }
    // Format operators/commas, handle inline comment
    format_code_line(body)
}

/// For a line that opens a triple-quoted string, only format the prefix before `"""`.
fn format_before_triple(body: &str) -> String {
    if let Some(pos) = body.find("\"\"\"") {
        let prefix  = &body[..pos];
        let rest    = &body[pos..];
        let fmt_prefix = if prefix.starts_with('#') {
            fix_comment(prefix)
        } else {
            format_code_line(prefix)
        };
        format!("{}{}", fmt_prefix, rest)
    } else {
        format_body(body)
    }
}

/// Ensure a `#` comment has a space after the `#`.
fn fix_comment(body: &str) -> String {
    debug_assert!(body.starts_with('#'));
    let rest = &body[1..];
    if rest.is_empty() || rest.starts_with(' ') || rest.starts_with('!') {
        return body.to_string(); // already fine (or shebang)
    }
    format!("# {}", rest.trim_start())
}

/// Walk the code portion of a line character-by-character:
///   • Add space after `,` when the next char is not whitespace
///   • Fix inline `#comment` → `# comment`
///   • Respect double-quoted string literals (skip their contents)
fn format_code_line(body: &str) -> String {
    let chars: Vec<char> = body.chars().collect();
    let n = chars.len();
    let mut result = String::with_capacity(n + 8);
    let mut i = 0;
    let mut in_str = false;
    let mut escape_next = false;

    while i < n {
        let c = chars[i];

        // Inside a regular string literal
        if in_str {
            result.push(c);
            if escape_next {
                escape_next = false;
            } else if c == '\\' {
                escape_next = true;
            } else if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }

        // Opening a string (check for triple first)
        if c == '"' {
            // Triple-quoted? (should have been caught by outer logic, but guard here)
            if i + 2 < n && chars[i + 1] == '"' && chars[i + 2] == '"' {
                // Emit rest verbatim from here
                let tail: String = chars[i..].iter().collect();
                result.push_str(&tail);
                return result;
            }
            in_str = true;
            result.push(c);
            i += 1;
            continue;
        }

        // Comment outside string: fix spacing and stop
        if c == '#' {
            let tail: String = chars[i..].iter().collect();
            // Ensure a space before the # if previous char isn't whitespace
            if result.ends_with(|ch: char| !ch.is_whitespace()) {
                result.push(' ');
            }
            result.push_str(&fix_comment(&tail));
            return result;
        }

        // Comma spacing
        if c == ',' {
            result.push(',');
            // Add space if next is not already a space or closing bracket
            let next = chars.get(i + 1).copied().unwrap_or('\0');
            if next != ' ' && next != ')' && next != ']' && next != '}' && next != '\0' {
                result.push(' ');
            }
            i += 1;
            continue;
        }

        result.push(c);
        i += 1;
    }

    result
}
