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

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn last(&self) -> Option<&Token> {
        self.tokens.last()
    }

    fn new() -> Lexer {
        Lexer {
            tokens: vec![],
            index: 0,
        }
    }
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<tokens::Token> {
    let patterns = Pattern::new();
    let mut lexer = Lexer::new();

    while lexer.index < buffer.len() {
        let slice = &buffer[lexer.index..];
        let index: u32 = lexer.index.try_into().unwrap();

        if let Some(length) = patterns.string.find(&slice) {
            handlers::default(&mut lexer, length, Token::String(index));
            continue;
        }

        if let Some(length) = patterns.identifier.find(&slice) {
            handlers::default(&mut lexer, length, Token::Identifier(index));
            continue;
        }

        if let Some(length) = patterns.number.find(&slice) {
            handlers::default(&mut lexer, length, Token::Number(index));
            continue;
        }

        if let Some(length) = patterns.open_paren.find(&slice) {
            handlers::default(&mut lexer, length, Token::OpenParen(index));
            continue;
        }

        if let Some(length) = patterns.close_paren.find(&slice) {
            handlers::default(&mut lexer, length, Token::CloseParen(index));
            continue;
        }

        if let Some(length) = patterns.plus.find(&slice) {
            handlers::default(&mut lexer, length, Token::Plus(index));
            continue;
        }

        if let Some(length) = patterns.minus.find(&slice) {
            handlers::default(&mut lexer, length, Token::Minus(index));
            continue;
        }

        if let Some(length) = patterns.star.find(&slice) {
            handlers::default(&mut lexer, length, Token::Star(index));
            continue;
        }

        if let Some(length) = patterns.slash.find(&slice) {
            handlers::default(&mut lexer, length, Token::Slash(index));
            continue;
        }

        if let Some(length) = patterns.percent.find(&slice) {
            handlers::default(&mut lexer, length, Token::Percent(index));
            continue;
        }

        if let Some(length) = patterns.end_of_line.find(&slice) {
            handlers::end_of_line(&mut lexer, length, index);
            continue;
        }

        if let Some(length) = patterns.semi.find(&slice) {
            handlers::default(&mut lexer, length, Token::Semi(index));
            continue;
        }

        if let Some(length) = patterns.colon.find(&slice) {
            handlers::default(&mut lexer, length, Token::Colon(index));
            continue;
        }

        if let Some(length) = patterns.question.find(&slice) {
            handlers::default(&mut lexer, length, Token::Question(index));
            continue;
        }

        if let Some(length) = patterns.whitespace.find(&slice) {
            handlers::skip(&mut lexer, length);
            continue;
        }

        // @TODO: error nothing matched (at ...)
        println!("end");
        break;
    }

    lexer.tokens
}
