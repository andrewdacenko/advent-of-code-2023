use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUM_RE: Regex = Regex::new(r"\d+").unwrap();
}

#[derive(Debug, Clone, Copy)]
pub struct MapRange {
    pub destination_start: u64,
    pub source_start: u64,
    pub range: u64,
}

#[derive(Debug, Clone)]
pub struct PlantMap {
    pub seeds: Vec<u64>,
    pub maps: Vec<Vec<MapRange>>,
}

pub fn parse_map(map: &str) -> PlantMap {
    let (seeds_str, maps_str) = map.split_once("\n\n").unwrap();

    PlantMap {
        seeds: NUM_RE
            .find_iter(seeds_str)
            .map(|x| x.as_str().parse::<u64>().unwrap())
            .collect(),
        maps: maps_str.split("\n\n").map(parse_mapping).collect(),
    }
}

fn parse_mapping(input: &str) -> Vec<MapRange> {
    return input
        .split("\n")
        .skip(1)
        .map(|numbers| {
            let [destination_start, source_start, range] = NUM_RE
            .find_iter(&numbers)
            .map(|x| x.as_str().parse::<u64>().unwrap())
            .collect::<Vec<u64>>()[..] else {panic!("Can't parse input:\n{input}")};
            return MapRange {
                destination_start,
                source_start,
                range,
            };
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mapping_correctly() {
        assert_eq!(format!("{:?}", parse_mapping("random map:\n10 1 2")), "");
    }

    #[test]
    fn parsed_map_correctly() {
        let plant_map = parse_map(
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

        assert_eq!(plant_map.seeds.len(), 4);
        assert_eq!(plant_map.maps.len(), 7);
    }
}
