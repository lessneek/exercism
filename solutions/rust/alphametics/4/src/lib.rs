pub mod combinations;
pub mod others;
pub mod slow;
pub mod solver;

use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    solver::solve(input)
}
