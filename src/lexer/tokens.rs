#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenKind {
    Eof,

    Number,
    String,
    Identifier,

    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    OpenCurly,
    CloseCurly,

    Assignment,
    Equals,
    Not,
    NotEquals,

    Less,
    LessEquals,
    Greater,
    GreaterEquals,

    Or,
    And,

    Dot,
    DotDot,
    DotDotDot,

    SemiColon,
    Colon,
    Question,
    Comma,
    Plus,
    PlusPlus,
    PlusEquals,
    Minus,
    MinusMinus,
    MinusEquals,
    Star,
    StarStar,
    StarEquals,
    Slash,
    SlashEquals,
    Percent,
    PercentEquals,

    // ----------------------------
    // Reserved
    // ----------------------------
    Let,
    Const,
    Mutable,
    Struct,
    New,
    Delete,

    Import,
    From,
    Use,

    If,
    Else,

    For,
    While,
    In,
    Of,

    Func,
    Throw,
    Yeld,
    Return,
    Exports,
    Typeof,
    Dispose,
    Provide,
}

pub struct Token {
    index: u32,
    kind: TokenKind,
}

// impl<'a> Token<'a> {
//     pub fn is_one_of_many(&self, kinds: &[TokenKind]) -> bool {
//         for kind in kinds {
//             if &self.kind == kind {
//                 return true;
//             }
//         }

//         return false;
//     }
// }

pub fn debug(token: &Token) {
    use TokenKind::*;

    match token.kind {
        Identifier | Number | String => {
            println!("{0:?}, ({1:?})", token.kind, token.index)
        }
        _ => {
            println!("{:?}, ()", token.kind)
        }
    }
}

pub fn create(kind: TokenKind, index: u32) -> Token {
    Token { index, kind }
}