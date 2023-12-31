use std::vec;

use self::parse::{MapRange, PlantMap};

mod parse;
mod planter;

pub fn plant_location(map: &str) -> u64 {
    let plant_map = parse::parse_map(map);
    return plant_map
        .seeds
        .clone()
        .iter()
        .map(|seed| find_location_for_seed(*seed, plant_map.maps.clone()))
        .min()
        .unwrap();
}

pub fn plant_ranged_location(map: &str) -> u64 {
    let plant_map = parse::parse_map(map);
    return get_all_seeds_from_map(plant_map.clone())
        .iter()
        .map(|seed_range| find_location_for_seed_range(*seed_range, plant_map.maps.clone()))
        .min()
        .unwrap();
}

fn get_all_seeds_from_map(plant_map: PlantMap) -> Vec<(u64, u64)> {
    let mut seeds: Vec<(u64, u64)> = vec![];
    for i in 0..(plant_map.seeds.len() / 2) {
        let seed = plant_map.seeds[i * 2];
        let seed_range = plant_map.seeds[i * 2 + 1];
        seeds.push((seed, seed_range))
    }
    return seeds;
}

fn find_location_for_seed(seed: u64, maps: Vec<Vec<MapRange>>) -> u64 {
    return maps.iter().fold(seed, |source, ranges| {
        ranges
            .iter()
            .find(|map_range| {
                source >= map_range.source_start
                    && source < map_range.source_start + map_range.range
            })
            .and_then(|map_range| {
                Some(map_range.destination_start + source - map_range.source_start)
            })
            .unwrap_or(source)
    });
}

fn find_location_for_seed_range(seed_range: (u64, u64), maps: Vec<Vec<MapRange>>) -> u64 {
    let plant_ranges: Vec<(u64, u64)> =
        maps.iter().fold(vec![seed_range], planter::get_dest_ranges);
    return *plant_ranges.iter().map(|r| &r.0).min().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_seed_location() {
        assert_eq!(
            find_location_for_seed(12, vec![vec![make_map_range(10, 1, 2)]]),
            12
        );
    }

    #[test]
    fn single_mapping() {
        assert_eq!(
            find_location_for_seed(12, vec![vec![make_map_range(48, 10, 3)]]),
            50
        );
    }

    #[test]
    fn find_range_locations() {
        assert_eq!(
            find_location_for_seed_range(
                (77, 11),
                vec![vec![make_map_range(0, 69, 1), make_map_range(1, 0, 69),]]
            ),
            77
        );

        assert_eq!(
            find_location_for_seed_range(
                (45, 3),
                vec![vec![make_map_range(0, 69, 1), make_map_range(1, 0, 69),]]
            ),
            46
        );

        assert_eq!(
            find_location_for_seed_range((74, 14), vec![vec![make_map_range(45, 77, 23),]]),
            45
        );

        assert_eq!(
            find_location_for_seed_range((10, 5), vec![vec![make_map_range(0, 8, 5)]]),
            2
        );

        assert_eq!(
            find_location_for_seed_range((10, 5), vec![vec![make_map_range(0, 20, 5)]]),
            10
        );

        assert_eq!(
            find_location_for_seed_range((10, 5), vec![vec![make_map_range(0, 10, 5)]]),
            0
        );

        assert_eq!(
            find_location_for_seed_range(
                (10, 5),
                vec![
                    vec![make_map_range(0, 10, 5)],
                    vec![make_map_range(10, 0, 5)]
                ]
            ),
            10
        );

        assert_eq!(
            find_location_for_seed_range((13, 5), vec![vec![make_map_range(0, 10, 5)]]),
            3
        );
    }

    #[test]
    fn parsed_map_correctly() {
        let plant_map = parse::parse_map(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );

        assert_eq!(
            find_location_for_seed(plant_map.seeds[0], plant_map.clone().maps),
            82
        );

        assert_eq!(
            find_location_for_seed(plant_map.seeds[1], plant_map.clone().maps),
            43
        );

        assert_eq!(
            find_location_for_seed(plant_map.seeds[2], plant_map.clone().maps),
            86
        );

        assert_eq!(
            find_location_for_seed(plant_map.seeds[3], plant_map.clone().maps),
            35
        );
    }

    fn make_map_range(destination_start: u64, source_start: u64, range: u64) -> MapRange {
        MapRange {
            destination_start,
            source_start,
            range,
        }
    }
}
