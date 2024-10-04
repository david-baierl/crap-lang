#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /* --- literals & constants --- */
    Number,
    String,

    /* --- punctuators --- */
    OpenParen,
    CloseParen,
    Semi,
    Question,
    Colon,

    /* --- operators --- */
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    /* --- identifier --- */
    Identifier,

    /* --- keywords --- */

    /* --- special characters --- */
    Eol,
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenNode {
    pub token: Token,
    pub index: u32,
}