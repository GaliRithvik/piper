mod lexer;
mod parser;
mod interpreter;

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
    interpreter::run(ast);
}

fn run_repl() {
    println!("Piper v0.1.0 — Python-like syntax, Rust-powered");
    println!("Type 'exit' to quit.\n");

    let stdin = io::stdin();
    let mut interp = interpreter::Interpreter::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap() == 0 { break; }
        let trimmed = line.trim();
        if trimmed == "exit" { break; }
        if trimmed.is_empty() { continue; }

        let tokens = lexer::tokenize(trimmed);
        let ast    = parser::parse(tokens);
        interp.exec(&ast);
    }
}
