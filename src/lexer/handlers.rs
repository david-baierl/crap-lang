use super::{tokens::{Token, TokenNode}, Lexer};

pub fn skip(lexer: &mut Lexer, length: usize) {
    lexer.advance(length);
}

pub fn end_of_line(lexer: &mut Lexer, length: usize, index: u32) {
    match lexer.last_type() {
        Some(Token::Eol) => skip(lexer, length),
        _ => default(lexer, length, Token::Eol, index),
    };
}

pub fn default(lexer: &mut Lexer, length: usize, token: Token, index: u32) {
    let node = TokenNode {
        token,
        index,
    };

    lexer.push(node);
    skip(lexer, length);
}
