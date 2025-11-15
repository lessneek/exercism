pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len == 0 {
        return vec!["".to_string(); digits.len() + 1];
    }
    digits
        .chars()
        .collect::<Vec<_>>()
        .windows(len)
        .map(String::from_iter)
        .collect()
}
