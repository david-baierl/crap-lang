use pattern::Pattern;
use tokens::{Token, TokenNode};

pub mod tokens;
mod pattern;
mod handlers;

struct Lexer {
    tokens: Vec<TokenNode>,
    index: usize,
}

impl Lexer {
    fn advance(&mut self, length: usize) {
        self.index += length
    }

    fn push(&mut self, token: TokenNode) {
        self.tokens.push(token);
    }

    fn last_type(&self) -> Option<&Token> {
        Some(&self.tokens.last()?.token)
    }

    fn new() -> Lexer {
        Lexer {
            tokens: vec![],
            index: 0,
        }
    }
}

pub fn tokenize<'a>(buffer: &'a str) -> Vec<TokenNode> {
    let patterns = Pattern::new();
    let mut lexer = Lexer::new();

    while lexer.index < buffer.len() {
        let slice = &buffer[lexer.index..];
        let index: u32 = lexer.index.try_into().unwrap();

        if let Some(value) = patterns.single_line_comment.find(&slice) {
            handlers::default(&mut lexer, value, Token::Comment, index);

            // the comment has eaten the new line character
            // for shy semi, we need to readd it manual
            lexer.push(TokenNode {
                index,
                token: Token::Eol,
            });

            continue;
        }

        if let Some(value) = patterns.multi_line_comment.find(&slice) {
            handlers::default(&mut lexer, value, Token::Comment, index);
            continue;
        }

        if let Some(value) = patterns.string.find(&slice) {
            handlers::default(&mut lexer, value, Token::String, index);
            continue;
        }

        if let Some(value) = patterns.identifier.find(&slice) {
            handlers::identifier(&mut lexer, value, index);
            continue;
        }

        if let Some(value) = patterns.number.find(&slice) {
            handlers::default(&mut lexer, value, Token::Number, index);
            continue;
        }

        if let Some(value) = patterns.open_paren.find(&slice) {
            handlers::default(&mut lexer, value, Token::OpenParen, index);
            continue;
        }

        if let Some(value) = patterns.close_paren.find(&slice) {
            handlers::default(&mut lexer, value, Token::CloseParen, index);
            continue;
        }

        if let Some(value) = patterns.equal.find(&slice) {
            handlers::default(&mut lexer, value, Token::Equal, index);
            continue;
        }

        if let Some(value) = patterns.plus.find(&slice) {
            handlers::default(&mut lexer, value, Token::Plus, index);
            continue;
        }

        if let Some(value) = patterns.minus.find(&slice) {
            handlers::default(&mut lexer, value, Token::Minus, index);
            continue;
        }

        if let Some(value) = patterns.star.find(&slice) {
            handlers::default(&mut lexer, value, Token::Star, index);
            continue;
        }

        if let Some(value) = patterns.slash.find(&slice) {
            handlers::default(&mut lexer, value, Token::Slash, index);
            continue;
        }

        if let Some(value) = patterns.percent.find(&slice) {
            handlers::default(&mut lexer, value, Token::Percent, index);
            continue;
        }

        if let Some(value) = patterns.end_of_line.find(&slice) {
            handlers::end_of_line(&mut lexer, value, index);
            continue;
        }

        if let Some(value) = patterns.semi.find(&slice) {
            handlers::default(&mut lexer, value, Token::Semi, index);
            continue;
        }

        if let Some(value) = patterns.colon.find(&slice) {
            handlers::default(&mut lexer, value, Token::Colon, index);
            continue;
        }

        if let Some(value) = patterns.question.find(&slice) {
            handlers::default(&mut lexer, value, Token::Question, index);
            continue;
        }

        if let Some(value) = patterns.whitespace.find(&slice) {
            handlers::skip(&mut lexer, value.len());
            continue;
        }

        // @TODO: error nothing matched (at ...)
        println!("end");
        break;
    }

    lexer.tokens
}
