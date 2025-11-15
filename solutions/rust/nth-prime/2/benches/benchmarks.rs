#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use nth_prime::*;
    use test::Bencher;

    use crate::*;

    const N_PRIME: u32 = 1_000;

    #[bench]
    fn bench_lessneek(b: &mut Bencher) {
        b.iter(|| nth(N_PRIME));
    }

    #[bench]
    fn bench_briankung(b: &mut Bencher) {
        b.iter(|| nth_briankung(N_PRIME));
    }

    #[bench]
    fn bench_parhamrm(b: &mut Bencher) {
        b.iter(|| nth_parhamrm(N_PRIME));
    }
}

pub fn nth_briankung(n: u32) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|candidate: &u32| {
            if !primes.iter().any(|i| candidate % i == 0) {
                primes.push(*candidate);
                true
            } else {
                false
            }
        })
        .nth(n as usize)
        .unwrap()
}

pub fn nth_parhamrm(n: u32) -> u32 {
    fn is_prime(number: u32) -> bool {
        !(2..=(number as f32).sqrt() as u32).any(|x| number % x == 0)
    }
    (2..).filter(|x| is_prime(*x)).nth(n as usize).unwrap()
}
