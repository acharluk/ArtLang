use artlang_ast::{
    expression::Expression,
    operators::{BinaryOperator, UnaryOperator},
};
use pest::iterators::Pair;

use crate::{Rule, build_function_call_parts, operators, statements::build_block};

pub fn build_expression(pair: Pair<'_, Rule>) -> Expression {
    match pair.as_rule() {
        Rule::expression => build_expression(pair.into_inner().next().unwrap()),

        Rule::logical_expression => {
            build_binary_chain_expression(pair, operators::parse_logic_operator)
        }
        Rule::comparison => {
            build_binary_chain_expression(pair, operators::parse_comparison_operator)
        }

        Rule::concat_expression => {
            build_right_chain_expression(pair, operators::parse_concatenate_operator)
        }

        Rule::additive_expression => {
            build_binary_chain_expression(pair, operators::parse_add_operator)
        }
        Rule::multiplicative_expression => {
            build_binary_chain_expression(pair, operators::parse_multiply_operator)
        }

        Rule::unary_expression => build_unary_expression(pair),
        Rule::power_expression => build_power_expression(pair),

        Rule::primary => build_primary_expression(pair),

        Rule::number => build_number_expression(pair),
        Rule::string => build_string_expression(pair),
        Rule::boolean => build_boolean_expression(pair),
        Rule::null => Expression::Null,

        Rule::name => build_variable_expression(pair),
        Rule::qualified_name => build_variable_expression(pair),
        Rule::function_call => build_function_call_expression(pair),
        Rule::anonymous_function => build_anonymous_function_expression(pair),

        other => unreachable!("Unknown expression: {other:?}"),
    }
}

pub fn build_binary_chain_expression(
    pair: Pair<'_, Rule>,
    operator: fn(&Pair<'_, Rule>) -> BinaryOperator,
) -> Expression {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    let mut left = build_expression(first);

    while let Some(operator_pair) = inner.next() {
        let op = operator(&operator_pair);
        let right_pair = inner.next().unwrap();
        let right = build_expression(right_pair);
        left = Expression::BinaryOperator(op, Box::new(left), Box::new(right));
    }

    left
}

pub fn build_right_chain_expression(
    pair: Pair<'_, Rule>,
    op_parser: fn(&Pair<'_, Rule>) -> BinaryOperator,
) -> Expression {
    let mut inner = pair.into_inner();
    let mut operands: Vec<Expression> = Vec::new();
    let mut ops: Vec<BinaryOperator> = Vec::new();

    operands.push(build_expression(inner.next().unwrap()));
    while let Some(op_pair) = inner.next() {
        ops.push(op_parser(&op_pair));
        operands.push(build_expression(inner.next().unwrap()));
    }

    let mut expr = operands.pop().unwrap();
    while let Some(left) = operands.pop() {
        let op = ops.pop().unwrap();
        expr = Expression::BinaryOperator(op, Box::new(left), Box::new(expr));
    }

    expr
}

pub fn build_unary_expression(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::unary_expression);
    let mut inner = pair.into_inner();

    let first = inner.next().unwrap();

    match first.as_rule() {
        Rule::unary_ops => {
            let op = match first.as_str().trim() {
                "-" => UnaryOperator::Minus,
                "not" => UnaryOperator::Not,
                "#" => UnaryOperator::Length,
                other => unreachable!("Unknown unary operator: {other}"),
            };

            let operand = build_expression(inner.next().unwrap());

            Expression::UnaryOperator(op, Box::new(operand))
        }
        _ => build_expression(first),
    }
}

pub fn build_power_expression(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::power_expression);
    let mut inner = pair.into_inner();

    let base = build_expression(inner.next().unwrap());

    if let Some(_pow_op) = inner.next() {
        let exp = build_expression(inner.next().unwrap());
        Expression::BinaryOperator(BinaryOperator::Power, Box::new(base), Box::new(exp))
    } else {
        base
    }
}

pub fn build_primary_expression(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::primary);
    let mut inner = pair.into_inner();

    let base_pair = inner.next().unwrap();
    let expression = build_expression(base_pair);

    // TODO: Add call args handling

    expression
}

pub fn build_number_expression(pair: Pair<'_, Rule>) -> Expression {
    let string = pair.as_str();
    let number = string.parse::<f64>().unwrap();

    if string.contains('.') {
        Expression::Float(number)
    } else {
        Expression::Number(number)
    }
}

pub fn build_string_expression(pair: Pair<'_, Rule>) -> Expression {
    let string = pair.as_str();
    Expression::String(string[1..string.len() - 1].to_string())
}

pub fn build_boolean_expression(pair: Pair<'_, Rule>) -> Expression {
    Expression::Boolean(pair.as_str() == "true")
}

pub fn build_variable_expression(pair: Pair<'_, Rule>) -> Expression {
    Expression::Variable(pair.as_str().to_string())
}

pub fn build_function_call_expression(pair: Pair<'_, Rule>) -> Expression {
    let (name, args) = build_function_call_parts(pair);
    Expression::FunctionCall(name, args)
}

pub fn build_anonymous_function_expression(pair: Pair<'_, Rule>) -> Expression {
    assert_eq!(pair.as_rule(), Rule::anonymous_function);
    let mut inner = pair.into_inner();

    let next = inner.next().unwrap();
    let (params, body) = if next.as_rule() == Rule::param_list {
        let params: Vec<String> = next.into_inner().map(|p| p.as_str().to_string()).collect();
        let body = build_block(inner.next().unwrap());

        (params, body)
    } else {
        (Vec::new(), build_block(next))
    };

    Expression::AnonymousFunction { params, body }
}
