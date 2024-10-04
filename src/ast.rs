use crate::tokens::Token;

#[derive(Debug)]
pub enum NodeKind {
    // --- literal --- //

    Literal, // [T]

    // --- unary --- //
    
    Prefix, // [E][T] -> [E][T]
    Sufix, // [T][E] -> [E][T]
    Block, // [T][E][T] -> [E][T]

    // --- binary --- //

    Binary, // [L][T][R] -> [L][R][T]

    // --- ternary --- //

    // @TODO: check if this is the best storying method
    Ternary, // [L][T][M][T][R] -> [R][M][L][T]

    // --- legend --- //

    // [E]: expression
    // [L]: left hand side expression
    // [M]: middle hand side expression
    // [R]: right hand side expression
    // [T]: token
}

#[derive(Debug)]
pub struct Node {
    pub index: u32,
    pub size: u16,
    pub token: Token,
    pub kind: NodeKind,
}

impl Node {
    pub fn new(index: u32, size: u16, token: Token, kind: NodeKind) -> Node {
        Node {
            index,
            size,
            token,
            kind,
        }
    }

    pub fn new_literal(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Literal)
    }

    pub fn new_prefix(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Prefix)
    }

    pub fn new_sufix(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Sufix)
    }

    pub fn new_block(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Block)
    }

    pub fn new_binary(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Binary)
    }

    pub fn new_ternary(index: u32, size: u16, token: Token) -> Node {
        Node::new(index, size, token, NodeKind::Ternary)
    }
}
