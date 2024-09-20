#[derive(Debug)]
pub enum Token {
    // --- literals & constants --- //
    Number(u32),
    String(u32),

    // --- punctuators --- //
    OpenParen(u32),
    CloseParen(u32),

    // --- operators --- //
    Plus(u32),
    Minus(u32),
    Star(u32),
    Slash(u32),
    Percent(u32),

    // --- identifier --- //
    Identifier(u32),

    // --- keywords --- //

    // --- special characters --- //

    // Eol,
    Eof(u32),
}