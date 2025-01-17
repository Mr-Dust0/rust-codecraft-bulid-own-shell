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

    arguments.truncate(arguments.len());
    file_path
}
pub fn handle_stderr_redirect(arguments: &mut Vec<String>) -> Box<dyn Write> {
    let mut file_path: Box<dyn Write> = Box::new(io::stdout());

    let mut i = 0;
    while i < arguments.len() {
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

    arguments.truncate(arguments.len());
    file_path
}
