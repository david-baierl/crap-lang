use crate::lexer::tokens::{Token, TokenNode};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Default,

    // Comma,
    // Assignment,
    Ternary,

    // Logical_OR,
    // Logical_AND,

    // Bitwise_OR,
    // Bitwise_XOR,
    // Bitwise_AND,

    // Logical_equal,
    // Logical_size,

    // Bitwise_shift,
    Additive,
    Multiplicative,

    Unary,
    Primary,
}

pub fn nud_power(node: &TokenNode) -> Precedence {
    use Precedence::*;
    use Token::*;

    match &node.token {
        // literals
        Number | Identifier => Primary,

        // postfix
        Plus | Minus => Unary,

        // end of expression
        _ => Default,
    }
}

pub fn led_power(node: &TokenNode) -> Precedence {
    use Precedence::*;
    use Token::*;

    match node.token {
        // infix
        Plus | Minus => Additive,
        Star | Slash | Percent => Multiplicative,
        Question | Colon => Ternary,

        // end of expression
        _ => Default,
    }
}
