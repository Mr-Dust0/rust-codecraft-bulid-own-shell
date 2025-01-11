#[allow(unused_imports)]
use std::io::{self, Write};
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
            "type" => {
                match tokens[1] {
                    "echo" | "type" | "exit" => {
                        println!("{} is a bultin type", tokens[1]);
                    }
                    _ => println!("{}: not found", tokens[1]),
                };
            }
            _ => println!("{}: command not found", trimmed_input),
        };
    }
}
