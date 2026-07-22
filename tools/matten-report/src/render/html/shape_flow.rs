use std::error::Error;
use std::fmt::Write as _;

use super::document::{escape, render as render_document, write_pre, write_shape_flow_table};
use crate::report::shape_flow::ShapeFlowReportData;

pub(crate) fn render(data: &ShapeFlowReportData) -> Result<String, Box<dyn Error>> {
    render_document(
        "matten shape-flow report",
        "Fixed demo report, not automatic expression tracing.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Broadcast add"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input a", format!("{:?}", data.broadcast.input_a_shape)),
                    ("input b", format!("{:?}", data.broadcast.input_b_shape)),
                    ("result", format!("{:?}", data.broadcast.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape(&format!("operation: {}", data.broadcast.operation))
            )?;
            write_pre(
                report,
                &format!("result values: {:?}", data.broadcast.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Reshape"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.reshape.input_shape)),
                    ("result", format!("{:?}", data.reshape.result_shape)),
                ],
            )?;
            writeln!(
                report,
                "<p>{}</p>",
                escape(&format!("operation: {}", data.reshape.operation))
            )?;
            write_pre(
                report,
                &format!("result values: {:?}", data.reshape.result_values),
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Axis reductions"))?;
            write_shape_flow_table(
                report,
                &[
                    ("input", format!("{:?}", data.axis.input_shape)),
                    (
                        "mean_axis(0)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis.input_shape, data.axis.mean_axis_0_shape
                        ),
                    ),
                    (
                        "mean_axis(1)",
                        format!(
                            "{:?} -> {:?}",
                            data.axis.input_shape, data.axis.mean_axis_1_shape
                        ),
                    ),
                ],
            )?;
            write_pre(
                report,
                &format!(
                    "mean_axis(0) values: {:?}\nmean_axis(1) values: {:?}",
                    data.axis.mean_axis_0_values, data.axis.mean_axis_1_values
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
                escape(&format!("operation: {}", data.matmul.operation))
            )?;
            write_pre(
                report,
                &format!("result values: {:?}", data.matmul.result_values),
            )?;
            writeln!(report, "</section>")
        },
    )
}

#[cfg(test)]
mod tests;
