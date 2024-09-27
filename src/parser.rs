use crate::tokens::Token;

pub mod ast;
use ast::*;
use expressions::Expression;
use precedence::Precedence;
use statements::Statement;

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

    fn advance(&mut self, length: usize) {
        self.index += length
    }

    fn push(&mut self, stmt: Statement) {
        self.stmt.push(stmt);
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.index)
    }

    fn pull(&mut self) -> Option<&Token> {
        if !self.has_tokens_left() {
            return None;
        }

        let current = self.tokens.get(self.index);
        self.index += 1;

        current
    }

    fn has_tokens_left(&self) -> bool {
        if let Some(Token::Eof(_)) = self.current() {
            return false;
        }

        self.index < self.tokens.len()
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<Vec<Statement>> {
    let mut parser = Parser::new(tokens);

    while parser.has_tokens_left() {
        match parse_stmt(&mut parser) {
            Some(stmt) => parser.push(stmt),
            _ => ()
        };
    }

    Some(parser.stmt)
}

fn parse_stmt(parser: &mut Parser) -> Option<Statement> {
    let expr = parse_expr(parser, Precedence::Default)?;
    Some(Statement::Expression(expr))
}

fn parse_expr(parser: &mut Parser, power: Precedence) -> Option<Vec<Expression>> {
    use Token::*;

    let token = parser.current()?;

    // nud handler
    let mut expr = match token {
        Number(_, _) => parse_primary_expr(parser)?,
        // @TODO: handle error,
        // @TODO: shy semi ?
        _ => {
            parser.advance(1);
            return None
        },
    };

    loop {
        if !parser.has_tokens_left() {
            return Some(expr);
        }

        let next_token = match parser.current() {
            Some(t) => t,
            None => return Some(expr)
        };

        let next_power = match led_power(next_token) {
            Some(p) => p,
            None => return Some(expr)
        };

        if next_power <= power {
            return Some(expr);
        }

        // led handler
        let next = match next_token {
            Plus(_) | Minus(_) => parse_binary_expr(parser, next_power),
            Star(_) | Slash(_) | Percent(_) => parse_binary_expr(parser, next_power),

            // @TODO: handle error,
            _ => None,
        };

        match next {
            Some(next_expr) => expr.extend(next_expr),
            None => return Some(expr),
        }
    }
}

fn parse_primary_expr(parser: &mut Parser) -> Option<Vec<Expression>> {
    let token = parser.pull()?;

    let expr = match token {
        Token::Number(_, _) => Expression::Literal(token.clone()),
        // @TODO: handle error
        _ => return None,
    };

    Some(vec![expr])
}

fn parse_binary_expr(parser: &mut Parser, power: Precedence) -> Option<Vec<Expression>> {
    let token = parser.pull()?.clone();

    let mut next = parse_expr(parser, power)?;
    next.push(Expression::Binary(token));

    Some(next)
}

fn led_power(token: &Token) -> Option<Precedence> {
    use Token::*;

    let power = match token {
        Plus(_) | Minus(_) => Precedence::Additive,
        Star(_) | Slash(_) | Percent(_) => Precedence::Multiplicative,
        // @TODO: handle error,
        _ => return None,
    };

    Some(power)
}
