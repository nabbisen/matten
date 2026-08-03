use std::error::Error;
use std::fmt::Write as _;

use super::grid::{debug_cell, render_matrix_block};
use crate::report::dynamic_readiness::DynamicReadinessReportData;
use crate::request::KIND_DYNAMIC_READINESS;

pub(crate) fn render(data: &DynamicReadinessReportData) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten dynamic-readiness report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_DYNAMIC_READINESS}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic data profiling"
    )?;
    writeln!(report)?;

    writeln!(report, "## Dynamic values")?;
    writeln!(report, "shape: {:?}", data.shape)?;
    writeln!(report, "row-major values:")?;
    for value in &data.values {
        writeln!(
            report,
            "- [{}, {}] {}",
            value.row, value.column, value.element
        )?;
    }
    writeln!(report, "schema summary:")?;
    for row in &data.schema_summary {
        writeln!(report, "- {}: {}", row.label, row.count)?;
    }
    writeln!(report)?;

    writeln!(report, "## Readiness masks")?;
    writeln!(report, "none mask:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.shape[0],
            data.shape[1],
            &data.none_mask_values,
            debug_cell
        )
    )?;
    writeln!(report, "numeric mask: strict policy readiness")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.shape[0],
            data.shape[1],
            &data.numeric_mask_values,
            debug_cell
        )
    )?;
    writeln!(
        report,
        "strict numeric-ready: {}",
        data.strict_numeric_ready
    )?;
    writeln!(report)?;

    writeln!(report, "## Strict conversion")?;
    writeln!(report, "result: {}", data.strict_conversion_result)?;
    writeln!(report)?;

    writeln!(report, "## Explicit policy conversion")?;
    writeln!(report, "policy: {}", data.explicit_policy)?;
    writeln!(report, "converted shape: {:?}", data.converted_shape)?;
    writeln!(report, "converted row-major values:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.converted_shape[0],
            data.converted_shape[1],
            &data.converted_values,
            debug_cell
        )
    )?;

    Ok(report)
}

#[cfg(test)]
mod tests;
