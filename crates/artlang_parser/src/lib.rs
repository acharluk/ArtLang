use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct ArtLangParser;

pub fn print_tree(pair: &Pair<'_, Rule>, indent: usize) {
    let indent_str = "  ".repeat(indent);
    println!("{}{:?}: {:?}", indent_str, pair.as_rule(), pair.as_str());
    for inner in pair.clone().into_inner() {
        print_tree(&inner, indent + 1);
    }
}

pub fn parse_program(input: &str) -> Result<Pair<'_, Rule>, String> {
    let mut pairs =
        ArtLangParser::parse(Rule::program, input).map_err(|e| format!("Parse error:\n{e}"))?;
    let program = pairs.next().unwrap();
    Ok(program)
}
