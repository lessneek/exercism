use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&value, chars)| {
            chars
                .iter()
                .cloned()
                .map(move |ch| (ch.to_ascii_lowercase(), value))
        })
        .collect()
}
