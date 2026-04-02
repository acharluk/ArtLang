pub mod environment;
pub mod interpreter;
pub mod value;

use artlang_parser::parse_program;

use crate::interpreter::Interpreter;

fn main() {
    let input = r#"
        -- a = 7 * 5 - 42 / 2 + -(5 % 3)
        -- print("Result is: " .. a)

        -- for b = 1, 10 do
        --     print("b = " .. b .. ", ")
        -- end

        c = 1
        while c < 10 do
          print("c = " .. c)
          c = c + 1
        end
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
