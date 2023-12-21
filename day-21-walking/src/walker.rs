use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug)]
enum Tile {
    Start,
    Garden,
    Rock,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Point(i64, i64);

impl Point {
    fn next(&self, grid: &Grid) -> Vec<Point> {
        STEPS
            .iter()
            .map(|step| {
                let row = self.0 + step.0;
                let col = self.1 + step.1;
                let point = Point(row, col);
                if row < 0 || col < 0 || row as usize >= grid.len() || col as usize >= grid[0].len()
                {
                    return None;
                }
                match grid[row as usize][col as usize] {
                    Tile::Rock => None,
                    _ => Some(point),
                }
            })
            .flatten()
            .collect()
    }
}

type Grid = Vec<Vec<Tile>>;

const STEPS: [(i64, i64); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn count_tiles(input: &str, steps: usize) -> usize {
    let mut start: Point = Point(0, 0);
    let grid: Grid = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .inspect(|(col, b)| {
                    if b.to_owned().eq(&b'S') {
                        start = Point(row as i64, *col as i64);
                    }
                })
                .map(|(_col, b)| match b {
                    b'S' => Tile::Start,
                    b'.' => Tile::Garden,
                    b'#' => Tile::Rock,
                    _ => panic!("Unknown tile {}", char::from(*b)),
                })
                .collect()
        })
        .collect();

    let mut visited: HashMap<Point, usize> = HashMap::from([(start.clone(), 0)]);
    let mut stack = vec![start.next(&grid)];
    let mut step = 0;
    while let Some(points) = stack.pop() {
        step += 1;
        if step > steps {
            break;
        }
        let mut next = vec![];
        for point in points {
            if visited.contains_key(&point) {
                continue;
            }
            visited.insert(point.clone(), step);
            next.extend(point.next(&grid));
        }
        if next.len() > 0 {
            stack.push(next);
        }
    }

    let res = steps % 2;
    return visited
        .iter()
        .filter(|(_point, step)| step.to_owned() % 2 == res)
        .map(|(point, _step)| point)
        .collect::<HashSet<&Point>>()
        .len();
}
