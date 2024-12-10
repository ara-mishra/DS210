use std::collections::BTreeMap;

pub type Graph = BTreeMap<String, BTreeMap<String, f64>>;

pub fn compute_averages(data: &Vec<(String, f64)>) -> BTreeMap<String, f64> {
    let mut sector_totals: BTreeMap<String, (f64, i32)> = BTreeMap::new();

    for (sector, value) in data.iter() {
        sector_totals
            .entry(sector.clone())
            .and_modify(|e| {
                e.0 += value;
                e.1 += 1;
            })
            .or_insert((*value, 1));
    }

    let mut averages = BTreeMap::new();
    for (sector, (total, count)) in sector_totals.iter() {
        averages.insert(sector.clone(), total / (*count as f64));
    }

    averages
}

pub fn build_graph(data: &Vec<(String, f64)>) -> Graph {
    let averages = compute_averages(data);

    let mut graph: Graph = BTreeMap::new();
    let total_gva: f64 = averages.values().sum();
    for (sector, avg_value) in averages.iter() {
        let transition_prob = avg_value / total_gva;
        for (other_sector, _) in averages.iter() {
            if sector != other_sector {
                let weight = transition_prob / (averages.len() as f64 - 1.0);
                graph
                    .entry(sector.clone())
                    .or_insert_with(BTreeMap::new)
                    .insert(other_sector.clone(), weight);
            }
        }
    }

    graph
}
