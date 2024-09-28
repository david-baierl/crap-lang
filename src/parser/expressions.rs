use std::mem;

use crate::tokens::Token;

use super::{led_power, nud_power, Parser, Precedence};

// this type of expression is more like a linear node list
// childs are always followring its parrents
// this way it is even more memory efficiant
// this is in reverse order to speed up other processes
#[derive(Debug)]
pub enum Node {
    // literals [T]
    NumberLiteral(u32),

    // Unary [E][T] | [T][E] -> [E][T]
    PlusUnary(u32),
    MinusUnary(u32),

    // binary [L][T][R] -> [L][R][T]
    AdditionBinary(u32),
    SubstractionBinary(u32),
    MultiplicationBinary(u32),
    DivisionBinary(u32),
    ModuloBinary(u32),

    // ternary [L][T][M][T][R] -> [R][M][L][T]
    ConditionTernary(u32),
}

pub type Expression = Vec<Node>;

pub fn parse_expr(parser: &mut Parser, prev_power: Precedence) -> Expression {
    use Token::*;

    let mut expr: Expression = vec![];

    // --- nud --- //

    let mut token = parser.peek();

    while let Eol(_) = token {
        parser.next();
        token = parser.peek();
    }

    let nud_power = nud_power(&token);

    // nud handler
    match token {
        Number(_) => parse_literal_expr(parser, &mut expr),
        Plus(_) | Minus(_) => parse_unary_expr(parser, &mut expr, nud_power),
        OpenParen(_) => parse_block_expr(parser, &mut expr),

        t => panic!("nud handler: bad token: {:?}", t),
    };

    // --- led --- //
    loop {
        let operator = match parser.peek() {
            Eol(_) => {
                parser.next();
                continue;
            }
            Eof | Semi(_) => break,
            t => t,
        };

        let next_power = led_power(&operator, parser);

        if next_power <= prev_power {
            break;
        }

        // led handler
        match operator {
            Plus(_) | Minus(_) => parse_binary_expr(parser, &mut expr, next_power),
            Star(_) | Slash(_) | Percent(_) => parse_binary_expr(parser, &mut expr, next_power),
            Question(_) => parse_ternary_expr(parser, &mut expr, next_power),

            t => panic!("led handler: bad token: {:?}", t),
        }
    }

    expr
}

fn parse_literal_expr(parser: &mut Parser, expr: &mut Expression) {
    let literal = match parser.next() {
        Token::Number(index) => Node::NumberLiteral(index),

        t => panic!("literal: bad token: {:?}", t),
    };

    expr.push(literal);
}

fn parse_unary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let operator = match parser.next() {
        Token::Plus(index) => Node::PlusUnary(index),
        Token::Minus(index) => Node::MinusUnary(index),

        t => panic!("unary: bad token: {:?}", t),
    };

    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let operator = match parser.next() {
        Token::Plus(index) => Node::AdditionBinary(index),
        Token::Minus(index) => Node::SubstractionBinary(index),
        Token::Star(index) => Node::MultiplicationBinary(index),
        Token::Slash(index) => Node::DivisionBinary(index),
        Token::Percent(index) => Node::ModuloBinary(index),

        t => panic!("literal: bad token: {:?}", t),
    };
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_ternary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let token = parser.next();

    let operator = match token {
        Token::Question(index) => Node::ConditionTernary(index),

        t => panic!("literal: bad token: {:?}", t),
    };

    // [M]
    let mhs = parse_expr(parser, power.clone());

    // eat middle token
    match token {
        Token::Question(_) => parser.eat(Token::Colon(0)),

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

    // [R][M][L]+[T]
    expr.push(operator);
}

fn parse_block_expr(parser: &mut Parser, expr: &mut Expression) {
    let token = parser.next();
    let rhs = parse_expr(parser, Precedence::Default);

    match token {
        Token::OpenParen(_) => parser.eat(Token::CloseParen(0)),

        t => panic!("bad token: {:?}", t),
    };

    expr.extend(rhs);
}
