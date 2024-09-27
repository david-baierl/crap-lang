use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Precedence {
    Default,

    // Comma,
    // Assignment,

    // Ternary,

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
    use Token::*;
    use Precedence::*;

    match token {
        // literals
        Number(_, _) => Primary,
        
        // postfix
        Plus(_) | Minus(_) => Unary,
        
        // blocks
        OpenParen(_) => Default,

        t => panic!("bad token: {:?}", t),
    }
}

pub fn led_power(token: &Token) -> Precedence {
    use Token::*;
    use Precedence::*;

    match token {
        Plus(_) | Minus(_) => Additive,
        Star(_) | Slash(_) | Percent(_) => Multiplicative,

        t => panic!("bad token: {:?}", t),
    }
}