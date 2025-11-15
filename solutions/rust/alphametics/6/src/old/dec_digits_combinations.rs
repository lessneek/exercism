/// SLOW. Do not use.
/// Only for history of algorithm's evolution.

pub type DigitType = u8;

pub struct DecDigitsCombinations {
    value: Vec<DigitType>,
    done: bool,
}

impl DecDigitsCombinations {
    pub fn new(size: usize) -> Option<Self> {
        if size > 10 {
            return None;
        }
        let value: Vec<DigitType> = vec![0; size];
        let mut digits = Self { value, done: true };
        digits.inc();
        Some(digits)
    }

    pub fn value(&self) -> &[DigitType] {
        &self.value
    }

    pub fn inc(&mut self) -> bool {
        if self.done {
            self.done = false;
            self.value
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = i as DigitType);
            return true;
        }

        loop {
            let mut done = true;

            for v in self.value.iter_mut().rev() {
                *v += 1;
                if *v < 10 {
                    done = false;
                    break;
                }
                *v = 0;
            }

            if done {
                self.done = true;
                return false;
            }

            // Check for duplicate digits.
            if !has_duplicates(&self.value) {
                break;
            }
        }
        true
    }
}

#[inline]
fn has_duplicates<T: PartialEq>(v: &[T]) -> bool {
    v.iter()
        .enumerate()
        .any(|(i, d)| v.iter().skip(i + 1).any(|dp| dp == d))
}

#[test]
#[ignore]
fn test_dec_digits_combinations() {
    for count in 2..=9 {
        let mut num = DecDigitsCombinations::new(count).unwrap();
        loop {
            assert!(
                !has_duplicates(num.value()),
                "Duplicates are not allowed. [{:?}]",
                num.value()
            );
            if !num.inc() {
                break;
            }
        }
    }
}

#[test]
#[ignore]
fn test_visual_output() {
    let mut num = DecDigitsCombinations::new(5).unwrap();
    for _ in 0..30 {
        println!("{:?}", num.value());
        num.inc();
    }
}
