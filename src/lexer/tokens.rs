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

    /* --- misc & special characters --- */
    Comment,
    Eol, // end of line (shy semi)
    Eof, // end of file
}

#[derive(Debug, Clone)]
pub struct TokenNode {
    pub index: u32,
    pub token: Token,
}