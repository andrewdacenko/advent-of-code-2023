use std::collections::HashMap;

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
    let (start, grid) = parse_input(input);
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
        .count();
}

pub fn count_tiles_infinite(input: &str, steps: usize) -> usize {
    let grid_size = input.lines().count();
    let expanded_input = expand_map(input);
    let expansion = steps / grid_size;
    let rest = steps - expansion * grid_size;

    let results = (
        count_tiles(&expanded_input, rest),
        count_tiles(&expanded_input, rest + grid_size), // expand once
        count_tiles(&expanded_input, rest + grid_size * 2), // expand twice
    );

    return interpolate(expansion, results);
}

fn expand_map(input: &str) -> String {
    return [input; 5]
        .join("\n")
        .lines()
        .map(|l| [l; 5].join(""))
        .collect::<Vec<String>>()
        .join("\n");
}

fn interpolate(n: usize, results: (usize, usize, usize)) -> usize {
    let a = (results.2 + results.0 - 2 * results.1) / 2;
    let b = results.1 - results.0 - a;
    let c = results.0;
    return a * n * n + b * n + c;
}

fn parse_input(input: &str) -> (Point, Grid) {
    let grid: Grid = input
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| match b {
                    b'S' => Tile::Start,
                    b'.' => Tile::Garden,
                    b'#' => Tile::Rock,
                    _ => panic!("Unknown tile {}", char::from(*b)),
                })
                .collect()
        })
        .collect();
    return (Point(grid.len() as i64 / 2, grid.len() as i64 / 2), grid);
}
