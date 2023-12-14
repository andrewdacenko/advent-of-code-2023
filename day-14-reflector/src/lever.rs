use std::collections::HashMap;

pub fn count_load_north_after_cycles(schema: &str, cycles: usize) -> usize {
    let platform: Vec<Vec<char>> = cycle(
        &schema
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect(),
        cycles,
    );

    return count_load_north(&platform);
}

pub fn cycle(v: &Vec<Vec<char>>, cycles: usize) -> Vec<Vec<char>> {
    let mut res = v.clone();
    let mut cache: HashMap<String, usize> = HashMap::from([(cache_key(v), 0)]);
    let mut final_cycles = 0;
    for i in 0..cycles {
        let cycled = cycle_east(&cycle_south(&cycle_west(&cycle_north(&res))));
        if are_same(&cycled, &res) {
            return res;
        }

        let key = cache_key(&cycled);
        if let Some(val) = cache.get_key_value(&key) {
            let cycle_loop = i - val.1;
            let remaining = cycles - i - 1;
            final_cycles = remaining % cycle_loop;
            res = cycled;
            break;
        } else {
            res = cycled;
            cache.insert(key, i);
        }
    }

    for _ in 0..final_cycles {
        res = cycle_east(&cycle_south(&cycle_west(&cycle_north(&res))));
    }

    return res;
}

fn count_load_north(v: &Vec<Vec<char>>) -> usize {
    transpose(&v).iter().map(count_line_load).sum()
}

fn count_line_load(line: &Vec<char>) -> usize {
    let size = line.len();
    let mut total_load = 0;
    for position in 0..size {
        if line[position] == 'O' {
            total_load += size - position;
        }
    }

    return total_load;
}

fn cache_key(v: &Vec<Vec<char>>) -> String {
    let mut key = String::new();
    for i in 0..v.len() {
        for j in 0..v[i].len() {
            if v[i][j] == 'O' {
                key.push_str(format!("({i},{j})").as_str());
            }
        }
    }
    return key;
}

fn are_same(a: &Vec<Vec<char>>, b: &Vec<Vec<char>>) -> bool {
    if a.len() != b.len() {
        return false;
    }

    if a.is_empty() {
        return true;
    }

    for i in 0..a.len() {
        if a[i].len() != b[i].len() {
            return false;
        }

        if a[i].is_empty() {
            return true;
        }

        for j in 0..a[0].len() {
            if a[i][j] != b[i][j] {
                return false;
            }
        }
    }

    return true;
}

fn cycle_north(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(&reposition(&transpose(v)))
}

fn cycle_east(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    flip(&reposition(&flip(v)))
}

fn cycle_south(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(&flip(&reposition(&flip(&transpose(v)))))
}

fn cycle_west(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    reposition(v)
}

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn flip<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    return v
        .iter()
        .map(|line| {
            let mut res = line.clone();
            res.reverse();
            return res;
        })
        .collect();
}

fn reposition(v: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    return v.iter().map(reposition_line).collect();
}

fn reposition_line(line: &Vec<char>) -> Vec<char> {
    let mut res = vec!['.'; line.len()];
    let mut rolls_before = 0;
    for position in 0..line.len() {
        if line[position] == '.' {
            rolls_before += 1;
            continue;
        }

        if line[position] == '#' {
            rolls_before = 0;
            res[position] = '#';
            continue;
        }

        if line[position] == 'O' {
            res[position - rolls_before] = 'O';
        }
    }

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_compares_vecs() {
        assert_eq!(
            are_same(
                &vec![vec!['a', 'b'], vec!['a', 'c']],
                &vec![vec!['a', 'b'], vec!['a', 'c']]
            ),
            true
        );
    }

    #[test]
    fn it_reposition_rocks() {
        assert_eq!(reposition_line(&vec!['.', 'O']), vec!['O', '.']);
        assert_eq!(reposition_line(&vec!['.', '#', 'O']), vec!['.', '#', 'O']);
        assert_eq!(reposition_line(&vec!['.', '.', 'O']), vec!['O', '.', '.']);
        assert_eq!(reposition_line(&vec!['#', '.', 'O']), vec!['#', 'O', '.']);
        assert_eq!(
            reposition_line(&vec!['#', '.', 'O', 'O']),
            vec!['#', 'O', 'O', '.']
        );
    }

    #[test]
    fn it_calcs_load() {
        assert_eq!(
            count_load_north(&make_vec(
                ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
            )),
            87
        );
        assert_eq!(
            count_load_north(&make_vec(
                ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
            )),
            69
        );
    }

    fn make_vec(s: &str) -> Vec<Vec<char>> {
        s.lines().map(|line| line.chars().collect()).collect()
    }
}
