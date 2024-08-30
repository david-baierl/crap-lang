#[derive(Debug, Copy, Clone)]
pub enum TokenKind {

    // --- literals & constants --- //

    Number,
    String,

    // --- punctuators --- //

    OpenParen,
    CloseParen,

    // --- operators --- //

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // --- identifier --- //

    Identifier,

    // --- keywords --- //

    // --- special characters --- //

    // Eol,
    Eof,
}

pub struct Token {
    pub index: u32,
    pub kind: TokenKind,
}

impl Token {

    pub fn debug(&self) {

        match self.kind {
            TokenKind::Identifier | TokenKind::Number | TokenKind::String => {
                // @TODO print value
                println!("{0:?}, ({1:?})", self.kind, self.index)
            }
            _ => {
                println!("{0:?}, ({1:?})", self.kind, self.index)
            }
        }
    }
}
