pub fn count(lines: &[&str]) -> u32 {
    let height = lines.len();
    if height == 0 {
        return 0;
    }

    let matrix: Vec<&[u8]> = lines.iter().map(|&line| line.as_bytes()).collect();

    let width = matrix[0].len();
    if !matrix.iter().all(|&line| line.len() == width) {
        panic!("Incorrect lines.")
    }

    let mut count = 0u32;

    for y in 0..height - 1 {
        for x in 0..width - 1 {
            // Searching for the left-top corner to start.
            // Processing always starts from the left-top corner.
            if matrix[y][x] != b'+' {
                continue;
            }

            // The left-top corner found. Go right.
            for x_right in x + 1..width {
                match matrix[y][x_right] {
                    b'+' => {
                        // The right-top corner found. Go down.
                        for y_down in (y + 1)..height {
                            match matrix[y_down][x_right] {
                                b'+' => {
                                    // The right-bottom corner found. Go left.
                                    for x_left in (x..x_right).rev() {
                                        match matrix[y_down][x_left] {
                                            b'+' => {
                                                // The left-bottom corner found. Go up.
                                                for y_up in (y..y_down).rev() {
                                                    match matrix[y_up][x_left] {
                                                        b'+' => {
                                                            if x_left == x && y_up == y {
                                                                // Rectangle found.
                                                                count += 1;
                                                                break;
                                                            }
                                                        }
                                                        b'|' => continue,
                                                        _ => break,
                                                    }
                                                }
                                            }
                                            b'-' => continue,
                                            _ => break,
                                        }
                                    }
                                }
                                b'|' => continue,
                                _ => break,
                            }
                        }
                    }
                    b'-' => continue,
                    _ => break,
                }
            }
        }
    }

    count
}
