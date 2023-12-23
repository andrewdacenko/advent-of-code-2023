use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, grid: &Grid, visited: &HashSet<Pos>) -> Vec<(Pos, HashSet<Pos>)> {
        let next = match grid[self.0][self.1] {
            '.' => vec![
                Pos(self.0, self.1 + 1),
                Pos(self.0, self.1 - 1),
                Pos(self.0 + 1, self.1 + 0),
                Pos(self.0 - 1, self.1 + 0),
            ],
            '>' => vec![Pos(self.0, self.1 + 1)],
            '<' => vec![Pos(self.0, self.1 - 1)],
            '^' => vec![Pos(self.0 - 1, self.1 + 0)],
            'v' => vec![Pos(self.0 + 1, self.1 + 0)],
            c => panic!("Unknown direction {c}"),
        };
        let mut next_visited = visited.clone();
        next_visited.extend(&HashSet::from([self.clone()]));

        next.iter()
            .filter(|next_pos| !visited.contains(&next_pos))
            .map(|next_pos| match grid[next_pos.0][next_pos.1] {
                '#' => None,
                _ => Some((*next_pos, next_visited.clone())),
            })
            .flatten()
            .collect()
    }
}

type Grid = Vec<Vec<char>>;

pub fn longest_route(input: &str) -> usize {
    let grid: Grid = input.lines().map(|l| l.chars().collect()).collect();
    let end = Pos(grid.len() - 2, grid[0].len() - 2);

    let mut queue: Vec<(Pos, HashSet<Pos>)> = vec![(
        Pos(1, 1),
        HashSet::from([Pos(0, 1), Pos(1, 1), Pos(grid.len() - 1, grid[0].len() - 2)]),
    )];
    let mut all_paths = vec![];
    while let Some((pos, visited)) = queue.pop() {
        if pos.eq(&end) {
            all_paths.push(visited.len());
            continue;
        }

        queue.append(&mut pos.successors(&grid, &visited));
    }

    return all_paths.iter().max().unwrap().to_owned();
}
