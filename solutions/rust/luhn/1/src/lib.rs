/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.len() <= 1 {
        return false;
    }

    let mut even = true;
    let mut sum = 0u32;
    let mut digits = 0u32;
    for ch in code.chars().rev() {
        if ch == ' ' { continue; }
        if let Some(mut digit) = ch.to_digit(10) {
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
