use std::collections::HashMap;
use std::io::repeat;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut cache = NearPointsMatricesCache::new();
    annotate_cached(minefield, &mut cache)
}

pub fn annotate_cached(minefield: &[&str], cache: &mut NearPointsMatricesCache) -> Vec<String> {
    let size = Size {
        height: minefield.len(),
        width: if !minefield.is_empty() { minefield[0].len() } else { 0 },
    };

    let near_points_matrix =
        cache.entry(size).or_insert_with_key(|key| new_near_points_matrix(*key));

    let mut result: Vec<String> = minefield.iter().map(|&row| row.to_string()).collect();

    for (point, near_points) in near_points_matrix.iter() {
        if result[point.x].as_bytes()[point.y] == b'*' {
            continue;
        }
        let mut near_mines_count: u8 = 0;
        for near_point in near_points {
            if result[near_point.x].as_bytes()[near_point.y] == b'*' {
                near_mines_count += 1;
            }
        }
        if near_mines_count > 0 {
            unsafe { result[point.x].as_bytes_mut()[point.y] = near_mines_count + b'0'; }
        }
    }

    result
}

pub type NearPointsMatricesCache = HashMap<Size, Vec<(Point, Vec<Point>)>>;

static NEAR_OFFSETS: &[(i32, i32)] =
    &[(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Size {
    height: usize,
    width: usize,
}

fn new_near_points_matrix(size: Size) -> Vec<(Point, Vec<Point>)> {
    (0..size.height).flat_map(move |x| {
        (0..size.width).map(move |y| {
            let point = Point { x, y };
            (point, get_near_points(&point, &size))
        }).collect::<Vec<(Point, Vec<Point>)>>()
    }).collect()
}

fn get_near_points(point: &Point, size: &Size) -> Vec<Point> {
    NEAR_OFFSETS.iter()
        .map(|&(ox, oy)| (point.x as i32 + ox, point.y as i32 + oy))
        .filter(|&(x, y)| (0 <= x && x < size.height as i32) && (0 <= y && y < size.width as i32))
        .map(|(x, y)| Point { x: x as usize, y: y as usize })
        .collect::<Vec<_>>()
}

#[test]
#[ignore]
fn bench_many_boards_with_matrix() {
    let mut cache = NearPointsMatricesCache::new();
    for _ in 0..1_000_000 {
        #[rustfmt::skip]
        annotate_cached(&[
            " *  * ",
            "  *   ",
            "    * ",
            "   * *",
            " *  * ",
            "      ",
        ], &mut cache);
    }
}