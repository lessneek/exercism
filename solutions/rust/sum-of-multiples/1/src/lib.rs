pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit)
        .filter(|&x| factors.iter().filter(|&&f| f > 0).any(|&f| x % f == 0))
        .sum()
}
