#[derive(Debug, PartialEq)]
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

pub struct Token<'a> {
    value: &'a str,
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
            println!("{0:?}, ({1:?})", token.kind, token.value)
        }
        _ => {
            println!("{:?}, ()", token.kind)
        }
    }
}

pub fn create<'a>(kind: TokenKind, value: &'a str) -> Token<'a> {
    Token { value, kind }
}
