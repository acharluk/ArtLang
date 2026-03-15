use artlang_ast::Expression;
use artlang_parser::parse_program;

fn main() {
    match parse_program("print(\"Hello world!\")") {
        Ok(expression) => {
            execute_expression(expression);
        }
        Err(e) => {
            println!("Error parsing the program :(\n{e}")
        }
    };
}

fn execute_expression(expression: Expression) {
    match expression {
        Expression::FunctionCall(name, args) => {
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
        _ => {
            println!("Nope")
        }
    }
}
