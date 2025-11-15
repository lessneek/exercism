#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use alphametics::CharKey;
    use test::Bencher;

    const WORDS: [&str; 9] = [
        "FFFFFFABCDEFGOCDCDFFFFFFFFO",
        "FAFAFAFAFAFAAAAAAAAAACGOOF",
        "ABFBFBBFBFFAFAFBAFG",
        "FFF",
        "AAA",
        "FA",
        "F",
        "GGGGGGGAAAAAAAAAABBBBBAAAAFFF",
        "EEEEEEEDAFFBFABFBABCBABCOCOAOC",
    ];

    #[bench]
    fn bench_word_to_number(b: &mut Bencher) {
        let key = CharKey::from(['B', 'A', 'D', 'E', '_', 'G', 'C', '_', 'O', 'F']);
        b.iter(|| {
            for word in WORDS {
                alphametics::word_to_number(word, &key);
            }
        });
    }

    #[bench]
    fn bench_dedup_dec_digit_number(b: &mut Bencher) {
        let n = test::black_box(100);
        b.iter(|| {
            let mut num = alphametics::DedupDecDigitNumber::new(10).unwrap();
            for _ in 0..n {
                num.next().unwrap();
            }
        });
    }
}
