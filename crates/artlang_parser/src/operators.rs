use artlang_ast::operators::BinaryOperator;
use pest::iterators::Pair;

use crate::Rule;

pub fn parse_add_operator(pair: &Pair<'_, Rule>) -> BinaryOperator {
    match pair.as_str() {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Subtract,
        other => unreachable!("Unknown add operator: {other}"),
    }
}

pub fn parse_multiply_operator(pair: &Pair<'_, Rule>) -> BinaryOperator {
    match pair.as_str() {
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        "//" => BinaryOperator::IDivide,
        "%" => BinaryOperator::Modulus,
        other => unreachable!("Unkown multiply operator: {other}"),
    }
}

pub fn parse_comparison_operator(pair: &Pair<'_, Rule>) -> BinaryOperator {
    match pair.as_str() {
        "==" => BinaryOperator::Equal,
        "~=" => BinaryOperator::NotEqual,
        "<" => BinaryOperator::LessThan,
        "<=" => BinaryOperator::LessEqual,
        ">" => BinaryOperator::GreaterThan,
        ">=" => BinaryOperator::GreaterEqual,
        other => unreachable!("Unknown comparison operator: {other}"),
    }
}

pub fn parse_concatenate_operator(pair: &Pair<'_, Rule>) -> BinaryOperator {
    match pair.as_str() {
        ".." => BinaryOperator::Concatenate,
        other => unreachable!("Unknown concatenate operator: {other}"),
    }
}

pub fn parse_logic_operator(pair: &Pair<'_, Rule>) -> BinaryOperator {
    match pair.as_str() {
        "and" => BinaryOperator::And,
        "or" => BinaryOperator::Or,
        other => unreachable!("Unknown logic operator: {other}"),
    }
}
