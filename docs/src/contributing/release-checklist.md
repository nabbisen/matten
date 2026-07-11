# Release checklist

This page documents the steps required before publishing any `matten` release.
It is the canonical gate referenced by RFC-015.

## Before every release

### 1. Source verification

```bash
cargo fmt --all --check
bash scripts/check-core-dependency-boundary.sh   # RFC-022 core boundary gate
bash scripts/check-published-dependency-isolation.sh  # RFC-049 §B1 per-crate peer-dep isolation
bash scripts/check-matten-data-scope.sh          # RFC-042 matten-data anti-scope guard
bash scripts/check-benchmark-dependency-sync.sh  # benchmark harness ndarray pin == workspace requirement
bash scripts/check-streaming-scope.sh            # RFC-037 streaming / large-CSV anti-scope guard
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
cargo run --example 57_visual_shape_axis_summary
cargo run --example dynamic_00_quickstart --features dynamic,json,csv
cargo run --example dynamic_05_dirty_csv_cleanup --features dynamic,json,csv
cargo check -p matten --example dynamic_09_visual_readiness_summary
cargo run -p matten --example dynamic_09_visual_readiness_summary --features dynamic
cargo run -p matten-data --example data_06_visual_readiness_summary
cargo run -p matten-mlprep --example mlprep_visual_standardize_summary
cargo check --manifest-path tools/matten-report/Cargo.toml
cargo test --manifest-path tools/matten-report/Cargo.toml
cargo clippy --manifest-path tools/matten-report/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo data-readiness --output target/matten-report-demo.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --input tools/matten-report/fixtures/small.csv --kind data-readiness --select sales,cost
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow --output target/matten-report-shape-flow.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo dynamic-readiness --output target/matten-report-dynamic-readiness.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo mlprep-standardization --output target/matten-report-mlprep-standardization.md
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo educational-path --output target/matten-report-educational-path.md
cargo check --manifest-path tools/matten-migrate/Cargo.toml
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- list-targets
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- inspect tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- report tools/matten-migrate/fixtures/simple-core-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- report tools/matten-migrate/fixtures/simple-core-project --output target/matten-migration-report.md
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target ndarray tools/matten-migrate/fixtures/receiver-method-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target polars-pandas tools/matten-migrate/fixtures/common-rust-collisions-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- suggest --target stay-with-matten tools/matten-migrate/fixtures/simple-core-project
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

### Public-dependency-minor changes

When a published crate re-exposes a third-party type in its public API (for example
`matten-ndarray` exposing `ndarray::ArrayD<f64>` through `to_arrayd`/`from_arrayd`), changing the
supported **minor** of that dependency is a public-API compatibility event — not a routine
`cargo update` — and is handled as a lock-step family minor (RFC-030). Before releasing such a
change:

- The change has an accepted RFC recording the supported version(s) and the decision (a single
  bump vs. a bounded range). Precedent: RFC-062 (`ndarray` → `0.17`), which weighed a `0.16`+`0.17`
  range before the maintainer chose a single-version requirement to keep `Cargo.toml` simple.
- If a **range** is supported, CI verifies the crate's tests, doctests, and examples against each
  supported minor — e.g. `cargo update -p <dep> --precise <ver>` in a fresh checkout (so the
  per-job lockfile edit is not committed). A single-version requirement needs only the normal job
  against the resolved patch (document which patch CI targets).
- **No version-conditional bridge/crate code.** If the unchanged crate cannot compile against every
  supported minor, narrow the range instead of adding `#[cfg(...)]` branches or per-version feature
  flags.
- Docs state that the resolved dependency minor is part of the crate's **public type identity**,
  name any yanked patch that is excluded and not a tested target, and note that docs.rs renders a
  single resolved minor even though CI verifies the full range.
- **MSRV is re-verified with the new dependency version in the graph.** A dependency's own
  `rust-version` is not sufficient — its transitive dependencies can raise the floor independently.
- Core `matten` dependency isolation is re-confirmed (the published-dependency-isolation guard still
  passes; the change must not leak a peer dependency into the core graph).
- If the dependency is also used by the workspace-excluded benchmark harness (e.g. a peer pin in
  `benchmarks/Cargo.toml`), its pin is synced by hand and `check-benchmark-dependency-sync.sh`
  passes — the harness cannot inherit `{ workspace = true }`, so this guard catches a forgotten sync.

### Workspace core-dependency requirement

Companion crates inherit core `matten` through `[workspace.dependencies]` as
`matten = { version = "0", path = "crates/matten", default-features = false }`
(RFC-064). Do not narrow this requirement during routine family releases. User
docs and examples still show explicit matched release pins so downstreams see the
supported family set.

Before release, verify that package dry-runs account for publish ordering:

- `matten` must be published first.
- Companion package dry-runs may fail before core is visible on crates.io if they
  need to resolve the just-released core version; record that as a sequencing
  caveat rather than a dependency-policy failure.
- If the broad requirement is intentionally changed, update RFC-030/RFC-064,
  this checklist, companion README compatibility notes, and package dry-run
  expectations in the same review slice.

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
