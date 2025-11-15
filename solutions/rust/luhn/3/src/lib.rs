/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let (mut even, mut sum, mut digits) = (true, 0u32, 0u32);
    for &ch in code.as_bytes().iter().rev().filter(|&&x| x != b' ') {
        if ch < b'0' || ch > b'9' {
            return false;
        }
        let mut digit = (ch - b'0') as u32;
        even = !even;
        if even {
            digit *= 2;
            if digit > 9 {
                digit -= 9;
            }
        }
        sum += digit;
        digits += 1;
    }
    if sum == 0 && digits < 2 {
        return false;
    }
    sum % 10 == 0
}
