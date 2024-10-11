use crate::utils::{
    bit_array::{Bit, BitArray, Byte, BIT_1},
    buffer::{Buffer, BufferReader}, u16vec::U16Vec,
};

use super::expressions::{debug_expr, ExpressionNode};

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
        expr: Buffer<ExpressionNode>, // [E]
        len: u16,
    },

    Variable {
        expr: Buffer<ExpressionNode>, // [E value][E symbol]
        len: u16,
        flags: Byte,
    },
}

// -------------------------------------------------
// vector trait
// -------------------------------------------------

impl Drop for Statement {
    fn drop(&mut self) {
        let size = self.len();

        unsafe {
            match self {
                Statement::Expression { expr, .. } => expr.drop(size),
                Statement::Variable { expr, .. } => expr.drop(size),
                // _ => ()
            }
        }
    }
}

impl BufferReader<ExpressionNode> for Statement {
    fn len(&self) -> usize {
        match self {
            Statement::Expression { len, .. } => len.clone().into(),
            Statement::Variable { len, .. } => len.clone().into(),
            // _ => 0,
        }
    }

    fn buf(&self) -> &Buffer<ExpressionNode> {
        match self {
            Statement::Expression { expr, .. } => expr,
            Statement::Variable { expr, .. } => expr,
            // _ => panic!("buffer access on non buffer variant"),
        }
    }
}

// -------------------------------------------------
// @DEBUG
// -------------------------------------------------

pub fn debug_stmt(stmt: &Statement) {
    match stmt {
        Statement::Expression { .. } => {
            println!("Expression Statement");
            debug_expr(stmt, &mut vec![1]);
        }
        Statement::Variable { flags, .. } => {
            print!("Variable Statement ");

            let is_const = StatementFlag::IsConst.has(*flags);

            if is_const {
                print!("(const)")
            } else {
                print!("(let)")
            }

            print!("\n");

            debug_expr(stmt, &mut vec![2]);
        }
    }
}
