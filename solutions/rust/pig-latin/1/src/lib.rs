const DO_NOT_MOVE: [&str; 7] = ["yt", "xr", "a", "e", "i", "o", "u"];
const MOVE_TO_SUFFIX: [&str; 27] = [
    "squ", "sch", "thr", "th", "ch", "qu", "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n",
    "p", "q", "r", "s", "t", "v", "w", "x", "z", "y",
];
const MOVE_TO_SUFFIX_SPECIAL: [(&str, usize); 1] = [("rhy", 2)];

fn will_be_suffix(word: &str) -> Option<usize> {
    if DO_NOT_MOVE.into_iter().any(|cons| word.starts_with(cons)) {
        return None;
    }

    if let Some(l) = MOVE_TO_SUFFIX_SPECIAL
        .into_iter()
        .find(|&(x, _)| word.starts_with(x))
        .map(|(_, l)| l)
    {
        return Some(l);
    }

    MOVE_TO_SUFFIX
        .into_iter()
        .find(|&cons| word.starts_with(cons))
        .map(|cons| cons.len())
}

pub fn translate(input: &str) -> String {
    input
        .split_ascii_whitespace()
        .fold(String::new(), |mut acc, word| {
            if !acc.is_empty() {
                acc.push(' ');
            }

            if word.len() < 3 {
                return acc;
            }

            if let Some(suffix) = will_be_suffix(word) {
                let (a, b) = word.split_at(suffix);
                acc.push_str(b);
                acc.push_str(a);
            } else {
                acc.push_str(word);
            }

            acc.push_str("ay");
            acc
        })
}
