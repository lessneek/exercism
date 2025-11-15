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

#[test]
fn non_alphabetic_chars() {
    assert!(!check("asdf_ghjk=qwer$#@yui"), "Non alphabetic chars should not be accepted.");
}
