//! Thin WebAssembly binding for the browser shape playground (RFC-093).
//!
//! Parses the page's text inputs, calls core `matten`, and formats a result
//! string in the vocabulary of
//! `crates/matten/examples/57_visual_shape_axis_summary.rs`. All shape and
//! error logic lives here, in Rust, where `cargo test` reaches it — the JS
//! glue (`docs/theme/playground.js`) only reads input boxes, calls these
//! exports, and writes the returned string into the page (RFC-093 handoff
//! §3). Workspace-excluded and `publish = false`; nothing here reaches
//! crates.io.

use matten::{MattenError, Tensor};
use wasm_bindgen::prelude::*;

mod render;
use render::format_tensor_block;

// ---- input parsing --------------------------------------------------------

/// Parses a comma-separated list of non-negative integers, e.g. `"2, 3"`.
fn parse_shape(s: &str) -> Result<Vec<usize>, String> {
    s.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.parse::<usize>()
                .map_err(|_| format!("\"{p}\" is not a non-negative integer"))
        })
        .collect()
}

/// Parses a comma-separated list of `f64` values, e.g. `"1, 2.5, -3"`.
fn parse_values(s: &str) -> Result<Vec<f64>, String> {
    s.split(',')
        .map(str::trim)
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.parse::<f64>()
                .map_err(|_| format!("\"{p}\" is not a number"))
        })
        .collect()
}

/// Parses shape and value text, then builds a `Tensor` via [`Tensor::try_new`]
/// — a length mismatch or invalid shape surfaces the real
/// [`MattenError::Shape`] text, not a playground-invented message.
fn build_tensor(shape_str: &str, values_str: &str) -> Result<Tensor, String> {
    let shape = parse_shape(shape_str)?;
    let values = parse_values(values_str)?;
    Tensor::try_new(values, &shape).map_err(|e| e.to_string())
}

// ---- output formatting -----------------------------------------------------

fn error_block(message: &str) -> String {
    format!("Error: {message}")
}

// ---- broadcasting -----------------------------------------------------------

/// Computes the NumPy-style broadcast result shape, or the incompatibility
/// message.
///
/// There is no public `try_add`/`try_broadcast` on `Tensor` — `+` panics on
/// incompatible shapes (RFC-006). Its panic message (`apply_binary` in
/// `crates/matten/src/ops/broadcast.rs`) discards the underlying
/// `MattenError::Broadcast` entirely and writes its own string, so that is
/// the text reproduced here verbatim — verified by triggering the real panic
/// natively and diffing the output character for character, since
/// `MattenError::Broadcast`'s own `Display` text (`crates/matten/src/error.rs`)
/// is worded differently and is not what a caller of `+` actually sees.
fn broadcast_result_shape(left: &[usize], right: &[usize]) -> Result<Vec<usize>, String> {
    let out_rank = left.len().max(right.len());
    let mut result = vec![0usize; out_rank];
    for (i, slot) in result.iter_mut().enumerate() {
        let l = left
            .len()
            .checked_sub(out_rank - i)
            .map_or(1, |idx| left[idx]);
        let r = right
            .len()
            .checked_sub(out_rank - i)
            .map_or(1, |idx| right[idx]);
        *slot = match (l, r) {
            (a, b) if a == b => a,
            (1, b) => b,
            (a, 1) => a,
            _ => {
                // "add" because playground_broadcast always drives `+`.
                return Err(format!(
                    "matten broadcast error in add: shapes {left:?} and {right:?} are not compatible"
                ));
            }
        };
    }
    Ok(result)
}

