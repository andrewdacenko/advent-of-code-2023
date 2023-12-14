pub fn count_load_north(schema: &str) -> usize {
    let platform: Vec<Vec<char>> = schema
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    return transpose(&platform).iter().map(count_line_load).sum();
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

fn count_line_load(line: &Vec<char>) -> usize {
    let size = line.len();
    let mut total_load = 0;
    let mut rolls_before = 0;
    for position in 0..size {
        if line[position] == '.' {
            rolls_before += 1;
            continue;
        }

        if line[position] == '#' {
            rolls_before = 0;
            continue;
        }

        if line[position] == 'O' {
            total_load += size - position + rolls_before;
        }
    }

    return total_load;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_calculates_load_in_line() {
        assert_eq!(count_line_load(&vec!['O']), 1);
        assert_eq!(count_line_load(&vec!['O', '.']), 2);
        assert_eq!(count_line_load(&vec!['O', '.', 'O']), 3 + 2);
        assert_eq!(count_line_load(&vec!['O', '.', '.', 'O']), 4 + 3);
        assert_eq!(count_line_load(&vec!['O', '#', '.', 'O']), 4 + 2);
        assert_eq!(count_line_load(&vec!['O', '.', '#', 'O']), 4 + 1);
        assert_eq!(count_line_load(&vec!['O', '#', '.', 'O', 'O']), 5 + 3 + 2);
    }
}
