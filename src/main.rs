#[allow(unused_imports)]
use std::env;
use std::{
    io::{self, Write}, 
    os::unix::fs::PermissionsExt, 
    path::PathBuf,
    process};

/** Types ************************/
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
            Command::Unknown { command } => println!("{}: not found", command),
        }
    }
}

enum BuiltinCommand {
    Exit,
    Echo { args: Vec<String> },
    Type { arg: String },
    Pwd,
    Cd { new_dir: PathBuf },
}

impl BuiltinCommand {
    fn execute(&self) {
        match self {
            BuiltinCommand::Exit => exit(),
            BuiltinCommand::Echo { args } => echo(args.to_vec()),
            BuiltinCommand::Type { arg } => print_command_type(arg.to_string()),
            BuiltinCommand::Pwd => pwd(),
            BuiltinCommand::Cd { new_dir } => cd(new_dir),
        }
    }
}

struct ExecutableCommand {
    name: String,
    path: String,
    args: Vec<String>,
}

impl ExecutableCommand {
    fn execute(&self) {
        let result = process::Command::new(&self.name)
                                                                .args(&self.args)
                                                                .status();                    
        match result {
            Ok(status) if status.success() => {}
            Ok(status) => println!("{} exited with {}", self.path, status),
            Err(error) => {
                println!("Failed to execute {}: {}", self.path, error); 
            }
        }
    }
}

/** Main *************************/

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
        ("pwd", _) => Command::Builtin(BuiltinCommand::Pwd),
        ("cd", args) => Command::Builtin(BuiltinCommand::Cd { new_dir: PathBuf::from(args.first().unwrap_or(&String::new())) }),
        ("type", args) => Command::Builtin(BuiltinCommand::Type { arg: args.first().cloned().unwrap_or(String::new()) }),
        (command, args) => {
            if let Some(path) = get_executable_path(&command.to_string()) {
                Command::Executable(ExecutableCommand { name: command.to_string(), path: path.to_string_lossy().to_string(), args })
            } else {
                Command::Unknown { command: command.to_string() }
            }
        }

    }
}

fn print_command_type(command_string: String) {
    let command = parse(&command_string);

    match command {
        Command::Unknown { command: type_} => println!("{}: not found", type_),
        Command::Builtin(_)  => { println!("{} is a shell builtin",  command_string) },
        Command::Executable(ExecutableCommand { name, path, args }) => { println!("{} is {}",  command_string, path) },
    }
}

fn get_executable_path(command: &String) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|path| path.join(command))
        .filter(|path| path.is_file())
        .find(|file| is_executable(file))
}

fn is_executable(file: &PathBuf) -> bool {
    let Ok(metadata) = file.metadata() else {
        return false
    };
    // 0o111 corresponds to the execution bits:
    // User (0o100), Group (0o010), or Others (0o001)
    metadata.permissions().mode() & 0o111 != 0
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
    let result = env::set_current_dir(&new_dir);
    match result {
        Ok(_) => {}
        Err(error) => println!("cd: {}: No such file or directory", new_dir.display()),
    }
}
