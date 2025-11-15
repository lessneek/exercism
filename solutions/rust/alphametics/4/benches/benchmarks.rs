#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use alphametics::{combinations::combinations_of_dec_digits, others, slow, solver};
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

    const EQUATION_7_LETTERS: &str = "HE + SEES + THE == LIGHT";
    const EQUATION_8_LETTER: &str = "SEND + MORE == MONEY";
    const EQUATION_10_LETTERS: &str = "AND + A + STRONG + OFFENSE + AS + A + GOOD == DEFENSE";
    const EQUATION_10_LETTERS_WITH_199_ADDENS: &str = "THIS + A + FIRE + THEREFORE + FOR + ALL + HISTORIES + I + TELL + A + TALE + THAT + FALSIFIES + ITS + TITLE + TIS + A + LIE + THE + TALE + OF + THE + LAST + FIRE + HORSES + LATE + AFTER + THE + FIRST + FATHERS + FORESEE + THE + HORRORS + THE + LAST + FREE + TROLL + TERRIFIES + THE + HORSES + OF + FIRE + THE + TROLL + RESTS + AT + THE + HOLE + OF + LOSSES + IT + IS + THERE + THAT + SHE + STORES + ROLES + OF + LEATHERS + AFTER + SHE + SATISFIES + HER + HATE + OFF + THOSE + FEARS + A + TASTE + RISES + AS + SHE + HEARS + THE + LEAST + FAR + HORSE + THOSE + FAST + HORSES + THAT + FIRST + HEAR + THE + TROLL + FLEE + OFF + TO + THE + FOREST + THE + HORSES + THAT + ALERTS + RAISE + THE + STARES + OF + THE + OTHERS + AS + THE + TROLL + ASSAILS + AT + THE + TOTAL + SHIFT + HER + TEETH + TEAR + HOOF + OFF + TORSO + AS + THE + LAST + HORSE + FORFEITS + ITS + LIFE + THE + FIRST + FATHERS + HEAR + OF + THE + HORRORS + THEIR + FEARS + THAT + THE + FIRES + FOR + THEIR + FEASTS + ARREST + AS + THE + FIRST + FATHERS + RESETTLE + THE + LAST + OF + THE + FIRE + HORSES + THE + LAST + TROLL + HARASSES + THE + FOREST + HEART + FREE + AT + LAST + OF + THE + LAST + TROLL + ALL + OFFER + THEIR + FIRE + HEAT + TO + THE + ASSISTERS + FAR + OFF + THE + TROLL + FASTS + ITS + LIFE + SHORTER + AS + STARS + RISE + THE + HORSES + REST + SAFE + AFTER + ALL + SHARE + HOT + FISH + AS + THEIR + AFFILIATES + TAILOR + A + ROOFS + FOR + THEIR + SAFE == FORTRESSES";

    const EQUATIONS: [&str; 4] = [
        EQUATION_7_LETTERS,
        EQUATION_8_LETTER,
        EQUATION_10_LETTERS,
        EQUATION_10_LETTERS_WITH_199_ADDENS,
    ];

    #[bench]
    fn bench_word_to_number_solver_1(b: &mut Bencher) {
        let key = slow::solver::CharKey::from(['B', 'A', 'D', 'E', '_', 'G', 'C', '_', 'O', 'F']);
        b.iter(|| {
            for word in WORDS {
                slow::solver::word_to_number(word, &key);
            }
        });
    }

    const DIGITS_COUNT: usize = 7;

    #[bench]
    fn bench_dedup_dec_digit_number(b: &mut Bencher) {
        b.iter(|| {
            let mut num = slow::digits::DedupDecDigits::new(DIGITS_COUNT).unwrap();
            while num.inc() {}
        });
    }

    #[bench]
    fn bench_combinations(b: &mut Bencher) {
        b.iter(|| {
            let mut num = combinations_of_dec_digits(DIGITS_COUNT).unwrap();
            while num.inc() {}
        });
    }

    #[bench]
    #[ignore]
    fn bench_slow_solver(b: &mut Bencher) {
        b.iter(|| {
            for equation in EQUATIONS {
                slow::solver::solve(equation);
            }
        });
    }

    #[bench]
    fn bench_solver(b: &mut Bencher) {
        b.iter(|| {
            for equation in EQUATIONS {
                solver::solve(equation);
            }
        });
    }

    #[bench]
    fn bench_solver_fungkwe(b: &mut Bencher) {
        b.iter(|| {
            for equation in EQUATIONS {
                others::fungkwe::solve(equation);
            }
        });
    }
}
