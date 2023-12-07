use std::{cmp::Ordering, collections::HashMap};

static CARDS: &str = "AKQJT987654321";

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
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

impl Hand {
    fn new(cards: &str) -> Hand {
        return Hand {
            cards: cards.to_string(),
            strength: Hand::strength(cards),
        };
    }

    fn new_with_joker(cards: &str) -> Hand {
        return Hand {
            cards: cards.replace("J", "1"),
            strength: Hand::strength_with_joker(cards),
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

    fn strength_with_joker(cards: &str) -> Strength {
        let jokers = cards.chars().filter(|card| card.eq(&'J')).count();
        if jokers == 5 {
            return Strength::FiveOfKind;
        }
        let strength = Hand::strength(
            cards
                .replacen("J", "X", 1)
                .replacen("J", "Y", 1)
                .replacen("J", "Z", 1)
                .replacen("J", "*", 1)
                .as_str(),
        );
        if strength.eq(&Strength::FiveOfKind) {
            return strength;
        }
        return match jokers {
            0 => strength,
            1 => match strength {
                Strength::FourOfKind => Strength::FiveOfKind,
                Strength::ThreeOfKind => Strength::FourOfKind,
                Strength::TwoPairs => Strength::FullHouse,
                Strength::OnePair => Strength::ThreeOfKind,
                Strength::HighCard => Strength::OnePair,
                _ => panic!(
                    "Can't map strength {:?} with single joker in {cards}",
                    strength
                ),
            },
            2 => match strength {
                Strength::ThreeOfKind => Strength::FiveOfKind,
                Strength::OnePair => Strength::FourOfKind,
                Strength::HighCard => Strength::ThreeOfKind,
                _ => {
                    panic!(
                        "Can't map strength {:?} with single 2 jokers in {cards}",
                        strength
                    )
                }
            },
            3 => match strength {
                Strength::OnePair => Strength::FiveOfKind,
                Strength::HighCard => Strength::FourOfKind,
                _ => panic!("Can't map strength with 3 jokers {:?}", strength),
            },
            4 => Strength::FiveOfKind,
            x => panic!("Too much jokers: {x}, max allowed 5"),
        };
    }

    fn cmp(&self, other: &Hand) -> Ordering {
        if self.strength.eq(&other.strength) {
            return self.cmp_start(other);
        } else if (self.strength as usize) < other.strength as usize {
            return Ordering::Greater;
        } else {
            return Ordering::Less;
        }
    }

    fn cmp_start(&self, other: &Hand) -> Ordering {
        for (index, card) in self.cards.char_indices() {
            let other_card = other.cards.chars().nth(index).unwrap();
            if card == other_card {
                continue;
            }

            if CARDS.find(card).unwrap() < CARDS.find(other_card).unwrap() {
                return Ordering::Greater;
            }

            return Ordering::Less;
        }
        return Ordering::Equal;
    }
}

pub fn winnings(game: &str, joker: bool) -> Vec<usize> {
    let mut hands: Vec<(Hand, usize)> = game
        .split("\n")
        .map(|line| {
            let (cards, bid) = line.split_once(" ").unwrap();
            let hand = if joker {
                Hand::new_with_joker(cards)
            } else {
                Hand::new(cards)
            };
            return (hand, bid.parse::<usize>().unwrap());
        })
        .collect();
    hands.sort_by(|a, b| a.0.cmp(&b.0));
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
    fn check_strength_with_no_jokers() {
        assert_eq!(Hand::strength("AAAAA"), Strength::FiveOfKind);
        assert_eq!(Hand::strength("AA8AA"), Strength::FourOfKind);
        assert_eq!(Hand::strength("A88AA"), Strength::FullHouse);
        assert_eq!(Hand::strength("A28AA"), Strength::ThreeOfKind);
        assert_eq!(Hand::strength("228AA"), Strength::TwoPairs);
        assert_eq!(Hand::strength("228JA"), Strength::OnePair);
        assert_eq!(Hand::strength("2K8JA"), Strength::HighCard);
    }

    #[test]
    fn check_strength_with_one_joker() {
        assert_eq!(Hand::strength_with_joker("AAAAA"), Strength::FiveOfKind);
        assert_eq!(Hand::strength_with_joker("JJJJJ"), Strength::FiveOfKind);
        assert_eq!(Hand::strength_with_joker("AJAAA"), Strength::FiveOfKind);
    }

    #[test]
    fn check_strength_with_two_jokers() {
        assert_eq!(Hand::strength_with_joker("AAA8J"), Strength::FourOfKind);
        assert_eq!(Hand::strength_with_joker("AA88J"), Strength::FullHouse);
        assert_eq!(Hand::strength_with_joker("AA87J"), Strength::ThreeOfKind);
        assert_eq!(Hand::strength_with_joker("A872J"), Strength::OnePair);
    }

    #[test]
    fn check_strength_with_three_jokers() {
        assert_eq!(Hand::strength_with_joker("AAA8J"), Strength::FourOfKind);
        assert_eq!(Hand::strength_with_joker("AA88J"), Strength::FullHouse);
        assert_eq!(Hand::strength_with_joker("AA87J"), Strength::ThreeOfKind);
        assert_eq!(Hand::strength_with_joker("A872J"), Strength::OnePair);
    }

    #[test]
    fn check_strength_with_four_jokers() {
        assert_eq!(Hand::strength_with_joker("AJJJJ"), Strength::FiveOfKind);
    }

    #[test]
    fn check_strength_with_five_jokers() {
        assert_eq!(Hand::strength_with_joker("JJJJJ"), Strength::FiveOfKind);
    }

    #[test]
    fn compare_hands() {
        assert_eq!(Hand::new("AA8AA").cmp(&Hand::new("AAAAA")), Ordering::Less);
        assert_eq!(Hand::new("AA8AA").cmp(&Hand::new("AAAAA")), Ordering::Less);
        assert_eq!(Hand::new("A88AA").cmp(&Hand::new("AA8AA")), Ordering::Less);
        assert_eq!(Hand::new("A28AA").cmp(&Hand::new("A88AA")), Ordering::Less);
        assert_eq!(Hand::new("228AA").cmp(&Hand::new("A28AA")), Ordering::Less);
        assert_eq!(Hand::new("228JA").cmp(&Hand::new("228AA")), Ordering::Less);
        assert_eq!(Hand::new("2K8JA").cmp(&Hand::new("228JA")), Ordering::Less);
        assert_eq!(Hand::new("2Q8JA").cmp(&Hand::new("2K8JA")), Ordering::Less);
        assert_eq!(Hand::new("KTJJT").cmp(&Hand::new("KK677")), Ordering::Less);
    }

    #[test]
    fn compare_hands_with_joker() {
        assert_eq!(Hand::new("AA8AA").cmp(&Hand::new("AAAAA")), Ordering::Less);
        assert_eq!(Hand::new("AA8AA").cmp(&Hand::new("AAAAA")), Ordering::Less);
        assert_eq!(Hand::new("A88AA").cmp(&Hand::new("AA8AA")), Ordering::Less);
        assert_eq!(Hand::new("A28AA").cmp(&Hand::new("A88AA")), Ordering::Less);
        assert_eq!(Hand::new("228AA").cmp(&Hand::new("A28AA")), Ordering::Less);
        assert_eq!(Hand::new("228JA").cmp(&Hand::new("228AA")), Ordering::Less);
        assert_eq!(Hand::new("2K8JA").cmp(&Hand::new("228JA")), Ordering::Less);
        assert_eq!(Hand::new("2Q8JA").cmp(&Hand::new("2K8JA")), Ordering::Less);
        assert_eq!(Hand::new("KTJJT").cmp(&Hand::new("KK677")), Ordering::Less);
    }

    #[test]
    fn check_winnings() {
        assert_eq!(winnings("AAAAA 10", false), vec![10]);
        assert_eq!(winnings("AAAAA 3\nJAJJJ 15", false), vec![15, 6]);
        assert_eq!(winnings("QQQJA 4\nT55J5 10", false), vec![10, 8]);
    }

    #[test]
    fn check_winnings_with_joker() {
        assert_eq!(winnings("AAAAA 10", true), vec![10]);
        assert_eq!(winnings("AAAAA 3\nJAJJJ 15", true), vec![15, 6]);
        assert_eq!(winnings("QQQJA 4\nT55J5 10", true), vec![10, 8]);
    }
}
