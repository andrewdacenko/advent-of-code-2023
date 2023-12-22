use std::{collections::HashMap, vec};

type BrickOnLevel = HashMap<Brick, usize>;
type BricksToLevel = HashMap<usize, Vec<Brick>>;

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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Brick(Point, Point);

impl Brick {
    fn is_supported_by(&self, brick: &Brick) -> bool {
        return self.0 .0 <= brick.1 .0
            && brick.0 .0 <= self.1 .0
            && self.0 .1 <= brick.1 .1
            && brick.0 .1 <= self.1 .1;
    }

    fn can_desintegrate(
        &self,
        settled_on_level: &BrickOnLevel,
        supports_level: &BricksToLevel,
    ) -> bool {
        let brick_level = settled_on_level.get(self).expect("Must have settled level");
        let support_level = brick_level + self.1 .2 - self.0 .2 + 1;

        settled_on_level
            .iter()
            .filter(|(_brick, level)| support_level.eq(&level))
            .filter(|(brick, _level)| brick.is_supported_by(self))
            .all(|(brick, level)| {
                let Some(bricks) = supports_level.get(level) else {return false};
                bricks.iter().filter(|b| brick.is_supported_by(b)).count() > 1
            })
    }
}

pub fn count_redundant(input: &str) -> usize {
    let mut bricks = input
        .lines()
        .map(|l| {
            let (start, end) = l.split_once("~").expect("Line should have ~");
            return Brick(Point::new(start), Point::new(end));
        })
        .collect::<Vec<Brick>>();
    bricks.sort_by(|a, b| a.0 .2.cmp(&b.0 .2));

    let mut heights = [[0; 10]; 10];
    let mut brick_on_level: HashMap<Brick, usize> = HashMap::new();
    let mut bricks_to_level: HashMap<usize, Vec<Brick>> = HashMap::new();
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

        brick_on_level.insert(brick.clone(), min_level);

        if let Some(res) = bricks_to_level.get_mut(&fill_level) {
            res.push(brick.to_owned())
        } else {
            bricks_to_level.insert(fill_level, vec![brick.to_owned()]);
        }
    }

    let mut count = 0;
    for brick in bricks.iter() {
        if brick.can_desintegrate(&brick_on_level, &bricks_to_level) {
            count += 1;
        }
    }

    return count;
}
