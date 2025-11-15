pub fn factors(mut n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }
    let mut res = vec![];
    let mut d = 2u64;
    while n >= d {
        while n % d == 0 {
            n /= d;
            res.push(d);
        }
        d += 1;
    }
    res
}
