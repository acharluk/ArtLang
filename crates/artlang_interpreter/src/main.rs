pub mod environment;
pub mod interpreter;
pub mod value;

use artlang_parser::parse_program;

use crate::interpreter::Interpreter;

fn main() {
    let input = r#"
        a = 7 * 5 - 42 / 2 + -(5 % 3)
        print("Result is: " .. a)
    "#;

    match parse_program(input) {
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
