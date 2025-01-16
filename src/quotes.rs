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

