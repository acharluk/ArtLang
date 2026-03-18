use artlang_ast::expression::Expression;
use pest::iterators::Pair;

use crate::Rule;

pub fn build_expression(pair: Pair<'_, Rule>) -> Expression {
    Expression::Null
}
