use artlang_ast::{Block, expression::Expression, statement::Statement};
use pest::iterators::Pair;

use crate::{Rule, build_function_call, expressions::build_expression};

pub fn build_block(pair: Pair<'_, Rule>) -> Block {
    assert_eq!(pair.as_rule(), Rule::block);

    pair.into_inner().map(build_statement).collect()
}

pub fn build_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::statement);
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::assignment => build_assignment_statement(inner),
        Rule::if_statement => build_if_statement(inner),
        Rule::for_statement => build_for_statement(inner),
        Rule::while_statement => build_while_statement(inner),
        Rule::function_call => build_function_call(inner),
        other => unreachable!("Unknown statement: {other:?}"),
    }
}

pub fn build_assignment_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::assignment);
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let expression = build_expression(inner.next().unwrap());

    Statement::Assignment(name, expression)
}

pub fn build_if_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::if_statement);
    let mut inner = pair.into_inner();

    let condition = build_expression(inner.next().unwrap());
    let then_block = build_block(inner.next().unwrap());

    let mut elseif_clauses: Vec<(Expression, Block)> = Vec::new();
    let mut else_block: Option<Block> = None;

    let remaining: Vec<Pair<'_, Rule>> = inner.collect();
    let mut i = 0;
    while i < remaining.len() {
        let pair = &remaining[i];

        match pair.as_rule() {
            Rule::expression => {
                let cond = build_expression(remaining[i].clone());
                let blk = build_block(remaining[i + 1].clone());
                elseif_clauses.push((cond, blk));
                i += 2;
            }
            Rule::block => {
                else_block = Some(build_block(pair.clone()));
                i += 1;
            }
            other => unreachable!("Unknown if statement: {other:?}"),
        }
    }

    Statement::If {
        condition,
        then_block,
        elseif_clauses,
        else_block,
    }
}

pub fn build_for_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::for_statement);
    let mut inner = pair.into_inner();

    let variable = inner.next().unwrap().as_str().to_string();
    let start = build_expression(inner.next().unwrap());
    let limit = build_expression(inner.next().unwrap());

    let next = inner.next().unwrap();
    let (step, body) = if next.as_rule() == Rule::expression {
        (
            Some(build_expression(next)),
            build_block(inner.next().unwrap()),
        )
    } else {
        (None, build_block(next))
    };

    Statement::For {
        variable,
        start,
        limit,
        step,
        body,
    }
}

pub fn build_while_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::while_statement);
    let mut inner = pair.into_inner();

    let condition = build_expression(inner.next().unwrap());
    let body = build_block(inner.next().unwrap());

    Statement::While { condition, body }
}

pub fn build_function_call_statement(pair: Pair<'_, Rule>) -> Statement {
    assert_eq!(pair.as_rule(), Rule::function_call);
    let mut inner = pair.into_inner();

    let name = inner.next().unwrap().as_str().to_string();
    let args: Vec<Expression> = inner.map(build_expression).collect();

    Statement::FunctionCall(name, args)
}
