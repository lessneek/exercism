/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    is_valid_classic(code)
    // is_valid_idiomatic(code)
}

#[inline]
fn luhn_dub(d: u32) -> u32 {
    [0, 2, 4, 6, 8, 1, 3, 5, 7, 9][d as usize]
}

#[inline]
fn ascii_to_digit_10(ch: u8) -> Option<u32> {
    match ch {
        b'0'..=b'9' => Some((ch - b'0') as u32),
        _ => None,
    }
}

pub fn is_valid_classic(code: &str) -> bool {
    let (mut sum, mut count) = (0u32, 0u32);
    for ch in code.bytes().rev().filter(|&x| x != b' ') {
        if ch < b'0' || ch > b'9' {
            return false;
        }
        let mut d = (ch - b'0') as u32;
        if count % 2 == 1 {
            d = luhn_dub(d)
        }
        sum += d;
        count += 1;
    }
    count > 1 && sum % 10 == 0
}

pub fn is_valid_idiomatic(code: &str) -> bool {
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
