use crate::{ast::statements::Statement, lexer::tokens::Token};

use super::{expressions::parse_expr, precedence::Precedence, Parser};

pub fn parse_stmt(parser: &mut Parser) -> Statement {
    match parser.peek().token {
        _ => parse_expr_stmt(parser),
    }
}

fn parse_expr_stmt(parser: &mut Parser) -> Statement {
    let expr = parse_expr(parser, Precedence::Default);

    if let Token::Semi = parser.peek().token {
        parser.next();
    }

    Statement::Expression { expr }
}
