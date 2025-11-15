use std::collections::HashMap;

fn create_dict<'a>(words: &[&'a str]) -> HashMap<&'a str, u16> {
    let mut dict: HashMap<&'a str, u16> = HashMap::new();
    for word in words {
        if let Some(v) = dict.get_mut(word) {
            *v += 1;
        } else {
            dict.insert(word, 1);
        }
    }
    dict
}

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine_dict = create_dict(magazine);
    let note_dict = create_dict(note);

    for (note_word, note_word_count) in note_dict {
        if let Some(mag_word_count) = magazine_dict.get(note_word) {
            if note_word_count > *mag_word_count {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
