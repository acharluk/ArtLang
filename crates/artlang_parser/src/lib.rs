use artlang_ast::Expression;
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

pub fn parse_program(input: &str) -> Result<Expression, String> {
    let mut pairs =
        ArtLangParser::parse(Rule::program, input).map_err(|e| format!("Parse error:\n{e}"))?;
    let program = pairs.next().unwrap();

    assert_eq!(program.as_rule(), Rule::program);
    let expression = program.into_inner().next().unwrap();
    assert_eq!(expression.as_rule(), Rule::expression);
    let function_call = expression.into_inner().next().unwrap();
    assert_eq!(function_call.as_rule(), Rule::function_call);

    let expression = build_function_call(function_call);
    Ok(expression)
}

pub fn build_function_call(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::function_call);

    let mut inner = pair.into_inner();
    let name = inner.next().unwrap().as_str().to_string();
    let args: Vec<Expression> = inner.map(build_string).collect();
    Expression::FunctionCall(name, args)
}

pub fn build_string(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::string);

    let str = pair.as_str().to_string();
    Expression::String(str[1..str.len() - 1].to_string())
}
