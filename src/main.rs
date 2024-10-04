use std::{fs, io::Result};

use ast::statements::debug_stmt;
use lexer::tokenize;
use parser::parse;

mod ast;
mod lexer;
mod parser;
mod utils;

fn main() -> Result<()> {
    let buffer = fs::read_to_string("./examples/1.crap")?;

    let tokens = tokenize(&buffer);
    // println!("\n--- tokens ---\n");
    // for token in tokens.iter() {
    //     println!("{:?}", &token);
    // }

    let statements = parse(tokens);
    println!("\n--- statements ---\n");
    for statement in statements.iter() {
        debug_stmt(statement);
        print!("\n");
    }

    Ok(())
}
