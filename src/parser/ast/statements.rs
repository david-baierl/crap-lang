use std::fmt::{Debug, Formatter, Result};

use super::Expression;

#[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Vec<Expression>),
}

