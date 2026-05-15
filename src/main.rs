mod lexer;
mod parser;
mod interpreter;
mod server;

use std::env;
use std::fs;
use std::io::{self, Write, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(|s| s.as_str()) {
        None | Some("repl") => run_repl(),
        Some(path)          => run_file(path),
    }
}

fn run_file(path: &str) {
    let source = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("Error: cannot read '{}'", path);
        std::process::exit(1);
    });
    let tokens = lexer::tokenize(&source);
    let ast    = parser::parse(tokens);

    // Run in a persistent interpreter so route()/honeypot()/serve() state is kept
    let mut interp = interpreter::Interpreter::new();
    interp.exec(&ast);

    // If the script called serve(...), start the HTTP server now
    if let Some(host) = interp.server_host.clone() {
        let port = interp.server_port;
        server::start_server(&mut interp, &host, port);
    }
}

fn needs_continuation(line: &str) -> bool {
    let t = line.trim_end();
    t.ends_with(':') || t.ends_with('\\')
}

fn run_repl() {
    println!("Piper v0.9.0 — Python-like syntax, Rust-powered");
    println!("Type 'exit' to quit. Lines ending with ':' auto-continue.\n");

    let stdin = io::stdin();
    let mut interp = interpreter::Interpreter::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut first = String::new();
        if stdin.lock().read_line(&mut first).unwrap() == 0 { break; }
        let trimmed = first.trim();
        if trimmed == "exit" { break; }
        if trimmed.is_empty() { continue; }

        let mut source = first.clone();

        if needs_continuation(trimmed) {
            loop {
                print!("... ");
                io::stdout().flush().unwrap();
                let mut cont = String::new();
                if stdin.lock().read_line(&mut cont).unwrap() == 0 { break; }
                if cont.trim().is_empty() { break; }
                source.push_str(&cont);
            }
        }

        let tokens = lexer::tokenize(&source);
        let ast    = parser::parse(tokens);
        interp.exec(&ast);
    }
}
