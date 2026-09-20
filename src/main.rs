mod builtin;
mod command;
mod executable;
mod parser;

use std::io::{self, Write};

use parser::parse;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        let command = parse(user_input.trim());
        command.execute();
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}
