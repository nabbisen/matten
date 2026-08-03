use std::error::Error;
use std::fmt::Write as _;

use super::grid::{debug_cell, render_matrix_block};
use crate::render::common::format_fixed_values;
use crate::report::educational_path::EducationalPathReportData;
use crate::request::KIND_EDUCATIONAL_PATH;

pub(crate) fn render(data: &EducationalPathReportData) -> Result<String, Box<dyn Error>> {
    let mut report = String::new();
    writeln!(report, "# matten educational-path report")?;
    writeln!(report)?;

    writeln!(report, "## Input")?;
    writeln!(report, "demo: {KIND_EDUCATIONAL_PATH}")?;
    writeln!(
        report,
        "note: fixed educational demo report, not automatic expression tracing"
    )?;
    writeln!(report)?;

    writeln!(report, "## How to read shapes first")?;
    for (index, step) in data.reading_steps.iter().enumerate() {
        writeln!(report, "{}. {}", index + 1, step)?;
    }
    writeln!(report)?;

    writeln!(report, "## Broadcasting")?;
    writeln!(
        report,
        "shape flow: {:?} + {:?} -> {:?}",
        data.broadcast.left_shape, data.broadcast.right_shape, data.broadcast.result_shape
    )?;
    writeln!(report, "axis 1: left repeats across 4 columns")?;
    writeln!(report, "axis 0: right repeats across 3 rows")?;
    writeln!(report, "result values:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.broadcast.result_shape[0],
            data.broadcast.result_shape[1],
            &data.broadcast.result_values,
            debug_cell
        )
    )?;
    writeln!(report)?;

    writeln!(report, "## Reshape and transpose")?;
    writeln!(
        report,
        "reshape: {:?} -> {:?}",
        data.reshape_transpose.input_shape, data.reshape_transpose.reshape_shape
    )?;
    writeln!(report, "reshape values:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.reshape_transpose.reshape_shape[0],
            data.reshape_transpose.reshape_shape[1],
            &data.reshape_transpose.reshape_values,
            debug_cell
        )
    )?;
    writeln!(
        report,
        "transpose: {:?} -> {:?}",
        data.reshape_transpose.input_shape, data.reshape_transpose.transpose_shape
    )?;
    writeln!(report, "transpose values:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.reshape_transpose.transpose_shape[0],
            data.reshape_transpose.transpose_shape[1],
            &data.reshape_transpose.transpose_values,
            debug_cell
        )
    )?;
    writeln!(
        report,
        "meaning: reshape changes grouping; transpose changes coordinate meaning"
    )?;
    writeln!(report)?;

    writeln!(report, "## Axis reductions")?;
    writeln!(
        report,
        "mean_axis(0): {:?} -> {:?}",
        data.axis_reductions.input_shape, data.axis_reductions.mean_axis_0_shape
    )?;
    writeln!(
        report,
        "mean_axis(0) keeps columns: {:?}",
        data.axis_reductions.mean_axis_0_values
    )?;
    writeln!(
        report,
        "mean_axis(1): {:?} -> {:?}",
        data.axis_reductions.input_shape, data.axis_reductions.mean_axis_1_shape
    )?;
    writeln!(
        report,
        "mean_axis(1) keeps rows: {:?}",
        data.axis_reductions.mean_axis_1_values
    )?;
    writeln!(report)?;

    writeln!(report, "## Matrix multiplication")?;
    writeln!(
        report,
        "shape flow: {:?} @ {:?} -> {:?}",
        data.matmul.left_shape, data.matmul.right_shape, data.matmul.result_shape
    )?;
    writeln!(
        report,
        "shared inner dimension: {}",
        data.matmul.shared_inner_dimension
    )?;
    writeln!(report, "result values:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.matmul.result_shape[0],
            data.matmul.result_shape[1],
            &data.matmul.result_values,
            debug_cell
        )
    )?;
    writeln!(report)?;

    writeln!(report, "## Dynamic readiness")?;
    writeln!(report, "dynamic shape: {:?}", data.dynamic_readiness.shape)?;
    writeln!(report, "none mask:")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.dynamic_readiness.shape[0],
            data.dynamic_readiness.shape[1],
            &data.dynamic_readiness.none_mask_values,
            debug_cell
        )
    )?;
    writeln!(report, "numeric mask: strict policy readiness")?;
    writeln!(
        report,
        "{}",
        render_matrix_block(
            data.dynamic_readiness.shape[0],
            data.dynamic_readiness.shape[1],
            &data.dynamic_readiness.numeric_mask_values,
            debug_cell
        )
    )?;
    writeln!(
        report,
        "Text values are not numeric-ready under the strict mask"
    )?;
    writeln!(report, "next step: clean values, then call try_numeric()")?;
    writeln!(report)?;

    writeln!(report, "## Standardization")?;
    writeln!(report, "operation: standardize_columns(input)")?;
    writeln!(
        report,
        "shape flow: {:?} -> {:?}",
        data.standardization.input_shape, data.standardization.output_shape
    )?;
    writeln!(
        report,
        "before column mean: {}",
        format_fixed_values(&data.standardization.before_mean)
    )?;
    writeln!(
        report,
        "before column population std: {}",
        format_fixed_values(&data.standardization.before_std)
    )?;
    writeln!(
        report,
        "after column mean: {}",
        format_fixed_values(&data.standardization.after_mean)
    )?;
    writeln!(
        report,
        "after column population std: {}",
        format_fixed_values(&data.standardization.after_std)
    )?;
    writeln!(report)?;

    writeln!(report, "## What this report is not")?;
    for non_goal in &data.non_goals {
        writeln!(report, "- {non_goal}")?;
    }

    Ok(report)
}

#[cfg(test)]
mod tests;
