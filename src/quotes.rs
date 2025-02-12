pub fn handle_backslash(userinput: &mut String) -> Vec<char> {
    // This will be returned so know what to replace the placeholders with
    let mut escaped_characters = Vec::new();
    // See if the userinput contains an backslash
    while userinput.contains("\\") {
        let index_1 = userinput.find('\\').unwrap();
        if let Some(ch) = userinput[index_1 + 1..index_1 + 2].chars().next() {
            // Add the character after the backslash to the escaped_chars array
            escaped_characters.push(ch);
        }

        // REplace the esccaped character with the placeholder
        userinput.replace_range(index_1 + 1..index_1 + 2, "£");
        let before_backslash = &userinput[..index_1];
        let after_backslash = &userinput[index_1 + 1..];
        // check the string again after the frist backslash is removed
        *userinput = before_backslash.to_string() + after_backslash;
    }
    // return all the characters that have been escaped
    return escaped_characters;
}

pub fn replace_escaped_chars(userinput: &mut Vec<String>, escaped_chars: Vec<char>) {
    // the index into escaped_chars to get the character to replace the placeholder with
    let mut indec: usize = 0;
    for (_, input) in userinput.into_iter().enumerate() {
        // Replace the placeholder with orginal char
        while input.contains("£") {
            *input = input.replacen("£", escaped_chars[indec].to_string().as_str(), 1);

            indec += 1;
        }
    }
}

pub fn noquotes(s: &str) -> String {
    let mut st = s.trim().to_string();
    st.push(' ');

    return st;
}

pub fn handle_quotes_last(quote: char, userinput: &[&str]) -> Vec<String> {
    // Join the input again that was split on spaces when passed in
    let mut collected_userinput = userinput.join(" ");
    let mut tokens = Vec::new();
    // See if the userinput joined back up contains doesnt contain an quote and if so return the
    // userinput just split on whitespaces.
    if collected_userinput.contains(quote) == false {
        let tokens: Vec<&str> = collected_userinput.split_whitespace().collect();
        return vec![tokens.join(" ")];
    }

    while collected_userinput.contains(quote) {
        // Get index of frist quote and second quote
        // Should probaly have better error hanlding if there is no 2nd quote to end the string
        let index_1 = collected_userinput.find(quote).unwrap();
        // Searching the string after index_1 and adding index_1 to the result because we are only
        // searching characters after the frist quote
        let index_2 = collected_userinput[index_1 + 1..].find(quote).unwrap() + index_1 + 1;

        let mut token = String::new();
        // Clone the string inside the quotes to token
        let _ = &collected_userinput[index_1 + 1..index_2].clone_into(&mut token);
        // Remove the quoted string from userinput so wont be in infite loop
        collected_userinput = String::from(&collected_userinput[index_2 + 1..]);
        // See if space is at the beging of userinput because will need to add an whitespace to
        // the tokwn for correct formating for echo
        if collected_userinput.chars().nth(0).unwrap() == ' ' {
            token.push(' ');
        }
        // Add token to token that will be returned latter and the loop will start again with
        // edited collected_userinput
        tokens.push(token.clone().to_string());
    }
    // If userinput is not null then still have agurments after quotes we need to return
    if collected_userinput != "" {
        collected_userinput.pop();

        // Split on whitespace becauase no quotes are left can just use whitespaces and add that to
        // tokens
        let rest = collected_userinput.split_whitespace();
        for token in rest {
            tokens.push(String::from(token));
        }
    }

    return tokens;
}
