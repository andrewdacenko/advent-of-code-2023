use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Point(usize, usize, usize);
impl Point {
    fn new(coords: &str) -> Self {
        let [x, y, z] = coords
            .split(',')
            .map(|s| s.parse::<usize>().expect("Must be number"))
            .collect::<Vec<_>>()[..] else { panic!("Must be 3 coordinates in {}", coords) };
        Self(x, y, z)
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Brick {
    name: String,
    level: usize,
    supports: Vec<String>,
    supported_by: Vec<String>,
}

impl Brick {
    fn can_desintegrate(&self, bricks: &HashMap<String, Brick>) -> bool {
        self.supports.iter().all(|brick| {
            bricks
                .get(brick)
                .and_then(|x| Some(x.supported_by.len() > 1))
                .unwrap_or(false)
        })
    }

    fn drop_count(&self, bricks: &HashMap<String, Brick>) -> usize {
        if self.can_desintegrate(bricks) {
            return 0;
        }

        let mut removed = vec![];
        let mut dropped = 0;
        let mut stack = vec![self];
        while let Some(brick) = stack.pop() {
            removed.push(&brick.name);
            dropped += 1;
            let should_fall = brick
                .supports
                .iter()
                .map(|x| bricks.get(x))
                .flatten()
                .filter(|x| x.supported_by.iter().all(|s| removed.contains(&s)));
            stack.extend(should_fall.into_iter());
        }
        return dropped - 1;
    }
}

pub fn count_redundant(input: &str) -> usize {
    let bricks = parse(input);
    let cache: &HashMap<String, Brick> = &bricks
        .iter()
        .map(|x| (x.name.to_owned(), x.to_owned()))
        .collect();

    let mut count = 0;
    for brick in bricks.iter() {
        if brick.can_desintegrate(cache) {
            count += 1;
        }
    }

    return count;
}

pub fn count_chain(input: &str) -> usize {
    let bricks = parse(input);
    let cache: &HashMap<String, Brick> = &bricks
        .iter()
        .map(|x| (x.name.to_owned(), x.to_owned()))
        .collect();

    let mut removeable: HashSet<Brick> = HashSet::new();
    for brick in bricks.iter() {
        if brick.can_desintegrate(cache) {
            removeable.insert(brick.clone());
        }
    }

    bricks
        .iter()
        .fold(0, |acc, brick| acc + brick.drop_count(&cache))
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("~").expect("Line should have ~");
            (Point::new(start), Point::new(end), l)
        })
        .collect::<Vec<(Point, Point, &str)>>();
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    let mut heights = [[0; 10]; 10];
    let mut brick_on_level: HashMap<(Point, Point, &str), usize> = HashMap::new();
    let mut bricks_to_level: HashMap<usize, Vec<(Point, Point, &str)>> = HashMap::new();
    for brick in bricks.iter() {
        // drop a brick down
        let mut min_level = 1;
        for i in brick.0 .0..=brick.1 .0 {
            for j in brick.0 .1..=brick.1 .1 {
                min_level = heights[i][j].max(min_level);
            }
        }

        let fill_level = min_level + brick.1 .2 - brick.0 .2 + 1;
        for i in brick.0 .0..=brick.1 .0 {
            for j in brick.0 .1..=brick.1 .1 {
                heights[i][j] = fill_level;
            }
        }

        brick_on_level.insert(brick.to_owned(), min_level);

        if let Some(res) = bricks_to_level.get_mut(&fill_level) {
            res.push(brick.to_owned())
        } else {
            bricks_to_level.insert(fill_level, vec![brick.to_owned()]);
        }
    }

    let is_supported_by = |main: &(Point, Point, &str), other: &(Point, Point, &str)| {
        main.0 .0 <= other.1 .0
            && other.0 .0 <= main.1 .0
            && main.0 .1 <= other.1 .1
            && other.0 .1 <= main.1 .1
    };

    bricks
        .iter()
        .map(|position| {
            let (start, end, name) = position;
            let brick_level = brick_on_level.get(position).unwrap();
            let mut brick = Brick {
                name: name.to_string(),
                level: brick_level.to_owned(),
                supports: vec![],
                supported_by: vec![],
            };
            let support_level = brick_level + end.2 - start.2 + 1;

            brick.supports = brick_on_level
                .iter()
                .filter(|(_brick, level)| support_level.eq(&level))
                .filter(|(brick, _level)| is_supported_by(brick, position))
                .map(|((_start, _end, name), _level)| name.to_string())
                .collect();

            brick.supported_by = brick_on_level
                .iter()
                .filter(|(brick, level)| brick_level.eq(&(*level + brick.1 .2 - brick.0 .2 + 1)))
                .filter(|(brick, _level)| is_supported_by(position, brick))
                .map(|((_start, _end, name), _level)| name.to_string())
                .collect();

            brick
        })
        .collect()
}
