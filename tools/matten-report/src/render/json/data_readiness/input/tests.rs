use super::*;

use matten_data::Table;

const SMALL_CSV: &str = include_str!("../../../../../fixtures/small.csv");
const NON_NUMERIC_CSV: &str = include_str!("../../../../../fixtures/non_numeric.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn build(csv: &str, label: &str) -> DataReadinessReportData {
    let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
    crate::report::data_readiness::build(label, &table, &selected(&["sales", "cost"]))
        .expect("input report data should build")
}

mod error;
mod policy;
mod success;
