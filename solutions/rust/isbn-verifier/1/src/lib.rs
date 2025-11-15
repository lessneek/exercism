/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    if isbn.len() < 10 {
        return false;
    }

    isbn.char_indices()
        .try_fold(vec![], |mut acc, (idx, ch)| {
            match ch {
                '0'..='9' => acc.push(ch.to_digit(10).unwrap()),
                'X' if idx == isbn.len() - 1 => acc.push(10),
                '-' => {}
                _ => return None,
            }
            if acc.len() > 10 {
                return None;
            }
            Some(acc)
        })
        .map_or(false, |isbn| {
            isbn.len() == 10
                && isbn
                    .iter()
                    .zip((1u32..=10).rev())
                    .fold(0u32, |acc, (&digit, x)| acc + digit * x)
                    % 11
                    == 0
        })
}
