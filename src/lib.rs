mod lexer;
mod parser;
mod interpreter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn wasm_start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run_piper(source: &str) -> String {
    std::panic::catch_unwind(|| {
        interpreter::capture_output(|| {
            let tokens = lexer::tokenize(source);
            let ast    = parser::parse(tokens);
            interpreter::run(ast);
        })
    }).unwrap_or_else(|e| {
        let msg = if let Some(s) = e.downcast_ref::<String>() {
            s.clone()
        } else if let Some(s) = e.downcast_ref::<&str>() {
            s.to_string()
        } else {
            "Runtime error (panic)".to_string()
        };
        format!("Error: {}", msg)
    })
}
