use std::{fs, io::Result};

mod ast;
mod lexer;
use lexer::*;
mod parser;
use parser::*;

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
        println!("{:?}", &statement);
    }

    Ok(())
}
