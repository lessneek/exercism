pub fn is_armstrong_number(num: u32) -> bool {
    let num = num as u64;
    let digits = num
        .to_string()
        .chars()
        .map(|c| c.to_string().parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let l = digits.len() as u32;

    num == digits.iter().fold(0u64, |acc, &d| acc + (d as u64).pow(l))
}
