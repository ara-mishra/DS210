mod utils;
mod graph;
mod pagerank;

use utils::parse_csv;
use graph::build_graph;
use pagerank::compute_pagerank;

fn main() {
    println!("Loading CSV data...");
    let economic_data = parse_csv("/opt/app-root/src/project_test/src/data.csv");

    println!("Creating Graph...");
    let graph = build_graph(&economic_data);

    println!("Running PageRank computations...");
    let ranks = compute_pagerank(&graph);

    println!("Results (influence scores of economic sectors over time):");
    let mut sorted_ranks: Vec<_> = ranks.iter().collect();
    sorted_ranks.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    for (sector, score) in sorted_ranks.iter().take(10) {
        println!("Sector: {}, Score: {:.4}", sector, score);
    }
}
