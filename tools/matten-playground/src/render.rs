//! Two-dimensional matrix rendering (RFC-095).
//!
//! Renders rank-0/1/2 tensors as an aligned, right-aligned grid — a
//! **representation** of the tensor's own structure (rows as rows, columns
//! as columns, numbers as numbers), not a **visualization** that encodes a
//! value as anything other than that value (RFC-093 §6, as amended by
//! RFC-095 §3). Rank > 2 is deliberately left as the flat `values=[...]`
//! list (RFC-095 §6): a 3-D-or-higher tensor has no honest 2-D arrangement,
//! and inventing a reading order would mislead more than the flat list does.
//!
//! The display constants and the `-0.000` clamp are copied verbatim from
//! `tools/matten-report/src/render/common.rs` (RFC-095 §4) rather than
//! shared by dependency — the playground crate is workspace-excluded and
//! cannot import from another workspace-excluded tool. If these values are
//! ever wrong, they are wrong in both places and should change in both.

use matten::Tensor;

/// Columns shown before a rank-2 grid is truncated (`tools/matten-report`'s
/// `MAX_DISPLAY_COLUMNS`).
const MAX_DISPLAY_COLUMNS: usize = 12;
/// Values shown before a rank-1 row is truncated (`tools/matten-report`'s
/// `MAX_TENSOR_PREVIEW_VALUES`) — numerically identical today, but a
/// distinct cap: a rank-1 tensor is a list of values, not a row of columns.
const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

/// `{:.3}`, with `|v| < 0.0005` clamped to `0.0` first — without the clamp a
/// tiny negative renders as `-0.000`, which is confusing on a page whose
/// entire point is that the numbers shown are exactly the numbers computed.
fn format_fixed_value(value: f64) -> String {
    let stable = if value.abs() < 0.0005 { 0.0 } else { value };
    format!("{stable:.3}")
}

/// `"{label:<16} shape={:?} values={:?}"` — the pre-RFC-095 flat line,
/// preserved verbatim for rank > 2 (RFC-095 §6) and reused as the header
/// line for rank ≤ 2's grid.
fn header_line(label: &str, shape: &[usize]) -> String {
    format!("{label:<16} shape={shape:?}")
}

/// Renders a tensor as text: an aligned grid for rank ≤ 2, or the unchanged
/// flat `values=[...]` form for rank > 2 (RFC-095 §5, §6).
pub(crate) fn format_tensor_block(label: &str, t: &Tensor) -> String {
    let shape = t.shape();
    match shape.len() {
        0 => format!(
            "{}\n{}",
            header_line(label, shape),
            format_fixed_value(t.as_slice()[0])
        ),
        1 => format!(
            "{}\n{}",
            header_line(label, shape),
            render_row(t.as_slice())
        ),
        2 => format!(
            "{}\n{}",
            header_line(label, shape),
            render_matrix(t.as_slice(), shape[0], shape[1])
        ),
        _ => format!("{label:<16} shape={shape:?} values={:?}", t.as_slice()),
    }
}

fn render_row(values: &[f64]) -> String {
    let shown = values.len().min(MAX_TENSOR_PREVIEW_VALUES);
    let cells: Vec<String> = values[..shown]
        .iter()
        .map(|&v| format_fixed_value(v))
        .collect();
    let width = cells.iter().map(String::len).max().unwrap_or(0);
    let row = cells
        .iter()
        .map(|c| format!("{c:>width$}"))
        .collect::<Vec<_>>()
        .join(" ");
    if values.len() > MAX_TENSOR_PREVIEW_VALUES {
        format!(
            "{row}\n... {} more values",
            values.len() - MAX_TENSOR_PREVIEW_VALUES
        )
    } else {
        row
    }
}

fn render_matrix(values: &[f64], rows: usize, cols: usize) -> String {
    let shown_cols = cols.min(MAX_DISPLAY_COLUMNS);

    let formatted: Vec<Vec<String>> = (0..rows)
        .map(|r| {
            (0..shown_cols)
                .map(|c| format_fixed_value(values[r * cols + c]))
                .collect()
        })
        .collect();

    let mut widths = vec![0usize; shown_cols];
    for row in &formatted {
        for (c, cell) in row.iter().enumerate() {
            widths[c] = widths[c].max(cell.len());
        }
    }

    let grid = formatted
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(c, cell)| format!("{cell:>w$}", w = widths[c]))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");

    if cols > MAX_DISPLAY_COLUMNS {
        format!("{grid}\n... {} more columns", cols - MAX_DISPLAY_COLUMNS)
    } else {
        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank0_renders_the_scalar_alone() {
        let t = Tensor::scalar(3.5);
        assert_eq!(
            format_tensor_block("value", &t),
            "value            shape=[]\n3.500"
        );
    }

    #[test]
    fn rank1_renders_a_single_right_aligned_row() {
        let t = Tensor::new(vec![1.0, 22.0, 3.0], &[3]);
        assert_eq!(
            format_tensor_block("row", &t),
            "row              shape=[3]\n 1.000 22.000  3.000"
        );
    }

    #[test]
    fn rank2_square_renders_an_aligned_grid() {
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0], &[2, 2]);
        assert_eq!(
            format_tensor_block("m", &t),
            "m                shape=[2, 2]\n1.000 2.000\n3.000 4.000"
        );
    }

    #[test]
    fn rank2_non_square_renders_per_column_widths() {
        // Column 1 is widest (6 chars, "-5.000"); columns 0 and 2 are 5.
        let t = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, -5.0, 6.0], &[2, 3]);
        assert_eq!(
            format_tensor_block("m", &t),
            "m                shape=[2, 3]\n1.000  2.000 3.000\n4.000 -5.000 6.000"
        );
    }

    #[test]
    fn negative_values_stay_right_aligned_against_positive_ones() {
        let t = Tensor::new(vec![-1.0, 2.0], &[2]);
        assert_eq!(
            format_tensor_block("row", &t),
            "row              shape=[2]\n-1.000  2.000"
        );
    }

    #[test]
    fn a_tiny_negative_is_clamped_to_positive_zero() {
        // Without the |v| < 0.0005 clamp this renders "-0.000".
        let t = Tensor::new(vec![-0.0001, 1.0], &[2]);
        assert_eq!(
            format_tensor_block("row", &t),
            "row              shape=[2]\n0.000 1.000"
        );
    }

    #[test]
    fn rank2_beyond_max_display_columns_is_truncated_and_marked() {
        let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
        let t = Tensor::new(values, &[1, 13]);
        let out = format_tensor_block("row", &t);
        assert_eq!(
            out,
            "row              shape=[1, 13]\n\
             1.000 2.000 3.000 4.000 5.000 6.000 7.000 8.000 9.000 10.000 11.000 12.000\n\
             ... 1 more columns"
        );
    }

    #[test]
    fn rank1_beyond_max_preview_values_is_truncated_and_marked() {
        let values: Vec<f64> = (1..=13).map(|x| x as f64).collect();
        let t = Tensor::new(values, &[13]);
        let out = format_tensor_block("row", &t);
        assert_eq!(
            out,
            "row              shape=[13]\n \
             1.000  2.000  3.000  4.000  5.000  6.000  7.000  8.000  9.000 10.000 11.000 12.000\n\
             ... 1 more values"
        );
    }

    #[test]
    fn rank3_is_unchanged_from_the_pre_rfc_095_flat_form() {
        let t = Tensor::new((1..=8).map(|x| x as f64).collect(), &[2, 2, 2]);
        assert_eq!(
            format_tensor_block("cube", &t),
            "cube             shape=[2, 2, 2] values=[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]"
        );
    }
}
