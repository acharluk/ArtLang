use artlang_ast::expression::Expression;
use pest::iterators::Pair;

use crate::Rule;

pub fn build_expression(pair: Pair<'_, Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => build_expression(pair.into_inner().next().unwrap()),
        Rule::string => build_string_expression(pair),
        other => unreachable!("Unknown expression: {other:?}"),
    }
}

pub fn build_string_expression(pair: Pair<'_, Rule>) -> Expression {
    let string = pair.as_str();
    Expression::String(string[1..string.len() - 1].to_string())
}
