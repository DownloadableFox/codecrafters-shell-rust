use std::{collections::HashMap, fs, path::PathBuf, process::{Command, Stdio}};

pub struct CommandContext<'a> {
    shell_env: &'a mut ShellEnvironment,
    args: &'a [String],
    writer: &'a mut dyn std::io::Write,
}

impl<'a> CommandContext<'a> {
    pub fn new(shell_env: &'a mut ShellEnvironment, args: &'a [String], writer: &'a mut dyn std::io::Write) -> CommandContext<'a> {
        CommandContext {
            shell_env,
            args,
            writer
        }
    }

    pub fn get_args(&self) -> &[String] {
        self.args 
    }

    pub fn get_writer(&mut self) -> &mut dyn std::io::Write {
        self.writer
    }

    pub fn get_env(&mut self) -> &mut ShellEnvironment {
        self.shell_env
    }
}

pub type ShellCommand = fn(&mut CommandContext) -> i32;

pub struct ShellEnvironment {
    working_dir: PathBuf,
    built_in: HashMap<&'static str, ShellCommand>
}

pub enum CommandType {
    BuiltIn(String),
    Executable(String),
    Unknown
}

#[allow(dead_code)]
impl ShellEnvironment {
    pub fn new(working_dir: PathBuf) -> ShellEnvironment {
        ShellEnvironment {
            working_dir,
            built_in: HashMap::new(),
        }
    }

    pub fn register_command(&mut self, name: &'static str, func: ShellCommand) {
        self.built_in.insert(name, func);
    }

    pub fn cd(&mut self, dir: PathBuf) {
        self.working_dir = dir;
    }

    pub fn pwd(&self) -> PathBuf {
        self.working_dir.clone()
    }

    pub fn execute(&mut self, name: &str, args: &[String], writer: &mut dyn std::io::Write) -> i32 {
        if let Some(&builtin) = self.built_in.get(name) {
            let mut context = CommandContext::new(self, &args, writer);
            builtin(&mut context)
        } else if let Ok(exec) = find_executable_in_path(name) {
            execute(&exec, args, writer);
            0
        } else if let Ok(exec) = find_executable_in_dir(&self.working_dir.to_string_lossy(), name) { 
            execute(&exec, args, writer);
            0
        } else {
            _ = writeln!(writer, "{}: not found", name);
            1
        }
    }

    pub fn find_command(&self, name: &str) -> CommandType {
        let is_registered = self.get_registered()
            .iter()
            .position(|cmd| cmd == name)
            .is_some();

        if is_registered {
            CommandType::BuiltIn(name.to_string()) 
        } else if let Ok(path) = find_executable_in_path(name) {
            CommandType::Executable(path.to_string_lossy().to_string())
        } else if let Ok(path) = find_executable_in_dir(&self.working_dir.to_string_lossy(), name) {
            CommandType::Executable(path.to_string_lossy().to_string())   
        } else {
            CommandType::Unknown
        }
    }

    pub fn get_registered(&self) -> Vec<String> {
        self.built_in
            .keys()
            .into_iter()
            .map(|k| k.to_string())
            .collect::<Vec<String>>()
    }
}

#[allow(dead_code)]
pub enum InternalError {
    ExecutableNotFound,
    UnexpectedState(&'static str)
}

pub fn find_executable_in_path(name: &str) -> Result<PathBuf, InternalError> {
    if let Ok(path) = std::env::var("PATH") {
        let dirs: Vec<&str> = path.split(':').collect();
        
        for dir in dirs { 
            if let Ok(path) = find_executable_in_dir(dir, name) {
                return Ok(path);
            }
        } 

        Err(InternalError::ExecutableNotFound)
    } else {
        Err(InternalError::UnexpectedState("PATH is not set in env"))
    }
}

pub fn find_executable_in_dir(dir: &str, name: &str) -> Result<PathBuf, InternalError> {
    if let Ok(files) = fs::read_dir(dir) {
        for file in files.flatten() {
            if file.path().is_dir() {
                continue;
            }

            if file.file_name().to_string_lossy() == name {
                return Ok(file.path());
            }
        }

        Err(InternalError::ExecutableNotFound)
    } else {
        Err(InternalError::UnexpectedState("failed to read directory"))
    }
}

pub fn execute(executable: &PathBuf, args: &[String], writer: &mut dyn std::io::Write)  {
    let path = executable.clone().to_string_lossy().to_string();
    let output = Command::new(path)
        .args(args)
        .stdout(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let to_print = String::from_utf8_lossy(&output.stdout);
            _ = write!(writer, "{}", to_print);
        }
        Err(e) => println!("lsh: error while executing \"{}\"", e)
    }
}


