pub mod environment;
pub mod interpreter;
pub mod value;

use artlang_ast::{Block, statement::Statement};
use artlang_parser::{parse_program, print_program};

use crate::interpreter::Interpreter;

fn main() {
    let input = r#"
        a = 7 * 5
        print("Hello world! A=" .. a)
    "#;

    print_program(input);

    match parse_program(input) {
        Ok(program) => {
            // execute_block(program);
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

fn execute_program(program: Statement) {
    execute_statement(program);
}

fn execute_block(block: Block) {
    for statement in block {
        execute_statement(statement);
    }
}

fn execute_statement(statement: Statement) {
    match statement {
        Statement::FunctionCall(name, args) => {
            if name == "print" {
                let parts: Vec<String> = args
                    .iter()
                    .map(|v| match v {
                        _ => format!("{v}"),
                    })
                    .collect();
                let output = parts.join("\t");
                print!("{output}");
            }
        }
        Statement::Assignment(name, expression) => {
            // TODO: Implement interpreter environment
        }

        _ => panic!("Not implemented!"),
    }
}
