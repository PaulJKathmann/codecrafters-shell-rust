#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let mut user_input = String::new();
    while let Ok(_) = io::stdin().read_line(&mut user_input) {
        println!("{}: command not found", user_input.trim());
        print!("$ ");
        io::stdout().flush().unwrap();
        user_input.clear();
    }
}
