use std::cmp::Ordering;

use itertools::Itertools;

use graphrs::algorithms::centrality::betweenness::betweenness_centrality;
use graphrs::{Graph, GraphSpecs};

pub fn central_edges_variants(input: &str) -> Vec<Vec<(&str, &str)>> {
    let mut graph: Graph<&str, ()> = Graph::new(GraphSpecs::undirected_create_missing());
    let edges = input
        .lines()
        .map(|line| {
            let (start, end) = line.split_once(":").unwrap();
            end.split_whitespace()
                .map(|i| (start, i.trim()))
                .collect::<Vec<(&str, &str)>>()
        })
        .flatten()
        .collect::<Vec<(&str, &str)>>();
    for (u, v) in edges.iter() {
        let _ = graph.add_edge_tuple(u, v);
    }

    let betweenness = betweenness_centrality(&graph, false, true).unwrap();
    let mut sorted: Vec<(&&str, &f64)> = betweenness.iter().collect();
    sorted.sort_by(|a, b| {
        if a.1.gt(b.1) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let nodes = sorted
        .iter()
        .take(6)
        .map(|item| *item.0)
        .collect::<Vec<&str>>();

    let comb = nodes.iter().combinations(2).collect_vec();
    let target_edges = comb
        .iter()
        .filter(|edge| edges.contains(&(edge[0], edge[1])) || edges.contains(&(edge[1], edge[0])))
        .collect_vec();

    return target_edges
        .iter()
        .combinations(3)
        .map(|v| {
            v.iter()
                .map(|x| (*x[0], *x[1]))
                .collect::<Vec<(&str, &str)>>()
        })
        .collect::<Vec<Vec<(&str, &str)>>>();
}
