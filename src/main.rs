use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
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
                        println!("{} is a shell builtin", tokens[1]);
                    }
                    _ => {
                        let paths = env::var("PATH").unwrap();
                        let mut found = false;
                        for p in paths.split(":") {
                            let pa = Path::new(p).join(tokens[1]);
                            if pa.exists() && !found {
                                println!(
                                    "{} is {}",
                                    tokens[1],
                                    pa.into_os_string().into_string().unwrap()
                                );
                                found = true;
                            }
                        }
                        if !found {
                            println!("{}: not found", tokens[1])
                        }
                    }
                };
            }
            _ => println!("{}: command not found", trimmed_input),
        };
    }
}
