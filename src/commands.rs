use std::path::Path;

use crate::{internal::{self, CommandContext}, pathutils};

// Type command, checks whether a given command is a builtin or an exec.
pub fn handle_type(ctx: &mut CommandContext)-> i32 {
    let cmd;
    match ctx.get_args().get(0) {
        Some(c) => cmd = c.to_string(),
        None => {
            _ = writeln!(ctx.get_writer(), "type: expected command argument");
            return 1;
        }
    }

    match ctx.get_env().find_command(&cmd) {
        internal::CommandType::BuiltIn(name) => {
            _ = writeln!(ctx.get_writer(), "{} is a shell builtin", name);
            0
        }
        internal::CommandType::Executable(path) => {
            _ = writeln!(ctx.get_writer(), "{} is {}", cmd, path);
            0
        }
        internal::CommandType::Unknown => {
            _ = writeln!(ctx.get_writer(), "{}: not found", cmd);
            1
        }
    }
}

// Prints the current directory
pub fn handle_pwd(ctx: &mut CommandContext) -> i32 {
    let path = ctx.get_env().pwd().to_string_lossy().to_string();
    _ = writeln!(ctx.get_writer(), "{}", path);

    0
}

// Changes the current directory
pub fn handle_cd(ctx: &mut CommandContext) -> i32 {
    let mut directory;
    match ctx.get_args().get(0) {
        Some(d) => directory = d.clone(),
        None => {
            _ = writeln!(ctx.get_writer(), "cd: expected directory argument");
            return 1;
        }
    }

    let mut path;
    match directory.chars().nth(0) { 
        Some('/') => path = Path::new(&directory).to_path_buf(),
        Some('~') => {
            if let Ok(home_path) = std::env::var("HOME") {
                directory = directory.trim_start_matches("~").to_string();
                directory.insert_str(0, &home_path); // push front home_path
            }
            
            path = Path::new(&directory).to_path_buf();
        },
        _ => {
            let current = ctx.get_env().pwd();
            let not_sanitized = current.join(&directory);
            path = Path::new(&not_sanitized).to_path_buf();
        },
    }

    path = pathutils::sanitize(&path);

    if path.exists() {
        ctx.get_env().cd(path.to_path_buf());
        0
    } else {
        _ = writeln!(ctx.get_writer(), "cd: {}: No such file or directory", directory);
        1
    }
}

// The echo command, it prints anything typed inside it.
pub fn handle_echo(ctx: &mut CommandContext) -> i32 {
    let message: String = ctx.get_args()
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<&str>>()
        .join(" ");

    _ = writeln!(ctx.get_writer(), "{}", message);

    0
}

pub fn handle_exit(ctx: &mut CommandContext) -> i32 {
    if let Some(exit_code) = ctx.get_args().get(0) {
        match exit_code.parse::<i32>() {
            Ok(code) => std::process::exit(code),
            Err(_) => { _ = writeln!(ctx.get_writer(), "exit: unexpected argument for exit code"); }
        };
    } else {
        std::process::exit(0);
    };

    0
}

