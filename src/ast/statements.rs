use crate::utils::{array_page_buffer::ArrayPageBuffer, bit_array::{Bit, BitArray, Byte, BIT_1}};

use super::expressions::{debug_expr, Expression, ExpressionNode};

pub enum StatementFlag {
    IsConst,
}

impl BitArray for StatementFlag {
    fn bit(&self) -> Bit {
        match self {
            StatementFlag::IsConst => BIT_1,
        }
    }
}

pub enum Statement {
    // --- legend --- //
    // [E]: expression

    // --- variants --- //

    // Block {
    //     stmts: Vec<Statement>,
    // },

    Expression {
        expr: ArrayPageBuffer<ExpressionNode>, // [E]
    },

    Variable {
        expr: Expression, // [E value][E symbol]
        flags: Byte,
    },
}

pub fn debug_stmt(stmt: &Statement) {
    use Statement::*;

    match stmt {
        Expression { expr } => {
            println!("Expression Statement");
            debug_expr(expr, &mut vec![1]);
        }
        Variable { expr, flags } => {
            print!("Variable Statement ");

            let is_const = StatementFlag::IsConst.has(*flags);

            if is_const {
                print!("(const)")
            } else {
                print!("(let)")
            }

            print!("\n");

            debug_expr(expr, &mut vec![2]);
        }
    }
}
