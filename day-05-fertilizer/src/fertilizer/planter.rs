use std::cmp;

use super::parse::MapRange;

pub fn get_dest_ranges(
    source_ranges: Vec<(u64, u64)>,
    mapped_ranges: &Vec<MapRange>,
) -> Vec<(u64, u64)> {
    let mut dest_ranges: Vec<(u64, u64)> = vec![];
    let mut source_ranges_tacker: Vec<(u64, u64)> = source_ranges.clone();
    for map_range in mapped_ranges {
        let mut delete_source_ranges: Vec<(u64, u64)> = vec![];
        let mut add_source_ranges: Vec<(u64, u64)> = vec![];
        for source_range in source_ranges_tacker.iter() {
            // source is fully below the mapping
            if source_range.0 + source_range.1 <= map_range.source_start {
                continue;
            }

            // mapping is fully below the source
            if map_range.source_start + map_range.range <= source_range.0 {
                continue;
            }

            // source is fully within the mapping
            if source_range.0 >= map_range.source_start
                && source_range.0 + source_range.1 <= map_range.source_start + map_range.range
            {
                let offset = source_range.0 - map_range.source_start;
                dest_ranges.push((map_range.destination_start + offset, source_range.1));
                delete_source_ranges.push(*source_range);
                continue;
            }

            // source is partially below the mapping
            if source_range.0 < map_range.source_start {
                let before_range = map_range.source_start - source_range.0;
                dest_ranges.push((map_range.destination_start, source_range.1 - before_range));
                delete_source_ranges.push(*source_range);
                add_source_ranges.push((source_range.0, before_range));
                continue;
            }

            // partially above
            let offset = source_range.0 - map_range.source_start;
            let inclusive_range = map_range.source_start + map_range.range - source_range.0;
            dest_ranges.push((map_range.destination_start + offset, inclusive_range));
            delete_source_ranges.push(*source_range);
            add_source_ranges.push((
                source_range.0 + inclusive_range,
                source_range.1 - inclusive_range,
            ));
        }

        source_ranges_tacker.retain(|x| !delete_source_ranges.contains(x));
        for x in add_source_ranges {
            source_ranges_tacker.push(x);
        }
    }
    dest_ranges.append(&mut source_ranges_tacker);
    return merge_ranges(dest_ranges);
}

fn merge_ranges(original: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut source_ranges = original.clone();
    source_ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged: Vec<(u64, u64)> = vec![];
    for range in source_ranges.iter() {
        if merged.len() == 0 {
            merged.push(*range);
            continue;
        }
        let last_index = merged.len() - 1;

        let last = merged[last_index];
        if range.0 < last.0 + last.1 {
            merged[last_index] = (
                last.0,
                cmp::max(last.0 + last.1, range.0 + range.1) - last.0,
            )
        } else {
            merged.push(*range);
        }
    }
    return merged;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_merges_ranges() {
        // sorted
        assert_eq!(merge_ranges(vec![(0, 1), (1, 1)]), vec![(0, 1), (1, 1)]);
        assert_eq!(merge_ranges(vec![(0, 1), (2, 1)]), vec![(0, 1), (2, 1)]);
        assert_eq!(merge_ranges(vec![(0, 2), (1, 1)]), vec![(0, 2)]);
        assert_eq!(merge_ranges(vec![(0, 10), (2, 1)]), vec![(0, 10)]);

        // unsorted
        assert_eq!(merge_ranges(vec![(1, 1), (0, 1)]), vec![(0, 1), (1, 1)]);
        assert_eq!(merge_ranges(vec![(2, 1), (0, 1)]), vec![(0, 1), (2, 1)]);
        assert_eq!(merge_ranges(vec![(2, 1), (0, 10)]), vec![(0, 10)]);

        assert_eq!(merge_ranges(vec![(5, 5), (8, 5), (10, 10)]), vec![(5, 15)]);
    }

    #[test]
    fn returns_correct_ranges() {
        assert_eq!(
            get_dest_ranges(vec![(10, 10)], &vec![make_map_range(0, 10, 5)]),
            vec![(0, 5), (15, 5)]
        );

        assert_eq!(
            get_dest_ranges(
                vec![(10, 10)],
                &vec![make_map_range(0, 10, 5), make_map_range(50, 15, 20)]
            ),
            vec![(0, 5), (50, 5)]
        );

        assert_eq!(
            get_dest_ranges(vec![(10, 10)], &vec![make_map_range(0, 20, 5)]),
            vec![(10, 10)]
        );

        assert_eq!(
            get_dest_ranges(vec![(10, 10)], &vec![make_map_range(0, 15, 5)]),
            vec![(0, 5), (10, 5)]
        );

        assert_eq!(
            get_dest_ranges(vec![(10, 10)], &vec![make_map_range(0, 5, 10)]),
            vec![(5, 5), (15, 5)]
        );

        assert_eq!(
            get_dest_ranges(
                vec![(55, 13)],
                &vec![make_map_range(50, 98, 2), make_map_range(52, 50, 48),]
            ),
            vec![(57, 13)]
        );

        assert_eq!(
            get_dest_ranges(
                vec![(57, 13)],
                &vec![
                    make_map_range(0, 15, 37),
                    make_map_range(37, 52, 2),
                    make_map_range(39, 0, 15),
                ]
            ),
            vec![(57, 13)]
        );

        assert_eq!(
            get_dest_ranges(
                vec![(57, 13)],
                &vec![
                    make_map_range(49, 53, 8),
                    make_map_range(0, 11, 42),
                    make_map_range(42, 0, 7),
                    make_map_range(57, 7, 4),
                ]
            ),
            vec![(53, 4), (61, 9)]
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
