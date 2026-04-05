use artlang_ast::{Block, Name, expression::Expression, statement::Statement};
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

use crate::{
    expressions::build_expression,
    statements::{build_block, build_statement},
};

pub mod expressions;
pub mod operators;
pub mod statements;

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

pub fn print_program(input: &str) {
    let pairs = ArtLangParser::parse(Rule::program, input);
    match pairs {
        Ok(mut pair) => {
            let program = pair.next().unwrap();
            print_tree(&program, 0);
        }
        Err(e) => {
            println!("Error parsing :(\n{e}");
        }
    }
}

pub fn parse_program(input: &str) -> Result<Block, String> {
    let mut pairs =
        ArtLangParser::parse(Rule::program, input).map_err(|e| format!("Parse error:\n{e}"))?;

    let block = pairs.next().unwrap();
    assert_eq!(block.as_rule(), Rule::block);

    Ok(build_block(block))
}

pub fn build_function_call(pair: Pair<'_, Rule>) -> Statement {
    let (name, args) = build_function_call_parts(pair);
    Statement::FunctionCall(name, args)
}

pub fn build_function_call_parts(pair: Pair<'_, Rule>) -> (Name, Vec<Expression>) {
    assert_eq!(pair.as_rule(), Rule::function_call);

    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let args: Vec<Expression> = inner.map(build_expression).collect();

    (name, args)
}

pub fn build_string(pair: Pair<'_, Rule>) -> Expression {
    let str = pair.as_str();
    Expression::String(str[1..str.len() - 1].to_string())
}

pub enum ReplResult {
    Statement(Statement),
    Expression(Expression),
}

pub fn parse_repl_line(input: &str) -> Result<ReplResult, String> {
    if let Ok(mut pairs) = ArtLangParser::parse(Rule::repl_expression, input) {
        let pair = pairs.next().unwrap();
        return Ok(ReplResult::Expression(build_expression(pair)));
    }

    if let Ok(mut pairs) = ArtLangParser::parse(Rule::repl_statement, input) {
        let pair = pairs.next().unwrap();
        return Ok(ReplResult::Statement(build_statement(pair)));
    }

    Err(ArtLangParser::parse(Rule::repl_statement, input)
        .map_err(|e| format!("{e}"))
        .unwrap_err())
}
