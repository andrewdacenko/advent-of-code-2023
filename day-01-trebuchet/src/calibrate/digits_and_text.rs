use phf::phf_map;

static NUMBER_NAMES: phf::Map<&'static str, u32> = phf_map! {
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

pub fn digits_and_text(line: &str) -> u32 {
    let mut digits = NUMBER_NAMES
        .entries()
        .map(|(name, value)| {
            line.match_indices(name)
                .map(|item| (item.0, value.to_owned()))
                .collect::<Vec<(usize, u32)>>()
        })
        .reduce(|acc, matches| acc.into_iter().chain(matches).collect())
        .unwrap();

    let literal_digits = line
        .chars()
        .enumerate()
        .filter_map(|(index, letter)| {
            letter
                .is_digit(10)
                .then(|| (index, letter.to_digit(10).unwrap()))
        })
        .collect::<Vec<(usize, u32)>>();

    digits.extend(literal_digits);
    digits.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let start = digits.first().unwrap().1;
    let end = digits.last().unwrap().1;
    let strnum = format!("{start}{end}");
    strnum.to_string().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_end_has_digit() {
        assert_eq!(digits_and_text("one2"), 12);
        assert_eq!(digits_and_text("a1andtwo"), 12);
        assert_eq!(digits_and_text("a1s2d"), 12);
        assert_eq!(digits_and_text("as12d"), 12);
    }

    #[test]
    fn single_digit() {
        assert_eq!(digits_and_text("1asd"), 11);
        assert_eq!(digits_and_text("a2sd"), 22);
        assert_eq!(digits_and_text("asd3"), 33);
    }

    #[test]
    fn single_text_digit() {
        assert_eq!(digits_and_text("one"), 11);
        assert_eq!(digits_and_text("two"), 22);
        assert_eq!(digits_and_text("three"), 33);
        assert_eq!(digits_and_text("four"), 44);
        assert_eq!(digits_and_text("five"), 55);
        assert_eq!(digits_and_text("six"), 66);
        assert_eq!(digits_and_text("seven"), 77);
        assert_eq!(digits_and_text("eight"), 88);
        assert_eq!(digits_and_text("nine"), 99);
    }

    #[test]
    fn more_then_two_digits() {
        assert_eq!(digits_and_text("onetwoasd3e"), 13);
        assert_eq!(digits_and_text("atwothrees4fived"), 25);
    }
}
