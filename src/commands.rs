use std::process::{Command, Stdio};

use crate::internal;

// Not really a command, but it works as a call to execute an external executable
pub fn handle_exec(args: Vec<String>) {
    if let Some(executable) = args.get(0) {
        if let Ok(path) = internal::find_executable(executable) {
            let int_args = args
                .iter()
                .skip(1)
                .map(|a| a.as_str())
                .collect::<Vec<&str>>();

            let output = Command::new(path)
                .args(int_args)
                .stdout(Stdio::piped())
                .output();

            match output {
                Ok(output) => {
                    let to_print = String::from_utf8_lossy(&output.stdout);
                    print!("{}", to_print);
                }
                Err(e) => println!("lsh: error while executing \"{}\"", e)
            }
        } else {
            println!("{}: command not found", executable)
        }
    }
}

// Type command, checks whether a given command is a builtin or an exec.
pub fn handle_type(args: Vec<String>) {
    let built_in = ["echo", "exit", "type"];
    
    if let Some(command) = args.get(1) {
        let found = built_in
            .iter()
            .position(|&cmd| cmd == command.as_str())
            .is_some();

        if found {
            println!("{} is a shell builtin", command);
        } else if let Ok(dir) = internal::find_executable(command) {
            println!("{} is {}", command, dir);            
        } else {
            println!("{}: not found", command);
        }
    } else {
        println!("type: expected a command");
    }
}

// Pretty simple function, it just exits with the code given.
pub fn handle_exit(args: Vec<String>) {
    if let Some(exit_code) = args.get(1) {
        match exit_code.parse::<i32>() {
            Ok(code) => std::process::exit(code),
            Err(_) => println!("exit: unexpected exit code"),
        }
    } else {
        std::process::exit(0);
    }
}

// The echo command, it prints anything typed inside it.
pub fn handle_echo(args: Vec<String>) {
    let message: String = args
        .iter()
        .skip(1)
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    println!("{}", message);
}
