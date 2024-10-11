use crate::{
    ast::statements::{Statement, StatementFlag},
    lexer::tokens::Token,
    utils::{
        bit_array::{BitArray, Byte},
        buffer::{BufferReader, BufferWriter},
    },
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
    value.resize(value.len() + symbol.len());
    value.concat(&symbol);

    let len: u16 = value.len().try_into().unwrap();

    Statement::Variable {
        expr: value.extract(),
        len,
        flags,
    }
}

fn parse_expr_stmt(parser: &mut Parser) -> Statement {
    let mut expr = parse_expr(parser, Precedence::Default);
    let len: u16 = expr.len().try_into().unwrap();

    Statement::Expression {
        expr: expr.extract(),
        len,
    }
}
