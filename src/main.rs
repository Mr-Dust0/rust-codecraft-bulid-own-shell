use std::env;
mod quotes;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
fn main() {
    let paths = env::var("PATH").unwrap();
    loop {
        let mut escaped_chars = Vec::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut trimmed_input = String::new();
        stdin.read_line(&mut trimmed_input).unwrap();
        let input = trimmed_input.clone();
        let mut input2 = trimmed_input.clone();
        let token: Vec<&str> = input.trim().split(" ").collect();
        let tokens: Vec<&str> = trimmed_input.split(" ").collect();
        let mut arguments = Vec::new();
        if trimmed_input.contains('"') && trimmed_input.contains("'") {
            let indexdq = trimmed_input.find('"');
            let indexsq = trimmed_input.find("'");
            if indexsq > indexdq {
                if token[0] == "echo" {
                    escaped_chars = quotes::handle_backslash(&mut input2);
                }

                let tokens: Vec<&str> = input2.split(" ").collect();
                arguments = quotes::handle_quotes_last('"', &tokens[1..]);
                if token[0] == "echo" {
                    quotes::replace_escaped_chars(&mut arguments, escaped_chars);
                }
            } else {
                arguments = quotes::handle_quotes('\'', &tokens[1..]);
            }
        } else if trimmed_input.contains('"') {
            if token[0] == "echo" {
                escaped_chars = quotes::handle_backslash(&mut input2);
            }
            let tokens: Vec<&str> = input2.split(" ").collect();
            arguments = quotes::handle_quotes_last('"', &tokens[1..]);
            if token[0] == "echo" {
                quotes::replace_escaped_chars(&mut arguments, escaped_chars);
            }
        } else if trimmed_input.contains("'") {
            arguments = quotes::handle_quotes('\'', &tokens[1..]);
        } else {
            let tokens: Vec<&str> = trimmed_input.split_whitespace().collect();
            arguments = tokens[1..].iter().map(|s| quotes::noquotes(*s)).collect();
            // Adding an comment to that i can push again
        }

        //let arguments = handle_quotes('\'', &tokens[1..]);
        //let v2: Vec<&str> = arguments.iter().map(|s| s.as_str()).collect();
        //let arguments = handle_quotes_last('"', &tokens[1..]);
        // println!("{:?}", arguments);
        match token[0] {
            "exit" => std::process::exit(0),
            "echo" => {
                println!("{}", &arguments[..].join(""));
                continue;
                // Adding an random comment so that i can send an push to the github
            }
            "type" => {
                match token[1] {
                    "echo" | "type" | "exit" | "pwd" | "cd" => {
                        println!("{} is a shell builtin", token[1].trim());
                    }
                    _ => {
                        let mut found = false;
                        for p in paths.split(":") {
                            let pa = Path::new(p).join(token[1]);
                            if pa.exists() && !found {
                                println!(
                                    "{} is {}",
                                    tokens[1].trim(),
                                    pa.into_os_string().into_string().unwrap()
                                );
                                found = true;
                            }
                        }

                        if !found {
                            println!("{}: not found", token[1].trim())
                            // Added coomments so that i can push again
                        }
                    }
                };
            }
            "pwd" => {
                let current_dir = std::env::current_dir().expect("cant get the current dir");
                println!("{}", current_dir.into_os_string().into_string().unwrap());
            }
            "cat" => {
                for path in arguments.into_iter() {
                    //println!("{}", path);
                    let content = std::fs::read_to_string(path.trim());
                    print!("{}", content.unwrap());
                }
                continue;
            }
            "cd" => {
                let home = env::var("HOME").unwrap();
                let full_path = if token[1].chars().nth(0).unwrap() == '~' {
                    token[1].replace("~", &home)
                } else {
                    token[1].to_string()
                };
                match std::env::set_current_dir(full_path) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        println!("cd: {}: No such file or directory", token[1]);
                        continue;
                    }
                };
            }
            _ => {
                let paths = get_path(&token[0]);
                if paths == "" {
                    println!("{}: command not found", token[0]);
                    continue;
                }
                let output = Command::new(token[0])
                    .args(&token[1..])
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
