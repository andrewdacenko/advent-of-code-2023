use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum TileType {
    Empty,
    SplitterTopBottom,
    SplitterLeftRight,
    MirrorForward,
    MirrorBackward,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Beam {
    direction: Direction,
    row: usize,
    col: usize,
}

type Grid = Vec<Vec<TileType>>;

fn parse_tiles(map: &str) -> Grid {
    map.lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .map(|byte| match byte {
                    b'.' => TileType::Empty,
                    b'|' => TileType::SplitterTopBottom,
                    b'-' => TileType::SplitterLeftRight,
                    b'/' => TileType::MirrorForward,
                    b'\\' => TileType::MirrorBackward,
                    _ => panic!("Unknown tile {}", byte),
                })
                .collect()
        })
        .collect()
}

pub fn count_energized_tiles(map: &str) -> usize {
    count_energized_tiles_for_beam(
        &Beam {
            direction: Direction::Right,
            col: 0,
            row: 0,
        },
        &parse_tiles(map),
    )
}

pub fn count_most_energized_tiles(map: &str) -> usize {
    let tiles = parse_tiles(map);
    return (0..tiles.len())
        .map(|row| {
            if row == 0 {
                return (0..tiles[row].len())
                    .map(|col| Beam {
                        direction: Direction::Bottom,
                        row,
                        col,
                    })
                    .chain([
                        Beam {
                            direction: Direction::Right,
                            row: 0,
                            col: 0,
                        },
                        Beam {
                            direction: Direction::Left,
                            row: 0,
                            col: tiles[row].len() - 1,
                        },
                    ])
                    .collect::<Vec<Beam>>();
            }

            if row == tiles.len() - 1 {
                return (0..tiles[row].len())
                    .map(|col| Beam {
                        direction: Direction::Top,
                        row,
                        col,
                    })
                    .chain([
                        Beam {
                            direction: Direction::Right,
                            row: 0,
                            col: 0,
                        },
                        Beam {
                            direction: Direction::Left,
                            row: 0,
                            col: tiles[row].len() - 1,
                        },
                    ])
                    .collect::<Vec<Beam>>();
            }

            return vec![
                Beam {
                    direction: Direction::Right,
                    row,
                    col: 0,
                },
                Beam {
                    direction: Direction::Left,
                    row,
                    col: tiles[row].len() - 1,
                },
            ];
        })
        .flatten()
        .map(|beam| count_energized_tiles_for_beam(&beam, &tiles))
        .max()
        .unwrap();
}

fn count_energized_tiles_for_beam(beam: &Beam, tiles: &Grid) -> usize {
    let mut beams: Vec<Beam> = vec![beam.clone()];
    let mut visited_tiles = HashSet::from([beam.clone()]);

    while beams.len() != 0 {
        beams = beams
            .iter()
            .map(|beam| move_beam(beam, &tiles))
            .flatten()
            .flatten()
            .filter(|item| !visited_tiles.contains(item))
            .collect();

        visited_tiles.extend(beams.iter());
    }

    return visited_tiles
        .iter()
        .map(|beam| (beam.row, beam.col))
        .collect::<HashSet<(usize, usize)>>()
        .len();
}

fn move_beam(beam: &Beam, tiles: &Grid) -> Vec<Option<Beam>> {
    match beam.direction {
        Direction::Right => move_beam_right(beam, tiles),
        Direction::Left => move_beam_left(beam, tiles),
        Direction::Top => move_beam_top(beam, tiles),
        Direction::Bottom => move_beam_bottom(beam, tiles),
    }
}

fn get_beam_top(beam: &Beam) -> Option<Beam> {
    if beam.row == 0 {
        None
    } else {
        Some(Beam {
            direction: Direction::Top,
            row: beam.row - 1,
            col: beam.col,
        })
    }
}

fn get_beam_bottom(beam: &Beam, tiles: &Grid) -> Option<Beam> {
    if beam.row + 1 >= tiles.len() {
        None
    } else {
        Some(Beam {
            direction: Direction::Bottom,
            row: beam.row + 1,
            col: beam.col,
        })
    }
}

fn get_beam_left(beam: &Beam) -> Option<Beam> {
    if beam.col == 0 {
        None
    } else {
        Some(Beam {
            direction: Direction::Left,
            row: beam.row,
            col: beam.col - 1,
        })
    }
}

fn get_beam_right(beam: &Beam, tiles: &Grid) -> Option<Beam> {
    assert!(tiles.len() > 0);
    if beam.col + 1 >= tiles[0].len() {
        None
    } else {
        Some(Beam {
            direction: Direction::Right,
            row: beam.row,
            col: beam.col + 1,
        })
    }
}

fn move_beam_right(beam: &Beam, tiles: &Grid) -> Vec<Option<Beam>> {
    match tiles[beam.row][beam.col] {
        TileType::Empty => vec![get_beam_right(beam, tiles)],
        TileType::SplitterTopBottom => {
            vec![get_beam_top(beam), get_beam_bottom(beam, tiles)]
        }
        TileType::SplitterLeftRight => vec![get_beam_right(beam, tiles)],
        TileType::MirrorForward => vec![get_beam_top(beam)],
        TileType::MirrorBackward => vec![get_beam_bottom(beam, tiles)],
    }
}

fn move_beam_left(beam: &Beam, tiles: &Grid) -> Vec<Option<Beam>> {
    match tiles[beam.row][beam.col] {
        TileType::Empty => vec![get_beam_left(beam)],
        TileType::SplitterTopBottom => {
            vec![get_beam_top(beam), get_beam_bottom(beam, tiles)]
        }
        TileType::SplitterLeftRight => vec![get_beam_left(beam)],
        TileType::MirrorForward => vec![get_beam_bottom(beam, tiles)],
        TileType::MirrorBackward => vec![get_beam_top(beam)],
    }
}

fn move_beam_top(beam: &Beam, tiles: &Grid) -> Vec<Option<Beam>> {
    match tiles[beam.row][beam.col] {
        TileType::Empty => vec![get_beam_top(beam)],
        TileType::SplitterTopBottom => vec![get_beam_top(beam)],
        TileType::SplitterLeftRight => {
            vec![get_beam_left(beam), get_beam_right(beam, tiles)]
        }
        TileType::MirrorForward => vec![get_beam_right(beam, tiles)],
        TileType::MirrorBackward => vec![get_beam_left(beam)],
    }
}

fn move_beam_bottom(beam: &Beam, tiles: &Grid) -> Vec<Option<Beam>> {
    match tiles[beam.row][beam.col] {
        TileType::Empty => vec![get_beam_bottom(beam, tiles)],
        TileType::SplitterTopBottom => vec![get_beam_bottom(beam, tiles)],
        TileType::SplitterLeftRight => {
            vec![get_beam_left(beam), get_beam_right(beam, tiles)]
        }
        TileType::MirrorForward => vec![get_beam_left(beam)],
        TileType::MirrorBackward => vec![get_beam_right(beam, tiles)],
    }
}
