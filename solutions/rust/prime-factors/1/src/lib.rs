pub fn factors(n: u64) -> Vec<u64> {
    let mut res = vec![];
    if n < 2 {
        return res;
    }
    let mut d = 2u64;
    let mut f = n;
    loop {
        if f % d == 0 {
            f /= d;
            res.push(d);
        } else {
            d += 1;
            if f < d {
                break;
            }
        }
    }
    res
}
