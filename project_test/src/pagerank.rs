use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::collections::BTreeMap;

pub fn compute_pagerank(graph: &BTreeMap<String, BTreeMap<String, f64>>) -> BTreeMap<String, f64> {
    let num_walks = 80;
    let walk_length = 80;
    let damping_factor = 0.1;

    let mut rng = StdRng::seed_from_u64(42);

    let mut visit_counts: BTreeMap<String, usize> = BTreeMap::new();
    for node in graph.keys() {
        visit_counts.insert(node.clone(), 0);
    }

    let vertices: Vec<&String> = graph.keys().collect();

    println!("Graph vertices: {:?}", vertices);

    for start_vertex in graph.keys() {
        for _ in 0..num_walks {
            let mut current_vertex = start_vertex;

            for _ in 0..walk_length {
                if rng.gen_bool(damping_factor) || graph[current_vertex].is_empty() {
                    current_vertex = vertices[rng.gen_range(0..vertices.len())];
                } else {
                    let neighbors = &graph[current_vertex];
                    let cumulative_weights: Vec<f64> = neighbors
                        .values()
                        .scan(0.0, |acc, &weight| {
                            *acc += weight;
                            Some(*acc)
                        })
                        .collect();

                    let total_weight = *cumulative_weights.last().unwrap();
                    let rand_weight = rng.gen_range(0.0..total_weight);

                    let next_vertex = neighbors
                        .keys()
                        .zip(cumulative_weights.iter())
                        .find(|(_, &weight)| rand_weight <= weight)
                        .map(|(neighbor, _)| neighbor)
                        .unwrap_or(&current_vertex);

                    current_vertex = next_vertex;
                }

                *visit_counts.get_mut(current_vertex).unwrap() += 1;
            }
        }
    }

    let total_visits: usize = visit_counts.values().sum();
    let mut pagerank = BTreeMap::new();
    for (vertex, count) in visit_counts.iter() {
        pagerank.insert(vertex.clone(), *count as f64 / total_visits as f64);
    }

    pagerank
}

#[cfg(test)]
mod tests {
    use super::*; 
    use crate::graph::build_graph; 

    #[test]
    fn test_pagerank_non_negative() {
        let data = vec![
            ("Agriculture".to_string(), 30.0),
            ("Industry".to_string(), 50.0),
            ("Services".to_string(), 20.0),
        ];

        let graph = build_graph(&data);
        let pagerank = compute_pagerank(&graph);

        for &score in pagerank.values() {
            assert!(score >= 0.0, "PageRank score is negative: {}", score);
        }
    }

    #[test]
    fn test_pagerank_sums_to_one() {
        let data = vec![
            ("Agriculture".to_string(), 30.0),
            ("Industry".to_string(), 50.0),
            ("Services".to_string(), 20.0),
        ];

        let graph = build_graph(&data);
        let pagerank = compute_pagerank(&graph);

        let total_score: f64 = pagerank.values().sum();
        assert!((total_score - 1.0).abs() < 1e-6, "PageRank scores do not sum to 1: {}", total_score);
    }
}
