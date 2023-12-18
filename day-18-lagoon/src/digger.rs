use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    length: i32,
}

impl Instruction {
    fn new(direction: &str, length: &str) -> Self {
        Self {
            direction: match direction {
                "D" => Direction::Down,
                "U" => Direction::Up,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction {}", direction),
            },
            length: length.parse().expect("Length should be digit"),
        }
    }
}

type DigPlan = Vec<Instruction>;
type Point = (i32, i32);
type Lagoon = HashSet<Point>;

pub fn volume(input: &str) -> usize {
    dig_lagoon(&parse_instructions(input)).len()
}

fn parse_instructions(input: &str) -> DigPlan {
    input
        .lines()
        .map(|line| {
            let [direction, length, _] = line.splitn(3, ' ').collect::<Vec<&str>>()[..] else {
                panic!("Can't parse {}", line);
            };
            return Instruction::new(direction, length);
        })
        .collect()
}

fn dig_lagoon(plan: &DigPlan) -> Lagoon {
    let (mut x, mut y) = (0, 0);
    let mut lagoon = HashSet::new();
    lagoon.insert((0, 0));

    for item in plan {
        match item.direction {
            Direction::Up => {
                for i in 1..=item.length {
                    lagoon.insert((x, y - i));
                }
                y -= item.length;
            }
            Direction::Down => {
                for i in 1..=item.length {
                    lagoon.insert((x, y + i));
                }
                y += item.length;
            }
            Direction::Left => {
                for i in 1..=item.length {
                    lagoon.insert((x - i, y));
                }
                x -= item.length;
            }
            Direction::Right => {
                for i in 1..=item.length {
                    lagoon.insert((x + i, y));
                }
                x += item.length;
            }
        }
    }

    let mut stack = vec![(1, 1)];
    while let Some(next) = stack.pop() {
        if lagoon.contains(&next) {
            continue;
        }
        lagoon.insert(next);
        stack.push((next.0 - 1, next.1));
        stack.push((next.0 + 1, next.1));
        stack.push((next.0, next.1 - 1));
        stack.push((next.0, next.1 + 1));
    }

    return lagoon;
}
