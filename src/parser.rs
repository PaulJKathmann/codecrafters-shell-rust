use std::{env, path::PathBuf};

use crate::builtin::BuiltinCommand;
use crate::command::Command;
use crate::executable::{get_executable_path, ExecutableCommand};

pub fn parse(input: &str) -> Command {
    let mut tokens = input.split_whitespace();
    let command = tokens.next().unwrap_or("");
    let args: Vec<String> = tokens.map(|s| s.to_string()).collect();
    match (command, args) {
        ("exit", _) => Command::Builtin(BuiltinCommand::Exit),
        ("echo", args) => Command::Builtin(BuiltinCommand::Echo { args }),
        ("pwd", _) => Command::Builtin(BuiltinCommand::Pwd),
        ("cd", args) => parse_cd(&args),
        ("type", args) => Command::Builtin(BuiltinCommand::Type {
            arg: args.first().cloned().unwrap_or_default(),
        }),
        (command, args) => parse_executable(command, args),
    }
}

fn parse_cd(args: &Vec<String>) -> Command {
    let new_dir_str = args.first();
    let home_dir = PathBuf::from(env::var("HOME").unwrap());
    match new_dir_str {
        Some(new_dir) if *new_dir == "~" => Command::Builtin(BuiltinCommand::Cd { new_dir: home_dir }),
        Some(new_dir) => Command::Builtin(BuiltinCommand::Cd { new_dir: PathBuf::from(new_dir) }),
        None => Command::Builtin(BuiltinCommand::Cd { new_dir: home_dir }),
    }
}

fn parse_executable(command: &str, args: Vec<String>) -> Command {
    if let Some(path) = get_executable_path(command) {
        Command::Executable(ExecutableCommand {
            name: command.to_string(),
            path: path.to_string_lossy().to_string(),
            args,
        })
    } else {
        Command::Unknown {
            command: command.to_string(),
        }
    }
}

