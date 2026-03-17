use crate::statement::Statement;

pub type Name = String;

pub mod expression;
pub mod operators;
pub mod statement;

pub type Block = Vec<Statement>;

pub type Program = Block;
