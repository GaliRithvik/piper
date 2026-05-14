mod lexer;
mod parser;
mod interpreter;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_piper(source: &str) -> String {
    interpreter::capture_output(|| {
        let tokens = lexer::tokenize(source);
        let ast    = parser::parse(tokens);
        interpreter::run(ast);
    })
}
