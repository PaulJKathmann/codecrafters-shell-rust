#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        eval(&user_input.trim());
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}

fn eval(command: &str) {
    match command {
        "exit" => exit(),
        _ => println!("{}: command not found", command),
    }
}
fn exit() {
    std::process::exit(0);
}
