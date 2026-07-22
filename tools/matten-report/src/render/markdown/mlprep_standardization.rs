use std::error::Error;
use std::fmt::Write as _;

use crate::render::common::format_fixed_values;
use crate::report::mlprep_standardization::MlprepStandardizationReportData;
use crate::request::KIND_MLPREP_STANDARDIZATION;

pub(crate) fn render(data: &MlprepStandardizationReportData) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten mlprep-standardization report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_MLPREP_STANDARDIZATION}")?;
    writeln!(
        report,
        "note: fixed demo report, not automatic model-quality analysis"
    )?;
    writeln!(report)?;

    writeln!(report, "## Operation")?;
    writeln!(report, "operation: standardize_columns(input)")?;
    writeln!(
        report,
        "meaning: each column is centered to mean 0 and population standard deviation 1"
    )?;
    writeln!(report)?;

    writeln!(report, "## Before")?;
    writeln!(report, "shape: {:?}", data.input_shape)?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(&data.input_values)
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(&data.before_mean)
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(&data.before_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## After")?;
    writeln!(report, "shape: {:?}", data.output_shape)?;
    writeln!(
        report,
        "row-major values: {}",
        format_fixed_values(&data.output_values)
    )?;
    writeln!(
        report,
        "column mean: {}",
        format_fixed_values(&data.after_mean)
    )?;
    writeln!(
        report,
        "column population std: {}",
        format_fixed_values(&data.after_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## Shape meaning")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.input_shape, data.output_shape
    )?;
    writeln!(report, "rows: samples unchanged")?;
    writeln!(report, "columns: features unchanged")?;

    Ok(report)
}

#[cfg(test)]
mod tests;
