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

pub fn game_power(line: &str) -> usize {
    let (_game, rounds) = line.split_once(": ").unwrap();
    let mut max_red: usize = 1;
    let mut max_green: usize = 1;
    let mut max_blue: usize = 1;
    for round in rounds.split("; ") {
        for rolls in round.split(", ") {
            for roll in rolls.split(", ") {
                let (cubes, color) = roll.split_once(" ").unwrap();
                let cubes_count = cubes.parse::<usize>().unwrap();
                if color == "red" && cubes_count.cmp(&max_red).is_gt() {
                    max_red = cubes_count
                }
                if color == "blue" && cubes_count.cmp(&max_blue).is_gt() {
                    max_blue = cubes_count
                }
                if color == "green" && cubes_count.cmp(&max_green).is_gt() {
                    max_green = cubes_count
                }
            }
        }
    }

    max_red * max_blue * max_green
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

    #[test]
    fn test_game_power() {
        assert_eq!(
            game_power("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            48
        );
        assert_eq!(
            game_power("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            12
        );
    }
}
