use std::mem;

#[derive(Debug, Clone)]
pub enum Token {
    /* --- literals & constants --- */
    Number(u32),
    String(u32),

    /* --- punctuators --- */
    OpenParen(u32),
    CloseParen(u32),
    Semi(u32),
    Question(u32),
    Colon(u32),

    /* --- operators --- */
    Plus(u32),
    Minus(u32),
    Star(u32),
    Slash(u32),
    Percent(u32),

    /* --- identifier --- */
    Identifier(u32),

    /* --- keywords --- */

    /* --- special characters --- */
    Eol(u32),
    Eof,
}

impl Token {
    pub fn is_type(&self, other: &Token) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
