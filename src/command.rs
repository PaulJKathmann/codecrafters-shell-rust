use crate::builtin::BuiltinCommand;
use crate::executable::ExecutableCommand;

pub enum Command {
    Builtin(BuiltinCommand),
    Executable(ExecutableCommand),
    Unknown { command: String },
}

impl Command {
    pub fn execute(&self) {
        match self {
            Command::Builtin(command) => command.execute(),
            Command::Executable(command) => command.execute(),
            Command::Unknown { command } => println!("{}: not found", command),
        }
    }
}
