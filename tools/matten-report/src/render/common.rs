pub(crate) fn format_fixed_values(values: &[f64]) -> String {
    let values = values
        .iter()
        .map(|&value| format_fixed_value(value))
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{values}]")
}

fn format_fixed_value(value: f64) -> String {
    let stable = if value.abs() < 0.0005 { 0.0 } else { value };
    format!("{stable:.3}")
}
