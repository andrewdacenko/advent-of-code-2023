use self::parse::{MapRange, PlantMap};

mod parse;

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
        .map(|seed| find_location_for_seed(*seed, plant_map.maps.clone()))
        .min()
        .unwrap();
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
            find_location_for_seed(plant_map.seeds[0], plant_map.clone().maps),
            86
        );

        assert_eq!(
            find_location_for_seed(plant_map.seeds[0], plant_map.clone().maps),
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
