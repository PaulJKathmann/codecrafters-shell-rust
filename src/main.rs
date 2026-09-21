mod builtin;
mod command;
mod executable;
mod parser;
mod lexer;

use std::io::{self, Write};
use lexer::lex;
use parser::parse;

fn main() {
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        let tokens = lex(&user_input);
        let command = parse(tokens);
        command.execute();
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}
