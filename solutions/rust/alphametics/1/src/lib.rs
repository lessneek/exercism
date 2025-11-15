use dedupnum::DedupDecDigitNumber;
use std::collections::{HashMap, HashSet};

type CharKey = [char; 10];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::parse(input)?;

    let mut num = DedupDecDigitNumber::new(equation.charset.len())?;

    let mut key: CharKey = ['_'; 10];
    while let Some(key_template) = num.next() {
        for ch in key.iter_mut() {
            *ch = '_';
        }

        for (i, &ch) in equation.charset.iter().enumerate() {
            key[key_template[i]] = ch;
        }

        if let Some(result) = equation.try_key(&key) {
            return Some(result);
        }
    }

    None
}

struct Equation<'a> {
    words: Vec<&'a str>,
    sum: &'a str,
    charset: HashSet<char>,
    non_zero_chars: HashSet<char>,
}

impl<'a> Equation<'a> {
    fn parse(input: &'a str) -> Option<Self> {
        let is_alphabetic = |word: &&str| word.chars().all(|c| c.is_ascii_alphabetic());

        let equ: Vec<&str> = input.split("==").map(str::trim).collect();
        if equ.len() != 2 {
            return None;
        }

        let words: Vec<&str> = equ[0].split('+').map(str::trim).collect();
        if !words.iter().all(is_alphabetic) {
            return None;
        }

        let sum = equ[1];
        if !is_alphabetic(&sum) {
            return None;
        }

        let charset: HashSet<char> = words
            .iter()
            .flat_map(|w| w.chars())
            .chain(sum.chars())
            .collect();
        if charset.len() > 10 {
            return None;
        }

        let mut non_zero_chars: HashSet<char> = words
            .iter()
            .filter(|&&w| w.len() > 1)
            .map(|&w| w.chars().next().unwrap())
            .collect();
        non_zero_chars.insert(sum.chars().next().unwrap());

        Some(Self {
            words,
            sum,
            charset,
            non_zero_chars,
        })
    }

    fn try_key(&self, key: &CharKey) -> Option<HashMap<char, u8>> {
        if self.non_zero_chars.contains(&key[0]) {
            return None;
        }
        let words_sum = self
            .words
            .iter()
            .map(|&w| word_to_number(w, key))
            .try_fold(0, |acc, d| Some(acc + d?))?;

        let sum = word_to_number(self.sum, key)?;

        if words_sum == sum {
            return Some(
                key.iter()
                    .enumerate()
                    .filter(|(_, &ch)| ch != '_')
                    .map(|(i, &d)| (d, i as u8))
                    .collect(),
            );
        }

        None
    }
}

fn word_to_number(word: &str, key: &CharKey) -> Option<u64> {
    word.chars()
        .map(|ch| key.iter().position(|&dch| dch == ch))
        .try_fold(0u64, |acc, d| Some(acc * 10 + d? as u64))
}

#[test]
fn test_word_to_number() {
    let key = ['_', 'L', '_', 'E', '_', '_', '_', '_', '_', 'S'];

    assert_eq!(word_to_number("LES", &key), Some(139));
    assert_eq!(word_to_number("SSS", &key), Some(999));
    assert_eq!(word_to_number("LA", &key), None);
    assert_eq!(word_to_number("", &key), Some(0));
    assert_eq!(word_to_number("L", &key), Some(1));
}

mod dedupnum {
    pub struct DedupDecDigitNumber {
        value: Vec<usize>,
        init_value: Vec<usize>,
    }

    impl DedupDecDigitNumber {
        pub fn new(size: usize) -> Option<Self> {
            if size > 10 {
                return None;
            }
            let value: Vec<usize> = (0..size).collect();
            let init_value = value.clone();
            Some(Self { value, init_value })
        }

        pub fn value(&self) -> &[usize] {
            self.value.as_slice()
        }

        pub fn next(&mut self) -> Option<&[usize]> {
            loop {
                for v in self.value.iter_mut().rev() {
                    *v += 1;
                    if *v < 10 {
                        break;
                    }
                    *v = 0;
                }
                // Check for duplicate digits.
                let has_duplicate = self.value.iter().enumerate().any(|(i, &d)| {
                    if let Some(ip) = self.value.iter().position(|&dp| dp == d) {
                        if ip != i {
                            return true;
                        }
                    }
                    false
                });
                if !has_duplicate {
                    break;
                }
            }

            if self.value == self.init_value {
                return None;
            }

            Some(self.value())
        }
    }

    #[test]
    fn test_dedup_dec_digit_number() {
        let mut num = DedupDecDigitNumber::new(3).unwrap();

        assert_eq!(num.value(), [0, 1, 2]);
        assert_eq!(num.next().unwrap(), [0, 1, 3]);

        for _ in 0..7 {
            num.next();
        }

        assert_ne!(
            num.value(),
            [0, 2, 0],
            "Duplicates are not possible. So it should auto-inc until no duplicates."
        );
        assert_eq!(num.value(), [0, 2, 1]);
    }
}
