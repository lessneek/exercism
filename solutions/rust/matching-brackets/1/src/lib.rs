pub fn brackets_are_balanced(string: &str) -> bool {
    let (mut brackets, mut braces, mut parentheses) = (0u32, 0u32, 0u32);
    let mut last_opened = vec![];
    for c in string.chars() {
        match c {
            '(' => {
                parentheses += 1;
                last_opened.push(c);
            }
            ')' => {
                if parentheses == 0 || last_opened.pop() != Some('(') {
                    return false;
                }
                parentheses -= 1;
            }
            '[' => {
                brackets += 1;
                last_opened.push(c);
            }
            ']' => {
                if brackets == 0 || last_opened.pop() != Some('[') {
                    return false;
                }
                brackets -= 1;
            }
            '{' => {
                braces += 1;
                last_opened.push(c);
            }
            '}' => {
                if braces == 0 || last_opened.pop() != Some('{') {
                    return false;
                }
                braces -= 1;
            }
            _ => continue,
        }
    }
    brackets == 0 && braces == 0 && parentheses == 0
}
