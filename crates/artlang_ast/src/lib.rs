use std::fmt;

pub type Name = String;

pub enum Expression {
    String(String),
}

pub enum Statement {
    Assignment(Name, Expression),
    FunctionCall(Name, Vec<Expression>),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::String(s) => write!(f, "{s}"),
        }
    }
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Assignment(name, _expression) => write!(f, "assignment ({name})"),
            Statement::FunctionCall(name, _args) => write!(f, "function call ({name})"),
        }
    }
}
