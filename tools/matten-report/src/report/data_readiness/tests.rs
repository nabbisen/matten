const SMALL_CSV: &str = include_str!("../../../fixtures/small.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

#[test]
fn selected_column_errors_are_readable() {
    let table = matten_data::Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");

    let missing =
        crate::report::data_readiness::build("fixture: small.csv", &table, &selected(&["profit"]))
            .err()
            .expect("missing selection should fail")
            .to_string();
    assert!(missing.contains("column \"profit\" does not exist"));

    let duplicate = crate::report::data_readiness::build(
        "fixture: small.csv",
        &table,
        &selected(&["sales", "sales"]),
    )
    .err()
    .expect("duplicate selection should fail")
    .to_string();
    assert!(duplicate.contains("column \"sales\" was selected more than once"));
}
