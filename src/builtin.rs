use std::{env, io, path::PathBuf};

use crate::command::ResolvedCommand;
use crate::executable::ExecutableCommand;
use crate::parser::parse;

pub enum BuiltinCommand {
    Exit,
    Echo { args: Vec<String> },
    Type { arg: String },
    Pwd,
    Cd { new_dir: PathBuf },
}

impl BuiltinCommand {
    pub fn execute(&self) {
        match self {
            BuiltinCommand::Exit => exit(),
            BuiltinCommand::Echo { args } => echo(args.to_vec()),
            BuiltinCommand::Type { arg } => print_command_type(arg.to_string()),
            BuiltinCommand::Pwd => pwd(),
            BuiltinCommand::Cd { new_dir } => cd(new_dir),
        }
    }
}

fn echo(args: Vec<String>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

fn exit() {
    std::process::exit(0);
}

fn pwd() {
    let cwd: Result<PathBuf, io::Error> = env::current_dir();
    match cwd {
        Ok(path) => println!("{}", path.display()),
        Err(error) => println!("{}", error),
    }
}

fn cd(new_dir: &PathBuf) {
    let result = env::set_current_dir(new_dir);
    match result {
        Ok(_) => {}
        Err(_) => println!("cd: {}: No such file or directory", new_dir.display()),
    }
}

fn print_command_type(command_string: String) {
    let command = parse(&command_string);

    match command {
        ResolvedCommand::Unknown { command: type_ } => println!("{}: not found", type_),
        ResolvedCommand::Builtin(_) => println!("{} is a shell builtin", command_string),
        ResolvedCommand::Executable(ExecutableCommand { path, .. }) => {
            println!("{} is {}", command_string, path)
        }
    }
}
