#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use grains::*;
    use test::Bencher;

    use crate::*;

    #[bench]
    fn bench_square_by_shift(b: &mut Bencher) {
        b.iter(|| square_by_shift(64));
    }

    #[bench]
    fn bench_square_by_pow(b: &mut Bencher) {
        b.iter(|| square_by_pow(64));
    }

    #[bench]
    fn bench_square_by_table(b: &mut Bencher) {
        b.iter(|| square_by_table(64));
    }
}
