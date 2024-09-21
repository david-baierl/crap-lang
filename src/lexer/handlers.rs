use super::{tokens::Token, Lexer};

pub fn skip(lexer: &mut Lexer, length: usize) {
    lexer.advance(length);
}

pub fn end_of_line(lexer: &mut Lexer, length: usize, index: u32) {
    match lexer.last() {
        Some(Token::Eol(_)) => skip(lexer, length),
        _ => default(lexer, length, Token::Eol(index)),
    };
}

pub fn default(lexer: &mut Lexer, length: usize, token: Token) {
    lexer.push(token);
    skip(lexer, length);
}
