enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    use Direction::*;
    let mut matrix = vec![vec![0; size]; size];

    let mut counter = 1u32;

    let mut right_lim = size - 1;
    let mut down_lim = size - 1;
    let mut left_lim = 0usize;
    let mut up_lim = 0usize;

    let mut row = 0usize;
    let mut column = 0usize;
    let mut dir = Right;

    'run: loop {
        matrix[row][column] = counter;
        counter += 1;

        match dir {
            Right => {
                if column == right_lim {
                    if row == down_lim {
                        break 'run;
                    }
                    dir = Down;
                    row += 1;
                    up_lim += 1;
                } else {
                    column += 1;
                }
            }
            Down => {
                if row == down_lim {
                    if column == left_lim {
                        break 'run;
                    }
                    dir = Left;
                    column -= 1;
                    right_lim -= 1;
                } else {
                    row += 1;
                }
            }
            Left => {
                if column == left_lim {
                    if row == up_lim {
                        break 'run;
                    }
                    dir = Up;
                    row -= 1;
                    down_lim -= 1;
                } else {
                    column -= 1;
                }
            }
            Up => {
                if row == up_lim {
                    if column == right_lim {
                        break 'run;
                    }
                    dir = Right;
                    column += 1;
                    left_lim += 1;
                } else {
                    row -= 1;
                }
            }
        }
    }

    matrix
}
