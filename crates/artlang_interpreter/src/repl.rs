use artlang_ast::{expression::Expression, statement::Statement};
use artlang_parser::{ReplResult, parse_program, parse_repl_line};
use rustyline::DefaultEditor;

use crate::{interpreter::Interpreter, value::Value};

const BANNER: &str = "ArtLang 0.0.1";
const PROMPT: &str = "> ";
const CONTINUATION: &str = ">> ";
const HISTORY_FILE: &str = ".artlang_history";

pub fn run_repl() {
    println!("{BANNER}");

    let mut editor = DefaultEditor::new().expect("Failed to start REPL");
    let _ = editor.load_history(HISTORY_FILE);
    let mut interpreter = Interpreter::new();
    let mut buffer = String::new();

    loop {
        let prompt = if buffer.is_empty() {
            PROMPT
        } else {
            CONTINUATION
        };

        match editor.readline(prompt) {
            Ok(line) => {
                if !buffer.is_empty() {
                    buffer.push('\n');
                }

                buffer.push_str(&line);

                if repl_line_incomplete(&buffer) {
                    continue;
                }

                let input = buffer.clone();
                buffer.clear();

                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                let _ = editor.add_history_entry(&input);

                execute_line(&mut interpreter, trimmed);
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }
}

pub fn execute_line(interpreter: &mut Interpreter, input: &str) {
    match parse_repl_line(input) {
        Ok(ReplResult::Expression(expression)) => {
            match evaluate_repl_expression(interpreter, &expression) {
                Ok(value) => {
                    println!("{value}");
                }
                Err(e) => eprintln!("Error evaluating expression: {e}"),
            }
            return;
        }
        Ok(ReplResult::Statement(statement)) => {
            if let Err(e) = execute_repl_statement(interpreter, &statement) {
                println!("Error executing statement: {e}");
            }

            return;
        }
        Err(_) => {}
    }

    match parse_program(input) {
        Ok(program) => {
            if let Err(e) = interpreter.run(&program) {
                eprintln!("Error executing program: {e}")
            }
        }
        Err(e) => {
            eprintln!("Error parsing program: {e}")
        }
    }
}

pub fn repl_line_incomplete(input: &str) -> bool {
    let mut depth: i32 = 0;
    for word in input.split_whitespace() {
        match word {
            "if" | "for" | "while" | "function" | "{" => depth += 1,
            "end" | "}" => depth -= 1,
            _ => {}
        }
    }

    return depth > 0;
}

pub fn evaluate_repl_expression(
    interpreter: &mut Interpreter,
    expression: &Expression,
) -> Result<Value, String> {
    interpreter
        .evaluate_expression(expression)
        .map_err(|e| e.to_string())
}

pub fn execute_repl_statement(
    interpreter: &mut Interpreter,
    statement: &Statement,
) -> Result<(), String> {
    interpreter
        .execute_statement(statement)
        .map_err(|e| e.to_string())
}
