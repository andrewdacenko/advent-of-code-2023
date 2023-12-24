use geo::Line;
use itertools::Itertools;
use line_intersection::LineInterval;

pub fn count_intersections(input: &str, start: f64, end: f64) -> usize {
    let lines: Vec<LineInterval<f64>> = input
        .lines()
        .map(|data| {
            let (posititon, velocity) = data.split_once("@").expect("must have @");
            let [px, py, _pz] = posititon
                .split(',')
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()[..] else { panic!("Can't parse posititon") };
            let [vx, vy, _vz] = velocity
                .split(',')
                .map(|x| x.trim().parse::<f64>().unwrap())
                .collect::<Vec<f64>>()[..] else { panic!("Can't parse velocity") };

            LineInterval::ray(Line {
                start: (px, py).into(),
                end: (px + vx, py + vy).into(),
            })
        })
        .collect();

    lines
        .iter()
        .combinations(2)
        .map(|pair| pair[0].relate(&pair[1]).unique_intersection())
        .flatten()
        .filter(|coord| {
            coord.0.x >= start as f64
                && coord.0.x <= end as f64
                && coord.0.y >= start as f64
                && coord.0.y <= end as f64
        })
        .count()
}
