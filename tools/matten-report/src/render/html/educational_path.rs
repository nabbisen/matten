use std::error::Error;
use std::fmt::Write as _;

use super::document::{escape, render as render_document, write_pre, write_shape_flow_table};
use crate::render::common::format_fixed_values;
use crate::report::educational_path::EducationalPathReportData;

pub(crate) fn render(data: &EducationalPathReportData) -> Result<String, Box<dyn Error>> {
    render_document(
        "matten educational-path report",
        "Fixed educational demo report, not automatic expression tracing.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("How to read shapes first"))?;
            writeln!(report, "<ol>")?;
            for item in data.reading_steps {
                writeln!(report, "<li>{}</li>", escape(item))?;
            }
            writeln!(report, "</ol>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Broadcasting"))?;
            write_shape_flow_table(
                report,
                &[
                    ("left", format!("{:?}", data.broadcast.left_shape)),
                    ("right", format!("{:?}", data.broadcast.right_shape)),
                    ("result", format!("{:?}", data.broadcast.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape(
                    "axis 1: left repeats across 4 columns; axis 0: right repeats across 3 rows"
                )
            )?;
            write_pre(
                report,
                &format!("result values: {:?}", data.broadcast.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Reshape and transpose"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.reshape_transpose.input_shape)),
                    (
                        "reshape",
                        format!("{:?}", data.reshape_transpose.reshape_shape),
                    ),
                    (
                        "transpose",
                        format!("{:?}", data.reshape_transpose.transpose_shape),
                    ),
                ],
            )?;
            write_pre(
                report,
                &format!(
                    "reshape values: {:?}\ntranspose values: {:?}",
                    data.reshape_transpose.reshape_values, data.reshape_transpose.transpose_values
                ),
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape("reshape changes grouping; transpose changes coordinate meaning")
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Axis reductions"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "mean_axis(0)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis_reductions.input_shape,
                            data.axis_reductions.mean_axis_0_shape
                        ),
                    ),
                    (
                        "mean_axis(1)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis_reductions.input_shape,
                            data.axis_reductions.mean_axis_1_shape
                        ),
                    ),
                ],
            )?;
            write_pre(
                report,
                &format!(
                    "mean_axis(0) keeps columns: {:?}\nmean_axis(1) keeps rows: {:?}",
                    data.axis_reductions.mean_axis_0_values,
                    data.axis_reductions.mean_axis_1_values
                ),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Matrix multiplication"))?;
            write_shape_flow_table(
                report,
                &[
                    ("left", format!("{:?}", data.matmul.left_shape)),
                    ("right", format!("{:?}", data.matmul.right_shape)),
                    ("result", format!("{:?}", data.matmul.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape(&format!(
                    "shared inner dimension: {}",
                    data.matmul.shared_inner_dimension
                ))
            )?;
            write_pre(
                report,
                &format!("result values: {:?}", data.matmul.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Dynamic readiness"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "dynamic shape",
                        format!("{:?}", data.dynamic_readiness.shape),
                    ),
                    (
                        "none mask",
                        format!("{:?}", data.dynamic_readiness.none_mask_values),
                    ),
                    (
                        "numeric mask",
                        format!(
                            "strict policy readiness {:?}",
                            data.dynamic_readiness.numeric_mask_values
                        ),
                    ),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape(
                    "Text values are not numeric-ready under the strict mask; clean values, then call try_numeric()."
                )
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Standardization"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "shape flow",
                        format!(
                            "{:?} -> {:?}",
                            data.standardization.input_shape, data.standardization.output_shape
                        ),
                    ),
                    (
                        "before mean",
                        format_fixed_values(&data.standardization.before_mean),
                    ),
                    (
                        "before population std",
                        format_fixed_values(&data.standardization.before_std),
                    ),
                    (
                        "after mean",
                        format_fixed_values(&data.standardization.after_mean),
                    ),
                    (
                        "after population std",
                        format_fixed_values(&data.standardization.after_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("What this report is not"))?;
            writeln!(report, "<ul>")?;
            for item in data.non_goals {
                writeln!(report, "<li>{}</li>", escape(item))?;
            }
            writeln!(report, "</ul>")?;
            writeln!(report, "</section>")?;

            Ok(())
        },
    )
}

#[cfg(test)]
mod tests;
