use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn next(&self, c: char) -> Option<Direction> {
        match c {
            '|' => {
                if self.eq(&Direction::North) {
                    Some(Direction::North)
                } else if self.eq(&Direction::South) {
                    Some(Direction::South)
                } else {
                    None
                }
            }
            '-' => {
                if self.eq(&Direction::East) {
                    Some(Direction::East)
                } else if self.eq(&Direction::West) {
                    Some(Direction::West)
                } else {
                    None
                }
            }
            'L' => {
                if self.eq(&Direction::South) {
                    Some(Direction::East)
                } else if self.eq(&Direction::West) {
                    Some(Direction::North)
                } else {
                    None
                }
            }
            'J' => {
                if self.eq(&Direction::South) {
                    Some(Direction::West)
                } else if self.eq(&Direction::East) {
                    Some(Direction::North)
                } else {
                    None
                }
            }
            '7' => {
                if self.eq(&Direction::North) {
                    Some(Direction::West)
                } else if self.eq(&Direction::East) {
                    Some(Direction::South)
                } else {
                    None
                }
            }
            'F' => {
                if self.eq(&Direction::North) {
                    Some(Direction::East)
                } else if self.eq(&Direction::West) {
                    Some(Direction::South)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Position {
    row: usize,
    column: usize,
    direction: Direction,
}

#[derive(Debug, Clone)]
struct Maze<'a> {
    rows: Vec<&'a str>,
}

impl Maze<'_> {
    fn from(map: &str) -> Maze {
        Maze {
            rows: map.split("\n").collect(),
        }
    }

    fn start(&self) -> (Position, Position) {
        for (row, line) in self.rows.iter().enumerate() {
            for (column, symbol) in line.chars().enumerate() {
                if symbol == 'S' {
                    let positions = [
                        Direction::North,
                        Direction::South,
                        Direction::East,
                        Direction::West,
                    ]
                    .map(|direction| Position {
                        row,
                        column,
                        direction,
                    });
                    let s: Vec<&Position> = positions
                        .iter()
                        .filter(|position| position.clone().next(&self).is_some())
                        .collect();
                    return (*s[0], *s[1]);
                }
            }
        }
        panic!("Maze should have animal starting position symbol 'S'")
    }
}

impl Position {
    fn from(row: usize, column: usize, direction: Direction) -> Position {
        Position {
            row,
            column,
            direction,
        }
    }

    fn next(&self, maze: &Maze) -> Option<Position> {
        match self.direction {
            Direction::North => {
                if self.row == 0 {
                    None
                } else {
                    match maze.rows[self.row - 1].chars().nth(self.column) {
                        Some(c) => match self.direction.next(c) {
                            Some(next_direction) => {
                                Some(Position::from(self.row - 1, self.column, next_direction))
                            }
                            None => None,
                        },
                        None => None,
                    }
                }
            }
            Direction::South => {
                if self.row + 1 == maze.rows.len() {
                    None
                } else {
                    match maze.rows[self.row + 1].chars().nth(self.column) {
                        Some(c) => match self.direction.next(c) {
                            Some(next_direction) => {
                                Some(Position::from(self.row + 1, self.column, next_direction))
                            }
                            None => None,
                        },
                        None => None,
                    }
                }
            }
            Direction::East => {
                if self.column + 1 == maze.rows[self.row].len() {
                    None
                } else {
                    match maze.rows[self.row].chars().nth(self.column + 1) {
                        Some(c) => match self.direction.next(c) {
                            Some(next_direction) => {
                                Some(Position::from(self.row, self.column + 1, next_direction))
                            }
                            None => None,
                        },
                        None => None,
                    }
                }
            }
            Direction::West => {
                if self.column == 0 {
                    None
                } else {
                    match maze.rows[self.row].chars().nth(self.column - 1) {
                        Some(c) => match self.direction.next(c) {
                            Some(next_direction) => {
                                Some(Position::from(self.row, self.column - 1, next_direction))
                            }
                            None => None,
                        },
                        None => None,
                    }
                }
            }
        }
    }
}

pub fn longest_path(map: &str) -> usize {
    let maze = Maze::from(map);
    let (mut first, mut second) = maze.clone().start();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    loop {
        first = first.clone().next(&maze).unwrap();
        second = second.clone().next(&maze).unwrap();
        let first_pos = (first.row, first.column);
        let second_pos = (second.row, second.column);

        if visited.contains(&first_pos) || visited.contains(&second_pos) {
            break;
        }

        if first_pos.eq(&second_pos) {
            return ((visited.len() / 2) as f64).ceil() as usize + 1;
        }

        visited.insert(first_pos);
        visited.insert(second_pos);
    }

    return ((visited.len() / 2) as f64).ceil() as usize;
}

pub fn enclosed_tiles(map: &str) -> usize {
    let maze = Maze::from(map);
    let (mut first, mut second) = maze.clone().start();
    let mut loop_path: HashSet<(usize, usize)> = HashSet::new();

    loop {
        first = first.clone().next(&maze).unwrap();
        second = second.clone().next(&maze).unwrap();
        let first_pos = (first.row, first.column);
        let second_pos = (second.row, second.column);

        if loop_path.contains(&first_pos) || loop_path.contains(&second_pos) {
            break;
        }

        loop_path.insert(first_pos);
        loop_path.insert(second_pos);
    }

    let mut tiles: usize = 0;
    for (row, line) in maze.rows.iter().enumerate() {
        let mut row_tiles: Vec<(usize, usize)> = vec![];
        for column in 0..line.len() {
            if is_enclosed_in_line(row, column, line, &loop_path) {
                row_tiles.push((row, column))
            }
        }
        tiles += row_tiles.len();
    }

    return tiles;
}

fn is_enclosed_in_line(
    row: usize,
    column: usize,
    line: &str,
    loop_path: &HashSet<(usize, usize)>,
) -> bool {
    if loop_path.contains(&(row, column)) {
        return false;
    }

    let mut north: usize = 0;
    for i in 0..column {
        if !loop_path.contains(&(row, i)) {
            continue;
        }
        let c = line.chars().nth(i).unwrap();
        if "|JSL".contains(c) {
            north += 1;
        }
    }

    return north % 2 == 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_find_starting_position() {
        assert_eq!(
            Maze::from("FS7").start(),
            (
                Position::from(0, 1, Direction::East),
                Position::from(0, 1, Direction::West),
            )
        );
        assert_eq!(
            Maze::from(".|.\n.S-").start(),
            (
                Position::from(1, 1, Direction::North),
                Position::from(1, 1, Direction::East),
            )
        );
        assert_eq!(
            Maze::from(".|.\n.SJ").start(),
            (
                Position::from(1, 1, Direction::North),
                Position::from(1, 1, Direction::East),
            )
        );
        assert_eq!(
            Maze::from(".SJ\n.|.").start(),
            (
                Position::from(0, 1, Direction::South),
                Position::from(0, 1, Direction::East),
            )
        );
    }

    #[test]
    fn it_detects_enclosed() {
        assert_eq!(
            is_enclosed_in_line(0, 1, ".|...|.", &HashSet::from([(0, 1), (0, 5)])),
            false
        );

        assert_eq!(
            is_enclosed_in_line(0, 3, ".|...|.", &HashSet::from([(0, 1), (0, 5)])),
            true
        );
    }
}
