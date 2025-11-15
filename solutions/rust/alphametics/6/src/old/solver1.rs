/// Slow first version. Do not use.
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use super::dec_digits_combinations::{DecDigitsCombinations, DigitType};

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equation = Equation::parse(input)?;

    let mut num = DecDigitsCombinations::new(equation.charset.len())?;
    let mut key = CharKey::new();

    while num.inc() {
        key.apply_template(num.value(), &equation.charset);

        if equation.try_key(&key) {
            return Some(key.to_hashmap());
        }
    }

    None
}

pub struct CharKey([char; 10]);

impl Default for CharKey {
    fn default() -> Self {
        Self::new()
    }
}

impl CharKey {
    const DEFAULT: [char; 10] = ['_'; 10];

    pub fn new() -> Self {
        Self(CharKey::DEFAULT)
    }

    pub fn from(value: [char; 10]) -> Self {
        Self(value)
    }

    pub fn value(&self) -> &[char; 10] {
        &self.0
    }

    pub fn value_mut(&mut self) -> &mut [char; 10] {
        &mut self.0
    }

    pub fn reset(&mut self) {
        self.0 = CharKey::DEFAULT;
    }

    pub fn apply_template(&mut self, template: &[DigitType], charset: &[char]) {
        self.reset();

        for (i, &ch) in charset.iter().enumerate() {
            self.0[template[i] as usize] = ch;
        }
    }

    pub fn iter(&self) -> core::slice::Iter<'_, char> {
        self.0.iter()
    }

    pub fn to_hashmap(&self) -> HashMap<char, u8> {
        self.iter()
            .enumerate()
            .filter(|(_, &ch)| ch != '_')
            .map(|(i, &d)| (d, i as u8))
            .collect()
    }
}

struct Equation<'a> {
    words: Vec<&'a str>,
    sum: &'a str,
    charset: Vec<char>,
    non_zero_chars: Vec<char>,
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

        let mut non_zero_chars: Vec<char> = words
            .iter()
            .chain(once(&sum))
            .filter(|&&w| w.len() > 1)
            .map(|&w| w.chars().next().unwrap())
            .collect::<HashSet<char>>()
            .iter()
            .copied()
            .collect();
        non_zero_chars.sort();

        Some(Self {
            words,
            sum,
            charset,
            non_zero_chars,
        })
    }

    fn try_key(&self, key: &CharKey) -> bool {
        if self.non_zero_chars.contains(&key.value()[0]) {
            return false;
        }
        if let (Some(words_sum), Some(sum)) = (
            self.words
                .iter()
                .map(|&w| word_to_number(w, key))
                .try_fold(0, |acc, d| Some(acc + d?)),
            word_to_number(self.sum, key),
        ) {
            return words_sum == sum;
        }

        false
    }
}

pub fn word_to_number(word: &str, key: &CharKey) -> Option<u64> {
    word.chars()
        .map(|ch| key.iter().position(|&kch| kch == ch))
        .try_fold(0u64, |acc, d| Some(acc * 10 + d? as u64))
}

#[test]
#[ignore]
fn test_word_to_number() {
    let key = CharKey(['_', 'L', '_', 'E', '_', '_', '_', '_', '_', 'S']);

    assert_eq!(word_to_number("LES", &key), Some(139));
    assert_eq!(word_to_number("SSS", &key), Some(999));
    assert_eq!(word_to_number("LA", &key), None);
    assert_eq!(word_to_number("", &key), Some(0));
    assert_eq!(word_to_number("L", &key), Some(1));
}
