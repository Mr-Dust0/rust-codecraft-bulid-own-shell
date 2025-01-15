use std::env;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
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
        let arguments = handle_quotes('\'', &tokens[1..]);
        match tokens[0] {
            "exit" => std::process::exit(0),
            "echo" => {
                println!("{}", &arguments[..].join(""));
                // Adding an random comment so that i can send an push to the github
            }
            "type" => {
                match tokens[1] {
                    "echo" | "type" | "exit" | "pwd" | "cd" | "cat" => {
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
            "cat" => {
                for path in arguments.into_iter() {
                    let content = std::fs::read_to_string(path.trim());
                    println!("{}", content.unwrap());
                }
            }
            "cd" => {
                let home = env::var("HOME").unwrap();
                let full_path = if tokens[1].chars().nth(0).unwrap() == '~' {
                    tokens[1].replace("~", &home)
                } else {
                    tokens[1].to_string()
                };
                match std::env::set_current_dir(full_path) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
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
fn handle_quotes(quote: char, userinput: &[&str]) -> Vec<String> {
    let mut collected_userinput = userinput.join(" ");
    let mut tokens = Vec::new();
    if collected_userinput.contains(quote) == false {
        //return collected_userinput
        //    .split(' ')
        //    .into_iter()
        //    .map(|s| String::from(s))
        //    .collect();

        let tokens: Vec<&str> = collected_userinput.split_whitespace().collect();
        return vec![tokens.join(" ")];
    }
    while collected_userinput.contains(quote) {
        let index_1 = collected_userinput.find(quote).unwrap();
        let index_2 = collected_userinput[index_1 + 1..].find(quote).unwrap() + index_1 + 1;
        //println!("Input {}", &collected_userinput[index_1 + 1..index_2]);
        let mut token = String::new();
        let _ = &collected_userinput[index_1 + 1..index_2].clone_into(&mut token);
        if collected_userinput.chars().nth(0).unwrap() == ' ' {
            token.insert_str(0, " ");
        }
        tokens.push(token.clone());
        //println!("Token {}", token);
        collected_userinput = String::from(&collected_userinput[index_2 + 1..]);
    }

    return tokens;
}
