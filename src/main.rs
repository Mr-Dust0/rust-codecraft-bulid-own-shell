#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output};
use std::{env, result};
fn main() {
    let paths = env::var("PATH").unwrap();
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
                    "echo" | "type" | "exit" | "pwd" => {
                        println!("{} is a shell builtin", tokens[1]);
                    }
                    _ => {
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
            "pwd" => {
                let current_dir = std::env::current_dir().expect("cant get the current dir");
                println!("{}", current_dir.into_os_string().into_string().unwrap());
            }
            "cd" => {
                let lol = match std::env::set_current_dir(tokens[1]) {
                    Ok(result) => {
                        continue;
                    }
                    Err(Error) => {
                        println!("cd: {}: No such file or directory", tokens[1]);
                        continue;
                    }
                };
            }
            _ => {
                let paths = get_path(&tokens[0]);
                if paths == "" {
                    println!("{}: command not found", tokens[0]);
                    continue;
                }
                let output = Command::new(tokens[0])
                    .args(&tokens[1..])
                    .output()
                    .expect("Cant execute the command")
                    .stdout;
                let stdout = String::from_utf8_lossy(&output);
                print!("{}", stdout)
                //if paths != "" {
                //    // need to use the & so the loop doesnt consume the tokens so it cant be used
                //    // outside of the loop
                //    let mut command = Command::new(paths);
                //    for arg in &tokens[1..] {
                //        command.arg(arg);
                //    }
                //    let output = command.output().expect("failed to execute");
                //    let stdout = String::from_utf8_lossy(&output.stdout);
                //    print!("{}", stdout)
                //}
            }
        }
    }
}

fn get_path(binary: &str) -> String {
    let paths = env::var("PATH").unwrap();
    for p in paths.split(":") {
        let pa = Path::new(p).join(binary);
        if pa.exists() {
            return pa.into_os_string().into_string().unwrap();
        }
    }
    return String::from("");
}
