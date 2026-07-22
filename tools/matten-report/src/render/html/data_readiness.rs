use std::error::Error;
use std::fmt::Write as _;

use super::document::{escape, render, write_shape_flow_table};
use crate::report::data_readiness::{DataReadinessConversion, DataReadinessReportData};

const MAX_DISPLAY_COLUMNS: usize = 12;
const MAX_DISPLAY_CHARS: usize = 120;
const MAX_ERROR_CHARS: usize = 240;
const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

pub(crate) fn render_demo(data: &DataReadinessReportData) -> Result<String, Box<dyn Error>> {
    let (tensor_shape, tensor_values) = match &data.conversion {
        DataReadinessConversion::Success {
            tensor_shape,
            tensor_values,
        } => (tensor_shape, tensor_values),
        DataReadinessConversion::Error { .. } => {
            return Err("fixed data-readiness HTML requires successful conversion".into());
        }
    };
    render(
        "matten data-readiness report",
        "Fixed demo report, not arbitrary CSV profiling.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Input"))?;
            write_shape_flow_table(report, &[("input", data.input_label.to_string())])?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Columns"))?;
            write_shape_flow_table(
                report,
                &[
                    ("source columns", data.source_columns.join(", ")),
                    ("selected columns", data.selected_columns.join(", ")),
                    ("columns left out", data.left_out_columns.join(", ")),
                ],
            )?;
            writeln!(report, "</section>")?;

            write_missing_values(report, data, false)?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Numeric conversion"))?;
            write_shape_flow_table(report, &[("strict conversion", "success".to_string())])?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Tensor preview"))?;
            write_shape_flow_table(
                report,
                &[
                    ("shape", format!("{tensor_shape:?}")),
                    ("row-major values", format!("{tensor_values:?}")),
                ],
            )?;
            writeln!(report, "</section>")
        },
    )
}

pub(crate) fn render_input(data: &DataReadinessReportData) -> Result<String, Box<dyn Error>> {
    render(
        "matten data-readiness report",
        "Bounded summary of the provided CSV file; not a full raw table rendering.",
        |report| {
            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Input"))?;
            write_shape_flow_table(
                report,
                &[("input", cap_display(&data.input_label, MAX_DISPLAY_CHARS))],
            )?;
            writeln!(report, "</section>")?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Columns"))?;
            write_shape_flow_table(
                report,
                &[
                    ("source columns", format_display_list(&data.source_columns)),
                    (
                        "selected columns",
                        format_display_list(&data.selected_columns),
                    ),
                    (
                        "columns left out",
                        format_display_list(&data.left_out_columns),
                    ),
                ],
            )?;
            writeln!(report, "</section>")?;

            write_missing_values(report, data, true)?;

            writeln!(report, "<section>")?;
            writeln!(report, "<h2>{}</h2>", escape("Numeric conversion"))?;
            match &data.conversion {
                DataReadinessConversion::Success {
                    tensor_shape,
                    tensor_values,
                } => {
                    write_shape_flow_table(
                        report,
                        &[("strict conversion", "success".to_string())],
                    )?;
                    writeln!(report, "</section>")?;

                    writeln!(report, "<section>")?;
                    writeln!(report, "<h2>{}</h2>", escape("Tensor preview"))?;
                    write_shape_flow_table(
                        report,
                        &[
                            ("shape", format!("{tensor_shape:?}")),
                            ("row-major values", format_tensor_preview(tensor_values)),
                        ],
                    )?;
                }
                DataReadinessConversion::Error { message } => {
                    write_shape_flow_table(
                        report,
                        &[
                            ("strict conversion", "error".to_string()),
                            ("error", cap_display(message, MAX_ERROR_CHARS)),
                        ],
                    )?;
                }
            }
            writeln!(report, "</section>")
        },
    )
}

fn write_missing_values(
    report: &mut String,
    data: &DataReadinessReportData,
    bounded: bool,
) -> Result<(), std::fmt::Error> {
    writeln!(report, "<section>")?;
    writeln!(report, "<h2>{}</h2>", escape("Missing values"))?;
    writeln!(report, "<table>")?;
    writeln!(
        report,
        "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
        escape("column"),
        escape("missing")
    )?;
    writeln!(report, "<tbody>")?;
    let limit = if bounded {
        MAX_DISPLAY_COLUMNS
    } else {
        data.missing_counts.len()
    };
    for row in data.missing_counts.iter().take(limit) {
        let column = if bounded {
            cap_display(&row.column, MAX_DISPLAY_CHARS)
        } else {
            row.column.clone()
        };
        writeln!(
            report,
            "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
            escape(&column),
            row.missing
        )?;
    }
    if bounded && data.missing_counts.len() > MAX_DISPLAY_COLUMNS {
        writeln!(
            report,
            "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
            escape(&format!(
                "... {} more",
                data.missing_counts.len() - MAX_DISPLAY_COLUMNS
            )),
            escape("not shown")
        )?;
    }
    writeln!(report, "</tbody>")?;
    writeln!(report, "</table>")?;
    writeln!(report, "</section>")
}

fn cap_display(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let keep = max_chars.saturating_sub(3);
    let mut capped: String = value.chars().take(keep).collect();
    capped.push_str("...");
    capped
}

fn format_display_list(values: &[String]) -> String {
    let mut parts: Vec<String> = values
        .iter()
        .take(MAX_DISPLAY_COLUMNS)
        .map(|value| cap_display(value, MAX_DISPLAY_CHARS))
        .collect();
    if values.len() > MAX_DISPLAY_COLUMNS {
        parts.push(format!("... {} more", values.len() - MAX_DISPLAY_COLUMNS));
    }
    parts.join(", ")
}

fn format_tensor_preview(values: &[f64]) -> String {
    let mut parts: Vec<String> = values
        .iter()
        .take(MAX_TENSOR_PREVIEW_VALUES)
        .map(|value| format!("{value:?}"))
        .collect();
    if values.len() > MAX_TENSOR_PREVIEW_VALUES {
        parts.push(format!(
            "... {} more",
            values.len() - MAX_TENSOR_PREVIEW_VALUES
        ));
    }
    format!("[{}]", parts.join(", "))
}

#[cfg(test)]
mod tests;
