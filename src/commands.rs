use crate::internal;

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
