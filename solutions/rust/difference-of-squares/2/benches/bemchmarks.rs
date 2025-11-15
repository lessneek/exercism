#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use difference_of_squares::*;
    use rand::Rng;
    use test::Bencher;

    fn get_n() -> u64 {
        rand::thread_rng().gen_range(6_000..7_000)
    }

    fn get_n_for_sum_of_squares() -> u64 {
        rand::thread_rng().gen_range(3_300_000..3_500_000)
    }

    #[bench]
    fn bench_sum_of_squares_basic(b: &mut Bencher) {
        b.iter(|| sum_of_squares_basic(get_n_for_sum_of_squares()));
    }

    #[bench]
    fn bench_sum_of_squares_advanced(b: &mut Bencher) {
        b.iter(|| sum_of_squares_advanced(get_n_for_sum_of_squares()));
    }

    #[bench]
    fn bench_square_of_sum_basic(b: &mut Bencher) {
        b.iter(|| square_of_sum_basic(get_n()));
    }

    #[bench]
    fn bench_square_of_sum_advanced(b: &mut Bencher) {
        b.iter(|| square_of_sum_advanced(get_n()));
    }
}
