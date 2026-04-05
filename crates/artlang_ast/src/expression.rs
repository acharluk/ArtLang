use std::fmt;

use crate::{
    Block, Name,
    operators::{BinaryOperator, UnaryOperator},
};

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,

    Variable(Name),

    BinaryOperator(BinaryOperator, Box<Expression>, Box<Expression>),
    UnaryOperator(UnaryOperator, Box<Expression>),

    FunctionCall(Name, Vec<Expression>),
    ExpressionCall(Box<Expression>, Vec<Expression>),

    AnonymousFunction { params: Vec<Name>, body: Block },
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::String(s) => write!(f, "{s}"),
            _ => write!(f, "ast::expression: fmt::Display not implemented"),
        }
    }
}
