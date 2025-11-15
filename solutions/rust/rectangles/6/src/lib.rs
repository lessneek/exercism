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
            // Searching for a left-top corner to start.
            // Processing always starts from a left-top corner.
            if matrix[y][x] != b'+' {
                continue;
            }

            // A left-top corner found. Go right.
            'right: for x_right in x + 1..width {
                match matrix[y][x_right] {
                    b'-' => continue 'right,
                    b'+' => {
                        // A right-top corner found. Go down.
                        'down: for y_down in (y + 1)..height {
                            match matrix[y_down][x_right] {
                                b'|' => continue 'down,
                                b'+' => {
                                    // Check if there is a left-bottom corner.
                                    if matrix[y_down][x] != b'+' {
                                        continue 'down;
                                    }
                                    // A right-bottom corner found.

                                    // Go left only checking an edge.
                                    'left: for x_left in (x..x_right).rev() {
                                        match matrix[y_down][x_left] {
                                            b'+' | b'-' => continue 'left,
                                            _ => break 'down,
                                        }
                                    }
                                    // A left-bottom corner found.

                                    // Go up only checking an edge.
                                    'up: for y_up in (y + 1..y_down).rev() {
                                        match matrix[y_up][x] {
                                            b'+' | b'|' => continue 'up,
                                            _ => break 'down,
                                        }
                                    }
                                    // A rectangle found.
                                    count += 1;
                                }
                                _ => break 'down,
                            }
                        }
                    }
                    _ => break 'right,
                }
            }
        }
    }

    count
}
