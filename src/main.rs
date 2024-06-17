#[allow(unused_imports)]
use std::io::{self, Write};

fn prompt() -> Result<String, std::io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush()?;
    
    stdin.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn handle_command(input: String) {
    println!("{}: command not found", input);
}

fn main() {
    loop {
        match prompt() {
            Ok(input) => handle_command(input),
            Err(e) => println!("lsh: an error occurred reading prompt \"{}\"", e),
        }
    }
}
