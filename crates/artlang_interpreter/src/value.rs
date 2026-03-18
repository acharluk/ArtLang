use std::fmt;

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "Null"),
            Value::Boolean(b) => write!(f, "Boolean({b})"),
            Value::Integer(n) => write!(f, "Integer({n})"),
            Value::Float(n) => write!(f, "Float({n})"),
            Value::String(s) => write!(f, "String({s:?})"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "nil"),
            Value::Boolean(b) => write!(f, "{b}"),
            Value::Integer(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::String(s) => write!(f, "{s}"),
        }
    }
}
