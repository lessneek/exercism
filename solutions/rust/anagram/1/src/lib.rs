use std::collections::HashSet;

fn to_sorted_chars(word: &str) -> Vec<char> {
    let mut word_chars: Vec<char> = word.chars().collect();
    word_chars.sort_unstable();
    word_chars
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let word_lower = word.to_lowercase();
    let word_chars: Vec<char> = to_sorted_chars(&word_lower);

    for &possible_anagram in possible_anagrams {
        let anagram_lower = possible_anagram.to_lowercase();
        if anagram_lower == word_lower {
            continue;
        }
        let anagram_chars = to_sorted_chars(&anagram_lower);
        if anagram_chars == word_chars {
            anagrams.insert(possible_anagram);
        }
    }

    anagrams
}