/// One short clause per axis that actually broadcasts (an axis where the
/// shapes already agree says nothing, matching the example's one-line
/// gloss style rather than an exhaustive per-axis dump).
fn describe_broadcast(
    left_shape: &[usize],
    right_shape: &[usize],
    result_shape: &[usize],
) -> String {
    let rank = result_shape.len();
    let pad = |s: &[usize]| -> Vec<usize> {
        let mut v = vec![1usize; rank];
        if !s.is_empty() {
            v[rank - s.len()..].copy_from_slice(s);
        }
        v
    };
    let lp = pad(left_shape);
    let rp = pad(right_shape);

    let clauses: Vec<String> = (0..rank)
        .filter(|&axis| lp[axis] != rp[axis])
        .map(|axis| {
            if lp[axis] == 1 {
                format!("left repeats along axis {axis}")
            } else {
                format!("right repeats along axis {axis}")
            }
        })
        .collect();

    if clauses.is_empty() {
        "shapes already match; no broadcasting".to_string()
    } else {
        clauses.join(", ")
    }
}

/// Two shapes and values -> the broadcast result, or the real incompatibility
/// message (RFC-093 §5).
#[wasm_bindgen]
pub fn playground_broadcast(
    left_shape: &str,
    left_values: &str,
    right_shape: &str,
    right_values: &str,
) -> String {
    let a = match build_tensor(left_shape, left_values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };
    let b = match build_tensor(right_shape, right_values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };

    let result_shape = match broadcast_result_shape(a.shape(), b.shape()) {
        Ok(s) => s,
        Err(e) => return error_block(&e),
    };

    // Guaranteed not to panic: the same compatibility rule was just checked.
    let sum = &a + &b;
    debug_assert_eq!(sum.shape(), result_shape.as_slice());

    format!(
        "{}\n{}\n{}\nmeaning          {}",
        format_tensor_block("input A", &a),
        format_tensor_block("input b", &b),
        format_tensor_block("A + b", &sum),
        describe_broadcast(a.shape(), b.shape(), &result_shape)
    )
}

// ---- reshape ----------------------------------------------------------------

/// Shape + values + a target shape -> the reshaped tensor, or the real
/// [`MattenError`] text from [`Tensor::try_reshape`] (RFC-093 §5).
#[wasm_bindgen]
pub fn playground_reshape(shape: &str, values: &str, target_shape: &str) -> String {
    let t = match build_tensor(shape, values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };
    let target = match parse_shape(target_shape) {
        Ok(s) => s,
        Err(e) => return error_block(&e),
    };

    match t.try_reshape(&target) {
        Ok(reshaped) => format!(
            "{}\n{}\nmeaning          row-major values stay in the same order",
            format_tensor_block("input", &t),
            format_tensor_block("reshaped", &reshaped)
        ),
        Err(e) => error_block(&format_matten_error(&e)),
    }
}

// ---- axis reductions ----------------------------------------------------------

/// A short, human phrase for what an axis reduction does, matching example
/// 57's rank-2 wording exactly and falling back to a rank-general phrase for
/// everything else.
fn describe_axis_reduction(rank: usize, axis: usize) -> String {
    match (rank, axis) {
        (2, 0) => "collapse rows, keep columns".to_string(),
        (2, 1) => "collapse columns, keep rows".to_string(),
        _ => format!("collapse axis {axis}, keep the rest"),
    }
}

/// Shape + values + an axis + a reduction kind (`sum`/`mean`/`min`/`max`) ->
/// the reduced tensor, or the real [`MattenError`] text (RFC-093 §5).
#[wasm_bindgen]
pub fn playground_axis_reduce(shape: &str, values: &str, axis: &str, op: &str) -> String {
    let t = match build_tensor(shape, values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };
    let axis_n: usize = match axis.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            return error_block(&format!(
                "\"{}\" is not a non-negative integer",
                axis.trim()
            ));
        }
    };

    let rank = t.shape().len();
    let result = match op {
        "sum" => t.try_sum_axis(axis_n),
        "mean" => t.try_mean_axis(axis_n),
        "min" => t.try_min_axis(axis_n),
        "max" => t.try_max_axis(axis_n),
        other => {
            return error_block(&format!(
                "unknown reduction \"{other}\"; choose one of sum, mean, min, max"
            ));
        }
    };

    match result {
        Ok(reduced) => format!(
            "{}\n{op}_axis({axis_n})    {}\n{}",
            format_tensor_block("input", &t),
            describe_axis_reduction(rank, axis_n),
            format_tensor_block("result", &reduced)
        ),
        Err(e) => error_block(&format_matten_error(&e)),
    }
}

