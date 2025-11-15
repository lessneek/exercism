use std::iter::repeat_n;

pub fn encode(number: u64) -> String {
    let mut result = String::new();

    let mut digits: Vec<u8> = num_to_digits_3(number);

    let prefix_zeros = 3 - digits.len() % 3;
    if prefix_zeros > 0 {
        digits.splice(0..0, repeat_n(0, prefix_zeros));
    }

    for (d, i) in digits
        .windows(3)
        .step_by(3)
        .zip((0..(digits.len() / 3)).rev())
    {
        println!("{}: {:?}", i, d);

        if d[0] == 0 && d[1] == 0 && d[2] == 0 {
            continue;
        }

        if !result.is_empty() {
            result.push(' ');
        }

        if let Some(hundred) = digit_to_word(d[0]) {
            result.push_str(hundred);
            result.push_str(" hundred");

            if d[1] != 0 || d[2] != 0 {
                result.push(' ');
            }
        }

        match d[1] as usize {
            1 => {
                if let Some(teen) = teen_to_word(d[2]) {
                    result.push_str(teen);
                }
            }
            oti @ (0 | 2..=9) => {
                result.push_str(OVERTEEN_WORDS[oti]);
                if d[2] > 0 {
                    if oti > 0 && d[2] > 0 {
                        result.push('-')
                    };
                    result.push_str(DIGITS_WORDS[d[2] as usize]);
                }
            }
            _ => {}
        }

        if i > 0 && (d[0] != 0 || d[1] != 0 || d[2] != 0) {
            result.push(' ');
            result.push_str(BIG_NUMBERS[i]);
        }
    }

    if result.is_empty() {
        result.push_str("zero");
    }

    result
}

const DIGITS_WORDS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const TEEN_WORDS: [&str; 10] = [
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const OVERTEEN_WORDS: [&str; 10] = [
    "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const BIG_NUMBERS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn num_to_digits_1(number: u64) -> Vec<u8> {
    number
        .to_string()
        .chars()
        .filter_map(|ch| ch.to_digit(10).map(|d| d as u8))
        .collect()
}

pub fn num_to_digits_2(mut number: u64) -> Vec<u8> {
    let mut result: Vec<u8> = vec![];
    loop {
        result.push((number % 10).try_into().unwrap());
        number /= 10;
        if number == 0 {
            break;
        }
    }
    result.reverse();
    result
}

pub fn num_to_digits_3(number: u64) -> Vec<u8> {
    number.to_string().chars().map(|ch| ch as u8 - 48).collect()
}

fn digit_to_word(digit: u8) -> Option<&'static str> {
    if digit == 0 || digit > 9 {
        return None;
    }
    Some(DIGITS_WORDS[digit as usize])
}

fn teen_to_word(digit: u8) -> Option<&'static str> {
    if digit > 9 {
        return None;
    }
    Some(TEEN_WORDS[digit as usize])
}
