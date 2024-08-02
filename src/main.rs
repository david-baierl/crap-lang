use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();

    match File::open("./debug/1.crap") {
        Ok(mut file) => {
            let _ = file.read_to_string(&mut buffer);
        }
        _ => {}
    }
    
    println!("{}", buffer);
}