// ---- matmul -------------------------------------------------------------------

/// Mirrors `Tensor::matmul`'s internal rank dispatch and its exact panic text
/// (`crates/matten/src/math.rs`), returning the result shape or that text.
///
/// `matmul`/`dot` have no `Result`-returning form and panic on a shape
/// mismatch; there is no `MattenError` this crate can obtain for that case.
/// The `wasm32-unknown-unknown` target cannot recover a panic's message
/// either — verified directly: a panic inside `std::panic::catch_unwind`,
/// compiled for this target, reaches the JS caller as a bare
/// `RuntimeError: unreachable` trap with no payload, not a caught `Err`.
/// Reproducing `matten`'s own panic text here, and never calling `matmul`
/// on shapes this check has not already approved, is the closest available
/// approximation to "the real message" that does not risk trapping the page.
fn matmul_result_shape(left: &[usize], right: &[usize]) -> Result<Vec<usize>, String> {
    const OP: &str = "dot"; // matmul() delegates to dot(), which hardcodes op = "dot".

    fn dim_mismatch(left_name: &str, left: usize, right_name: &str, right: usize) -> String {
        format!(
            "matten shape error in {OP}: {left_name} ({left}) must equal {right_name} ({right})"
        )
    }

    match (left.len(), right.len()) {
        (1, 1) => {
            if left[0] != right[0] {
                return Err(format!(
                    "matten shape error in {OP}: vector lengths must match (left {}, right {})",
                    left[0], right[0]
                ));
            }
            Ok(vec![])
        }
        (2, 1) => {
            let [m, n] = [left[0], left[1]];
            if n != right[0] {
                return Err(dim_mismatch("left columns", n, "right length", right[0]));
            }
            Ok(vec![m])
        }
        (1, 2) => {
            let [n, p] = [right[0], right[1]];
            if left[0] != n {
                return Err(dim_mismatch("left length", left[0], "right rows", n));
            }
            Ok(vec![p])
        }
        (2, 2) => {
            let [m, n] = [left[0], left[1]];
            let [nb, p] = [right[0], right[1]];
            if n != nb {
                return Err(dim_mismatch("left columns", n, "right rows", nb));
            }
            Ok(vec![m, p])
        }
        (lr, rr) => Err(format!(
            "matten shape error in {OP}: unsupported rank combination (left rank {lr}, right rank {rr}); \
             supported: [n]\u{d7}[n], [m,n]\u{d7}[n], [n]\u{d7}[n,p], [m,n]\u{d7}[n,p]"
        )),
    }
}

/// Two shapes and values -> the matrix product, or the real matmul rejection
/// text (RFC-093 §5).
#[wasm_bindgen]
pub fn playground_matmul(
    left_shape: &str,
    left_values: &str,
    right_shape: &str,
    right_values: &str,
) -> String {
    let a = match build_tensor(left_shape, left_values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };
    let b = match build_tensor(right_shape, right_values) {
        Ok(t) => t,
        Err(e) => return error_block(&e),
    };

    let result_shape = match matmul_result_shape(a.shape(), b.shape()) {
        Ok(s) => s,
        Err(e) => return error_block(&e),
    };

    // Guaranteed not to panic: the same rank/dimension rules were just checked.
    let product = a.matmul(&b);
    debug_assert_eq!(product.shape(), result_shape.as_slice());

    format!(
        "{}\n{}\n{}\nmeaning          {:?} x {:?} -> {:?}",
        format_tensor_block("left", &a),
        format_tensor_block("right", &b),
        format_tensor_block("left.matmul", &product),
        a.shape(),
        b.shape(),
        product.shape()
    )
}

