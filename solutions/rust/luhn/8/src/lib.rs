/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    is_valid_classic(code)
    // is_valid_idiomatic(code)
}

pub fn is_valid_classic(code: &str) -> bool {
    const INC_TABLE: [u32; 20] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 2, 4, 6, 8, 1, 3, 5, 7, 9];
    let (mut sum, mut off, mut count) = (0u32, 0u8, 0u32);
    for ch in code.bytes().rev() {
        match ch {
            b'0'..=b'9' => {
                sum += INC_TABLE[(ch - b'0' + off) as usize] as u32;
                off = 10 - off;
                count += 1;
            }
            b' ' => {}
            _ => return false,
        }
    }
    count > 1 && sum % 10 == 0
}

pub fn is_valid_idiomatic(code: &str) -> bool {
    let ascii_to_digit_10 = |ch| match ch {
        b'0'..=b'9' => Some((ch - b'0') as u32),
        _ => None,
    };
    let luhn_dub = |d: u32| [0, 2, 4, 6, 8, 1, 3, 5, 7, 9][d as usize];

    code.bytes()
        .rev()
        .filter(|&ch| ch != b' ')
        .try_fold((0u32, 0u32), |(sum, count), ch| {
            ascii_to_digit_10(ch)
                .map(|d| if count % 2 == 1 { luhn_dub(d) } else { d })
                .map(|d| (sum + d, count + 1))
        })
        .map_or(false, |(sum, count)| count > 1 && sum % 10 == 0)
}

pub fn is_valid_idiomatic_janel(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(sum, count), val| {
            val.to_digit(10)
                .map(|num| if count % 2 == 1 { num * 2 } else { num })
                .map(|num| if num > 9 { num - 9 } else { num })
                .map(|num| (num + sum, count + 1))
        })
        .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
}
