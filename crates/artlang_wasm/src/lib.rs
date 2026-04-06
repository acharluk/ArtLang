use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct RunResult {
    success: bool,
    error: String,
    output: String,
}

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn run_program(source: &str) -> JsValue {
    let program = match artlang_parser::parse_program(source) {
        Ok(program) => program,
        Err(err) => {
            let result = RunResult {
                success: false,
                error: err,
                output: String::new(),
            };
            return serde_wasm_bindgen::to_value(&result).unwrap();
        }
    };

    let mut interpreter = artlang_interpreter::interpreter::Interpreter::new();

    match interpreter.run(&program) {
        Ok(()) => {
            let output = interpreter.take_output();
            let result = RunResult {
                success: true,
                error: String::new(),
                output,
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        }
        Err(err) => {
            let output = interpreter.take_output();
            let result = RunResult {
                success: false,
                error: err,
                output,
            };
            serde_wasm_bindgen::to_value(&result).unwrap()
        }
    }
}

#[wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
