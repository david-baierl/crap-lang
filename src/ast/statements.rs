use super::expressions::{debug_expr, Expression};

pub enum Statement {
    // --- legend --- //
    // [E]: expression

    // --- variants --- //

    // Block {
    //     stmts: Vec<Statement>,
    // },

    Expression {
        expr: Expression, // [E]
    },

    Variable {
        expr: Expression, // [E value][E symbol]
        is_const: bool,
    },
}

pub fn debug_stmt(stmt: &Statement) {
    use Statement::*;

    match stmt {
        Expression { expr } => {
            println!("Expression Statement");
            debug_expr(expr, &mut vec![1]);
        }
        Variable { expr, is_const } => {
            print!("Variable Statement ");

            if *is_const {
                print!("(const)")
            } else {
                print!("(let)")
            }

            print!("\n");

            debug_expr(expr, &mut vec![2]);
        }
    }
}
