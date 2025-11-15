use std::collections::HashMap;

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

    annotate_with_matrix(minefield, near_points_matrix)
}

pub fn annotate_with_matrix(minefield: &[&str], near_points_matrix: &[(Point, Vec<Point>)]) -> Vec<String> {
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

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub struct Size {
    pub height: usize,
    pub width: usize,
}

pub fn new_near_points_matrix(size: Size) -> Vec<(Point, Vec<Point>)> {
    (0..size.height).flat_map(move |x| {
        (0..size.width).map(move |y| {
            let point = Point { x, y };
            (point, get_near_points(&point, &size))
        }).collect::<Vec<(Point, Vec<Point>)>>()
    }).collect()
}

fn get_near_points(point: &Point, size: &Size) -> Vec<Point> {
    (-1..=1).flat_map(|a| (-1..=1).map(move |b| (a, b)))
        .filter(|&(x, y)| x != 0 || y != 0)
        .map(|(ox, oy)| (point.x as i32 + ox, point.y as i32 + oy))
        .filter(|&(x, y)| (0 <= x && x < size.height as i32) && (0 <= y && y < size.width as i32))
        .map(|(x, y)| Point { x: x as usize, y: y as usize })
        .collect::<Vec<_>>()
}
