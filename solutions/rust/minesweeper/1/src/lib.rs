pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height > 0 { minefield[0].len() } else { 0 };

    let mut data: Vec<Vec<u8>> = minefield.iter().map(|&s| s.bytes().collect()).collect();
    let around_matrix: [i32; 3] = [-1, 0, 1];
    for i in 0..height {
        for j in 0..width {
            data[i][j] = match data[i][j] {
                b' ' => {
                    let i_shifts: Vec<usize> = around_matrix.iter()
                        .map(|&k| (i as i32 + k))
                        .filter(|&k| k >= 0 && k < height as i32)
                        .map(|k| k as usize)
                        .collect();
                    let j_shifts: Vec<usize> = around_matrix.iter()
                        .map(|&k| j as i32 + k)
                        .filter(|&k| k >= 0 && k < width as i32)
                        .map(|k| k as usize)
                        .collect();

                    let mut mines_around: u8 = b'0';

                    for x in i_shifts {
                        for &y in j_shifts.as_slice() {
                            if data[x][y] == b'*' {
                                mines_around += 1;
                            }
                        }
                    }
                    if mines_around != b'0' { mines_around } else { b' ' }
                }
                _ => data[i][j],
            };
        }
    }

    data.iter().map(|c| c.iter().map(|&c| c as char).collect()).collect()
}