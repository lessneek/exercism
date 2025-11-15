pub mod sacherjj;
pub mod boso;

use lazy_static::lazy_static;
use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    is_pangram_by_shifting(sentence)
}

pub fn is_pangram_by_hashset(sentence: &str) -> bool {
    let chars = sentence
        .chars()
        .filter(|ch| ch.is_ascii_alphabetic())
        .map(|ch| ch.to_ascii_lowercase())
        .collect::<HashSet<char>>();

    ('a'..='z').all(|ch| chars.contains(&ch))
}

lazy_static! {
    static ref ALL_CHARS_REF_VALUE: u32 =
        (b'a'..=b'z').fold(1u32, |acc, ch| acc | (1 << (ch - b'a')));
}

pub fn is_pangram_by_shifting(sentence: &str) -> bool {
    let ref_value = *ALL_CHARS_REF_VALUE;

    let value = sentence
        .chars()
        .filter(char::is_ascii_alphabetic)
        .map(|ch| 1u32 << ((ch.to_ascii_lowercase() as u8) - b'a'))
        .fold(0u32, |acc, ch| acc | ch);

    value == ref_value
}
