use petgraph::graphmap::UnGraphMap;

use crate::centrality::central_edges_variants;

pub fn count_groups(input: &str) -> usize {
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

    let mut graph: UnGraphMap<&str, ()> = UnGraphMap::from_edges(edges.iter());

    for target_edges in central_edges_variants(input) {
        for x in target_edges {
            graph.remove_edge(x.0, x.1);
        }
        let sub = petgraph::algo::kosaraju_scc(&graph);
        if sub.len().eq(&2) {
            return sub[0].len() * sub[1].len();
        }
    }

    panic!("Not found");
}
