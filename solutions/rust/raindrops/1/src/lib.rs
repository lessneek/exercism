pub fn raindrops(n: u32) -> String {
    let res = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|(x, _)| n % x == 0)
        .map(|(_, s)| *s)
        .collect::<String>();

    if res.is_empty() {
        return n.to_string();
    }

    return res;
}
