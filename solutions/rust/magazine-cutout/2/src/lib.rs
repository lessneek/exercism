use std::collections::HashMap;

fn create_dict<'a>(words: &[&'a str]) -> HashMap<&'a str, u16> {
    let mut dict = HashMap::new();
    for &word in words {
        *dict.entry(word).or_default() += 1;
    }
    dict
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_dict = create_dict(magazine);
    let note_dict = create_dict(note);

    note_dict
        .iter()
        .all(|(word, count)| magazine_dict.get(word).unwrap_or(&0) >= count)
}
