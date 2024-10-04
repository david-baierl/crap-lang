use super::expressions::{debug_expr, Expression};

pub enum Statement {
    Expression { expr: Expression },
}

pub fn debug_stmt(stmt: &Statement) {
    use Statement::*;

    match stmt {
        Expression { expr } => {
            println!("Expression");
            debug_expr(expr);
        }
    }
}
