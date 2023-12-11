use itertools::Itertools;

pub fn shortest_paths(space: &str, expansion: usize) -> Vec<usize> {
    let galaxies: Vec<(usize, usize)> = space
        .split("\n")
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| if c.eq(&'#') { Some((row, col)) } else { None })
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    let galaxies_rows = galaxies.iter().map(|x| x.0).collect::<Vec<usize>>();
    let galaxies_cols = galaxies.iter().map(|x| x.1).collect::<Vec<usize>>();

    let empty_rows = (0..space.split("\n").count())
        .filter(|v| !galaxies_rows.contains(v))
        .collect::<Vec<usize>>();
    let empty_cols = (0..space.split("\n").count())
        .filter(|v| !galaxies_cols.contains(v))
        .collect::<Vec<usize>>();

    let expanded_galaxies = galaxies
        .iter()
        .map(|(row, col)| {
            let expand_row = empty_rows.iter().filter(|x| x.lt(&row)).count();
            let expand_col = empty_cols.iter().filter(|x| x.lt(&col)).count();

            return (
                row + expand_row * (expansion - 1),
                col + expand_col * (expansion - 1),
            );
        })
        .collect::<Vec<(usize, usize)>>();

    return expanded_galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let start = pair[0];
            let end = pair[1];

            return distance(start, end);
        })
        .collect::<Vec<usize>>();
}

fn distance(start: &(usize, usize), end: &(usize, usize)) -> usize {
    let steps = (start.0 as i32 - end.0 as i32).abs() + (start.1 as i32 - end.1 as i32).abs();
    return steps as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_find_distance() {
        assert_eq!(distance(&(0, 4), &(10, 9)), 15);
    }
}
