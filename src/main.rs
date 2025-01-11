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
        let tokens: Vec<&str> = trimmed_input.split(' ').collect();
        match tokens[0] {
            "exit" => std::process::exit(0),
            "echo" => {
                println!("{}", tokens[1..].join(" "));
            }
            _ => println!("{}: command not found", trimmed_input),
        };
    }
}
