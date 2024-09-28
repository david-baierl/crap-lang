use std::mem;

use crate::tokens::Token;

use super::{led_power, nud_power, Parser, Precedence};

// this type of expression is more like a linear node list
// childs are always followring its parrents
// this way it is even more memory efficiant
// this is in reverse order to speed up other processes
#[derive(Debug)]
pub enum Node {
    // [T]
    Literal(Token),

    // [E][T] | [T][E] -> [E][T]
    Unary(Token),

    // [L][T][R] -> [L][R][T]
    Binary(Token),

    // [L][T][M][T][R] -> [R][M][L][T]
    Ternary(Token),
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
        Number(_, _) => parse_literal_expr(parser, &mut expr),
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
    let literal = Node::Literal(parser.next());

    expr.push(literal);
}

fn parse_unary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let operator = Node::Unary(parser.next());
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let operator = Node::Binary(parser.next());
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_ternary_expr(parser: &mut Parser, expr: &mut Expression, power: Precedence) {
    let token = parser.next();
    let operator = Node::Ternary(token.clone());

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
