pub mod combinations2;
pub mod old;
pub mod others;
pub mod solver5;

use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solver5::solve(input)
}
