use std::fmt;

use super::{parse_expr, Expression, Parser, Precedence};

// #[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Vec<Expression>),
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ident: isize = 0;

        match self {
            Statement::Expression(list) => {
                let mut i = list.len();
                let mut deph: Vec<isize> = vec![1];

                while i > 0 {
                    i -= 1;

                    let index = deph.len() - 1;
                    let expr = list.get(i).unwrap();
                    let indent: String = vec!["  "; deph.len()].join("");

                    if let Some(_) = deph.get(index) {
                        deph[index] = deph[index] - 1;
                    }

                    match expr {
                        Expression::Literal(_) => {
                            // deph.push(0);
                        },
                        Expression::Unary(_) => {
                            deph.push(1);
                        },
                        Expression::Binary(_) => {
                            deph.push(2);
                        },
                        Expression::Ternary(_) => {
                            deph.push(3);
                        },
                    }

                    while let Some(0) = deph.last() {
                        deph.pop();
                    }
                    
                    write!(f, "\n{}{:?}", indent, expr)?;
                }
            },
            Statement::Block(list) => {
                for stmt in list {
                    write!(f, "\n{:?}", stmt)?;
                }
            },
        };

        write!(f, "\n")
    }
}

pub fn parse_stmt(parser: &mut Parser) -> Statement {
    let expr = parse_expr(parser, Precedence::Default);

    // @TODO: check keywords first
    // Expression statements are a fallback
    Statement::Expression(expr)
}
