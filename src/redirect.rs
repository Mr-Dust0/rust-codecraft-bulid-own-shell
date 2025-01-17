use std::io::{self, Write};
pub fn handle_stdout_redirect(arguments: &mut Vec<String>) -> Box<dyn Write> {
    let mut file_path: Box<dyn Write> = Box::new(io::stdout());

    let mut i = 0;
    while i < arguments.len() {
        if arguments[i].trim() == "2>" {
            return Box::new(io::stdout());
        }
        if arguments[i].trim() == ">"
            || arguments[i].trim() == "1>"
            || arguments[i].trim() == ">>"
            || arguments[i].trim() == "1>>"
        {
            if i + 1 < arguments.len() {
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
        }
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
