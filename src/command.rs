use crate::builtin::BuiltinCommand;
use crate::executable::ExecutableCommand;

pub enum ResolvedCommand {
    Builtin(BuiltinCommand),
    Executable(ExecutableCommand),
    Unknown { command: String },
}

impl ResolvedCommand {
    pub fn execute(&self) {
        match self {
            ResolvedCommand::Builtin(command) => command.execute(),
            ResolvedCommand::Executable(command) => command.execute(),
            ResolvedCommand::Unknown { command } => println!("{}: not found", command),
        }
    }
}
