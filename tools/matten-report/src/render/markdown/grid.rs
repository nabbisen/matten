//! Rank-2 grid rendering for the Markdown demo renderers (RFC-097 §5).
//!
//! A rank-2 tensor printed as a flat `[v0, v1, v2, ...]` list loses the one
//! thing that made it rank 2 in the first place — the arrangement is the
//! information a reader needs (RFC-097 §2). This is the third local
//! implementation of the fix RFC-095 (the playground) and RFC-096 (the
//! shipped example) already applied elsewhere: `tools/matten-playground` is
//! workspace-excluded and cannot be imported from a published-adjacent tool
//! either, and a shared public helper in core `matten` was rejected for the
//! same reason RFC-096 §4(a) rejected it there — a presentation concern does
//! not earn public surface for one benefit.
//!
//! Rank ≤ 1 values are UNCHANGED elsewhere in this crate's renderers — a
//! rank-1 list already *is* a list, so only call sites confirmed rank-2 use
//! this module (RFC-097 §5.1).
//!
//! Cell formatting matches what each call site already used before this
//! change — only the *arrangement* changes here, not the number format.
//! Most sites used `{:?}` (e.g. `1.0`, natural/`Debug` float rendering, the
//! same choice RFC-096's C1 correction made for the shipped example — never
//! bare `Display`, so a whole number still reads as a float);
//! `mlprep-standardization`'s two sites already used
//! [`format_fixed_value`](crate::render::common::format_fixed_value)
//! (`{:.3}` with the `-0.000` clamp) and keep doing so, passed in by the
//! caller rather than hardcoded here.
//!
//! An mdBook Markdown page collapses runs of whitespace outside a fenced
//! code block, so the aligned grid is wrapped in a `` ```text `` fence —
//! without it, right-alignment would render correctly in this crate's tests
//! but collapse to single spaces once mdBook turns the page into HTML.

/// The default cell formatter: natural/`Debug` float rendering (`1.0`, not
/// bare `Display`'s `1`), matching what every call site except
/// `mlprep-standardization`'s already printed before the grid existed.
pub(crate) fn debug_cell(v: f64) -> String {
    format!("{v:?}")
}

/// Renders `values` (row-major, `rows` × `cols`) as a right-aligned grid
/// inside a fenced code block, ready to follow a `writeln!(report, "...:")`
/// header line. `format_cell` renders one value to text; pass whatever this
/// call site already used before the grid existed.
pub(crate) fn render_matrix_block(
    rows: usize,
    cols: usize,
    values: &[f64],
    format_cell: impl Fn(f64) -> String,
) -> String {
    debug_assert_eq!(
        rows * cols,
        values.len(),
        "render_matrix_block: rows * cols must match values.len()"
    );

    let formatted: Vec<Vec<String>> = (0..rows)
        .map(|r| {
            (0..cols)
                .map(|c| format_cell(values[r * cols + c]))
                .collect()
        })
        .collect();

    let mut widths = vec![0usize; cols];
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

    format!("```text\n{grid}\n```")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_a_square_grid() {
        let block = render_matrix_block(2, 2, &[1.0, 2.0, 3.0, 4.0], debug_cell);
        assert_eq!(block, "```text\n1.0 2.0\n3.0 4.0\n```");
    }

    #[test]
    fn aligns_per_column_width_including_a_negative_value() {
        // Column 1 is widest ("-5.0"); column 0 is uniformly "1.0"/"4.0".
        let block = render_matrix_block(2, 2, &[1.0, -5.0, 4.0, 6.0], debug_cell);
        assert_eq!(block, "```text\n1.0 -5.0\n4.0  6.0\n```");
    }

    #[test]
    fn renders_a_non_square_grid() {
        let block = render_matrix_block(3, 2, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0], debug_cell);
        assert_eq!(block, "```text\n1.0 2.0\n3.0 4.0\n5.0 6.0\n```");
    }

    #[test]
    fn accepts_a_custom_cell_formatter() {
        // mlprep-standardization's sites pass format_fixed_value ({:.3} with
        // the -0.000 clamp) instead of the default {:?} used everywhere else.
        use crate::render::common::format_fixed_value;
        let block = render_matrix_block(1, 2, &[-0.0001, 1.0], format_fixed_value);
        assert_eq!(block, "```text\n0.000 1.000\n```");
    }
}
