use std::mem::needs_drop;

pub fn handle_backslash(userinput: &mut String) -> Vec<char> {
    let mut escaped_characters = Vec::new();
    while userinput.contains("\\") {
        let index_1 = userinput.find('\\').unwrap();
        if let Some(ch) = userinput[index_1 + 1..index_1 + 2].chars().next() {
            if ch == '\\' {
                userinput.replace_range(index_1 + 1..index_1 + 2, "");
            }
            escaped_characters.push(ch);
        }
        //escaped_characters.push(
        //    userinput
        //        .chars()
        //        .nth(index_1 - 1)
        //        .expect("Why does this not woek"),
        //);
        userinput.replace_range(index_1 + 1..index_1 + 2, "£");
        let before_backslash = &userinput[..index_1]; // Part before the backslash
        let after_backslash = &userinput[index_1 + 1..]; // Part after the backslash
        *userinput = before_backslash.to_string() + after_backslash;
    }
    return escaped_characters;
}
pub fn replace_escaped_chars(userinput: &mut Vec<String>, escaped_chars: Vec<char>) {
    let mut indec: usize = 0;
    for (index, input) in userinput.into_iter().enumerate() {
        while input.contains("£") {
            *input = input.replacen("£", escaped_chars[indec].to_string().as_str(), 1);
            indec += 1;
        }
    }
}
pub fn noquotes(s: &str) -> String {
    let mut st = s.trim().to_string();
    while st.contains("\\") {
        let index_1 = st.find("\\").unwrap();
        st = st[..index_1].to_string() + &st[index_1 + 1..];
    }
    st.push(' ');
    //st.insert_str(st.len() - 2, " ");
    return st;
}
pub fn handle_quotes_last(quote: char, userinput: &[&str]) -> Vec<String> {
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
pub fn handle_quotes(quote: char, userinput: &[&str]) -> Vec<String> {
    let mut collected_userinput = userinput.join(" ");
    let mut tokens = Vec::new();
    if collected_userinput.contains(quote) == false {
        //return collected_userinput
        //    .split(' ')
        //    .into_iter()
        //    .map(|s| String::from(s))
        //    .collect();

        let tokens: Vec<&str> = collected_userinput.split(" ").collect();
        return vec![tokens.join(" ")];
    }
    while collected_userinput.contains(quote) {
        let index_1 = collected_userinput.find(quote).unwrap();
        if collected_userinput.contains('"') {
            let indexdq_1 = collected_userinput.find(quote).unwrap();
            if indexdq_1 < index_1 {
                let tokens: Vec<&str> = collected_userinput.split(" ").collect();
                return vec![tokens.join(" ")];
            }
        }
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
