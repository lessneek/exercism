pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brack = vec![];
    for c in string.chars() {
        match c {
            '(' | '[' | '{' => brack.push(c),
            ')' if brack.pop() != Some('(') => return false,
            ']' if brack.pop() != Some('[') => return false,
            '}' if brack.pop() != Some('{') => return false,
            _ => continue,
        }
    }
    brack.is_empty()
}
