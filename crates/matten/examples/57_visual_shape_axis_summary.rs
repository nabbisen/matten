//! Visual shape and axis summary for a few core tensor operations.
//!
//! Run: cargo run --example 57_visual_shape_axis_summary
//!
//! This example prints short, deterministic summaries: inputs, operation,
//! output shape, and small output values. It is intentionally not a full
//! tutorial; the mdBook reference pages carry the larger diagrams.

use matten::Tensor;

/// Renders a tensor as an aligned rank-2 grid, a rank-1 row, or the rank-0
/// scalar, so Reshape's two blocks visibly *look* different instead of
/// printing two identical value lists (RFC-096, applying RFC-095's playground
/// fix here too). Natural float rendering (`{:?}`, not `{:.3}`) — this file's
/// values are hand-chosen teaching numbers like `1.0`, and forcing three
/// decimal places would add noise for no gain (RFC-096 §5). `{:?}`, not `{}`:
/// `matten`'s only element type is `f64`, and `Display` drops the `.0` on
/// whole numbers, which would make a matrix of floats read as one of ints —
/// worse than the flat list this RFC set out to fix. The rank > 2 arm is
/// kept for safety even though no tensor in this file reaches it.
///
/// Deliberately local to this example, not a core `matten` public helper or
/// an import of the playground's formatter (RFC-096 §4): a presentation
/// concern for one example does not earn core public surface, and the
/// playground crate is workspace-excluded and unpublishable. Because this
/// logic cannot be unit-tested — examples build as binaries, so `#[test]`
/// inside one never runs — every call site below asserts the exact rendered
/// block, padding included, before printing it.
fn render_block(label: &str, t: &Tensor) -> String {
    let shape = t.shape();
    let header = format!("{label:<16} shape={shape:?}");
    match shape.len() {
        0 => format!("{header}\n{}", t.as_slice()[0]),
        1 => format!("{header}\n{}", render_row(t.as_slice())),
        2 => format!(
            "{header}\n{}",
            render_matrix(t.as_slice(), shape[0], shape[1])
        ),
        _ => format!("{label:<16} shape={shape:?} values={:?}", t.as_slice()),
    }
}

fn render_row(values: &[f64]) -> String {
    let cells: Vec<String> = values.iter().map(|v| format!("{v:?}")).collect();
    let width = cells.iter().map(String::len).max().unwrap_or(0);
    cells
        .iter()
        .map(|c| format!("{c:>width$}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn render_matrix(values: &[f64], rows: usize, cols: usize) -> String {
    let formatted: Vec<Vec<String>> = (0..rows)
        .map(|r| {
            (0..cols)
                .map(|c| format!("{:?}", values[r * cols + c]))
                .collect()
        })
        .collect();

    let mut widths = vec![0usize; cols];
    for row in &formatted {
        for (c, cell) in row.iter().enumerate() {
            widths[c] = widths[c].max(cell.len());
        }
    }

    formatted
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .map(|(c, cell)| format!("{cell:>w$}", w = widths[c]))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let a = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let b = Tensor::new(vec![10.0, 20.0, 30.0], &[3]);

    println!("== Broadcasting ==");
    let input_a = render_block("input A", &a);
    assert_eq!(
        input_a,
        "input A          shape=[2, 3]\n1.0 2.0 3.0\n4.0 5.0 6.0"
    );
    println!("{input_a}");
    let input_b = render_block("input b", &b);
    assert_eq!(input_b, "input b          shape=[3]\n10.0 20.0 30.0");
    println!("{input_b}");
    let broadcast = &a + &b;
    let broadcast_block = render_block("A + b", &broadcast);
    assert_eq!(
        broadcast_block,
        "A + b            shape=[2, 3]\n11.0 22.0 33.0\n14.0 25.0 36.0"
    );
    println!("{broadcast_block}");
    println!("meaning         b repeats across rows");
    assert_eq!(broadcast.shape(), &[2, 3]);
    assert_eq!(broadcast.as_slice(), &[11.0, 22.0, 33.0, 14.0, 25.0, 36.0]);

    println!();
    println!("== Reshape ==");
    let reshaped = a.reshape(&[3, 2]);
    let reshape_input = render_block("[2, 3] input", &a);
    assert_eq!(
        reshape_input,
        "[2, 3] input     shape=[2, 3]\n1.0 2.0 3.0\n4.0 5.0 6.0"
    );
    println!("{reshape_input}");
    let reshape_view = render_block("[3, 2] view", &reshaped);
    assert_eq!(
        reshape_view,
        "[3, 2] view      shape=[3, 2]\n1.0 2.0\n3.0 4.0\n5.0 6.0"
    );
    println!("{reshape_view}");
    println!("meaning         row-major values stay in the same order");
    assert_eq!(reshaped.shape(), &[3, 2]);
    assert_eq!(reshaped.as_slice(), a.as_slice());

    println!();
    println!("== Axis reductions ==");
    let col_means = a.mean_axis(0);
    let row_means = a.mean_axis(1);
    println!(
        "mean_axis(0)    collapse rows, keep columns -> shape {:?}, values {:?}",
        col_means.shape(),
        col_means.as_slice()
    );
    println!(
        "mean_axis(1)    collapse columns, keep rows -> shape {:?}, values {:?}",
        row_means.shape(),
        row_means.as_slice()
    );
    assert_eq!(col_means.shape(), &[3]);
    assert_eq!(col_means.as_slice(), &[2.5, 3.5, 4.5]);
    assert_eq!(row_means.shape(), &[2]);
    assert_eq!(row_means.as_slice(), &[2.0, 5.0]);

    println!();
    println!("== Matrix multiplication ==");
    let left = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[2, 3]);
    let right = Tensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], &[3, 2]);
    let product = left.matmul(&right);
    let left_block = render_block("left", &left);
    assert_eq!(
        left_block,
        "left             shape=[2, 3]\n1.0 2.0 3.0\n4.0 5.0 6.0"
    );
    println!("{left_block}");
    let right_block = render_block("right", &right);
    assert_eq!(
        right_block,
        "right            shape=[3, 2]\n1.0 2.0\n3.0 4.0\n5.0 6.0"
    );
    println!("{right_block}");
    let product_block = render_block("left.matmul", &product);
    assert_eq!(
        product_block,
        "left.matmul      shape=[2, 2]\n22.0 28.0\n49.0 64.0"
    );
    println!("{product_block}");
    println!("meaning         [2, 3] x [3, 2] -> [2, 2]");
    assert_eq!(product.shape(), &[2, 2]);
    assert_eq!(product.as_slice(), &[22.0, 28.0, 49.0, 64.0]);

    println!();
    println!("57_visual_shape_axis_summary: OK");
}
