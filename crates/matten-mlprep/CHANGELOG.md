# Changelog — matten-mlprep

All notable changes to `matten-mlprep` are documented here.
This crate uses independent SemVer (RFC-022 §7).

## [0.1.0] - 2026-06-21

**Experimental first release.** Transparent, deterministic preprocessing helpers
for `matten::Tensor` (RFC-024, RFC-028).

### Added

- `standardize_columns(&Tensor)` — per-column z-score using population std.
- `minmax_scale_columns(&Tensor)` — per-column scaling to `[0, 1]`.
- `add_bias_column(&Tensor)` — prepend a constant `1.0` intercept column
  (`[n, m] -> [n, m+1]`).
- `train_test_split(&Tensor, train_ratio)` — ordered, deterministic row split
  (no shuffle).
- `MattenMlprepError` (`#[non_exhaustive]`): `DynamicTensor`, `ExpectedMatrix`,
  `InvalidRatio`, `EmptySplit`, `ZeroVariance`, `Matten`; implements `Display`
  and `std::error::Error`.
- `dynamic` feature (forwards `matten/dynamic`): dynamic tensors are rejected
  with `DynamicTensor` instead of panicking.

### Notes

- Rank-2 only; `rows = samples`, `columns = features` (enforced, no transposition).
- Constant (zero-variance / zero-range) columns are rejected explicitly rather
  than silently producing a zero column.
- No third-party dependency (only core `matten`, no default features); no `rand`.
