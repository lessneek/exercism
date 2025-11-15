pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut cur_ch_count = 0u32;

    while let Some(cur_ch) = chars.next() {
        cur_ch_count += 1;
        if chars.peek() != Some(&cur_ch) {
            if cur_ch_count > 1 {
                result.push_str(&cur_ch_count.to_string());
            }
            result.push(cur_ch);
            cur_ch_count = 0;
        }
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut cur_ch_count = 0u32;

    for cur_ch in source.chars() {
        if let Some(cur_dig) = cur_ch.to_digit(10) {
            cur_ch_count *= 10;
            cur_ch_count += cur_dig;
        } else if cur_ch_count > 0 {
            for _ in 0..cur_ch_count {
                result.push(cur_ch);
            }
            cur_ch_count = 0;
        } else {
            result.push(cur_ch);
        }
    }

    result
}
