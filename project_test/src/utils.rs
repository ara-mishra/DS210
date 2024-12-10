use csv::ReaderBuilder;

pub fn parse_csv(file_path: &str) -> Vec<(String, f64)> {
    let mut rdr = ReaderBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .expect("Failed to open CSV file");

    let mut data = Vec::new();
    for result in rdr.records() {
        if let Ok(record) = result {
            let series = &record[3];
            let value: f64 = record[4].parse().unwrap_or(0.0);

            if series != "" {
                data.push((series.to_string(), value));
            }
        }
    }

    data
}
