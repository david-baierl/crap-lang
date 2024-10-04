use std::mem;

use crate::{
    ast::expressions::{Expression, ExpressionKind, ExpressionNode},
    lexer::tokens::{Token, TokenNode},
};

use super::{
    precedence::{led_power, nud_power, Precedence},
    Parser,
};

pub fn parse_expr(parser: &mut Parser, prev_power: Precedence) -> Expression {
    use Token::*;

    let mut expr: Expression = vec![];

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
    let literal = ExpressionNode::new(index, 1, token, ExpressionKind::Literal);

    expr.push(literal);
}

fn parse_prefix_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, power);
    expr.extend(rhs);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Prefix);
    expr.push(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Binary);
    expr.push(operator);
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
    rhs.extend(mhs);

    // rhs: [L], expr: [R][M]
    mem::swap(expr, &mut rhs);

    // [R][M]+[L]
    expr.extend(rhs);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Ternary);

    // [R][M][L]+[T]
    expr.push(operator);
}

fn parse_block_expr(parser: &mut Parser, expr: &mut Expression) {
    let TokenNode { index, token } = parser.next();
    let rhs = parse_expr(parser, Precedence::Default);

    match token {
        Token::OpenParen => parser.eat(Token::CloseParen),

        t => panic!("bad token: {:?}", t),
    };

    expr.extend(rhs);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Block);

    expr.push(operator);
}
