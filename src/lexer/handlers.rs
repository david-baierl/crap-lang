use regex::Match;

use super::{tokens::{Token, TokenKind}, Lexer};

pub fn end_of_line(lexer: &mut Lexer, find: Match) {
    // @TODO: implicit semicolon ?
    // match lexer.tokens.last().map(|token| token.kind) {
    //     Some(TokenKind::Eol) => skip(lexer, find),
    //     _ => default(lexer, find, TokenKind::Eol),
    // };

    skip(lexer, find);
}

pub fn default(lexer: &mut Lexer, find: Match, kind: TokenKind) {
    let index = lexer.index.try_into().unwrap();
    let token = Token { kind, index };

    lexer.advance(find.len());
    lexer.tokens.push(token);
}

pub fn skip(lexer: &mut Lexer, find: Match) {
    lexer.advance(find.len());
}