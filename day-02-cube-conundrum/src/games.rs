use std::usize;

use phf::phf_map;

static MAX_CUBES: phf::Map<&'static str, usize> = phf_map! {
    "red" => 12,
    "green" => 13,
    "blue" => 14,
};

pub fn process_game(line: &str) -> usize {
    let (game, rounds) = line.split_once(": ").unwrap();
    match is_game_possible(rounds) {
        true => game.split_once(" ").unwrap().1.parse::<usize>().unwrap(),
        false => 0,
    }
}

fn is_game_possible(rounds: &str) -> bool {
    rounds.split("; ").all(|round| is_round_possible(round))
}

fn is_round_possible(round: &str) -> bool {
    round.split(", ").all(|roll| {
        let (cubes, color) = roll.split_once(" ").unwrap();
        cubes
            .to_owned()
            .parse::<usize>()
            .unwrap()
            .cmp(MAX_CUBES.get(color).unwrap())
            .is_le()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn possible_round() {
        assert_eq!(is_round_possible("3 blue, 4 red"), true);
        assert_eq!(is_round_possible("1 red, 2 green, 6 blue"), true);
        assert_eq!(is_round_possible("2 green"), true);
    }

    #[test]
    fn impossible_round() {
        assert_eq!(is_round_possible("13 red"), false);
        assert_eq!(is_round_possible("15 blue, 4 red"), false);
        assert_eq!(is_round_possible("1 red, 14 green, 6 blue"), false);
    }
}
