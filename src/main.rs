use std::ptr::write;
use std::{env, fs};
mod quotes;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Output};
fn main() {
    let paths = env::var("PATH").unwrap();
    'outer: loop {
        let mut escaped_chars = Vec::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut trimmed_input = String::new();
        stdin.read_line(&mut trimmed_input).unwrap();
        let input = trimmed_input.clone();
        let mut input2 = trimmed_input.clone();
        let mut test = trimmed_input.clone();
        let token: Vec<&str> = input.trim().split(" ").collect();
        let tokens: Vec<&str> = trimmed_input.split(" ").collect();
        let mut arguments = Vec::new();

        let mut args = tokens[1..].to_vec();
        escaped_chars = quotes::handle_backslash(&mut test);

        match tokens[0].chars().nth(0) {
            Some(first_char) if first_char == '\'' || first_char == '"' => {
                if first_char == '\'' {
                    arguments = quotes::handle_quotes_last('\'', &tokens[..]);
                } else {
                    arguments = quotes::handle_quotes_last('"', &tokens[..]);
                }
                let paths = get_path(&arguments[0].trim());
                if paths == "" {
                    println!("{}: command not found", arguments[0]);
                    continue;
                }
                //println!("{:?}", arguments);
                //println!("{:?}", paths);
                //let pa = Path::new(&paths);
                //println!("{}", pa.display());
                //
                //println!("{}", paths);
                let mut command = Command::new(paths.trim());
                for arg in arguments {
                    command.arg(arg.trim());
                }
                let output = command.output().expect("Failed to execute command");

                //.arg(&arguments[1].trim()) // Execute the command with space handling
                //.output()
                //.expect("Failed to execute command");
                print!("{}", String::from_utf8(output.stdout).unwrap());

                continue;
            }
            _ => {}
        }

        //println!("{a}", test);
        if test.contains('"') && test.contains("'") {
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
        } else if test.contains('"') {
            if token[0] == "echo" {
                escaped_chars = quotes::handle_backslash(&mut input2);
            }
            let tokens: Vec<&str> = input2.split(" ").collect();
            arguments = quotes::handle_quotes_last('"', &tokens[1..]);
            if token[0] == "echo" {
                quotes::replace_escaped_chars(&mut arguments, escaped_chars);
            }
        } else if test.contains("'") {
            arguments = quotes::handle_quotes_last('\'', &tokens[1..]);
            // Adding an comment so that i can push
        } else {
            if token[0] == "echo" {
                escaped_chars = quotes::handle_backslash(&mut input2);
            }
            let tokens: Vec<&str> = input2.split_whitespace().collect();
            arguments = tokens[1..].iter().map(|s| quotes::noquotes(*s)).collect();
            if token[0] == "echo" {
                quotes::replace_escaped_chars(&mut arguments, escaped_chars);
            }
            // Adding an comment to that i can push again
        }

        //let arguments = handle_quotes('\'', &tokens[1..]);
        //let v2: Vec<&str> = arguments.iter().map(|s| s.as_str()).collect();
        //let arguments = handle_quotes_last('"', &tokens[1..]);
        //println!("{:?}", arguments);
        match token[0] {
            "exit" => std::process::exit(0),
            "echo" => {
                let mut file_path = handle_stdout_redirect(&token[0], &mut arguments);
                let mut file_path_err = handle_stdout_redirect(&token[0], &mut arguments);

                match writeln!(file_path, "{}", &arguments[..].join("")) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        writeln!(file_path_err, "Cant write to that file");
                        continue;
                    }
                }
                writeln!(file_path, "{}", &arguments[..].join(""));
                //println!("{}", &arguments[..].join(""));
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
                // I dobt know what this is going wrong at the moment
                let mut file_path = handle_stdout_redirect("cat", &mut arguments);
                let mut file_path_err = handle_stderr_redirect("cat", &mut arguments);
                let mut output = String::new();
                for path in arguments.into_iter() {
                    if path.trim() != "" {
                        match std::fs::read_to_string(path.trim()) {
                            Ok(content) => output = output + content.trim(),
                            Err(_) => {
                                write!(
                                    file_path_err,
                                    "cat: {}: No such file or directory",
                                    path.trim()
                                );
                                continue;
                            }
                        }
                    }
                }
                writeln!(file_path, "{}", output);

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
                let mut command = Command::new(token[0]);
                let arguments2 = arguments.clone();
                //println!("{:?}", arguments);
                let mut file_path = handle_stdout_redirect("", &mut arguments);
                let mut file_path_err = handle_stderr_redirect("", &mut arguments);
                //println!("{:?}", arguments);

                for (index, arg) in arguments.into_iter().enumerate() {
                    command.arg(arg.trim());
                }
                let output = command.output().expect("Failed to execute the command ");

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                write!(file_path, "{}", stdout);
                write!(file_path_err, "{}", stderr);
            } //if paths != "" {
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
fn handle_stdout_redirect(command: &str, arguments: &mut Vec<String>) -> Box<dyn Write> {
    let mut file_path: Box<dyn Write> = Box::new(io::stdout());

    // Iterate over the arguments to check for redirection
    let mut i = 0;
    while i < arguments.len() {
        if arguments[i].trim() == "2>" {
            match std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open("/dev/null")
            {
                Ok(file) => {
                    file_path = Box::new(file);
                    // Remove the redirection operator and the path from the arguments
                    return file_path; // Return the file handle for writing
                }
                Err(e) => {
                    return Box::new(io::stdout()); // Return stdout on error
                }
            }
        }
        if arguments[i].trim() == ">" || arguments[i].trim() == "1>" {
            //println!("{}", arguments[i]);
            // Ensure there's an argument after the redirection operator
            if i + 1 < arguments.len() {
                let path = &arguments[i + 1].trim();

                // Try to open the file for writing
                if arguments[2].trim() == "" {
                    file_path = Box::new(io::stderr());
                    arguments.truncate(i); // Keep only arguments before the operator
                } else {
                    match std::fs::OpenOptions::new()
                        .create(true)
                        .write(true)
                        .open(path)
                    {
                        Ok(file) => {
                            file_path = Box::new(file);
                            // Remove the redirection operator and the path from the arguments
                            arguments.truncate(i); // Keep only arguments before the operator
                            return file_path; // Return the file handle for writing
                        }
                        Err(e) => {
                            eprintln!("Error opening file '{}': {}", path, e);
                            return Box::new(io::stdout()); // Return stdout on error
                        }
                    }
                }
            }
        }
        i += 1;
    }

    // If no redirection was found, return stdout
    arguments.truncate(arguments.len()); // Ensure arguments before the redirect are kept
    file_path
}
fn handle_stderr_redirect(command: &str, arguments: &mut Vec<String>) -> Box<dyn Write> {
    let mut file_path: Box<dyn Write> = Box::new(io::stdout());

    // Iterate over the arguments to check for redirection
    let mut i = 0;
    while i < arguments.len() {
        if arguments[i].trim() == "2>" {
            // Ensure there's an argument after the redirection operator
            if i + 1 < arguments.len() {
                let path = &arguments[i + 1].trim();
                let mut file = std::fs::File::create(path);

                match std::fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .open(path)
                {
                    Ok(file) => {
                        file_path = Box::new(file);
                        // Remove the redirection operator and the path from the arguments
                        arguments.truncate(i); // Keep only arguments before the operator
                        return file_path; // Return the file handle for writing
                    }
                    Err(e) => {
                        eprintln!("Error opening file '{}': {}", path, e);
                        return Box::new(io::stdout()); // Return stdout on error
                    }
                }
            }
        }
        i += 1;
    }

    // If no redirection was found, return stdout
    arguments.truncate(arguments.len()); // Ensure arguments before the redirect are kept
    file_path
}
