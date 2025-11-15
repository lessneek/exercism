#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use crate::*;
    use test::Bencher;

    const SIZE: usize = 100;

    // test tests::bench_denenr      ... bench:      18,911 ns/iter (+/- 581)
    // test tests::bench_lessneek    ... bench:      14,058 ns/iter (+/- 1,134)
    // test tests::bench_punkstarman ... bench:     328,206 ns/iter (+/- 11,011)
    // test tests::bench_yawpitch    ... bench:      10,835 ns/iter (+/- 492)

    #[bench]
    fn bench_lessneek(b: &mut Bencher) {
        b.iter(|| spiral_matrix::spiral_matrix(SIZE));
    }

    #[bench]
    fn bench_denenr(b: &mut Bencher) {
        b.iter(|| denenr::spiral_matrix(SIZE));
    }

    #[bench]
    fn bench_yawpitch(b: &mut Bencher) {
        b.iter(|| yawpitch::spiral_matrix(SIZE));
    }

    #[bench]
    fn bench_punkstarman(b: &mut Bencher) {
        b.iter(|| punkstarman::spiral_matrix(SIZE as u32));
    }
}

mod denenr {
    use std::iter;

    const VECTORS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
        let mut matrix = vec![vec![0; size]; size];

        let mut movement = VECTORS.iter().cycle();

        let (mut x, mut y, mut n) = (-1, 0, 1..);

        for (move_x, move_y) in iter::once(size)
            .chain((1..size).rev().flat_map(|n| iter::repeat(n).take(2)))
            .flat_map(|steps| iter::repeat(movement.next().unwrap()).take(steps))
        {
            x += move_x;
            y += move_y;

            matrix[y as usize][x as usize] = n.next().unwrap();
        }

        matrix
    }
}

mod yawpitch {
    static VECTORS_X: [isize; 4] = [0, 1, 0, -1];
    static VECTORS_Y: [isize; 4] = [1, 0, -1, 0];

    pub fn spiral_matrix(size: usize) -> Vec<Vec<u32>> {
        let mut result: Vec<Vec<u32>> = vec![vec![0; size]; size];

        if size == 0 {
            return result;
        }

        let mut x = 0isize;
        let mut y = -1isize;
        let mut v = 1u32;

        for i in 0..(size + size - 1) {
            for _ in 0..((size + size - i) / 2) {
                x += VECTORS_X[i % 4];

                y += VECTORS_Y[i % 4];

                result[x as usize][y as usize] = v;

                v += 1;
            }
        }

        result
    }
}

mod punkstarman {
    fn spiral(width: u32, height: u32, x: u32, y: u32) -> u32 {
        if y == 0 {
            x + 1
        } else {
            width + spiral(height - 1, width, y - 1, width - x - 1)
        }
    }

    pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
        (0..size)
            .map(|y| (0..size).map(|x| spiral(size, size, x, y)).collect())
            .collect()
    }
}
