pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars();
    let mut cur_ch = chars.next();
    let mut cur_ch_count = 1u32;

    loop {
        let next_ch = chars.next();
        if let Some(cur_ch) = cur_ch {
            let need_write;

            if let Some(next_ch) = next_ch {
                if next_ch == cur_ch {
                    cur_ch_count += 1;
                    need_write = false;
                } else {
                    need_write = true;
                }
            } else {
                need_write = true;
            }

            if need_write {
                if cur_ch_count > 1 {
                    result.push_str(&cur_ch_count.to_string());
                }
                result.push(cur_ch);
                cur_ch_count = 1;
            }
        }

        if next_ch.is_none() {
            break;
        }

        cur_ch = next_ch;
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
