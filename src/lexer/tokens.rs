#[derive(Debug, Clone)]
pub enum Token {
    /* --- literals & constants --- */
    // @TODO: remove value from token
    Number(u32, f64),
    String(u32),

    /* --- punctuators --- */
    OpenParen(u32),
    CloseParen(u32),

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