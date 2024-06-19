use crate::internal::{self, CommandContext};

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
            _ = writeln!(ctx.get_writer(), "{} is {}", cmd, name);
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

