/// Old version of combinations.

pub struct ItemsIndeces {
    ixv: Vec<usize>,
    ix: usize,
}

impl ItemsIndeces {
    #[inline]
    pub fn value(&self) -> usize {
        self.ixv[self.ix]
    }

    #[inline]
    pub fn inc(&mut self) -> bool {
        if self.ix == self.ixv.len() - 1 {
            return false;
        }
        self.ix += 1;
        true
    }

    #[inline]
    pub fn reset(&mut self) {
        self.ix = 0;
    }

    #[inline]
    pub fn inc_or_reset(&mut self) -> bool {
        if !self.inc() {
            self.reset();
            return false;
        }
        true
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.ixv.len()
    }
}

pub struct Combinations<T> {
    size: usize,
    items: Vec<T>,
    value: Vec<T>,
    itix: Vec<ItemsIndeces>,
}

impl<T: PartialEq + Copy> Combinations<T> {
    pub fn new(size: usize, items: &[T]) -> Option<Self> {
        if size > items.len() {
            return None;
        }
        let com = Self {
            size,
            items: Vec::from(items),
            value: items.iter().take(size).copied().collect(),
            itix: (0..size)
                .map(|i| ItemsIndeces {
                    ixv: (i..items.len()).collect(),
                    ix: 0,
                })
                .collect(),
        };
        Some(com)
    }

    pub fn value(&self) -> &[T] {
        &self.value
    }

    pub fn inc(&mut self) -> bool {
        for i in (0..self.size).rev() {
            if !self.itix[i].inc_or_reset() {
                if i == 0 {
                    return false;
                }
                continue;
            }

            let mut zeros = 0usize;
            for j in i..self.size - 1 {
                if self.itix[j].ix == 0 {
                    zeros += 1;

                    for z in (0..zeros).rev() {
                        self.itix[j + 1].ixv.rotate_left(1);
                        *self.itix[j + 1].ixv.last_mut().unwrap() =
                            *self.itix[i + 1].ixv.iter().nth_back(z).unwrap();
                    }

                    if let Some(pix) = self.itix[j - zeros].ix.checked_sub(zeros) {
                        if pix > 0 && pix < self.itix[j + 1].ixv.len() {
                            self.itix[j + 1].ixv[pix - 1] = self.itix[j].ixv[pix];
                        }
                    }
                } else {
                    zeros = 0;
                    let k = self.itix[j].ix - 1;
                    self.itix[j + 1].ixv[k] = self.itix[j].ixv[k];
                }
            }

            break;
        }

        self.update_value();
        true
    }

    fn update_value(&mut self) {
        for (vix, iii) in self.itix.iter().enumerate() {
            self.value[vix] = self.items[iii.value()];
        }
    }
}

impl Combinations<u8> {
    pub fn of_dec_digits(size: usize) -> Option<Combinations<u8>> {
        Combinations::<u8>::new(size, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
    }
}

#[inline]
#[cfg(test)]
#[cfg(feature="other_tests")]
fn has_duplicates<T: PartialEq>(v: &[T]) -> bool {
    v.iter()
        .enumerate()
        .any(|(i, d)| v.iter().skip(i + 1).any(|dp| dp == d))
}

#[test]
#[ignore]
#[cfg(feature="other_tests")]
fn test_combinations_visual_output() {
    let mut num = Combinations::of_dec_digits(7).unwrap();
    let mut dups = 0usize;

    println!("Items: {:?}", num.items);

    for i in 1usize.. {
        println!(
            "{0}: {1:?}  ix: {2:?} {3}",
            i,
            num.value(),
            num.itix.iter().map(|x| x.ix).collect::<Vec<_>>(),
            if has_duplicates(num.value()) {
                dups += 1;
                "DUP!"
            } else {
                ""
            }
        );
        if !num.inc() {
            break;
        }
    }
    println!("DUPS: {dups}");
}

#[test]
#[ignore]
#[cfg(feature="other_tests")]
fn test_combinations_for_dups() {
    for count in 1..=9 {
        let mut num = Combinations::of_dec_digits(count).unwrap();
        let mut set = std::collections::HashSet::<Vec<u8>>::new();

        loop {
            assert!(
                !has_duplicates(num.value()),
                "Duplicate items are not allowed. [{:?}]",
                num.value()
            );
            assert!(
                set.insert(Vec::from(num.value())),
                "Must not repeat values. {:?}",
                num.value()
            );
            if !num.inc() {
                break;
            }
        }
    }
}
