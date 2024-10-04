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

    fn peek(&mut self) -> TokenNode {
        for i in self.index..self.tokens.len() {
            let node = match self.tokens.get(i) {
                Some(node) => node,
                None => break,
            };

            match node.token {
                Token::Eol | Token::Comment => continue,
                _ => {
                    self.index = i;
                    return node.clone()
                },
            }
        }

        return TokenNode {
            token: Token::Eof,
            index: 0,
        };
    }

    fn next(&mut self) -> TokenNode {
        let current = self.peek();
        self.index += 1;

        current
    }

    fn eat(&mut self, target: Token) -> TokenNode {
        let node = self.next();

        if target == node.token {
            return node;
        }

        panic!("bad token: {:?}, expected: {:?}", node, target)
    }

    fn has_tokens_left(&mut self) -> bool {
        let node = self.peek();
        node.token != Token::Eof
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
