use lazy_static::lazy_static;
use regex::{Match, Regex};
use std::collections::HashSet;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct Part {
    number: usize,
    start: usize,
    end: usize,
    line: usize,
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

fn maybe_gear(x: usize, y: usize, parts: &Vec<Part>) -> Option<usize> {
    let adjucent_parts = parts
        .iter()
        .filter(|part| {
            let row_min = if part.line > 0 { part.line - 1 } else { 0 };
            let col_min = if part.start > 0 { part.start - 1 } else { 0 };
            for i in row_min..(part.line + 2) {
                for j in col_min..(part.end + 1) {
                    if i == x && j == y {
                        return true;
                    }
                }
            }
            return false;
        })
        .collect::<Vec<&Part>>();

    if adjucent_parts.len() == 2 {
        let raion = adjucent_parts.iter().fold(1, |acc, part| acc * part.number);
        return Some(raion);
    } else {
        return None;
    };
}

pub fn get_part_numbers(schema: &str) -> Vec<usize> {
    get_part_numbers_and_positions(schema)
        .iter()
        .map(|part| part.number)
        .collect()
}

pub fn get_gear_ratios(schema: &str) -> Vec<usize> {
    let parts = get_part_numbers_and_positions(schema);
    let mut gears: Vec<usize> = vec![];

    schema
        .split("\n")
        .enumerate()
        .for_each(|(line_index, line)| {
            line.chars().enumerate().for_each(|(char_index, c)| {
                if c == '*' {
                    maybe_gear(line_index, char_index, &parts)
                        .and_then(|gear| Some(gears.push(gear)));
                }
            });
        });

    return gears;
}

fn get_part_numbers_and_positions(schema: &str) -> Vec<Part> {
    let symbols = get_symbols(schema);
    let lines = schema.split("\n").collect::<Vec<&str>>();
    return lines
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| {
            let matches = NUM_RE
                .find_iter(row)
                .filter_map(|res| maybe_part(res, row_index, &symbols))
                .collect::<Vec<Part>>();

            return Some(matches);
        })
        .flatten()
        .collect::<Vec<Part>>();
}

fn is_symbol(c: char) -> bool {
    !(c.is_digit(10) || c.eq(&'.'))
}

fn maybe_part<'a>(
    item: Match<'a>,
    row_index: usize,
    symbols: &HashSet<(usize, usize)>,
) -> Option<Part> {
    let row_min = if row_index > 0 { row_index - 1 } else { 0 };
    let col_min = if item.start() > 0 {
        item.start() - 1
    } else {
        0
    };
    for i in row_min..(row_index + 2) {
        for j in col_min..(item.end() + 1) {
            if symbols.contains(&(i, j)) {
                return Some(Part {
                    number: item.as_str().parse::<usize>().unwrap(),
                    start: item.start(),
                    end: item.end(),
                    line: row_index,
                });
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
    fn test_gear_ratios() {
        assert_eq!(
            get_gear_ratios(
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
            [16345, 451490]
        );
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
