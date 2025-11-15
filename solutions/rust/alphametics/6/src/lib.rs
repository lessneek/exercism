pub mod combinations2;
pub mod old;
pub mod others;
pub mod solver4;

use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solver4::solve(input)
}
