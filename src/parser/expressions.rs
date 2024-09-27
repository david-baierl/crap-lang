use crate::tokens::Token;

use super::{led_power, nud_power, Parser, Precedence};

// this type of expression is more like a linear node list
// childs are always followring its parrents
// this way it is even more memory efficiant
// this is in reverse order to speed up other processes
#[derive(Debug)]
pub enum Expression {
    // [T]
    Literal(Token),

    // [E][T] | [T][E] -> [E] [T]
    Unary(Token),

    // [L][T][R] -> [L][R] [T]
    Binary(Token),

    // [L][T][C][T][R] -> [R][C][L] [T]
    Ternary(Token),
}

pub fn parse_expr(parser: &mut Parser, prev_power: Precedence) -> Vec<Expression> {
    use Token::*;

    let mut expr: Vec<Expression> = vec![];

    // --- nud --- //

    let token = parser.peek();
    let nud_power = nud_power(&token);

    // nud handler
    match token {
        Number(_, _) => parse_literal_expr(parser, &mut expr),
        Plus(_) | Minus(_) => parse_unary_expr(parser, &mut expr, nud_power),
        OpenParen(_) => parse_block_expr(parser, &mut expr),

        t => panic!("bad token: {:?}", t),
    };

    // --- led --- //
    loop {
        let operator = match parser.peek() {
            Eof => break,
            t => t,
        };

        let next_power = led_power(&operator);

        if next_power <= prev_power {
            break;
        }

        // led handler
        match operator {
            Plus(_) | Minus(_) => parse_binary_expr(parser, &mut expr, next_power),
            Star(_) | Slash(_) | Percent(_) => parse_binary_expr(parser, &mut expr, next_power),

            t => panic!("bad token: {:?}", t),
        }
    }

    expr
}

fn parse_literal_expr(parser: &mut Parser, expr: &mut Vec<Expression>) {
    let literal = Expression::Literal(parser.next());

    expr.push(literal);
}

fn parse_unary_expr(parser: &mut Parser, expr: &mut Vec<Expression>, power: Precedence) {
    let operator = Expression::Unary(parser.next());
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_binary_expr(parser: &mut Parser, expr: &mut Vec<Expression>, power: Precedence) {
    let operator = Expression::Binary(parser.next());
    let rhs = parse_expr(parser, power);

    expr.extend(rhs);
    expr.push(operator);
}

fn parse_block_expr(parser: &mut Parser, expr: &mut Vec<Expression>) {
    let rhs = parse_expr(parser, Precedence::Default);

    expr.extend(rhs);
    parser.next();
}
