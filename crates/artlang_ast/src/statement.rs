use std::fmt;

use crate::{Block, Name, expression::Expression};

#[derive(Debug, Clone)]
pub enum Statement {
    Assignment(Name, Expression),
    FunctionCall(Name, Vec<Expression>),

    If {
        condition: Expression,
        then_block: Block,
        elseif_clauses: Vec<(Expression, Block)>,
        else_block: Option<Block>,
    },

    For {
        variable: Name,
        start: Expression,
        limit: Expression,
        step: Option<Expression>,
        body: Block,
    },

    While {
        condition: Expression,
        body: Block,
    },

    FunctionDefinition {
        name: Name,
        params: Vec<Name>,
        body: Block,
    },

    Return(Option<Expression>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assignment(name, _expression) => write!(f, "assignment ({name})"),
            Statement::FunctionCall(name, _args) => write!(f, "function call ({name})"),
            _ => write!(f, "Not implemented"),
        }
    }
}
