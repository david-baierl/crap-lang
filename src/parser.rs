use crate::tokens::Token;

mod expressions; use expressions::*;
mod precedence; use precedence::*;
mod statements; use statements::*;

struct Parser {
    index: usize,
    stmt: Vec<Statement>,
    tokens: Vec<Token>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            index: 0,
            stmt: vec![],
            tokens,
        }
    }

    fn push(&mut self, stmt: Statement) {
        self.stmt.push(stmt);
    }

    fn peek(&self) -> Token {
        match self.tokens.get(self.index) {
            Some(t) => t.clone(),
            None => Token::Eof,
        }
    }

    fn next(&mut self) -> Token {
        let current = self.peek();
        self.index += 1;

        current
    }

    fn has_tokens_left(&self) -> bool {
        match self.peek() {
            Token::Eof => return false,
            _ => self.index < self.tokens.len(),
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Statement>> {
    let mut parser = Parser::new(tokens);

    while parser.has_tokens_left() {
        let stmt = parse_stmt(&mut parser);
        parser.push(stmt);
    }

    Some(parser.stmt)
}
