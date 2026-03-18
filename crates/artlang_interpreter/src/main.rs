use artlang_ast::{Block, statement::Statement};
use artlang_parser::{parse_program, print_program};

fn main() {
    let input = r#"
        print("Hello world!")
    "#;

    print_program(input);

    match parse_program(input) {
        Ok(program) => {
            execute_block(program);
        }
        Err(e) => {
            println!("Error parsing the program :(\n{e}")
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
        _ => panic!("Not implemented!"),
    }
}
