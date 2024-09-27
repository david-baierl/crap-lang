use crate::tokens::Token;

// this type of expression is more like a linear node list
// childs are always followring its parrents
// this way it is even more memory efficiant
// this is in reverse order to speed up other processes

#[derive(Debug)]
pub enum Expression {
    // [T]
    // eg: true
    Literal(Token),

    // led | nud
    // [E][T] | [T][E] -> [E] [T]
    // eg: ! true
    Unary(Token),

    // led
    // [L][T][R] -> [L][R] [T]
    // eg: true || false
    Binary(Token),

    // led
    // needs two in a row
    // [L][T][C][T][R] -> [L][C][R] [T]
    // eg: true ? true : false
    Ternary(Token),
}

// impl Expression {
//     pub fn literal(token: Token) -> Expression {
//         Expression::Literal(token)
//     }

//     pub fn unary(token: Token, expr: Expression) -> [Expression; 2] {
//         [expr, Expression::Unary(token)]
//     }

//     pub fn binary(token: Token, left: Expression, right: Expression) -> [Expression; 3] {
//         [left, right, Expression::Binary(token)]
//     }

//     pub fn ternary(
//         first: Token,
//         second: Token,
//         left: Expression,
//         center: Expression,
//         right: Expression,
//     ) -> [Expression; 5] {
//         [
//             left,
//             center,
//             right,
//             Expression::Ternary(second),
//             Expression::Ternary(first),
//         ]
//     }
// }
