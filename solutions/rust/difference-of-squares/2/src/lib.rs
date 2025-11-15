pub fn square_of_sum(n: u64) -> u64 {
    square_of_sum_basic(n)
}

pub fn sum_of_squares(n: u64) -> u64 {
    sum_of_squares_basic(n)
}

pub fn difference(n: u64) -> u64 {
    square_of_sum(n) - sum_of_squares(n)
}

pub fn square_of_sum_basic(n: u64) -> u64 {
    (1..=n).sum::<u64>().pow(2)
}

pub fn sum_of_squares_basic(n: u64) -> u64 {
    (1..=n).map(|x| x * x).sum()
}

// According to https://en.wikipedia.org/wiki/Square_pyramidal_number
pub fn sum_of_squares_advanced(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}
// According to https://en.wikipedia.org/wiki/1_%2B_2_%2B_3_%2B_4_%2B_%E2%8B%AF
pub fn square_of_sum_advanced(n: u64) -> u64 {
    (n * (n + 1) / 2).pow(2)
}