// ---- shared error formatting ------------------------------------------------

fn format_matten_error(e: &MattenError) -> String {
    e.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ---- live fidelity to the real panic text (review C1) -------------------
    //
    // The four `*_matches_the_real_panic_text_exactly` tests below assert
    // against a string this crate hand-transcribed from a one-time
    // `catch_unwind` run (see the review request). A hand-transcription does
    // not notice if core `matten` ever rewords the panic it mirrors — the
    // test keeps passing while the page quietly starts lying about what
    // matten says. These tests close that gap: they trigger the REAL panic,
    // live, every run, and assert the playground's output against whatever
    // core actually produced this time, not a frozen guess. Native-only:
    // `catch_unwind` does not recover a panic's message on
    // `wasm32-unknown-unknown` (verified separately, see the review
    // request) — this target is where the crate actually ships, so these
    // tests exist to protect that build without needing to run on it.

    #[cfg(not(target_arch = "wasm32"))]
    fn real_panic_message(f: impl FnOnce() + std::panic::UnwindSafe) -> String {
        let prev_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {})); // silence the default panic printout
        let result = std::panic::catch_unwind(f);
        std::panic::set_hook(prev_hook);
        match result {
            Err(payload) => payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| s.to_string()))
                .expect("panic payload was not a string"),
            Ok(()) => panic!("expected the operation to panic, but it did not"),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn broadcast_mismatch_matches_the_live_real_panic_payload() {
        let a = Tensor::new(vec![1.0; 6], &[2, 3]);
        let b = Tensor::new(vec![1.0; 4], &[4]);
        let real = real_panic_message(|| {
            let _ = &a + &b;
        });
        let out = playground_broadcast("2,3", "1,1,1,1,1,1", "4", "1,1,1,1");
        assert_eq!(out, format!("Error: {real}"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn matmul_dim_mismatch_matches_the_live_real_panic_payload() {
        let l = Tensor::new(vec![1.0; 6], &[2, 3]);
        let r = Tensor::new(vec![1.0; 4], &[2, 2]);
        let real = real_panic_message(|| {
            let _ = l.matmul(&r);
        });
        let out = playground_matmul("2,3", "1,1,1,1,1,1", "2,2", "1,1,1,1");
        assert_eq!(out, format!("Error: {real}"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn matmul_vector_length_mismatch_matches_the_live_real_panic_payload() {
        let v1 = Tensor::new(vec![1.0, 2.0, 3.0], &[3]);
        let v2 = Tensor::new(vec![1.0, 2.0], &[2]);
        let real = real_panic_message(|| {
            let _ = v1.matmul(&v2);
        });
        let out = playground_matmul("3", "1,2,3", "2", "1,2");
        assert_eq!(out, format!("Error: {real}"));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn matmul_rank_mismatch_matches_the_live_real_panic_payload() {
        let l3 = Tensor::new((1..=24).map(|x| x as f64).collect(), &[2, 3, 4]);
        let r2 = Tensor::new(vec![1.0; 4], &[2, 2]);
        let real = real_panic_message(|| {
            let _ = l3.matmul(&r2);
        });
        let out = playground_matmul(
            "2,3,4",
            "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24",
            "2,2",
            "1,1,1,1",
        );
        assert_eq!(out, format!("Error: {real}"));
    }

    // ---- broadcasting -------------------------------------------------------

    #[test]
    fn broadcast_compatible_shapes_computes_the_real_sum() {
        let out = playground_broadcast("2,3", "1,2,3,4,5,6", "3", "10,20,30");
        assert_eq!(
            out,
            "input A          shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             input b          shape=[3]\n\
             10.000 20.000 30.000\n\
             A + b            shape=[2, 3]\n\
             11.000 22.000 33.000\n\
             14.000 25.000 36.000\n\
             meaning          right repeats along axis 0"
        );
    }

    #[test]
    fn broadcast_identical_shapes_reports_no_broadcasting() {
        let out = playground_broadcast("2,2", "1,2,3,4", "2,2", "1,1,1,1");
        assert_eq!(
            out,
            "input A          shape=[2, 2]\n\
             1.000 2.000\n\
             3.000 4.000\n\
             input b          shape=[2, 2]\n\
             1.000 1.000\n\
             1.000 1.000\n\
             A + b            shape=[2, 2]\n\
             2.000 3.000\n\
             4.000 5.000\n\
             meaning          shapes already match; no broadcasting"
        );
    }

    #[test]
    fn broadcast_incompatible_shapes_matches_the_real_panic_text_exactly() {
        // Verified by triggering the actual `&a + &b` panic natively and
        // diffing byte for byte (see the review request for the transcript);
        // `apply_binary`'s panic format is reproduced here, NOT
        // `MattenError::Broadcast`'s `Display` text, which is worded
        // differently and unreachable through the public `+` operator.
        let out = playground_broadcast("2,3", "1,2,3,4,5,6", "4", "10,20,30,40");
        assert_eq!(
            out,
            "Error: matten broadcast error in add: shapes [2, 3] and [4] are not compatible"
        );
    }

    // ---- reshape --------------------------------------------------------------

    #[test]
    fn reshape_preserves_row_major_values() {
        let out = playground_reshape("2,3", "1,2,3,4,5,6", "3,2");
        assert_eq!(
            out,
            "input            shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             reshaped         shape=[3, 2]\n\
             1.000 2.000\n\
             3.000 4.000\n\
             5.000 6.000\n\
             meaning          row-major values stay in the same order"
        );
    }

    #[test]
    fn reshape_mismatch_shows_the_real_try_reshape_error() {
        let out = playground_reshape("2,3", "1,2,3,4,5,6", "4,2");
        assert_eq!(
            out,
            "Error: matten shape error in reshape: cannot reshape tensor with 6 elements into shape [4, 2] requiring 8 elements"
        );
    }

    // ---- axis reductions --------------------------------------------------------

    #[test]
    fn axis_reduce_mean_axis_0_matches_hand_computed_values() {
        let out = playground_axis_reduce("2,3", "1,2,3,4,5,6", "0", "mean");
        assert_eq!(
            out,
            "input            shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             mean_axis(0)    collapse rows, keep columns\n\
             result           shape=[3]\n\
             2.500 3.500 4.500"
        );
    }

    #[test]
    fn axis_reduce_axis_1_uses_the_columns_gloss() {
        let out = playground_axis_reduce("2,3", "1,2,3,4,5,6", "1", "sum");
        assert_eq!(
            out,
            "input            shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             sum_axis(1)    collapse columns, keep rows\n\
             result           shape=[2]\n \
             6.000 15.000"
        );
    }

    #[test]
    fn axis_reduce_rank3_uses_the_general_gloss() {
        let out = playground_axis_reduce("2,2,2", "1,2,3,4,5,6,7,8", "0", "sum");
        assert!(out.contains("collapse axis 0, keep the rest"));
    }

    #[test]
    fn axis_reduce_all_four_ops_are_wired() {
        for op in ["sum", "mean", "min", "max"] {
            let out = playground_axis_reduce("2,3", "1,2,3,4,5,6", "0", op);
            assert!(
                !out.starts_with("Error"),
                "{op} unexpectedly errored: {out}"
            );
        }
    }

    #[test]
    fn axis_reduce_out_of_range_matches_the_real_try_sum_axis_error() {
        let out = playground_axis_reduce("2,3", "1,2,3,4,5,6", "5", "sum");
        assert_eq!(
            out,
            "Error: matten shape error in sum_axis: axis 5 is out of range for a rank-2 tensor"
        );
    }

    #[test]
    fn axis_reduce_unknown_op_is_a_playground_level_message() {
        let out = playground_axis_reduce("2,3", "1,2,3,4,5,6", "0", "median");
        assert_eq!(
            out,
            "Error: unknown reduction \"median\"; choose one of sum, mean, min, max"
        );
    }

    // ---- matmul -----------------------------------------------------------------

    #[test]
    fn matmul_2x2_matches_hand_computed_product() {
        let out = playground_matmul("2,2", "1,2,3,4", "2,2", "5,6,7,8");
        assert_eq!(
            out,
            "left             shape=[2, 2]\n\
             1.000 2.000\n\
             3.000 4.000\n\
             right            shape=[2, 2]\n\
             5.000 6.000\n\
             7.000 8.000\n\
             left.matmul      shape=[2, 2]\n\
             19.000 22.000\n\
             43.000 50.000\n\
             meaning          [2, 2] x [2, 2] -> [2, 2]"
        );
    }

    #[test]
    fn matmul_mv_and_vm_and_vv_all_compute() {
        // [m,n] x [n] -> [m]
        let mv = playground_matmul("2,3", "1,2,3,4,5,6", "3", "1,2,3");
        assert_eq!(
            mv,
            "left             shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             right            shape=[3]\n\
             1.000 2.000 3.000\n\
             left.matmul      shape=[2]\n\
             14.000 32.000\n\
             meaning          [2, 3] x [3] -> [2]"
        );
        // [n] x [n,p] -> [p]
        let vm = playground_matmul("3", "1,2,3", "3,2", "1,2,3,4,5,6");
        assert_eq!(
            vm,
            "left             shape=[3]\n\
             1.000 2.000 3.000\n\
             right            shape=[3, 2]\n\
             1.000 2.000\n\
             3.000 4.000\n\
             5.000 6.000\n\
             left.matmul      shape=[2]\n\
             22.000 28.000\n\
             meaning          [3] x [3, 2] -> [2]"
        );
        // [n] x [n] -> [] (scalar)
        let vv = playground_matmul("3", "1,2,3", "3", "1,2,3");
        assert_eq!(
            vv,
            "left             shape=[3]\n\
             1.000 2.000 3.000\n\
             right            shape=[3]\n\
             1.000 2.000 3.000\n\
             left.matmul      shape=[]\n\
             14.000\n\
             meaning          [3] x [3] -> []"
        );
    }

    #[test]
    fn matmul_dimension_mismatch_matches_the_real_panic_text_exactly() {
        let out = playground_matmul("2,3", "1,2,3,4,5,6", "2,2", "1,2,3,4");
        assert_eq!(
            out,
            "Error: matten shape error in dot: left columns (3) must equal right rows (2)"
        );
    }

    #[test]
    fn matmul_vector_length_mismatch_matches_the_real_panic_text_exactly() {
        let out = playground_matmul("3", "1,2,3", "2", "1,2");
        assert_eq!(
            out,
            "Error: matten shape error in dot: vector lengths must match (left 3, right 2)"
        );
    }

    #[test]
    fn matmul_unsupported_rank_combination_matches_the_real_panic_text_exactly() {
        let out = playground_matmul(
            "2,3,4",
            "1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24",
            "2,2",
            "1,2,3,4",
        );
        assert_eq!(
            out,
            "Error: matten shape error in dot: unsupported rank combination (left rank 3, right rank 2); \
             supported: [n]\u{d7}[n], [m,n]\u{d7}[n], [n]\u{d7}[n,p], [m,n]\u{d7}[n,p]"
        );
    }

    // ---- input parsing --------------------------------------------------------

    #[test]
    fn non_numeric_value_is_a_playground_level_parse_error() {
        let out = playground_reshape("2,3", "1,2,x,4,5,6", "3,2");
        assert_eq!(out, "Error: \"x\" is not a number");
    }

    #[test]
    fn shape_value_count_mismatch_matches_the_real_try_new_error() {
        let out = playground_reshape("2,3", "1,2,3,4,5", "3,2");
        assert_eq!(
            out,
            "Error: matten shape error in try_new: data length 5 does not match shape [2, 3], which requires 6 elements"
        );
    }
}
