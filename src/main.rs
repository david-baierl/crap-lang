use std::{fs, io::Result};

mod ast;
mod lexer;
use lexer::*;
mod parser;
use parser::*;

fn main() -> Result<()> {
    let buffer = fs::read_to_string("./examples/1.crap")?;
    let tokens = tokenize(&buffer);
    let statements = parse(tokens);

    for statement in statements.iter() {
        println!("{:?}", &statement);
    }

    Ok(())
}
