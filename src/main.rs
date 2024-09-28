use std::{fs, io::Result};

mod lexer;
use lexer::*;
mod parser;
use parser::*;

fn main() -> Result<()> {
    let buffer = fs::read_to_string("./examples/1.crap")?;

    println!("\n--- tokens ---\n");
    let tokens = tokenize(&buffer);
    for token in tokens.iter() {
        println!("{:?}", &token);
    }

    println!("\n--- statements ---\n");
    let statements = parse(tokens);
    for statement in statements.iter() {
        println!("{:?}", &statement);
    }

    Ok(())
}
