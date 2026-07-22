use std::error::Error;
use std::fmt::Write as _;

use super::document::{escape, render as render_document, write_shape_flow_table};
use crate::report::dynamic_readiness::DynamicReadinessReportData;

pub(crate) fn render(data: &DynamicReadinessReportData) -> Result<String, Box<dyn Error>> {
    render_document(
        "matten dynamic-readiness report",
        "Fixed demo report, not automatic data profiling.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Dynamic values"))?;
            write_shape_flow_table(report, &[("shape", format!("{:?}", data.shape))])?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th><th>{}</th></tr></thead>",
                escape("row"),
                escape("column"),
                escape("value")
            )?;
            writeln!(report, "<tbody>")?;
            for value in &data.values {
                writeln!(
                    report,
                    "<tr><td>{}</td><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    value.row,
                    value.column,
                    escape(&value.element)
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Schema summary"))?;
            writeln!(report, "<table>")?;
            writeln!(
                report,
                "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
                escape("element kind"),
                escape("count")
            )?;
            writeln!(report, "<tbody>")?;
            for row in &data.schema_summary {
                writeln!(
                    report,
                    "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
                    escape(row.label),
                    row.count
                )?;
            }
            writeln!(report, "</tbody>")?;
            writeln!(report, "</table>")?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Readiness masks"))?;
            write_shape_flow_table(
                report,
                &[
                    ("none mask", format!("{:?}", data.none_mask_values)),
                    (
                        "numeric mask",
                        format!("strict policy readiness {:?}", data.numeric_mask_values),
                    ),
                    (
                        "strict numeric-ready",
                        data.strict_numeric_ready.to_string(),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Strict conversion"))?;
            write_shape_flow_table(
                report,
                &[("result", data.strict_conversion_result.to_string())],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Explicit policy conversion"))?;
            write_shape_flow_table(
                report,
                &[
                    ("policy", data.explicit_policy.to_string()),
                    ("converted shape", format!("{:?}", data.converted_shape)),
                    (
                        "converted row-major values",
                        format!("{:?}", data.converted_values),
                    ),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

#[cfg(test)]
mod tests;
