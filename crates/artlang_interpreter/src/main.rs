pub mod environment;
pub mod interpreter;
pub mod value;

use std::{env, fs};

use artlang_parser::parse_program;

use crate::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        println!("Usage: art <filename>");
    }
}

fn run_file(path: &String) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file '{path}': {error}");
            std::process::exit(1);
        }
    };

    match parse_program(&source) {
        Ok(program) => {
            let mut interpreter = Interpreter::new();
            if let Err(error) = interpreter.run(&program) {
                eprintln!("Runtime error: {error}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
}
