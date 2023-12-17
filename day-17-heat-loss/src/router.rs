use pathfinding::prelude::dijkstra;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Block {
    row: usize,
    col: usize,
    direction: Direction,
    step: usize,
}

impl Block {
    fn new(row: usize, col: usize, direction: Direction, step: usize) -> Self {
        Self {
            row,
            col,
            direction,
            step,
        }
    }

    fn loss(&self, grid: &Grid) -> usize {
        (grid[self.row][self.col] - 48) as usize
    }

    fn successors(&self, grid: &Grid) -> Vec<(Block, usize)> {
        if self.row == grid.len() - 1 && self.col == grid[0].len() - 1 {
            return vec![];
        }

        let max_row = grid.len();
        let max_col = grid[0].len();
        let transform = |&direction| match direction {
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
            Direction::Top => (-1, 0),
            Direction::Bottom => (1, 0),
        };
        let next_directions = match self.direction {
            Direction::Right => [Direction::Right, Direction::Top, Direction::Bottom],
            Direction::Left => [Direction::Left, Direction::Top, Direction::Bottom],
            Direction::Top => [Direction::Right, Direction::Left, Direction::Top],
            Direction::Bottom => [Direction::Right, Direction::Left, Direction::Bottom],
        };
        let successors: Vec<(Block, usize)> = next_directions
            .iter()
            .map(|direction| {
                if self.direction.eq(direction) && self.step == 3 {
                    return None;
                }

                let (row, col) = transform(direction);
                let next_row = self.row as i32 + row;
                let next_col = self.col as i32 + col;
                if next_row == -1
                    || next_col == -1
                    || next_row == max_row as i32
                    || next_col == max_col as i32
                {
                    return None;
                }

                let step = if self.direction.eq(direction) {
                    self.step + 1
                } else {
                    1
                };

                let next_block = Block {
                    row: next_row as usize,
                    col: next_col as usize,
                    direction: direction.clone(),
                    step,
                };

                let distance = grid[next_row as usize][next_col as usize] - 48;
                return Some((next_block, distance as usize));
            })
            .flatten()
            .collect();

        return successors;
    }
}

type Grid<'a> = Vec<&'a [u8]>;

pub fn min_heat_loss(map: &str) -> usize {
    let grid: Grid = map.lines().map(|line| line.as_bytes()).collect();
    let goal = |block: &Block| block.row == grid.len() - 1 && block.col == grid[0].len() - 1;

    let right = dijkstra(
        &Block::new(0, 1, Direction::Right, 1),
        |block| block.successors(&grid),
        goal,
    );

    let bottom = dijkstra(
        &Block::new(1, 0, Direction::Bottom, 1),
        |block| block.successors(&grid),
        goal,
    );

    [right, bottom]
        .iter()
        .flatten()
        .inspect(|(route, distance)| {
            println!("---- Route ----");
            print_route(route, &grid);
            println!("---- Distance: {:?} ---", distance + route[0].loss(&grid));
        })
        .map(|(route, distance)| distance + route[0].loss(&grid))
        .min()
        .unwrap_or_default()
}

fn print_route(route: &Vec<Block>, grid: &Grid) {
    let mut buff = String::from("");
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let ch =
                if let Some(block) = route.iter().find(|block| block.row == i && block.col == j) {
                    match block.direction {
                        Direction::Right => '>',
                        Direction::Left => '<',
                        Direction::Top => '^',
                        Direction::Bottom => 'v',
                    }
                } else {
                    '.'
                };
            buff.push(ch)
        }
        buff.push('\n');
    }

    print!("{buff}");
}
