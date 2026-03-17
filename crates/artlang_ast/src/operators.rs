#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Not,
    Minus,
    Length,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,

    Multiply,
    Divide,
    IDivide,
    Modulus,

    Power,

    Equal,
    NotEqual,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,

    Concatenate,

    And,
    Or,
}
