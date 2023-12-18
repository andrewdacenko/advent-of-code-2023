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
    length: i64,
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

pub fn volume(input: &str) -> usize {
    shoelace_polygon(&parse_instructions(input))
}

pub fn volume_hex(input: &str) -> usize {
    shoelace_polygon(&parse_instructions_hex(input))
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

fn parse_instructions_hex(input: &str) -> DigPlan {
    input
        .lines()
        .map(|line| {
            let [_direction, _length, color] = line.splitn(3, ' ').collect::<Vec<&str>>()[..] else {
                panic!("Can't parse {}", line);
            };
            let hex_color = &color[2..8];
            let (length, dir) = hex_color.split_at(5);
            return Instruction {
                direction: match dir {
                    "0" => Direction::Right,
                    "1" => Direction::Down,
                    "2" => Direction::Left,
                    "3" => Direction::Up,
                    _ => panic!("Can't parse {}", dir),
                },
                length: i64::from_str_radix(length, 16).unwrap(),
            };
        })
        .collect()
}

// https://en.wikipedia.org/wiki/Shoelace_formula
fn shoelace_polygon(plan: &DigPlan) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut points = vec![];
    points.push((0, 0));
    let mut perimeter = 0;
    for item in plan {
        perimeter += item.length;
        match item.direction {
            Direction::Up => {
                points.push((x, y - item.length));
                y -= item.length;
            }
            Direction::Down => {
                points.push((x, y + item.length));
                y += item.length;
            }
            Direction::Left => {
                points.push((x - item.length, y));
                x -= item.length;
            }
            Direction::Right => {
                points.push((x + item.length, y));
                x += item.length;
            }
        }
    }
    let mut sum = 0;
    for i in 0..points.len() - 2 {
        let (x1, y1) = points[i];
        let (x2, y2) = points[i + 1];
        sum += (y1 + y2) * (x1 - x2);
    }
    let area = (sum / 2).abs();
    return (area + perimeter / 2 + 1).try_into().unwrap();
}
