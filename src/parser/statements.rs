use std::fmt;

use crate::{ast::NodeKind, tokens::Token};

use super::{parse_expr, Expression, Parser, Precedence};

// #[derive(Debug)]
pub enum Statement {
    Block(Vec<Statement>),
    Expression(Expression),
}

impl fmt::Debug for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Expression(list) => {
                write!(f, "Expression")?;
                let mut i = list.len();
                let mut deph: Vec<isize> = vec![1];

                while i > 0 {
                    i -= 1;

                    let index = deph.len() - 1;
                    let node = list.get(i).unwrap();

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
                        NodeKind::Literal => {
                            // deph.push(0);
                        }
                        NodeKind::Prefix | NodeKind::Sufix | NodeKind::Block => {
                            deph.push(1);
                        }
                        NodeKind::Binary => {
                            deph.push(2);
                        }
                        NodeKind::Ternary => {
                            deph.push(3);
                        }
                    }

                    while let Some(0) = deph.last() {
                        deph.pop();
                    }

                    write!(f, "\n{}{:?}", indent, node)?;
                }
            }
            Statement::Block(list) => {
                write!(f, "Block")?;
                for stmt in list {
                    write!(f, "\n    {:?}", stmt)?;
                }
            }
        };

        write!(f, "\n")
    }
}

pub fn parse_stmt(parser: &mut Parser) -> Statement {
    let expr = parse_expr(parser, Precedence::Default);

    if let Token::Semi = parser.peek().token {
        parser.next();
    }

    // @TODO: check keywords first
    // Expression statements are a fallback
    Statement::Expression(expr)
}
