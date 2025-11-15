// rustup run nightly cargo bench
// test tests::bench_jgilray   ... bench:       1,020 ns/iter (+/- 43)
// test tests::bench_lessneek  ... bench:         573 ns/iter (+/- 30)
// test tests::bench_minhdoboi ... bench:       4,378 ns/iter (+/- 65)
// test tests::bench_nmeuleman ... bench:      11,594 ns/iter (+/- 453)

#![feature(test)]
#![feature(option_result_contains)]

extern crate test;

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::*;

    const LINES: &[&str] = &[
        "+---+--+----+",
        "|   +--+----+",
        "+---+--+    |",
        "|   +--+----+",
        "+---+--+--+-+",
        "+---+--+--+-+",
        "+------+  | |",
        "          +-+",
    ];

    #[bench]
    fn bench_lessneek(b: &mut Bencher) {
        b.iter(|| rectangles::count(LINES));
    }

    #[bench]
    fn bench_nmeuleman(b: &mut Bencher) {
        b.iter(|| nmeuleman::count(LINES));
    }

    #[bench]
    fn bench_jgilray(b: &mut Bencher) {
        b.iter(|| jgilray::count(LINES));
    }

    #[bench]
    fn bench_minhdoboi(b: &mut Bencher) {
        b.iter(|| minhdoboi::count(LINES));
    }
}

mod nmeuleman {
    pub fn count(input: &[&str]) -> usize {
        // loop through all corner (row, col) indexes
        // treat them as the top_left corner of potential rectangles
        // loop through all row_offset and col_offset so the potential rectangle is 1 + row_offset high and 1 + col_offset wide
        // check if a potential rectangle has correct corners and correct edges, if it does, that's a bingo
        let corners = input.iter().enumerate().flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, c)| if c == '+' { Some((row, col)) } else { None })
        });
        let rows = input.len();
        let cols = input.get(0).map(|line| line.len()).unwrap_or(0);
        corners
            .flat_map(|(row, col)| {
                (1..)
                    .take_while(move |i| i + col < cols)
                    .flat_map(move |col_offset| {
                        (1..)
                            .take_while(move |i| i + row < rows)
                            .filter(move |&row_offset| {
                                let top_left = (row, col);
                                let top_right = (row, col + col_offset);
                                let bottom_left = (row + row_offset, col);
                                let bottom_right = (row + row_offset, col + col_offset);
                                let corners = [top_left, top_right, bottom_left, bottom_right];
                                has_corners(input, corners) && has_edges(input, corners)
                            })
                    })
            })
            .count()
    }

    fn has_corners(input: &[&str], corners: [(usize, usize); 4]) -> bool {
        corners
            .iter()
            .all(|&(row, col)| input[row].chars().nth(col).contains(&'+'))
    }

    fn has_edges(input: &[&str], corners: [(usize, usize); 4]) -> bool {
        // first line is between top_left and top_right, second is between bottom_left and bottom_right
        let horizontal_points = [(corners[0], corners[1]), (corners[2], corners[3])];
        // first line is between top_left and bottom_left, second is between top_right and bottom_right
        let vertical_points = [(corners[0], corners[2]), (corners[1], corners[3])];
        horizontal_points
            .iter()
            .map(|&((row, col_left), (_, col_right))| {
                input[row]
                    .chars()
                    .skip(col_left)
                    .take((col_right - col_left) + 1)
            })
            .all(|mut edge| edge.all(|c| c == '+' || c == '-'))
            .then(|| {
                vertical_points
                    .iter()
                    .map(|&((row_top, col), (row_bottom, _))| {
                        input
                            .iter()
                            .filter_map(move |line| line.chars().nth(col))
                            .skip(row_top)
                            .take((row_bottom - row_top) + 1)
                    })
                    .all(|mut edge| edge.all(|c| c == '+' || c == '|'))
            })
            .contains(&true)
    }
}

mod jgilray {
    pub fn count(lines: &[&str]) -> u32 {
        // create a vector for the data
        let v: Vec<Vec<char>> = lines.iter().map(|row| row.chars().collect()).collect();
        // count rectangles in v
        let mut count = 0;
        for top in 0..v.len() {
            for left in 0..v[top].len() {
                if v[top][left] == '+' {
                    // found upper left corner
                    for right in left + 1..v[top].len() {
                        if v[top][right] == '+' {
                            // found upper right corner
                            for side in top + 1..v.len() {
                                match (v[side][left], v[side][right]) {
                                    ('+', '+') => {
                                        // found possible rectangle, check bottom
                                        if v[side][left + 1..right].iter().all(|c| *c == '-' || *c == '+') {
                                            count += 1;
                                        } else {
                                            continue;
                                        }
                                    }
                                    ('+', '|') => continue,
                                    ('|', '+') => continue,
                                    ('|', '|') => continue,
                                    (_, _) => break,  // invalid rectangle side
                                }
                            }
                        } else if v[top][right] != '-' {
                            // invalid rectangle top
                            break;
                        }
                    }
                }
            }
        }
        count
    }
}

mod minhdoboi {
    pub fn count(v: &[&str]) -> usize {
        let mut corners = Vec::new();
        for i in 0..v.len() {
            for (j, c) in v[i].chars().enumerate() {
                if c == '+' {
                    corners.push((i, j));
                }
            }
        }
        let mut candidates = Vec::new();
        for &c in &corners {
            for &c2 in &corners {
                if c2.0 == c.0 && c2.1 > c.1 {
                    for &c3 in &corners {
                        if c3.1 == c2.1 && c3.0 > c2.0 {
                            for &c4 in &corners {
                                if c4.0 == c3.0 && c4.1 < c3.1 && c4.1 == c.1 {
                                    candidates.push((c, c2, c3, c4))
                                }
                            }
                        }
                    }
                }
            }
        }
        candidates
            .into_iter()
            .filter(|&candidate| {
                let (c1, c2, c3, c4) = candidate;
                v[c1.0][c1.1..c2.1].chars().all(|c| c == '-' || c == '+')
                    && v[c3.0][c4.1..c3.1].chars().all(|c| c == '-' || c == '+')
                    && v[c1.0..c4.0].iter().all(|s| nchar_is_vfilled(s, c1.1))
                    && v[c2.0..c3.0].iter().all(|s| nchar_is_vfilled(s, c2.1))
            }).count()
    }

    fn nchar_is_vfilled(s: &str, n: usize) -> bool {
        let c = s.chars().nth(n).unwrap().clone();
        c == '|' || c == '+'
    }
}