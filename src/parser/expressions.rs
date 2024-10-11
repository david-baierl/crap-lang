use std::mem;

use crate::{
    ast::expressions::{Expression, ExpressionKind, ExpressionNode},
    lexer::tokens::{Token, TokenNode},
    utils::{buffer::BufferWriter, u16vec::U16Vec},
};

use super::{
    precedence::{led_power, nud_power, Precedence},
    Parser,
};

pub fn parse_expr(parser: &mut Parser, prev_power: Precedence) -> Expression {
    use Token::*;

    let mut expr = U16Vec::new();

    // --- nud --- //

    let node = parser.peek();
    let nud_power = nud_power(&node);

    // nud handler
    match node.token {
        Number | Identifier => parse_literal_expr(parser, &mut expr),
        Plus | Minus => parse_prefix_expr(parser, &mut expr, nud_power),
        OpenParen => parse_block_expr(parser, &mut expr),

        _ => panic!("nud handler: bad token: {:?}", node),
    };

    // --- led --- //
    loop {
        let node = parser.peek();
        match node.token {
            Eof | Semi => break,
            _ => (),
        };

        let next_power = led_power(&node);

        if next_power <= prev_power {
            break;
        }

        // led handler
        match node.token {
            Plus | Minus => parse_binary_expr(parser, &mut expr, next_power),
            Star | Slash | Percent => parse_binary_expr(parser, &mut expr, next_power),
            Question => parse_ternary_expr(parser, &mut expr, next_power),

            _ => panic!("led handler: bad token: {:?}", node),
        }
    }

    expr
}

fn parse_literal_expr(parser: &mut Parser, expr: &mut Expression) {
    let TokenNode { index, token } = parser.next();

    let literal = ExpressionNode {
        kind: ExpressionKind::Literal,
        size: 1,
        index,
        token,
    };

    expr.append(literal);
}

fn parse_prefix_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, power);
    expr.concat(&rhs);

    let operator = ExpressionNode {
        kind: ExpressionKind::Prefix,
        size: expr.u16_len() + 1,
        index,
        token,
    };

    expr.append(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, power);

    expr.concat(&rhs);

    let operator = ExpressionNode {
        kind: ExpressionKind::Binary,
        size: expr.u16_len() + 1,
        index,
        token,
    };

    expr.append(operator);
}

fn parse_ternary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();

    // [M]
    let mhs = parse_expr(parser, power.clone());

    // eat middle token
    match token {
        Token::Question => parser.eat(Token::Colon),

        t => panic!("bad token: {:?}", t),
    };

    // [R]
    let mut rhs = parse_expr(parser, power.clone());

    // [R]+[M]
    rhs.concat(&mhs);

    // rhs: [L], expr: [R][M]
    mem::swap(expr, &mut rhs);

    // [R][M]+[L]
    expr.concat(&rhs);

    let operator = ExpressionNode {
        kind: ExpressionKind::Ternary,
        size: expr.u16_len() + 1,
        index,
        token,
    };

    // [R][M][L]+[T]
    expr.append(operator);
}

fn parse_block_expr(parser: &mut Parser, expr: &mut Expression) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, Precedence::Default);

    match token {
        Token::OpenParen => parser.eat(Token::CloseParen),

        t => panic!("bad token: {:?}", t),
    };

    expr.concat(&rhs);

    let operator = ExpressionNode {
        kind: ExpressionKind::Block,
        size: expr.u16_len() + 1,
        index,
        token,
    };

    expr.append(operator);
}
