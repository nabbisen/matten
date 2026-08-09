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

use matten::{Element, MattenError, Tensor};
use wasm_bindgen::prelude::*;

mod render;
use render::format_tensor_block;

// ---- input parsing --------------------------------------------------------

/// Splits `s` on commas **and newlines** (RFC-115) — a grid pasted as rows,
/// one line each, is how a learner naturally types one — trims each token,
/// and drops **only a trailing run** of empty tokens, so `"1,2,3,"` and a
/// trailing blank line both stay forgiving.
///
/// An empty token that is *not* trailing is an interior blank. It is not
/// dropped: dropping it would silently shift every value after it into the
/// wrong position and change how many values are read, with no sign anything
/// was wrong (RFC-115 §2). It is reported instead, naming its 1-based
/// position in the flat, row-major sequence the field is read as.
fn split_forgiving_trailing(s: &str) -> Result<Vec<&str>, String> {
    let tokens: Vec<&str> = s.split([',', '\n']).map(str::trim).collect();

    // No non-empty token anywhere: an all-blank (or empty) field is zero
    // values, unchanged from before this RFC — not an interior-blank error.
    let Some(last_significant) = tokens.iter().rposition(|t| !t.is_empty()) else {
        return Ok(Vec::new());
    };

    let significant = &tokens[..=last_significant];
    if let Some(pos) = significant.iter().position(|t| t.is_empty()) {
        return Err(format!(
            "value {} is blank — an interior blank is not dropped silently, since that \
             would shift every value after it and change how many values are read; a \
             trailing separator (e.g. \"1,2,3,\") is fine, but a gap in the middle is not",
            pos + 1
        ));
    }

    Ok(significant.to_vec())
}

/// Parses a comma/newline-separated list of non-negative integers, e.g.
/// `"2, 3"` or `"2\n3"`.
fn parse_shape(s: &str) -> Result<Vec<usize>, String> {
    split_forgiving_trailing(s)?
        .into_iter()
        .map(|p| {
            p.parse::<usize>()
                .map_err(|_| format!("\"{p}\" is not a non-negative integer"))
        })
        .collect()
}

/// Parses a comma/newline-separated list of `f64` values, e.g. `"1, 2.5, -3"`
/// or a grid pasted as rows, `"1, 2, 3\n4, 5, 6"`.
fn parse_values(s: &str) -> Result<Vec<f64>, String> {
    split_forgiving_trailing(s)?
        .into_iter()
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

/// Two shapes and values -> the matrix product, or the real
/// [`MattenError`] text from [`Tensor::try_matmul`] (RFC-093 §5, RFC-113).
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

    match a.try_matmul(&b) {
        Ok(product) => format!(
            "{}\n{}\n{}\nmeaning          {:?} x {:?} -> {:?}",
            format_tensor_block("left", &a),
            format_tensor_block("right", &b),
            format_tensor_block("left.matmul", &product),
            a.shape(),
            b.shape(),
            product.shape()
        ),
        Err(e) => error_block(&format_matten_error(&e)),
    }
}

// ---- dynamic: try_numeric (RFC-115 Part B) -----------------------------------

/// Infers the most specific [`Element`] for one cell's text, mirroring core's
/// own CSV inference policy exactly (`crates/matten/src/dynamic/parse/csv.rs`,
/// RFC-011 §8) so the playground never invents a rule core doesn't already
/// use elsewhere: empty -> `None`, `"true"`/`"false"` (case-insensitive) ->
/// `Bool`, else `i64` -> `Int`, else `f64` -> `Float`, else `Text`.
fn infer_element(field: &str) -> Element {
    if field.is_empty() {
        return Element::None;
    }
    if field.eq_ignore_ascii_case("true") {
        return Element::Bool(true);
    }
    if field.eq_ignore_ascii_case("false") {
        return Element::Bool(false);
    }
    if let Ok(i) = field.parse::<i64>() {
        return Element::Int(i);
    }
    if let Ok(f) = field.parse::<f64>() {
        return Element::Float(f);
    }
    Element::text(field)
}

/// Splits `values` on commas and newlines like [`parse_shape`]/[`parse_values`],
/// but — deliberately unlike them — every token here is significant,
/// including a blank one: it becomes [`Element::None`] rather than being
/// reported as an error or forgiven as trailing. This is Part A's blank-cell
/// fix composing with a dynamic tensor's own representation of "missing"
/// (RFC-115 §3): on this form only, a blank is data, not a mistake. A field
/// that is blank in its entirety is zero cells, matching the numeric forms'
/// existing convention for a zero-sized shape.
fn parse_elements(values: &str) -> Vec<Element> {
    if values.trim().is_empty() {
        return Vec::new();
    }
    values
        .split([',', '\n'])
        .map(str::trim)
        .map(infer_element)
        .collect()
}

