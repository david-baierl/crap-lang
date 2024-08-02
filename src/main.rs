use std::{fs, io::Result};

mod lexer;

fn main() -> Result<()> {
    let buffer = fs::read_to_string("./examples/1.crap")?;

    lexer::tokenize(&buffer);

    Ok(())
}
