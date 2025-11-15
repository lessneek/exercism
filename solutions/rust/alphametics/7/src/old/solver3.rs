use std::{collections::HashMap, iter::once};

use crate::combinations2::Combinations;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let equ = Equation::parse(input)?;
    equ.solve()
}

pub struct VChar {
    ch: char,
    value: i64,
    non_zero: bool,
}

pub struct Equation {
    vchars: Vec<VChar>,
}

impl Equation {
    pub fn parse(input: &str) -> Option<Self> {
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

        let mut vchars: Vec<VChar> = words
            .iter()
            .map(|&w| (w, 1i64))
            .chain(once((sum, -1i64)))
            .fold(
                HashMap::<char, VChar>::new(),
                |mut map, (word, mut value)| {
                    for ch in word.chars().rev() {
                        map.entry(ch)
                            .and_modify(|vch| vch.value += value)
                            .or_insert(VChar {
                                ch,
                                value,
                                non_zero: false,
                            });
                        value *= 10;
                    }
                    map.entry(word.chars().next().unwrap())
                        .and_modify(|vch| vch.non_zero = true);
                    map
                },
            )
            .into_values()
            .collect();

        vchars.sort_by_key(|vch| vch.ch);

        Some(Self { vchars })
    }

    pub fn solve(&self) -> Option<HashMap<char, u8>> {
        let mut comb = Combinations::of_dec_digits(self.vchars.len())?;
        loop {
            let key = comb.value();

            if self.try_key(key) {
                return Some(
                    self.vchars
                        .iter()
                        .zip(key.iter().copied())
                        .map(|(vch, digit)| (vch.ch, digit))
                        .collect(),
                );
            }

            if !comb.inc() {
                break;
            }
        }
        None
    }

    pub fn try_key(&self, key: &[u8]) -> bool {
        self.vchars
            .iter()
            .zip(key.iter())
            .try_fold(0i64, |mut acc, (vch, &digit)| {
                if digit == 0 && vch.non_zero {
                    return None;
                }
                acc += vch.value * digit as i64;
                Some(acc)
            })
            .map_or(false, |res| res == 0)
    }
}
