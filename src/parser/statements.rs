use crate::{
    ast::statements::{Statement, StatementFlag},
    lexer::tokens::Token,
    utils::{BitArray, Byte},
};

use super::{expressions::parse_expr, precedence::Precedence, Parser};

pub fn parse_stmt(parser: &mut Parser) -> Statement {
    use Token::*;

    let stmt = match parser.peek().token {
        Let | Const => parse_variable_stmt(parser),
        _ => parse_expr_stmt(parser),
    };

    if let Token::Semi = parser.peek().token {
        parser.next();
    }

    stmt
}

fn parse_variable_stmt(parser: &mut Parser) -> Statement {
    let node = parser.next();
    let mut flags: Byte = 0;

    match node.token {
        Token::Const => flags |= StatementFlag::IsConst.bit(),
        Token::Let => (),
        _ => panic!("bad token: {:?}", node),
    };

    let symbol = parse_expr(parser, Precedence::Default);

    parser.eat(Token::Equal);

    let mut value = parse_expr(parser, Precedence::Default);

    // [Value][Symbol]
    value.extend(symbol);

    Statement::Variable {
        expr: value.into(),
        flags,
    }
}

fn parse_expr_stmt(parser: &mut Parser) -> Statement {
    let expr = parse_expr(parser, Precedence::Default);
    Statement::Expression { expr: expr.into() }
}
