use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

pub fn card_value(card: &str) -> u32 {
    let (start, end) = card.split_once(" | ").unwrap();
    let winning_numbers: Vec<u32> = NUM_RE
        .find_iter(start.split_once(": ").unwrap().1)
        .filter_map(|s| s.as_str().parse::<u32>().ok())
        .collect();
    let num_win: usize = NUM_RE
        .find_iter(end)
        .filter_map(|s| s.as_str().parse::<u32>().ok())
        .filter(|num| winning_numbers.contains(num))
        .count();
    if num_win == 0 {
        return 0;
    } else {
        return 2_u32.pow(num_win as u32 - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculations() {
        assert_eq!(
            card_value("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            8
        );
        assert_eq!(
            card_value("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            2
        );
        assert_eq!(
            card_value("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            2
        );
        assert_eq!(
            card_value("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            1
        );
        assert_eq!(
            card_value("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            0
        );
        assert_eq!(
            card_value("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            0
        );
    }
}
