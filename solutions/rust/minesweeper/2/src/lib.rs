pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };

    (0..height).map(|i| {
        let row = minefield[i].as_bytes();
        let i_offsets = get_offsets(i, height);
        (0..width).map(|j| {
            if row[j] == b'*' {
                return '*';
            }
            let j_offsets = get_offsets(j, width);

            let mut mines_around: u8 = 0;
            for &x in &i_offsets {
                for &y in &j_offsets {
                    if minefield[x].as_bytes()[y] == b'*' {
                        mines_around += 1;
                    }
                }
            }
            if mines_around == 0 {
                return ' ';
            }
            (mines_around + b'0') as char
        }).collect()
    }).collect()
}

fn get_offsets(base: usize, limit: usize) -> Vec<usize> {
    if base >= limit {
        vec!()
    } else if base == 0 && limit == 1 {
        vec![0]
    } else if base == 0 {
        vec![base, base + 1]
    } else if base + 1 < limit {
        vec![base - 1, base, base + 1]
    } else if base + 1 == limit {
        vec![base - 1, base]
    } else { vec!() }
}
