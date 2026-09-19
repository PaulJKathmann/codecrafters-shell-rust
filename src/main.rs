#[allow(unused_imports)]
use std::io::{self, Write};

enum BuiltinCommand {
    Exit,
    Echo { args: Vec<String> },
    Type { arg: String },
}

struct ExecutableCommand {
    path: String,
    args: Vec<String>,
}
enum Command {
    Builtin(BuiltinCommand),
    Executable(ExecutableCommand),
    Unknown {
        command: String,
    }
}

impl Command {

    fn execute(&self) {
        match self {
            Command::Builtin(command) => command.execute(),
            Command::Executable(command) => command.execute(),
            Command::Unknown { command } => println!("{} not found", command),
        }
    }
}


fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        let command = parse(&user_input.trim());
        command.execute();
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}

fn parse(input: &str) -> Command {
    let mut tokens= input.split_whitespace();
    let command = tokens.next().unwrap_or("");
    let args: Vec<String>  = tokens.map(|s| s.to_string()).collect();
    match (command, args) {
        ("exit", _) => Command::Builtin(BuiltinCommand::Exit),
        ("echo", args) => Command::Builtin(BuiltinCommand::Echo { args }),
        ("type", args) => Command::Builtin(BuiltinCommand::Type { arg: args.first().cloned().unwrap_or(String::new()) }),
        (command, _) => Command::Unknown { command: command.to_string() },
    }
}

impl BuiltinCommand {
    fn execute(&self) {
        match self {
            BuiltinCommand::Exit => exit(),
            BuiltinCommand::Echo { args } => echo(args.to_vec()),
            BuiltinCommand::Type { arg } => print_command_type(arg.to_string()),
        }
    }
}

impl ExecutableCommand {
    fn execute(&self) {
        // TODO: Make this actually execute a real program later
        println!("Executing {} {}", self.path, self.args.join(" "));
    }
}

fn print_command_type(command_string: String) {
    let command = parse(&command_string);
    match command {
        Command::Unknown { command: type_} => println!("{}: not found", type_),
        Command::Builtin(_)  => { println!("{} is a shell builtin",  command_string) },
        Command::Executable(_) => { println!("{} is a shell executable",  command_string) },
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
