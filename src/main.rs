mod commands;
mod internal;

use std::io::{self, Write};

use internal::{ShellCommand, ShellEnvironment};

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

fn handle_command(shell: &mut ShellEnvironment, args: Vec<String>) {
    let name; 
    match args.first() {
        Some(s) => name = s.clone(),
        None => return,
    }

    let cargs = args.iter()
        .skip(1)
        .map(|s| s.clone())
        .collect::<Vec<String>>();
   
    shell.execute(&name, cargs.as_slice(), &mut std::io::stdout());
}

fn initialize_shell(shell: &mut ShellEnvironment) {
    shell.register_command("exit", commands::handle_exit as ShellCommand);
    shell.register_command("echo", commands::handle_echo as ShellCommand);
    shell.register_command("type", commands::handle_type as ShellCommand);
    shell.register_command("pwd", commands::handle_pwd as ShellCommand);
}

fn main() {
    // Create and initialize shell
    let current_dir = std::env::current_dir().unwrap();
    let mut shell = ShellEnvironment::new(current_dir);

    initialize_shell(&mut shell);

    loop {
        match prompt() {
            Ok(input) => {
                let args = parse_args(input);

                match args {
                    Ok(args) => handle_command(&mut shell, args),
                    Err(e) => println!("lsh: failed to parse input \"{}\"", e),
                }
            }
            Err(e) => println!("lsh: an error occurred reading prompt \"{}\"", e),
        }
    }
}
