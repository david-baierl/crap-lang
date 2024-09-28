use crate::tokens::Token;

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

pub fn nud_power(token: &Token) -> Precedence {
    use Precedence::*;
    use Token::*;

    match token {
        // literals
        Number(_) => Primary,

        // postfix
        Plus(_) | Minus(_) => Unary,

        // blocks
        OpenParen(_) | Semi(_) => Default,

        t => panic!("nud power: bad token: {:?}", t),
    }
}

pub fn led_power(token: &Token, parser: &Parser) -> Precedence {
    use Precedence::*;
    use Token::*;

    match token {
        // infix
        Plus(_) | Minus(_) => Additive,
        Star(_) | Slash(_) | Percent(_) => Multiplicative,
        Question(_) | Colon(_) => Ternary,

        // blocks
        CloseParen(_) => Default,

        // end of expression
        t => {
            // call nud, will panic if it can not start again
            if let Token::Eol(_) = parser.prev() {
                nud_power(t);
                return Default;
            }

            panic!("led power: bad token: {:?}", t)
        }
    }
}
