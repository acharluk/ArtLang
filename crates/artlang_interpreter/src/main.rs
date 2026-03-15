use artlang_parser::{parse_program, print_tree};

fn main() {
    let program = match parse_program("test") {
        Ok(pairs) => {
            print_tree(&pairs, 0);
        }
        Err(e) => {
            println!("Error parsing the program :(\n{e}")
        }
    };
}
