use crate::utils::Singleton;

pub mod tokens;
use tokens::Token;

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
        let index:u32 = lexer.index.try_into().unwrap();

        if let Some(find) = patterns.string.find(&slice) {
            handlers::default(&mut lexer, find, Token::String(index));
            continue;
        }

        if let Some(find) = patterns.identifier.find(&slice) {
            handlers::default(&mut lexer, find, Token::Identifier(index));
            continue;
        }

        if let Some(find) = patterns.number.find(&slice) {
            handlers::default(&mut lexer, find, Token::Number(index));
            continue;
        }

        if let Some(find) = patterns.open_paren.find(&slice) {
            handlers::default(&mut lexer, find, Token::OpenParen(index));
            continue;
        }

        if let Some(find) = patterns.close_paren.find(&slice) {
            handlers::default(&mut lexer, find, Token::CloseParen(index));
            continue;
        }

        if let Some(find) = patterns.plus.find(&slice) {
            handlers::default(&mut lexer, find, Token::Plus(index));
            continue;
        }

        if let Some(find) = patterns.minus.find(&slice) {
            handlers::default(&mut lexer, find, Token::Minus(index));
            continue;
        }

        if let Some(find) = patterns.star.find(&slice) {
            handlers::default(&mut lexer, find, Token::Star(index));
            continue;
        }

        if let Some(find) = patterns.slash.find(&slice) {
            handlers::default(&mut lexer, find, Token::Slash(index));
            continue;
        }

        if let Some(find) = patterns.percent.find(&slice) {
            handlers::default(&mut lexer, find, Token::Percent(index));
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

    let index:u32 = lexer.index.try_into().unwrap();
    lexer.tokens.push(Token::Eof(index));

    for token in lexer.tokens.iter() {
        println!("{:?}", &token);
    }

    lexer.tokens
}
