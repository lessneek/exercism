pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    (dividend / divisor, dividend % divisor)
}

pub fn evens<'a, T>(iter: impl Iterator<Item=T>) -> impl Iterator<Item=T>
    where T: 'a {
    iter.enumerate().filter(|(i, _)| (i % 2) == 0).map(|(_, x)| x)
}

pub struct Position(pub i16, pub i16);

impl Position {
    pub fn manhattan(&self) -> i16 {
        (self.0.unsigned_abs() + self.1.unsigned_abs()) as i16
    }
}
