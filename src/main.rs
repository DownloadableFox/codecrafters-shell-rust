mod commands;

use std::io::{self, Write};

fn prompt() -> Result<String, std::io::Error> {
    let stdin = io::stdin();
    let mut input = String::new();

    print!("$ ");
    io::stdout().flush()?;

    stdin.read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_args(input: String) -> Result<Vec<String>, std::io::Error> {
    // The ideal scenario here is to properly implement a parser,
    // something that I am not going to do just now. So a string split will do.
    let args = input.split(' ').map(|arg| arg.to_string()).collect();

    Ok(args)
}

fn handle_command(args: Vec<String>) {
    if let Some(command) = args.first() {
        match command.as_str() {
            "exit" => commands::handle_exit(args), 
            other => println!("{}: command not found", other),
        }
    }
}

fn main() {
    loop {
        match prompt() {
            Ok(input) => {
                let args = parse_args(input);

                match args {
                    Ok(args) => handle_command(args),
                    Err(e) => println!("lsh: failed to parse input \"{}\"", e),
                }
            }
            Err(e) => println!("lsh: an error occurred reading prompt \"{}\"", e),
        }
    }
}
