use crate::{
    ast::expressions::{ExpressionKind, ExpressionNode, Expression},
    lexer::tokens::{Token, TokenNode}, utils::array_page_buffer::ArrayPageBuffer,
};

use super::{
    precedence::{led_power, nud_power, Precedence},
    Parser,
};

pub fn parse_expr(parser: &mut Parser, expr: &mut Expression, prev_power: Precedence) {
    use Token::*;

    // let mut expr: MutExpression = ArrayPageBuffer::<ExpressionNode>::new();

    // --- nud --- //

    let node = parser.peek();
    let nud_power = nud_power(&node);

    // nud handler
    match node.token {
        Number | Identifier => parse_literal_expr(parser, expr),
        Plus | Minus => parse_prefix_expr(parser, expr, nud_power),
        OpenParen => parse_block_expr(parser, expr),

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
            Plus | Minus => parse_binary_expr(parser, expr, next_power),
            Star | Slash | Percent => parse_binary_expr(parser, expr, next_power),
            Question => parse_ternary_expr(parser, expr, next_power),

            _ => panic!("led handler: bad token: {:?}", node),
        }
    }
}

fn parse_literal_expr(parser: &mut Parser, expr: &mut Expression) {
    let TokenNode { index, token } = parser.next();
    let literal = ExpressionNode::new(index, 1, token, ExpressionKind::Literal);

    expr.push(literal);
}

fn parse_prefix_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    parse_expr(parser, expr, power);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Prefix);
    expr.push(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    parse_expr(parser, expr, power);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Binary);
    expr.push(operator);
}

fn parse_ternary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let TokenNode { index, token } = parser.next();
    let mut tmp = ArrayPageBuffer::<ExpressionNode>::new();

    parse_expr(parser, &mut tmp, power.clone());
    expr.prepend(&mut tmp);

    // eat middle token
    match token {
        Token::Question => parser.eat(Token::Colon),

        t => panic!("bad token: {:?}", t),
    };

    parse_expr(parser, &mut tmp, power.clone());
    expr.prepend(&mut tmp);

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Ternary);

    // [R][M][L]+[T]
    expr.push(operator);
}

fn parse_block_expr(parser: &mut Parser, expr: &mut Expression) {
    let TokenNode { index, token } = parser.next();
    parse_expr(parser, expr, Precedence::Default);

    match token {
        Token::OpenParen => parser.eat(Token::CloseParen),

        t => panic!("bad token: {:?}", t),
    };

    let operator = ExpressionNode::new(index, expr.len() + 1, token, ExpressionKind::Block);

    expr.push(operator);
}
