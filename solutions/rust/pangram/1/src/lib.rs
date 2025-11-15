use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let chars = sentence
        .chars()
        .map(|ch| ch.to_ascii_lowercase())
        .filter(|ch| ch.is_ascii_alphabetic())
        .collect::<HashSet<char>>();
    
    ('a'..='z').all(|ch| chars.contains(&ch))
}
