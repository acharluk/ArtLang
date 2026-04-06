use std::env;
use std::fs;

use artlang_interpreter::interpreter::Interpreter;
use artlang_parser::parse_program;
use rustyline::DefaultEditor;

const BANNER: &str = "ArtLang 0.0.1";
const PROMPT: &str = "> ";
const CONTINUATION: &str = ">> ";
const HISTORY_FILE: &str = ".artlang_history";

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

fn run_repl() {
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

                let _ = editor.add_history_entry(trimmed);

                execute_input(&mut interpreter, trimmed);
            }
            Err(err) => {
                eprintln!("Error: {err}");
                break;
            }
        }
    }

    let _ = editor.save_history(HISTORY_FILE);
}

fn execute_input(interpreter: &mut Interpreter, input: &str) {
    match parse_program(input) {
        Ok(program) => {
            if let Err(error) = interpreter.run(&program) {
                flush_output(interpreter);
                eprintln!("Runtime error: {error}");
            } else {
                flush_output(interpreter);
            }
        }
        Err(error) => {
            eprintln!("Parse error: {error}");
        }
    }
}

fn repl_line_incomplete(input: &str) -> bool {
    let mut depth: i32 = 0;

    for token in input.split_whitespace() {
        match token {
            "if" | "for" | "while" | "function" | "{" => depth += 1,
            "end" | "}" => depth -= 1,
            _ => {}
        }
    }

    depth > 0
}

fn flush_output(interpreter: &mut Interpreter) {
    let output = interpreter.take_output();
    if !output.is_empty() {
        print!("{output}");
    }
}
