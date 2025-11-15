#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    match (1..=num / 2).filter(|x| num % x == 0).sum::<u64>() {
        sum if sum == num => Some(Classification::Perfect),
        sum if sum > num => Some(Classification::Abundant),
        sum if sum < num => Some(Classification::Deficient),
        _ => None,
    }
}
