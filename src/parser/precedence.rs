use crate::lexer::tokens::{Token, TokenNode};

use super::Parser;

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
        Number => Primary,

        // postfix
        Plus | Minus => Unary,

        // blocks
        OpenParen => Default,

        _ => panic!("nud power: bad token: {:?}", node),
    }
}

pub fn led_power(node: &TokenNode, parser: &Parser) -> Precedence {
    use Precedence::*;
    use Token::*;

    match node.token {
        // infix
        Plus | Minus => Additive,
        Star | Slash | Percent => Multiplicative,
        Question | Colon => Ternary,

        // blocks
        CloseParen => Default,

        // end of expression
        _ => {
            // call nud, will panic if it can not start again
            if parser.prev().token == Token::Eol {
                nud_power(node);
                return Default;
            }

            panic!("led power: bad token: {:?}", node)
        }
    }
}
