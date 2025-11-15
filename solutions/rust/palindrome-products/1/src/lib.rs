/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if !is_palindrome(value) {
            return None;
        }
        Some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut palindromes = (min..=max)
        .flat_map(|a| (min..=max).filter_map(move |b| Palindrome::new(a * b)))
        .collect::<Vec<_>>();

    for pal in palindromes.iter() {
        println!("PAL: {:?}", pal);
    }

    if palindromes.len() > 1 {
        palindromes.sort_unstable();
        return Some((*palindromes.first().unwrap(), *palindromes.last().unwrap()));
    }
    None
}

fn is_palindrome(value: u64) -> bool {
    let vs = value.to_string();
    let count = vs.len() / 2 + 1;
    vs.chars()
        .take(count)
        .zip(vs.chars().rev().take(count))
        .all(|(a, b)| a == b)
}
