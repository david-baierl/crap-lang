#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    
    // ----------------------------
    // identifier
    // ----------------------------

    Identifier,

    // ----------------------------
    // operators
    // ----------------------------

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // ----------------------------
    // constants
    // ----------------------------

    // ----------------------------
    // keywords
    // ----------------------------

    // ----------------------------
    // literals
    // ----------------------------

    Number,
    String,

    // ----------------------------
    // punctuators
    // ----------------------------

    OpenParen,
    CloseParen,

    // ----------------------------
    // special characters
    // ----------------------------
    
    Eof,
}

pub struct Token {
    index: u32,
    kind: TokenKind,
}

impl Token {
    pub fn from(kind: TokenKind, index: u32) -> Token {
        Token { index, kind }
    }
}

pub fn debug(token: &Token) {
    use TokenKind::*;

    match token.kind {
        Identifier | Number | String => {
            // @TODO print value
            println!("{0:?}, ({1:?})", token.kind, token.index)
        }
        _ => {
            println!("{0:?}, ({1:?})", token.kind, token.index)
        }
    }
}
