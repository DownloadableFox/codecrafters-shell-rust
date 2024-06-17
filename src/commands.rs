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
