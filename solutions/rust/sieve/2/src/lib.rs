pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound < 2 {
        return vec![];
    }
    let n = upper_bound as usize;

    let mut flags = vec![true; n + 1];

    for i in 2..=(n as f64).sqrt() as usize {
        if !flags[i] {
            continue;
        }
        let mut j = i * i;
        while j <= n {
            flags[j] = false;
            j += i;
        }
    }

    flags
        .iter()
        .enumerate()
        .skip(2)
        .filter(|(_, &f)| f)
        .map(|(i, _)| i as u64)
        .collect()
}
