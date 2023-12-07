use std::{cmp::Ordering, collections::HashMap};

static CARDS: &str = "AKQJT98765432";

#[derive(Debug, Clone, Copy)]
struct Hand<'a> {
    cards: &'a str,
    strength: Strength,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Strength {
    FiveOfKind = 0,
    FourOfKind = 1,
    FullHouse = 2,
    ThreeOfKind = 3,
    TwoPairs = 4,
    OnePair = 5,
    HighCard = 6,
}

impl Hand<'_> {
    fn new(cards: &str) -> Hand {
        return Hand {
            cards,
            strength: Hand::strength(cards),
        };
    }

    fn strength(cards: &str) -> Strength {
        let mut groups: HashMap<char, usize> = HashMap::new();
        for c in cards.chars() {
            *groups.entry(c).or_insert(0) += 1;
        }

        return match groups.len() {
            1 => Strength::FiveOfKind,
            2 => match groups.iter().any(|group| group.1.eq(&4)) {
                true => Strength::FourOfKind,
                false => Strength::FullHouse,
            },
            3 => match groups.iter().any(|group| group.1.eq(&3)) {
                true => Strength::ThreeOfKind,
                false => Strength::TwoPairs,
            },
            4 => Strength::OnePair,
            5 => Strength::HighCard,
            x => panic!("Not allowed group count {x}, cards: {cards}"),
        };
    }

    fn cmp(&self, other: Hand<'_>) -> Ordering {
        if self.strength.eq(&other.strength) {
            return self.cmp_start(other);
        } else if (self.strength as usize) < other.strength as usize {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    fn cmp_start(&self, other: Hand<'_>) -> Ordering {
        for (index, card) in self.cards.char_indices() {
            let other_card = other.cards.chars().nth(index).unwrap();
            if card == other_card {
                continue;
            }

            if CARDS.find(card.to_string().as_str()).unwrap()
                < CARDS.find(other_card.to_string().as_str()).unwrap()
            {
                return Ordering::Greater;
            }

            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

pub fn winnings(game: &str) -> Vec<usize> {
    let mut hands: Vec<(Hand, usize)> = game
        .split("\n")
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            return (Hand::new(cards), bid.parse::<usize>().unwrap());
        })
        .collect();
    hands.sort_by(|a, b| a.0.cmp(b.0));
    return hands
        .iter()
        .enumerate()
        .map(|(rank, (_hand, bid))| (rank + 1) * bid)
        .collect::<Vec<usize>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_strength() {
        assert_eq!(Hand::strength("AAAAA"), Strength::FiveOfKind);
        assert_eq!(Hand::strength("AA8AA"), Strength::FourOfKind);
        assert_eq!(Hand::strength("A88AA"), Strength::FullHouse);
        assert_eq!(Hand::strength("A28AA"), Strength::ThreeOfKind);
        assert_eq!(Hand::strength("228AA"), Strength::TwoPairs);
        assert_eq!(Hand::strength("228JA"), Strength::OnePair);
        assert_eq!(Hand::strength("2K8JA"), Strength::HighCard);
    }

    #[test]
    fn compare_hands() {
        assert_eq!(
            Hand::new("AAAAA").cmp(Hand::new("AA8AA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("AAAAA").cmp(Hand::new("AA8AA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("AA8AA").cmp(Hand::new("A88AA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("A88AA").cmp(Hand::new("A28AA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("A28AA").cmp(Hand::new("228AA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("228AA").cmp(Hand::new("228JA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("228JA").cmp(Hand::new("2K8JA")),
            Ordering::Greater
        );
        assert_eq!(
            Hand::new("2K8JA").cmp(Hand::new("2Q8JA")),
            Ordering::Greater
        );
        assert_eq!(Hand::new("KTJJT").cmp(Hand::new("KK677")), Ordering::Less);
    }

    #[test]
    fn check_winnings() {
        assert_eq!(winnings("AAAAA 10"), vec![10]);
        assert_eq!(winnings("AAAAA 3\nJAJJJ 15"), vec![15, 6]);
        assert_eq!(winnings("QQQJA 4\nT55J5 10"), vec![10, 8]);
    }
}
