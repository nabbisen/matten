# Release checklist

This page documents the steps required before publishing any `matten` release.
It is the canonical gate referenced by RFC-015.

## Before every release

### 1. Source verification

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh   # RFC-022 core boundary gate
bash scripts/check-matten-data-scope.sh          # RFC-042 matten-data anti-scope guard
bash scripts/check-release-docs.sh               # doc-truth + examples naming-band guards
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo clippy --all-targets --no-default-features --features dynamic -- -D warnings
RUSTFLAGS="-D warnings" cargo check --all-targets --all-features
cargo test --all-targets
cargo test --doc --all-features
```

### 2. Feature matrix

```bash
cargo test --no-default-features
cargo test --no-default-features --features serde
cargo test --no-default-features --features json
cargo test --no-default-features --features csv
cargo test --no-default-features --features dynamic
cargo test --no-default-features --features dynamic,json
cargo test --no-default-features --features dynamic,csv
cargo test --no-default-features --features dynamic,json,csv
cargo test --all-features
```

### 3. Examples

```bash
cargo check --examples
cargo check --examples --all-features
cargo run --example 00_quickstart
cargo run --example 06_broadcasting
cargo run --example 08_slicing_builder
cargo run --example 12_boundary_error_handling
cargo run --example dynamic_00_quickstart --features dynamic,json,csv
cargo run --example dynamic_05_dirty_csv_cleanup --features dynamic,json,csv
```

### 4. MSRV

```bash
cargo +1.85.0 build
cargo +1.85.0 test --all-features --quiet
```

### 5. Public API audit

Compare the current public surface against `docs/src/reference/public-api-snapshot.md`.

Allowed root exports:

- `Tensor`
- `MattenError`
- `DataFormat`
- `MattenLimits`
- `SliceBuilder`
- `Element` (under `#[cfg(feature = "dynamic")]`)
- `NumericPolicy` (under `#[cfg(feature = "dynamic")]`)

Allowed `#[doc(hidden)]` exports (compiler visibility only, not user-facing):

- `IntoSliceRange`
- `SliceConvert`
- `SliceSpecRepr`

Run a spot-check:

```bash
grep -n "^pub use" src/lib.rs
```

Verify no module accidentally became `pub mod`.

### 6. Documentation truth pass

```bash
# No stale version strings in user-facing files
grep -R "Status:.*0\.[0-9]\{2\}\." README.md docs/src/ src/lib.rs || true

# No stale "matten 0.x" in runtime messages
grep -rn "matten 0\." src/ | grep -v "CHANGELOG\|#\[" || true

# No version-specific claims in lib.rs crate docs
grep "This is.*0\." src/lib.rs || true
```

### 7. CHANGELOG

- Every API change has a changelog entry.
- Changelog entries describe actual changes, not planned ones.
- No changelog entry claims a fix that is not in the code.

### 8. Version bump

Update `Cargo.toml` version. During v0.x, patch releases (0.13.x) should not
introduce new public API unless a minor release (0.14.0) is intended.

---

## Additional gates for minor releases (0.14.0, 0.15.0, …)

- New public API has a corresponding accepted RFC.
- Public API snapshot is regenerated and reviewed.
- mdBook examples for new APIs compile and run.
- Migration guide updated if any method signature changed.

---

## v1.0.0 gate

v1.0.0 requires **explicit confirmation from the maintainer (nabbisen)**.
It is not triggered automatically by any feature or test passing.

Before v1.0.0, the project should have:

- stable core public API;
- clear dynamic on-ramp story;
- strong, scoped examples;
- reliable diagnostics;
- documented companion-crate boundary (RFC-022);
- clean feature matrix across all profiles.
