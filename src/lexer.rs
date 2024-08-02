use std::io::{Error, Result};

use regex::Regex;

mod tokens;

pub type RegexHandler<'a> = dyn Fn(&'a mut Lexer<'a>, &'a mut Regex) -> ();

pub struct RegexPattern<'a> {
    regex: Regex,
    handler: *mut RegexHandler<'a>,
}

pub struct Lexer<'a> {
    patterns: Vec<RegexPattern<'a>>,
    tokens: Vec<tokens::Token<'a>>,
    source: &'a str,
    index: usize,
}

impl<'a> Lexer<'a> {
    pub fn advance(&mut self, lenght: usize) {
        self.index += lenght
    }
}

fn defaultHandler<'a>(
    kind: tokens::TokenKind,
    value: &'a str
) -> impl Fn(&'a mut Lexer<'a>, &'a mut Regex) -> ()
{
    let length = value.len();

    let handler = move |
        lexer: &'a mut Lexer<'a>,
        regex: &'a mut Regex,
    | {
        lexer.advance(length)
    };

    return handler
}

pub fn create<'a>(buffer: &'a str) -> Result<Lexer<'a>> {
    use tokens::TokenKind::*;

    let b: *mut RegexHandler<'a> = defaultHandler(OpenBracket, "[");

    Ok(Lexer {
        patterns: vec![RegexPattern {
            // using unwrap here, because this regex are hard coded
            regex: Regex::new(r"\[").unwrap(),
            handler: b,
        }],
        tokens: vec![],
        source: &buffer,
        index: 0,
    })
}

pub fn tokenize<'a>(buffer: &'a str) -> Result<Vec<tokens::Token<'a>>> {
    let lexer = create(&buffer)?;

    Ok(lexer.tokens)
}
