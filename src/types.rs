pub enum BuiltinCommand {
    Exit,
    Echo { args: Vec<String> },
    Type { arg: String },
}

pub struct ExecutableCommand {
    path: String,
    args: Vec<String>,
}
pub enum Command {
    Builtin(BuiltinCommand),
    Executable(ExecutableCommand),
    Unknown {
        command: String,
    }
}