/// Shape and values (which may contain text or be blank) -> the dynamic
/// tensor exactly as entered, followed by [`Tensor::try_numeric`]'s outcome:
/// either the converted numeric tensor, or the real error naming the first
/// offending cell and why (RFC-115 Part B).
///
/// `values`' blank-cell handling deliberately differs from the four numeric
/// forms above: see [`parse_elements`].
#[wasm_bindgen]
pub fn playground_try_numeric(shape: &str, values: &str) -> String {
    let shape = match parse_shape(shape) {
        Ok(s) => s,
        Err(e) => return error_block(&e),
    };
    let elements = parse_elements(values);

    let t = match Tensor::try_from_elements(elements, &shape) {
        Ok(t) => t,
        Err(e) => return error_block(&format_matten_error(&e)),
    };

    let dynamic_block = format!("{:<16} shape={:?}\n{t}", "input (dynamic)", t.shape());

    match t.try_numeric() {
        Ok(numeric) => format!(
            "{dynamic_block}\n{}\nmeaning          every cell converted; try_numeric() succeeded",
            format_tensor_block("numeric", &numeric)
        ),
        Err(e) => format!("{dynamic_block}\nError: {}", format_matten_error(&e)),
    }
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
    // RFC-113 deleted matmul's copy of this pattern along with
    // matmul_result_shape: playground_matmul calls try_matmul now, so there is
    // no panic text left to keep honest for matmul, and its three sync tests
    // (which called `real_panic_message` around a live `.matmul()` call) are
    // gone with it.
    //
    // The one remaining case is broadcast's `+`, which has no `try_add` to
    // call instead (RFC-113 §2 — unavoidable, not this RFC's to fix). Its
    // hand-transcribed `broadcast_incompatible_shapes_matches_the_real_panic_
    // text_exactly` test, below, asserts against a string this crate
    // hand-transcribed from a one-time `catch_unwind` run (see the review
    // request). A hand-transcription does not notice if core `matten` ever
    // rewords the panic it mirrors — the test keeps passing while the page
    // quietly starts lying about what matten says.
    // `broadcast_mismatch_matches_the_live_real_panic_payload` closes that
    // gap: it triggers the REAL panic, live, every run, and asserts the
    // playground's output against whatever core actually produced this time,
    // not a frozen guess. Native-only: `catch_unwind` does not recover a
    // panic's message on `wasm32-unknown-unknown` (verified separately, see
    // the review request) — this target is where the crate actually ships,
    // so this test exists to protect that build without needing to run on
    // it.

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

    #[test]
    fn broadcast_zero_sized_input_is_incompatible_not_a_panic() {
        // RFC-113 Change 2: [0,3] and [2,3] are typeable into the form since
        // RFC-111 made zero-sized shapes constructible. Axis 0 sizes 0 vs 2
        // are genuinely incompatible -- this is a real Shape error, not a
        // consequence of the zero-sized dimension itself.
        let out = playground_broadcast("0,3", "", "2,3", "1,2,3,4,5,6");
        assert_eq!(
            out,
            "Error: matten broadcast error in add: shapes [0, 3] and [2, 3] are not compatible"
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

    #[test]
    fn reshape_to_a_zero_sized_target_is_an_element_count_mismatch() {
        // RFC-113 Change 2: [0,6] is typeable since RFC-111, but a 6-element
        // source cannot reshape into a 0-element target -- the same
        // element-count check that already governs every other reshape.
        let out = playground_reshape("2,3", "1,2,3,4,5,6", "0,6");
        assert_eq!(
            out,
            "Error: matten shape error in reshape: cannot reshape tensor with 6 elements into shape [0, 6] requiring 0 elements"
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

    #[test]
    fn axis_reduce_zero_length_reduced_axis_shows_rfc110s_message() {
        // RFC-113 Change 2: [0,3] is typeable since RFC-111. Reducing axis 0
        // (length 0) is RFC-110's territory -- InvalidArgument, not a panic
        // and not a NaN/inf leak.
        let out = playground_axis_reduce("0,3", "", "0", "mean");
        assert_eq!(
            out,
            "Error: matten invalid argument error in mean_axis: axis: mean is undefined for a reduced axis of length 0 (axis 0)"
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
    fn matmul_dimension_mismatch_shows_the_real_try_matmul_error() {
        // RFC-113: was matmul_dimension_mismatch_matches_the_real_panic_text_exactly,
        // asserting a hand-rolled mirror of matmul()'s panic text. Now a real
        // try_matmul Err; text confirmed unchanged (SS3 of the review request).
        let out = playground_matmul("2,3", "1,2,3,4,5,6", "2,2", "1,2,3,4");
        assert_eq!(
            out,
            "Error: matten shape error in dot: left columns (3) must equal right rows (2)"
        );
    }

    #[test]
    fn matmul_vector_length_mismatch_shows_the_real_try_matmul_error() {
        // RFC-113: was matmul_vector_length_mismatch_matches_the_real_panic_text_exactly.
        let out = playground_matmul("3", "1,2,3", "2", "1,2");
        assert_eq!(
            out,
            "Error: matten shape error in dot: vector lengths must match (left 3, right 2)"
        );
    }

    #[test]
    fn matmul_unsupported_rank_combination_shows_the_real_try_matmul_error() {
        // RFC-113: was matmul_unsupported_rank_combination_matches_the_real_panic_text_exactly.
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

    #[test]
    fn matmul_zero_length_contraction_dimension_computes_the_zero_matrix() {
        // RFC-113 Change 2: [3,0] x [0,2] -- n = 0. Sum over zero terms is
        // correctly all-zero, not an error (matches matten's own behaviour;
        // this case never depended on RFC-108). A zero-row or zero-column
        // grid renders as blank lines (render_matrix's join of empty rows),
        // not an empty string -- the shape header is still shown, so no
        // information is lost; captured from the real output, not
        // hand-computed.
        let out = playground_matmul("3,0", "", "0,2", "");
        assert_eq!(
            out,
            "left             shape=[3, 0]\n\n\n\n\
             right            shape=[0, 2]\n\n\
             left.matmul      shape=[3, 2]\n\
             0.000 0.000\n\
             0.000 0.000\n\
             0.000 0.000\n\
             meaning          [3, 0] x [0, 2] -> [3, 2]"
        );
    }

    #[test]
    fn matmul_zero_output_columns_does_not_panic() {
        // RFC-113 Change 2: [2,3] x [3,0] -- p = 0, the exact shape RFC-108
        // fixed a live panic for. Confirms the playground inherits that fix
        // through try_matmul rather than re-triggering it through the now-
        // deleted hand-rolled guard. Blank lines are the empty-grid rendering
        // (see the sibling test above), captured from the real output.
        let out = playground_matmul("2,3", "1,2,3,4,5,6", "3,0", "");
        assert_eq!(
            out,
            "left             shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             right            shape=[3, 0]\n\n\n\n\
             left.matmul      shape=[2, 0]\n\n\n\
             meaning          [2, 3] x [3, 0] -> [2, 0]"
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

    // ---- RFC-115 Part A: newlines, interior blanks, trailing forgiveness ------

    #[test]
    fn t1_a_grid_pasted_across_newlines_parses_for_values_and_shape() {
        // A 2x3 grid typed the way it looks -- one row per line -- must work,
        // for both the values field and the shape field.
        let out = playground_reshape("2,3", "1, 2, 3\n4, 5, 6", "3,2");
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

        let out = playground_reshape("2,3", "1,2,3,4,5,6", "3\n2");
        assert!(
            !out.starts_with("Error"),
            "newline-separated shape unexpectedly errored: {out}"
        );
    }

    #[test]
    fn t2_an_interior_blank_is_reported_naming_its_position() {
        // Exact input/output quoted per the handoff's required evidence.
        let out = playground_reshape("2,3", "1,2,,4,5,6", "3,2");
        assert_eq!(
            out,
            "Error: value 3 is blank — an interior blank is not dropped silently, since that \
             would shift every value after it and change how many values are read; a \
             trailing separator (e.g. \"1,2,3,\") is fine, but a gap in the middle is not"
        );

        // The same bug, the same fix, in the shape field (handoff R5: fix both).
        let out = playground_reshape("2,,3", "1,2,3,4,5,6", "3,2");
        assert_eq!(
            out,
            "Error: value 2 is blank — an interior blank is not dropped silently, since that \
             would shift every value after it and change how many values are read; a \
             trailing separator (e.g. \"1,2,3,\") is fine, but a gap in the middle is not"
        );
    }

    #[test]
    fn t3_a_trailing_separator_still_works() {
        // Exact inputs from the handoff's own examples.
        let out = playground_axis_reduce("2,3,", "1,2,3,4,5,6", "0", "sum");
        assert!(
            !out.starts_with("Error"),
            "trailing comma in shape unexpectedly errored: {out}"
        );
        let out = playground_reshape("2,3", "1,2,3,4,5,6,", "3,2");
        assert!(
            !out.starts_with("Error"),
            "trailing comma in values unexpectedly errored: {out}"
        );
        // A trailing blank LINE, the newline-separator analogue of a trailing comma.
        let out = playground_reshape("2,3", "1,2,3,4,5,6\n", "3,2");
        assert!(
            !out.starts_with("Error"),
            "trailing newline unexpectedly errored: {out}"
        );
    }

    #[test]
    fn t4_every_pre_rfc115_test_in_this_file_is_unmodified() {
        // Not a new assertion of its own: the 33 tests preceding this section
        // in the file are the pre-RFC-115 suite, verbatim -- byte-identical
        // pass/fail against the new parser is the evidence for "no computed
        // result changes for any currently-valid input" (RFC-115 R1/T4). This
        // test exists only to name that fact at the point a reviewer would
        // look for it; the real evidence is that nothing above this line was
        // touched.
    }

    // ---- RFC-115 Part B: try_numeric demo --------------------------------------

    #[test]
    fn t5_mixed_input_shows_elements_then_numeric_result() {
        let out = playground_try_numeric("2,3", "1, 2, 3, 4, 5, 6");
        assert_eq!(
            out,
            "input (dynamic)  shape=[2, 3]\n\
             1 2 3\n\
             4 5 6\n\
             numeric          shape=[2, 3]\n\
             1.000 2.000 3.000\n\
             4.000 5.000 6.000\n\
             meaning          every cell converted; try_numeric() succeeded"
        );
    }

    #[test]
    fn t5_a_text_cell_produces_the_real_error_naming_that_cell() {
        let out = playground_try_numeric("2,3", "1, 2, x, 4, 5, 6");
        assert_eq!(
            out,
            "input (dynamic)  shape=[2, 3]\n\
             1 2 x\n\
             4 5 6\n\
             Error: matten unsupported error in try_numeric: element at position 2 is Text(\"x\") and cannot be coerced to f64; use fill_none or explicit conversion first"
        );
    }

    #[test]
    fn t5_a_blank_cell_is_shown_as_none_and_reported_by_try_numeric() {
        // Part A's numeric forms reject an interior blank outright (T2); this
        // form accepts it as Element::None and shows it, then try_numeric()
        // names it as the reason conversion failed -- both correct, and
        // different, per the handoff's explicit instruction to say so.
        let out = playground_try_numeric("2,3", "1, 2, , 4, 5, 6");
        assert_eq!(
            out,
            "input (dynamic)  shape=[2, 3]\n\
             1 2 None\n\
             4 5    6\n\
             Error: matten unsupported error in try_numeric: element at position 2 is None and cannot be coerced to f64; use fill_none or explicit conversion first"
        );
    }

    #[test]
    fn t5_bool_and_int_and_float_all_infer_correctly() {
        let out = playground_try_numeric("1,4", "1, 1.5, true, false");
        assert!(
            out.contains("1 1.5 true false"),
            "expected inferred Int/Float/Bool cells, got: {out}"
        );
    }

    #[test]
    fn t5_grid_pasted_across_newlines_works_here_too() {
        let out = playground_try_numeric("2,3", "1, 2, 3\n4, 5, 6");
        assert!(
            !out.starts_with("Error"),
            "newline-separated dynamic input unexpectedly errored: {out}"
        );
    }

    #[test]
    fn t6_try_numeric_demo_calls_no_panicking_core_form() {
        // T6: try_from_elements always sets `dynamic: Some(..)`, so
        // try_numeric() (which panics only on a non-dynamic tensor) can never
        // reach its panic branch here. Asserted by using both outcomes above
        // without a catch_unwind guard -- if it could panic, T5's tests would
        // already trap in `cargo test`, let alone under wasm.
        let empty_shape_ok = playground_try_numeric("0,3", "");
        assert!(
            !empty_shape_ok.starts_with("Error"),
            "zero-sized dynamic input unexpectedly errored: {empty_shape_ok}"
        );
    }
}
