use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    candidate
        .chars()
        .try_fold(HashSet::new(), |mut acc, ch| {
            if ch == '-'
                || ch.is_ascii_whitespace()
                || (ch.is_alphabetic() && acc.insert(ch.to_ascii_lowercase()))
            {
                Some(acc)
            } else {
                None
            }
        })
        .is_some()
}
