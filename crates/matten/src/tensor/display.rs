//! `Display` for `Tensor` (RFC-100): a human-facing grid, split from
//! `tensor.rs` per the 300-ELOC guideline. Distinct from the single-line
//! `Debug` in `tensor.rs`, which RFC-020 owns and this module does not touch.
//!
//! Rank 0/1/2 render as a right-aligned grid using `{:?}` per cell — never
//! bare `Display` on the cell, which would drop the `.0` on whole numbers
//! and make an all-float grid read as integers (the defect RFC-096's C1
//! corrected, made permanent here). Rank > 2 falls back to the flat
//! `shape=... values=[...]` form used before this RFC existed (RFC-095 §6 /
//! RFC-096): a 3-D-or-higher tensor has no honest 2-D arrangement, and this
//! is a boundary, not a gap.
//!
//! Truncates a rank-1 row at `MAX_TENSOR_PREVIEW_VALUES` values and a rank-2
//! grid at `MAX_DISPLAY_COLUMNS` columns (both 12, matching the constants
//! `tools/matten-report/src/render/common.rs` already fixed), so `Display`
//! on a huge tensor cannot flood a terminal. `{:#}` (the alternate flag)
//! disables truncation — an explicit escape hatch, not the default (RFC-100
//! §5.4's open question, resolved for this implementation; see the review
//! request for the reasoning). Row count is not truncated, matching all
//! three formatters this RFC replaces — a pre-existing gap this RFC does
//! not close.
//!
//! Dynamic tensors render using `Element`'s own `Display` (RFC-100 §5.5) —
//! **except `Float`**, which renders via `{:?}` on the inner `f64` instead
//! (review C1): `Element`'s own `Display` for `Float` is bare `Display`
//! (`Float(2.0)` -> `"2"`), which is indistinguishable from `Int(2)` -> `"2"`.
//! A dynamic tensor exists precisely to carry mixed types in one grid, so
//! erasing that distinction defeats the point more than it did for the pure
//! numeric case RFC-100 §5.2 already covers. `Element`'s own `Display` impl
//! is deliberately left untouched — it is separate public surface with its
//! own users; this override lives only in the grid renderer below.

use crate::Tensor;
use std::fmt;

#[cfg(feature = "dynamic")]
use crate::dynamic::Element;
#[cfg(feature = "dynamic")]
use crate::dynamic::storage::DynamicTensor;

/// Columns shown in a rank-2 grid before truncation (unless `{:#}`).
const MAX_DISPLAY_COLUMNS: usize = 12;
/// Values shown in a rank-1 row before truncation (unless `{:#}`).
const MAX_TENSOR_PREVIEW_VALUES: usize = 12;

impl fmt::Display for Tensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "dynamic")]
        if let Some(dyn_t) = &self.dynamic {
            return fmt_dynamic(dyn_t, f);
        }
        fmt_numeric(&self.data, &self.shape, f)
    }
}

fn fmt_numeric(data: &[f64], shape: &[usize], f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match shape {
        [] => write!(f, "{:?}", data[0]),
        [n] => f.write_str(&row(*n, f.alternate(), |i| format!("{:?}", data[i]))),
        [rows, cols] => f.write_str(&grid(*rows, *cols, f.alternate(), |i| {
            format!("{:?}", data[i])
        })),
        _ => write!(f, "shape={shape:?} values={data:?}"),
    }
}

/// `Element`'s own `Display`, except `Float`, which uses `{:?}` on the inner
/// `f64` so it stays visibly distinct from `Int` in a mixed-type grid (C1).
#[cfg(feature = "dynamic")]
fn dynamic_cell_text(e: &Element) -> String {
    match e {
        Element::Float(v) => format!("{v:?}"),
        other => other.to_string(),
    }
}

#[cfg(feature = "dynamic")]
fn fmt_dynamic(dyn_t: &DynamicTensor, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let shape = dyn_t.shape.as_slice();
    let cell = |i: usize| dyn_t.get_flat(i).map(dynamic_cell_text).unwrap_or_default();
    match shape {
        [] => f.write_str(&cell(0)),
        [n] => f.write_str(&row(*n, f.alternate(), cell)),
        [rows, cols] => f.write_str(&grid(*rows, *cols, f.alternate(), cell)),
        _ => {
            let values: Vec<String> = (0..dyn_t.len).map(cell).collect();
            write!(f, "shape={shape:?} values=[{}]", values.join(", "))
        }
    }
}

/// Right-aligns `n` cells (produced by `cell(flat_index)`) into one row,
/// truncated at `MAX_TENSOR_PREVIEW_VALUES` unless `alternate`.
fn row(n: usize, alternate: bool, cell: impl Fn(usize) -> String) -> String {
    let shown = if alternate {
        n
    } else {
        n.min(MAX_TENSOR_PREVIEW_VALUES)
    };
    let cells: Vec<String> = (0..shown).map(cell).collect();
    let width = cells.iter().map(String::len).max().unwrap_or(0);
    let line = cells
        .iter()
        .map(|c| format!("{c:>width$}"))
        .collect::<Vec<_>>()
        .join(" ");
    if shown < n {
        format!("{line}\n... {} more values", n - shown)
    } else {
        line
    }
}

/// Right-aligns a `rows` × `cols` grid (produced by `cell(flat_index)`),
/// per-column widths, truncated at `MAX_DISPLAY_COLUMNS` columns unless
/// `alternate`.
fn grid(rows: usize, cols: usize, alternate: bool, cell: impl Fn(usize) -> String) -> String {
    let shown_cols = if alternate {
        cols
    } else {
        cols.min(MAX_DISPLAY_COLUMNS)
    };
    let formatted: Vec<Vec<String>> = (0..rows)
        .map(|r| (0..shown_cols).map(|c| cell(r * cols + c)).collect())
        .collect();
    let mut widths = vec![0usize; shown_cols];
    for line in &formatted {
        for (c, cell) in line.iter().enumerate() {
            widths[c] = widths[c].max(cell.len());
        }
    }
    let body = formatted
        .iter()
        .map(|line| {
            line.iter()
                .enumerate()
                .map(|(c, cell)| format!("{cell:>w$}", w = widths[c]))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n");
    if shown_cols < cols {
        format!("{body}\n... {} more columns", cols - shown_cols)
    } else {
        body
    }
}

#[cfg(test)]
mod tests;
