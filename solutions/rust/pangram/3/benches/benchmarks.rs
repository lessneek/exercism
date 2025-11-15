#![feature(test)]

extern crate test;

use pangram::*;
use test::Bencher;

const SENTENCE: &str = "the quick brown fox jumps over the lazy dog";

#[bench]
pub fn benchmark_is_pangram_by_hashset(b: &mut Bencher) {
    b.iter(|| is_pangram_by_hashset(SENTENCE))
}

#[bench]
pub fn benchmark_is_pangram_by_shifting(b: &mut Bencher) {
    b.iter(|| is_pangram_by_shifting(SENTENCE))
}

#[bench]
pub fn benchmark_is_pangram_sacherjj(b: &mut Bencher) {
    b.iter(|| sacherjj::is_pangram(SENTENCE))
}

#[bench]
pub fn benchmark_is_pangram_boso(b: &mut Bencher) {
    b.iter(|| boso::is_pangram(SENTENCE))
}

// > rustup run nightly cargo bench
// test benchmark_is_pangram_by_hashset  ... bench:         816 ns/iter (+/- 32)
// test benchmark_is_pangram_by_shifting ... bench:          29 ns/iter (+/- 1)
// test benchmark_is_pangram_sacherjj    ... bench:          95 ns/iter (+/- 5)