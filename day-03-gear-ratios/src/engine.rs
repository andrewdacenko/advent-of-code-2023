use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::HashSet;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn get_symbols(schema: &str) -> HashSet<(usize, usize)> {
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();
    schema
        .split("\n")
        .enumerate()
        .for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(char_index, c)| {
                if is_symbol(c) {
                    symbols.insert((line_index, char_index));
                }
            });
        });

    symbols.to_owned()
}

pub fn get_part_numbers(schema: &str) -> Vec<usize> {
    let symbols = get_symbols(schema);
    let lines = schema.split("\n").collect::<Vec<&str>>();
    return lines
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            let matches = NUM_RE
                .find_iter(row)
                .filter_map(|res| maybe_part(res, row_index, &symbols))
                .collect::<Vec<usize>>();

            return Some(matches);
        })
        .flatten()
        .collect::<Vec<usize>>();
}

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c.eq(&'.'))
}

fn maybe_part<'a>(
    item: Match<'a>,
    row_index: usize,
    symbols: &HashSet<(usize, usize)>,
) -> Option<usize> {
    let row_min = if row_index > 0 { row_index - 1 } else { 0 };
    let col_min = if item.start() > 0 {
        item.start() - 1
    } else {
        0
    };
    for i in row_min..(row_index + 2) {
        for j in col_min..(item.end() + 1) {
            if symbols.contains(&(i, j)) {
                return Some(item.as_str().parse::<usize>().unwrap());
            }
        }
    }
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_recognition() {
        assert_eq!(is_symbol('.'), false);
        assert_eq!(is_symbol('*'), true);
        assert_eq!(is_symbol('1'), false);
    }

    #[test]
    fn symbols_hasmap() {
        assert_eq!(get_symbols("."), HashSet::new());
        assert_eq!(get_symbols("*.^4"), HashSet::from([(0, 0), (0, 2)]));
        assert_eq!(get_symbols("1"), HashSet::new());
    }

    #[test]
    fn small_parts() {
        assert_eq!(get_part_numbers("4.\n.$"), [4]);
        assert_eq!(get_part_numbers("1..\n..$"), []);
    }

    #[test]
    fn collects_parts() {
        assert_eq!(
            get_part_numbers(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            ),
            [467, 35, 633, 617, 592, 755, 664, 598]
        );
        assert_eq!(
            get_part_numbers(
                "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."
            )
            .iter()
            .sum::<usize>(),
            4361
        )
    }
}
