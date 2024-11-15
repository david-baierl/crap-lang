use crate::{
    ast::{expressions::ExpressionNode, statements::{Statement, StatementFlag}},
    lexer::tokens::Token, utils::{array_page_buffer::ArrayPageBuffer, bit_array::{BitArray, Byte}},
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

    let mut expr = ArrayPageBuffer::<ExpressionNode>::new();
    parse_expr(parser, &mut expr, Precedence::Default);

    parser.eat(Token::Equal);

    let mut value = ArrayPageBuffer::<ExpressionNode>::new();
    parse_expr(parser, &mut value, Precedence::Default);

    // [Value][Symbol]
    expr.prepend(&mut value);

    Statement::Variable {
        expr,
        flags,
    }
}

fn parse_expr_stmt(parser: &mut Parser) -> Statement {
    let mut expr = ArrayPageBuffer::<ExpressionNode>::new();
    parse_expr(parser, &mut expr, Precedence::Default);

    Statement::Expression { expr }
}
