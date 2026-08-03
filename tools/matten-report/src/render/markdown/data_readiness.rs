use std::error::Error;
use std::fmt::Write as _;

use super::grid::{debug_cell, render_matrix_block};
use crate::report::data_readiness::{DataReadinessConversion, DataReadinessReportData};

pub(crate) fn render(data: &DataReadinessReportData) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten data-readiness report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "{}", data.input_label)?;
    writeln!(report)?;

    writeln!(report, "## Source columns")?;
    write_list(&mut report, &data.source_columns)?;
    writeln!(report)?;

    writeln!(report, "## Selected columns")?;
    write_list(&mut report, &data.selected_columns)?;
    writeln!(report)?;

    writeln!(report, "## Columns left out")?;
    write_list(&mut report, &data.left_out_columns)?;
    writeln!(report)?;

    writeln!(report, "## Missing values")?;
    writeln!(report, "| column | missing |")?;
    writeln!(report, "|---|---:|")?;
    for row in &data.missing_counts {
        writeln!(report, "| {} | {} |", row.column, row.missing)?;
    }
    writeln!(report)?;

    writeln!(report, "## Numeric conversion")?;
    match &data.conversion {
        DataReadinessConversion::Success {
            tensor_shape,
            tensor_values,
        } => {
            writeln!(report, "strict conversion: success")?;
            writeln!(report)?;
            writeln!(report, "## Tensor preview")?;
            writeln!(report, "shape: {tensor_shape:?}")?;
            writeln!(report, "row-major values:")?;
            // Table::to_tensor() always builds shape [rows, columns] (see
            // crates/matten-data/src/numeric.rs) -- rank 2 unconditionally,
            // for both this fixed demo and live --input mode, so this call
            // is safe without a rank check.
            writeln!(
                report,
                "{}",
                render_matrix_block(tensor_shape[0], tensor_shape[1], tensor_values, debug_cell)
            )?;
        }
        DataReadinessConversion::Error { message } => {
            writeln!(report, "strict conversion: error: {message}")?;
        }
    }

    Ok(report)
}

fn write_list(report: &mut String, values: &[String]) -> Result<(), std::fmt::Error> {
    if values.is_empty() {
        writeln!(report, "- none")?;
    } else {
        for value in values {
            writeln!(report, "- {value}")?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
