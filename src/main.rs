use std::{fs, io::Result};

mod lexer;
use lexer::tokenize;

fn main() -> Result<()> {
    let buffer = fs::read_to_string("./examples/1.crap")?;

    tokenize(&buffer);

    Ok(())
}
