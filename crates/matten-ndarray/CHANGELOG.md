# Changelog — matten-ndarray

All notable changes to `matten-ndarray` are documented here.
This crate uses independent SemVer (RFC-022 §7).

## [0.1.0] - 2026-06-21

**Experimental first release.** Conversion bridge between `matten::Tensor` and
`ndarray::ArrayD<f64>` (RFC-025, RFC-027).

### Added

- `to_arrayd(&Tensor) -> Result<ArrayD<f64>, MattenNdarrayError>`.
- `from_arrayd(ArrayD<f64>) -> Result<Tensor, MattenNdarrayError>`, which:
  - preserves logical element order for non-standard-layout (transposed /
    sliced) inputs;
  - rejects shapes with a zero-length axis (`ZeroSizedAxis`);
  - maps core rejections (e.g. rank > 8) to `MattenNdarrayError::Matten`.
- `MattenNdarrayError` (`#[non_exhaustive]`): `DynamicTensor`, `ZeroSizedAxis`,
  `NdarrayShape`, `Matten`; implements `Display` and `std::error::Error`.
- `dynamic` feature (forwards `matten/dynamic`): a dynamic tensor passed to
  `to_arrayd` returns `Err(DynamicTensor)` instead of panicking.

### Notes

- Both conversions copy; no zero-copy is claimed.
- Supported `ndarray`: the `0.16` minor.
