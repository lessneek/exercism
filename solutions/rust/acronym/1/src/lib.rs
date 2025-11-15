pub fn abbreviate(phrase: &str) -> String {
    let mut abbr = String::new();

    for word in phrase.split(&[' ', '-', ',']) {
        let word = word.trim_matches(|c: char| c.is_ascii_punctuation());
        if word.is_empty() {
            continue;
        }
        let chars: Vec<char> = word.chars().collect();
        abbr.push(chars[0].to_ascii_uppercase());

        for w in chars.windows(3) {
            if w[0].is_ascii_lowercase() && w[1].is_ascii_uppercase() && w[2].is_ascii_lowercase() {
                abbr.push(w[1].to_ascii_uppercase());
            }
        }
    }

    abbr
}
