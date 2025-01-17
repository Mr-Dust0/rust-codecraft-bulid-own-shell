use std::env;
mod quotes;
mod redirect;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
fn main() {
    // Get the path env so can check the user env when executing commands
    loop {
        let mut escaped_chars = Vec::new();
        print!("$ ");
        // Flush it so that the dollar is printed to the screen
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut trimmed_input = String::new();
        // Read userinput into trimmed_input
        stdin.read_line(&mut trimmed_input).unwrap();
        let input = trimmed_input.clone();
        let mut input2 = trimmed_input.clone();
        let mut test = trimmed_input.clone();
        // Split userinput on space to get the command the user wants to execute
        let token: Vec<&str> = input.trim().split(" ").collect();
        let tokens: Vec<&str> = trimmed_input.split(" ").collect();
        let mut arguments = Vec::new();

        // Replace all characters that are backsashed
        escaped_chars = quotes::handle_backslash(&mut test);

        // Handle the input when the command has spaces in it so we check to see if the commands
        // starts with an quote of somekind
        match tokens[0].chars().nth(0) {
            Some(first_char) if first_char == '\'' || first_char == '"' => {
                // Tokenize the userinput
                arguments = quotes::handle_quotes_last(first_char, &tokens[..]);
                // Get the path form the users env variable print command not found if the path
                // cannot be found
                let paths = get_path(&arguments[0].trim());
                if paths == "" {
                    println!("{}: command not found", arguments[0]);
                    continue;
                }

                let mut command = Command::new(paths.trim());
                for arg in arguments {
                    // Add all the arguments but trim the argument so the arguments dont have an
                    // trailing new line could use map instead
                    command.arg(arg.trim());
                }
                let output = command.output().expect("Failed to execute command");
                // Print output from the command executed
                print!("{}", String::from_utf8(output.stdout).unwrap());
                continue;
            }
            // Handle the case when we cant get the frist charater of the token
            _ => {}
        }

        // see if the userinput contains both quotes with backslashed characters converted to an £
        if test.contains('"') && test.contains("'") {
            let indexdq = trimmed_input.find('"');
            let indexsq = trimmed_input.find("'");
            // check what quote comes first
            if indexsq > indexdq {
                // Check if command being run is echo so can handle backslashed characters as they
                // act differenlty with echo
                if token[0] == "echo" {
                    escaped_chars = quotes::handle_backslash(&mut input2);
                }
                let tokens: Vec<&str> = input2.split(" ").collect();
                // Tokwnize the arguments based on qauotes.
                arguments = quotes::handle_quotes_last('"', &tokens[1..]);
                if token[0] == "echo" {
                    quotes::replace_escaped_chars(&mut arguments, escaped_chars);
                }
            } else {
                // if the quote is '' then the backslasah is ingored.
                arguments = quotes::handle_quotes('\'', &tokens[1..]);
            }
        // see if the userinput contains double quotes backslashed characters converted to an £
        } else if test.contains('"') {
            if token[0] == "echo" {
                escaped_chars = quotes::handle_backslash(&mut input2);
            }
            let tokens: Vec<&str> = input2.split(" ").collect();
            arguments = quotes::handle_quotes_last('"', &tokens[1..]);
            if token[0] == "echo" {
                quotes::replace_escaped_chars(&mut arguments, escaped_chars);
            }
        // see if the userinput contains single quotes backslashed characters converted to an £
        } else if test.contains("'") {
            arguments = quotes::handle_quotes_last('\'', &tokens[1..]);
            // User input with backslashed characters removed doesnt contain any quotes.
        } else {
            if token[0] == "echo" {
                escaped_chars = quotes::handle_backslash(&mut input2);
            }
            let tokens: Vec<&str> = input2.split_whitespace().collect();
            arguments = tokens[1..].iter().map(|s| quotes::noquotes(*s)).collect();
            if token[0] == "echo" {
                quotes::replace_escaped_chars(&mut arguments, escaped_chars);
            }
        }

        // Start the match for the command to be executed.
        match token[0] {
            "exit" => std::process::exit(0),
            "echo" => {
                // Get what file to put stdout to if no file is speficed then stdout is used
                let mut file_path = redirect::handle_stdout_redirect(&mut arguments);
                // Get what file to put stderr to if no file is speficed then stderr is used
                let mut file_path_err = redirect::handle_stderr_redirect(&mut arguments);
                // Try to write to the file for stdout if it fails write err to the stderr file
                match writeln!(file_path, "{}", &arguments[..].join("")) {
                    Ok(_) => {
                        continue;
                    }
                    Err(_) => {
                        let _ = writeln!(file_path_err, "Cant write to that file");
                        continue;
                    }
                }
            }
            "type" => {
                // See if the command being passaed as an argumnt to type is an bultin that this
                // shell offers.
                match token[1] {
                    "echo" | "type" | "exit" | "pwd" | "cd" => {
                        println!("{} is a shell builtin", token[1].trim());
                    }
                    // Hanle all other arguments that are not bultins
                    _ => {
                        // Check to see if the command actually exists
                        let paths = get_path(&token[1]);
                        if paths == "" {
                            println!("{}: not found", token[1]);
                            continue;
                        } else {
                            println!("{} is {}", tokens[1].trim(), paths);
                        }
                    }
                };
            }
            // Print the current working directory
            "pwd" => {
                let current_dir = std::env::current_dir().expect("cant get the current dir");
                println!("{}", current_dir.into_os_string().into_string().unwrap());
            }
            "cat" => {
                let mut file_path = redirect::handle_stdout_redirect(&mut arguments);
                let mut file_path_err = redirect::handle_stderr_redirect(&mut arguments);
                let mut output = String::new();
                for path in arguments.into_iter() {
                    // Check to see if the argument is not empty
                    if path.trim() != "" {
                        // Read the file and if it can be read then add the content in the file to
                        // the content variable that will be printed latter.
                        match std::fs::read_to_string(path.trim()) {
                            Ok(content) => output = output + content.trim(),
                            // Print error if file cant be read
                            Err(_) => {
                                let _ = writeln!(
                                    file_path_err,
                                    "cat: {}: No such file or directory",
                                    path.trim()
                                );
                                continue;
                            }
                        }
                    }
                }
                // Check to see if any content was actaully read before printing because will print
                // an new line even if output is not null
                if output != "" {
                    let _ = writeln!(file_path, "{}", output);
                }

                continue;
            }
            "cd" => {
                // Get the home variable in case the user enters ~ so we can expand that to the
                // users home dir
                let home = env::var("HOME").unwrap();
                let full_path = if token[1].chars().nth(0).unwrap() == '~' {
                    // replace ~ with home dir
                    token[1].replace("~", &home)
                } else {
                    token[1].to_string()
                };
                // Change directory and print error if the path does not exist.
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
            // Handle userinput if the command entered was  not an bulitin command.
            _ => {
                // See if the command can be found in the Path variable
                let paths = get_path(&token[0]);
                if paths == "" {
                    println!("{}: command not found", token[0]);
                    continue;
                }
                let mut command = Command::new(token[0]);
                let mut file_path = redirect::handle_stdout_redirect(&mut arguments);
                let mut file_path_err = redirect::handle_stderr_redirect(&mut arguments);

                // Add arguments to the command but remove the line break on some arguments that
                // are entered
                for arg in arguments.into_iter() {
                    command.arg(arg.trim());
                }
                let output = command.output().expect("Failed to execute the command ");

                // Output the correct stdout and stderr to the correct file
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let _ = write!(file_path, "{}", stdout);
                let _ = write!(file_path_err, "{}", stderr);
            }
        }
    }
}

// Get the path of the binary to be executed by using the path varibale in the users shell
fn get_path(binary: &str) -> String {
    let paths = env::var("PATH").unwrap();
    for p in paths.split(":") {
        // Add binary name to end of PATH and check if the file exists and return that file if it
        // exists
        let pa = Path::new(p).join(binary);
        if pa.exists() {
            return pa.into_os_string().into_string().unwrap();
        }
    }
    //if the path cannot be found then return an empty string
    return String::from("");
}
