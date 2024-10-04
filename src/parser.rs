use crate::tokens::{Token, TokenNode};

mod expressions;
use expressions::*;
mod precedence;
use precedence::*;
mod statements;
use statements::*;

struct Parser {
    index: usize,
    stmt: Vec<Statement>,
    tokens: Vec<TokenNode>,
}

impl Parser {
    fn new(tokens: Vec<TokenNode>) -> Parser {
        Parser {
            index: 0,
            stmt: vec![],
            tokens,
        }
    }

    fn push(&mut self, stmt: Statement) {
        self.stmt.push(stmt);
    }

    fn prev(&self) -> TokenNode {
        match self.tokens.get(self.index - 1) {
            Some(t) => t.clone(),
            None => TokenNode{ token: Token::Eof, index: 0 },
        }
    }

    fn peek(&self) -> TokenNode {
        match self.tokens.get(self.index) {
            Some(t) => t.clone(),
            None => TokenNode{ token: Token::Eof, index: 0 },
        }
    }

    fn next(&mut self) -> TokenNode {
        let current = self.peek();
        self.index += 1;

        current
    }

    fn eat(&mut self, target: Token) -> TokenNode {
        let mut token = self.next();

        // eat shy (implicit semicolons)
        while token.token == Token::Eol {
            token = self.next();
        }

        if target == token.token {
            return token;
        }

        panic!("bad token: {:?}, expected: {:?}", token, target)
    }

    fn has_tokens_left(&self) -> bool {
        match self.peek().token {
            Token::Eof => return false,
            _ => self.index < self.tokens.len(),
        }
    }
}

pub fn parse(tokens: Vec<TokenNode>) -> Vec<Statement> {
    let mut parser = Parser::new(tokens);

    while parser.has_tokens_left() {
        let stmt = parse_stmt(&mut parser);
        parser.push(stmt);
    }

    parser.stmt
}
