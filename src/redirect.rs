use std::io::{self, Write};
pub fn handle_stdout_redirect(arguments: &mut Vec<String>) -> Box<dyn Write> {
    // The defualt case if there is no file to redirect output to
    let mut file_path: Box<dyn Write> = Box::new(io::stdout());

    let mut i = 0;
    while i < arguments.len() {
        // Check all the ways output can be redirected
        if arguments[i].trim() == ">"
            || arguments[i].trim() == "1>"
            || arguments[i].trim() == ">>"
            || arguments[i].trim() == "1>>"
        {
            // Check if there is an file after the > so don't get an out of bounds erorr
            if i + 1 < arguments.len() {
                // Get path after >
                let path = &arguments[i + 1].trim();

                // See if the redirect is an append not an overwrite of the file
                let write = arguments[i].contains(">>");

                match std::fs::OpenOptions::new()
                    .create(true)
                    .write(!write) // If its apppend dont overwite
                    .append(write)
                    .open(path)
                {
                    Ok(file) => {
                        file_path = Box::new(file);
                        // Get rid of the > and file so they arent interperted by the command when
                        // executed
                        arguments.truncate(i);
                        return file_path;
                    }
                    Err(e) => {
                        // if cant open the file then just use stdout
                        eprintln!("Error opening file '{}': {}", path, e);
                        return Box::new(io::stdout());
                    }
                }
            }
        }
        // Add one to loop index
        i += 1;
    }

    // Return stdout if the output was not reditcted
    return file_path;
}
pub fn handle_stderr_redirect(arguments: &mut Vec<String>) -> Box<dyn Write> {
    // Set defalt to stderr if no redirect is found
    let mut file_path: Box<dyn Write> = Box::new(io::stderr());

    let mut i = 0;
    while i < arguments.len() {
        // See if the arument is redirecting the stderrr
        if arguments[i].trim() == "2>" || arguments[i].trim() == "2>>" {
            let path = &arguments[i + 1].trim();

            let write = arguments[i].contains(">>");

            match std::fs::OpenOptions::new()
                .create(true)
                .write(!write)
                .append(write)
                .open(path)
            {
                Ok(file) => {
                    file_path = Box::new(file);

                    // Get rid of > and file
                    arguments.truncate(i);
                    return file_path;
                }
                Err(e) => {
                    eprintln!("Error opening file '{}': {}", path, e);
                    return Box::new(io::stdout());
                }
            }
        }
        i += 1;
    }

    return file_path;
}
