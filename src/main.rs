#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let trimmed_input = input.trim();
        match trimmed_input {
            "exit" => std::process::exit(3),
            _ => println!("{}: command not found", trimmed_input),
        };
    }
}
