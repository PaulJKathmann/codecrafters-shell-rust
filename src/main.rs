use std::env::Args;
#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        let (command, args) = parse(&user_input.trim());
        eval(command, args);
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}

fn parse(input: &str) -> (&str, Vec<&str>) {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.is_empty() {
        return ("", vec![]);
    }
    let (command, args) = tokens.split_at(1);
    return (command[0], args.to_vec())
}

fn eval(command: &str, args: Vec<&str>) {
    match command {
        "exit" => exit(),
        "echo" => echo(args),
        _ => println!("{}: command not found", command),
    }
}

fn echo(args: Vec<&str>) {
    for arg in args {
        print!("{} ", arg);
    }
    println!();
}

fn exit() {
    std::process::exit(0);
}
