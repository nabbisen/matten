# RFC-031: Feature-Robust Dynamic Rejection and Unconditional `Tensor::is_dynamic()`

**Status:** Implemented (v0.19.1)
**Replaces / amends:** Supersedes the feature-gated rejection guard pattern adopted informally
in RFC-012 and RFC-022 §8; does not reopen those RFCs.
**Shipped in:** v0.19.1

---

## 1. Summary

Add `Tensor::is_dynamic()` as a public, unconditionally available method on the core
`Tensor` type, and require every companion crate to call it without a
`#[cfg(feature = "dynamic")]` gate when guarding numeric conversion paths.

This closes a panic window that opens under Cargo feature unification when a downstream
user enables `matten/dynamic` but leaves a companion crate's own mirror `dynamic` feature
off. In that configuration the rejection guard was compiled out and a dynamic `Tensor`
could reach `Tensor::to_vec()` or `Tensor::as_slice()`, triggering an internal panic
instead of returning a companion `Err`.

---

## 2. Motivation

### 2.1 The bug

`matten-ndarray` and `matten-mlprep` both gate their dynamic-rejection guards:

```rust
// matten-ndarray/src/convert.rs  (v0.19.0)
#[cfg(feature = "dynamic")]
if tensor.is_dynamic() {
    return Err(MattenNdarrayError::DynamicTensor);
}
```

`is_dynamic()` itself is defined only inside the `#[cfg(feature = "dynamic")]` module
of core `matten`. If a downstream `Cargo.toml` is written as:

```toml
[dependencies]
matten        = { version = "0.19", features = ["dynamic"] }
matten-ndarray = "0.19"   # companion dynamic feature not explicitly enabled
```

Cargo resolves a single `matten` compiled **with** `dynamic`, but `matten-ndarray`'s
own feature (`dynamic`) is off, so the `#[cfg(feature = "dynamic")]` guard is not
compiled. A dynamic `Tensor` passed to `to_arrayd()` reaches `to_vec()` and panics
internally.

The same applies to `matten-mlprep` via `matrix_dims()` → `as_slice()`.

### 2.2 Why a new RFC

The fix crosses crate boundaries (core public API + both companions) and changes a
documented guarantee from feature-dependent to feature-robust. It also adds a method
to core's public surface. Per RFC-000 and the design-before-coding rule, this warrants
a dedicated paper trail rather than a silent patch.

---

## 3. Decision

### 3.1 Core: unconditional `Tensor::is_dynamic()`

Add the following method to `Tensor` in the unconditional (`#[cfg]`-free) block of
`crates/matten/src/tensor.rs`:

```rust
/// Returns `true` if this tensor uses dynamic ([`Element`]) storage.
///
/// This method is available in all builds, regardless of whether the `dynamic`
/// feature is enabled:
///
/// - When `dynamic` is **off**, no dynamic tensor can exist, so this always
///   returns `false`.
/// - When `dynamic` is **on**, it returns the true storage state.
///
/// Companion crates should call this unconditionally before numeric conversion
/// so that a dynamic `Tensor` is rejected with a typed `Err` rather than
/// reaching internal numeric accessors and panicking — even when Cargo feature
/// unification enables core `dynamic` while the companion's own mirror feature
/// is off.
#[must_use]
pub fn is_dynamic(&self) -> bool {
    #[cfg(feature = "dynamic")]
    {
        self.dynamic.is_some()
    }
    #[cfg(not(feature = "dynamic"))]
    {
        false
    }
}
```

The existing `is_dynamic()` definition inside the `#[cfg(feature = "dynamic")]`-gated
`tensor_ext.rs` is removed; `tensor.rs` is the sole definition.

**Constraints:**
- must not allocate;
- must not inspect element values;
- must not expose storage internals beyond the boolean.

### 3.2 Companion crates: unconditional rejection guards

Every companion function that converts a `Tensor` to a numeric form must call
`is_dynamic()` without a `#[cfg]` gate:

```rust
// Good (v0.19.1 onwards)
if tensor.is_dynamic() {
    return Err(CompanionError::DynamicTensor);
}

// Removed pattern
#[cfg(feature = "dynamic")]
if tensor.is_dynamic() { ... }
```

Affected sites:
- `crates/matten-ndarray/src/convert.rs` — `to_arrayd`, `from_arrayd`
- `crates/matten-mlprep/src/util.rs` — `matrix_dims`

### 3.3 Companion `dynamic` mirror features: retained

The companion crates retain their `dynamic` feature flags. They are re-documented as
**compatibility forwarding features**: they forward to `matten/dynamic` but are no
longer required for dynamic-rejection correctness.

Recommended rustdoc / feature wording:

```text
`dynamic` — Compatibility forwarding feature. No longer required for dynamic
rejection as of v0.19.1. Dynamic tensors are rejected at companion boundaries
regardless of whether this feature is enabled. Reconsider removal no earlier
than v0.20.0.
```

No Rust `#[deprecated]` attribute is used on Cargo features (Cargo has no equivalent).

---

## 4. Public API surface

```rust
impl Tensor {
    pub fn is_dynamic(&self) -> bool;
}
```

This method is additive and non-breaking. It does not alter any existing method
signature, error type, or feature flag name.

---

## 5. Compatibility

| Dimension | Impact |
|---|---|
| SemVer | Additive — no breaking change |
| Release label | `0.19.1` (patch) under RFC-030 lock-step |
| Feature flags | Companion `dynamic` feature names unchanged |
| MSRV | Unchanged (1.85) |
| Downstream Cargo manifests | No changes required |

---

## 6. Required tests

### 6.1 Unit tests in core

A test in `crates/matten/src/tests/tensor.rs` (or the `dynamic/` sub-suite) must
cover both cfg branches:

- `is_dynamic()` returns `false` for a numeric tensor (always).
- `is_dynamic()` returns `true` for a tensor constructed via `Tensor::from_elements()`
  (requires `dynamic` feature).

### 6.2 Regression fixture

A fixture crate must be created at `tests/fixtures/dynamic_rejection_unification/`
(excluded from the workspace via `[workspace] exclude`) that simulates the
feature-unification scenario:

```toml
[dependencies]
matten        = { path = "../../crates/matten", features = ["dynamic"], default-features = false }
matten-ndarray = { path = "../../crates/matten-ndarray", default-features = true }
matten-mlprep  = { path = "../../crates/matten-mlprep",  default-features = true }
```

Expected behavior (validated via `cargo run` in CI or locally):

```text
dynamic Tensor -> matten_ndarray::to_arrayd(...)
    -> Err(MattenNdarrayError::DynamicTensor)   ✓ no panic

dynamic Tensor -> matten_mlprep::standardize_columns(...)
    -> Err(MattenMlprepError::DynamicTensor)    ✓ no panic
```

The fixture must call `std::panic::catch_unwind` and assert that no panic occurs,
so a regression is immediately visible without needing to parse output.

---

## 7. Non-goals

- No dynamic arithmetic.
- No dynamic-to-ndarray value conversion.
- No dynamic mlprep computation.
- No removal of companion `dynamic` feature flags in v0.19.1.
- No change to `MattenError`, companion error types, or any other public type.

---

## 8. Document history

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-06-21 | Initial proposal; all four decisions from the v0.19.1 decision-request review incorporated. |
