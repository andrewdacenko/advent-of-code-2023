use lazy_static::lazy_static;
use regex::{Match, Regex};

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug)]
struct RaceData {
    time: usize,
    distance: usize,
}

pub fn num_ways_to_win(records: &str) -> usize {
    let leaderboard = parse_race(records);

    return leaderboard
        .iter()
        .map(num_ways_to_win_race)
        .fold(1, |acc, res| acc * res);
}

fn num_ways_to_win_race(race_data: &RaceData) -> usize {
    let mut counter = 0;
    for i in 0..race_data.time {
        if (race_data.time - i) * i > race_data.distance {
            counter += 1;
        }
    }
    return counter;
}

fn parse_race(records: &str) -> Vec<RaceData> {
    let (time_str, distance_str) = records.split_once("\n").unwrap();
    let mut res: Vec<RaceData> = vec![];

    let time_data: Vec<usize> = NUM_RE.find_iter(time_str).map(match_tom_num).collect();
    let distance_data: Vec<usize> = NUM_RE.find_iter(distance_str).map(match_tom_num).collect();

    for index in 0..time_data.len() {
        res.push(RaceData {
            time: time_data[index],
            distance: distance_data[index],
        });
    }

    return res;
}

fn match_tom_num(x: Match) -> usize {
    return x.as_str().parse::<usize>().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_counts_ways_to_win_race() {
        assert_eq!(
            num_ways_to_win_race(&RaceData {
                time: 7,
                distance: 9
            }),
            4
        );

        assert_eq!(
            num_ways_to_win_race(&RaceData {
                time: 15,
                distance: 40
            }),
            8
        );

        assert_eq!(
            num_ways_to_win_race(&RaceData {
                time: 30,
                distance: 200
            }),
            9
        );
    }
}
