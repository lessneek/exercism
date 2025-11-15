pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .bytes()
        .collect::<Vec<_>>()
        .windows(len)
        .filter_map(|s| String::from_utf8(s.to_vec()).ok())
        .collect()
}
