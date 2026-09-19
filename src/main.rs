#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();
    let user_input = &mut String::new();
    io::stdin().read_line(user_input).unwrap();
    println!("{} command not found", user_input.trim());
}
