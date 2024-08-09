use std::io::{Error, Result};

use regex::Regex;
use tokens::TokenKind;

mod tokens;

type PatternHandler = Box<dyn Fn(u32, &mut Lexer) -> () + 'static>;
pub struct RegexPattern {
    regex: Regex,
    handler: PatternHandler,
}

pub struct Lexer<'a> {
    patterns: Vec<RegexPattern>,
    tokens: Vec<tokens::Token>,
    source: &'a str,
    index: u32,
}

impl<'a> Lexer<'a> {
    pub fn advance(&mut self, lenght: u32) {
        self.index += lenght
    }
}

fn default_handler(length: u32, kind: tokens::TokenKind) -> PatternHandler {
    Box::new(move |index: u32, lexer| {
        lexer.tokens.push(tokens::create(kind, index));
        lexer.advance(length);
    })
}

pub fn create<'a>(buffer: &'a str) -> Lexer<'a> {
    use tokens::TokenKind::*;

    let patterns = vec![{
        RegexPattern {
            regex: Regex::new(r"\[").unwrap(),
            handler: default_handler(1, OpenBracket),
        }
    }];

    Lexer {
        patterns,
        tokens: vec![],
        source: &buffer,
        index: 0,
    }
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<tokens::Token> {
    let lexer = create(&buffer);

    lexer.tokens
}
