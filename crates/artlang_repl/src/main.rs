use std::env;
use std::fs;

use artlang_interpreter::interpreter::Interpreter;
use artlang_parser::parse_program;

use crate::repl::run_repl;

mod repl;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        run_repl();
    }
}

fn run_file(path: &str) {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error reading file '{path}': {error}");
            std::process::exit(1);
        }
    };

    let program = match parse_program(&source) {
        Ok(program) => program,
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    };

    let mut interpreter = Interpreter::new();

    if let Err(error) = interpreter.run(&program) {
        flush_output(&mut interpreter);
        eprintln!("Runtime error: {error}");
        std::process::exit(1);
    }

    flush_output(&mut interpreter);
}

fn flush_output(interpreter: &mut Interpreter) {
    let output = interpreter.take_output();
    if !output.is_empty() {
        print!("{output}");
    }
}
