pub fn digits_only(line: &str) -> u32 {
    let numbers = line.chars().filter(|c| c.is_digit(10));
    let start = numbers.to_owned().next().unwrap();
    let end = numbers.last().unwrap();
    let strnum = format!("{start}{end}");
    strnum.to_string().parse().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_and_end_has_digit() {
        assert_eq!(digits_only("1asd2"), 12);
        assert_eq!(digits_only("a1sd2"), 12);
        assert_eq!(digits_only("a1s2d"), 12);
        assert_eq!(digits_only("as12d"), 12);
    }

    #[test]
    fn single_digit() {
        assert_eq!(digits_only("1asd"), 11);
        assert_eq!(digits_only("a2sd"), 22);
        assert_eq!(digits_only("asd3"), 33);
    }

    #[test]
    fn more_then_two_digits() {
        assert_eq!(digits_only("12asd3e"), 13);
        assert_eq!(digits_only("a23s45d"), 25);
    }
}
