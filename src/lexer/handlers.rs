use super::{tokens::{Token, TokenNode}, Lexer};

pub fn skip(lexer: &mut Lexer, length: usize) {
    lexer.advance(length);
}

pub fn end_of_line(lexer: &mut Lexer, value: &str, index: u32) {
    match lexer.last_type() {
        Some(Token::Eol) => skip(lexer, value.len()),
        _ => default(lexer, value, Token::Eol, index),
    };
}

pub fn identifier(lexer: &mut Lexer, value: &str, index: u32) {
    // reserved keywords
    let token = match value {
        "let" => Token::Let,
        "const" => Token::Const,
        _ => Token::Identifier
    };

    default(lexer, value, token, index)
}

pub fn default(lexer: &mut Lexer, value: &str, token: Token, index: u32) {
    let node = TokenNode {
        token,
        index,
    };

    lexer.push(node);
    skip(lexer, value.len());
}
