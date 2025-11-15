pub struct DedupDecDigitNumber {
    value: Vec<usize>,
    done: bool,
}

impl DedupDecDigitNumber {
    pub fn new(size: usize) -> Option<Self> {
        if size > 10 {
            return None;
        }
        let value: Vec<usize> = vec![0; size];
        Some(Self { value, done: true })
    }

    pub fn next(&mut self) -> Option<&[usize]> {
        if self.done {
            self.done = false;
            self.value.iter_mut().enumerate().for_each(|(i, v)| *v = i);
            return Some(self.value.as_slice());
        }

        loop {
            self.done = true;
            for v in self.value.iter_mut().rev() {
                *v += 1;
                if *v < 10 {
                    self.done = false;
                    break;
                }
                *v = 0;
            }
            if self.done {
                return None;
            }

            // Check for duplicate digits.
            if !has_duplicates(&self.value) {
                break;
            }
        }

        Some(self.value.as_slice())
    }
}

fn has_duplicates<T: PartialEq>(v: &[T]) -> bool {
    v.iter()
        .enumerate()
        .any(|(i, d)| v.iter().skip(i + 1).any(|dp| dp == d))
}

#[test]
fn test_dedup_dec_digit_number() {
    let mut num = DedupDecDigitNumber::new(4).unwrap();

    assert_eq!(num.next().unwrap(), [0, 1, 2, 3]);

    for _ in 0..7 {
        num.next();
    }

    assert_ne!(
        num.value,
        [0, 1, 3, 0],
        "Duplicates are not possible. So it should auto-inc until no duplicates."
    );
    assert_eq!(num.value, [0, 1, 3, 2]);
}

#[test]
fn test_visual_output() {
    let mut num = DedupDecDigitNumber::new(5).unwrap();
    for _ in 0..30 {
        println!("{:?}", num.next().unwrap());
    }
}
