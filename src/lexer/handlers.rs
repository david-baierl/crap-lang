use regex::Match;

use super::{tokens::Token, Lexer};

pub fn skip(lexer: &mut Lexer, find: Match) {
    lexer.advance(find.len());
}

pub fn end_of_line(lexer: &mut Lexer, find: Match) {
    // @TODO: implicit semicolon ?
    // match lexer.tokens.last().map(|token| token.kind) {
    //     Some(TokenKind::Eol) => skip(lexer, find),
    //     _ => default(lexer, find, TokenKind::Eol),
    // };

    skip(lexer, find);
}

pub fn default(lexer: &mut Lexer, find: Match, token: Token) {
    lexer.tokens.push(token);
    skip(lexer, find);
}
