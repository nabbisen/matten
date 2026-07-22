use std::error::Error;
use std::fmt::Write as _;

use super::document::{escape, render as render_document, write_shape_flow_table};
use crate::render::common::format_fixed_values;
use crate::report::mlprep_standardization::MlprepStandardizationReportData;
use crate::request::KIND_MLPREP_STANDARDIZATION;

pub(crate) fn render(data: &MlprepStandardizationReportData) -> Result<String, Box<dyn Error>> {
    render_document(
        "matten mlprep-standardization report",
        "Fixed demo report, not automatic model-quality analysis.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Input"))?;
            write_shape_flow_table(
                report,
                &[
                    ("demo", KIND_MLPREP_STANDARDIZATION.to_string()),
                    ("shape", format!("{:?}", data.input_shape)),
                    ("row-major values", format_fixed_values(&data.input_values)),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Operation"))?;
            write_shape_flow_table(
                report,
                &[
                    ("operation", "standardize_columns(input)".to_string()),
                    (
                        "meaning",
                        "each column is centered to mean 0 and population standard deviation 1"
                            .to_string(),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Before"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{:?}", data.input_shape)),
                    ("row-major values", format_fixed_values(&data.input_values)),
                    ("column mean", format_fixed_values(&data.before_mean)),
                    (
                        "column population std",
                        format_fixed_values(&data.before_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("After"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{:?}", data.output_shape)),
                    ("row-major values", format_fixed_values(&data.output_values)),
                    ("column mean", format_fixed_values(&data.after_mean)),
                    (
                        "column population std",
                        format_fixed_values(&data.after_std),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Shape meaning"))?;
            write_shape_flow_table(
                report,
                &[
                    (
                        "shape flow",
                        format!("{:?} -> {:?}", data.input_shape, data.output_shape),
                    ),
                    ("rows", "samples unchanged".to_string()),
                    ("columns", "features unchanged".to_string()),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

#[cfg(test)]
mod tests;
