use statements::parse_stmt;

use crate::{
    ast::statements::Statement,
    lexer::tokens::{Token, TokenNode},
};

mod expressions;
mod precedence;
mod statements;

struct Parser {
    index: usize,
    stmts: Vec<Statement>,
    tokens: Vec<TokenNode>,
}

impl Parser {
    fn new(tokens: Vec<TokenNode>) -> Parser {
        Parser {
            index: 0,
            stmts: vec![],
            tokens,
        }
    }

    fn push(&mut self, stmt: Statement) {
        self.stmts.push(stmt);
    }

    fn prev(&self) -> TokenNode {
        match self.tokens.get(self.index - 1) {
            Some(t) => t.clone(),
            None => TokenNode {
                token: Token::Eof,
                index: 0,
            },
        }
    }

    fn peek(&self) -> TokenNode {
        match self.tokens.get(self.index) {
            Some(t) => t.clone(),
            None => TokenNode {
                token: Token::Eof,
                index: 0,
            },
        }
    }

    fn next(&mut self) -> TokenNode {
        let current = self.peek();
        self.index += 1;

        current
    }

    fn eat(&mut self, target: Token) -> TokenNode {
        let mut node = self.next();

        // eat shy (implicit semicolons) and comments
        while node.token == Token::Eol || node.token == Token::Comment {
            node = self.next();
        }

        if target == node.token {
            return node;
        }

        panic!("bad token: {:?}, expected: {:?}", node, target)
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

    parser.stmts
}
