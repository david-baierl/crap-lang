use core::fmt;

use crate::{lexer::tokens::Token, utils::array_page_buffer::ArrayPageBuffer};

#[derive(Debug)]
pub enum ExpressionKind {
    // --- legend --- //
    // [E]: expression
    // [L]: left hand side expression
    // [M]: middle hand side expression
    // [R]: right hand side expression
    // [T]: token

    // --- literal --- //
    Literal, // [T]

    // --- unary --- //
    Prefix, // [E][T] -> [E][T]
    // Sufix,  // [T][E] -> [E][T]
    Block,  // [T][E][T] -> [E][T]

    // --- binary --- //
    Binary, // [L][T][R] -> [L][R][T]

    // --- ternary --- //

    // @TODO: check if this is the best storing method
    Ternary, // [L][T][M][T][R] -> [R][M][L][T]
}

pub struct ExpressionNode {
    pub index: u32,
    // pub size: u16,
    pub token: Token,
    pub kind: ExpressionKind,
}

impl ExpressionNode {
    pub fn new(index: u32, size: usize, token: Token, kind: ExpressionKind) -> ExpressionNode {
        if size > u16::MAX.into() {
            panic!("max expression size reached");
        }

        ExpressionNode {
            index,
            // size: size.try_into().unwrap(),
            token,
            kind,
        }
    }
}

impl fmt::Debug for ExpressionNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} i:{}", self.kind, self.token, self.index)
    }
}

// drop capacity field
pub type Expression = ArrayPageBuffer<ExpressionNode>;

pub fn debug_expr(expr: &Expression, deph: &mut Vec<isize>) {
    use ExpressionKind::*;

    let mut i = expr.len();

    while i > 0 {
        i -= 1;

        let index = deph.len() - 1;
        let node = expr.get(i).unwrap();

        let mut indent = String::new();
        if deph.len() > 0 {
            for i in 0..(deph.len() - 1) {
                if let Some(0) = deph.get(i) {
                    indent.push_str("    ");
                } else {
                    indent.push_str("  │ ");
                }
            }

            if let Some(1) = deph.last() {
                indent.push_str("  └─");
            } else {
                indent.push_str("  ├─");
            }
        }

        if let Some(_) = deph.get(index) {
            deph[index] = deph[index] - 1;
        }

        match node.kind {
            Ternary => {
                deph.push(3);
            }
            Binary => {
                deph.push(2);
            }
            Literal => {
                // deph.push(0);
            }
            _ => {
                deph.push(1);
            }
        }

        while let Some(0) = deph.last() {
            deph.pop();
        }

        println!("{}{:?}", indent, node);
    }
}
