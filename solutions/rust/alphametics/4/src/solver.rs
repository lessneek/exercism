use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use crate::combinations::combinations_of_dec_digits;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::parse(input)?;

    let mut num = combinations_of_dec_digits(equation.charset.len())?;
    while num.inc() {
        let key = num.value();

        if equation.try_key(key) {
            return Some(
                equation
                    .charset
                    .iter()
                    .copied()
                    .zip(key.iter().copied())
                    .collect(),
            );
        }
    }

    None
}

struct Equation {
    iwords: Vec<Vec<u8>>,
    isum: Vec<u8>,
    inon_zero_chars: Vec<u8>,
    charset: Vec<char>,
}

impl Equation {
    fn parse(input: &str) -> Option<Self> {
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

        let mut charset: Vec<char> = words
            .iter()
            .flat_map(|w| w.chars())
            .chain(sum.chars())
            .collect::<HashSet<char>>()
            .iter()
            .copied()
            .collect();
        if charset.len() > 10 {
            return None;
        }
        charset.sort();

        let char_to_index = |&ch: &char| {
            charset
                .iter()
                .position(|&c| c == ch)
                .expect("There must be a char.") as u8
        };

        let word_to_iword = |word: &str| {
            word.char_indices()
                .fold(vec![0u8; word.len()], |mut acc, (i, ch)| {
                    acc[i] = char_to_index(&ch);
                    acc
                })
        };

        let mut inon_zero_chars: Vec<u8> = words
            .iter()
            .chain(once(&sum))
            .filter(|&&w| w.len() > 1)
            .map(|&w| w.chars().next().unwrap())
            .collect::<HashSet<char>>()
            .iter()
            .map(char_to_index)
            .collect();
        inon_zero_chars.sort();

        let iwords: Vec<Vec<u8>> = words.iter().map(|&w| word_to_iword(w)).collect();

        let isum = word_to_iword(sum);

        Some(Self {
            iwords,
            isum,
            inon_zero_chars,
            charset,
        })
    }

    fn try_key(&self, key: &[u8]) -> bool {
        if self.inon_zero_chars.iter().any(|&i| key[i as usize] == 0) {
            return false;
        }
        let words_sum: u64 = self.iwords.iter().map(|w| word_to_number(w, key)).sum();

        let sum = word_to_number(&self.isum, key);

        words_sum == sum
    }
}

pub fn word_to_number(iword: &[u8], key: &[u8]) -> u64 {
    iword
        .iter()
        .map(|&i| key[i as usize])
        .fold(0u64, |acc, d| acc * 10 + d as u64)
}
