/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut even = true;
    let mut sum = 0u32;
    let mut digits = 0u32;
    for &ch in code.as_bytes().iter().rev() {
        if ch == b' ' { continue; }
        if ch >= b'0' && ch <= b'9' {
            let mut digit = (ch - b'0') as u32;
            even = !even;
            if even {
                digit = digit * 2;
                if digit > 9 {
                    digit = digit - 9;
                }
            }
            sum += digit;
            digits += 1;
        } else {
            return false;
        }
    }
    if sum == 0 && digits <= 1 {
        return false;
    }
    sum % 10 == 0
}
