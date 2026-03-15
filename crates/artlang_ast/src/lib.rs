use std::fmt::{self, write};

pub type Name = String;

pub enum Expression {
    String(String),
    FunctionCall(Name, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::FunctionCall(name, args) => write!(f, "function call ({name})"),
            Expression::String(s) => write!(f, "{s}"),
        }
    }
}
