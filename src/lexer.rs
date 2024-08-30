use crate::utils::Singleton;

pub mod tokens;
use tokens::{Token, TokenKind};

mod pattern;
use pattern::Pattern;

mod handlers;

struct Lexer {
    tokens: Vec<tokens::Token>,
    index: usize,
}

impl Lexer {
    fn advance(&mut self, length: usize) {
        self.index += length
    }

    fn new() -> Lexer {
        Lexer {
            tokens: vec![],
            index: 0,
        }
    }
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<tokens::Token> {
    let patterns = Pattern::instance();
    let mut lexer = Lexer::new();

    while lexer.index < buffer.len() {
        let slice = &buffer[lexer.index..];

        if let Some(find) = patterns.string.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::String);
            continue;
        }

        if let Some(find) = patterns.identifier.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Identifier);
            continue;
        }

        if let Some(find) = patterns.number.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Number);
            continue;
        }

        if let Some(find) = patterns.open_paren.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::OpenParen);
            continue;
        }

        if let Some(find) = patterns.close_paren.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::CloseParen);
            continue;
        }

        if let Some(find) = patterns.plus.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Plus);
            continue;
        }

        if let Some(find) = patterns.minus.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Minus);
            continue;
        }

        if let Some(find) = patterns.star.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Star);
            continue;
        }

        if let Some(find) = patterns.slash.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Slash);
            continue;
        }

        if let Some(find) = patterns.percent.find(&slice) {
            handlers::default(&mut lexer, find, TokenKind::Percent);
            continue;
        }

        if let Some(find) = patterns.end_of_line.find(&slice) {
            handlers::end_of_line(&mut lexer, find);
            continue;
        }

        if let Some(find) = patterns.whitespace.find(&slice) {
            handlers::skip(&mut lexer, find);
            continue;
        }

        // @TODO: error nothing matched (at ...)
        println!("end");
        break;
    }

    lexer.tokens.push(Token {
        kind: TokenKind::Eof,
        index: lexer.index.try_into().unwrap(),
    });

    for token in lexer.tokens.iter() {
        token.debug();
    }

    lexer.tokens
}
