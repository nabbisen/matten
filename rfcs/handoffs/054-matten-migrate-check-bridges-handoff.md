# RFC-054 `matten-migrate` Bridge Check Handoff

**Project:** `matten`
**Related RFC:** RFC-054: `matten-migrate` Assisted Migration Tool
**Document kind:** Compact implementation handoff
**Status:** Accepted; implementation prepared for review
**Scope:** Local-only bridge-readiness advisory output for `tools/matten-migrate`

---

## 1. Summary

This handoff defines the next small RFC-054 slice after the reviewed
`explain-api` command.

Authorized next slice, if this handoff is accepted:

```text
matten-migrate check-bridges <path>
```

The command should inspect a local project with the existing scanner boundary
and print a deterministic Markdown note about bridge-related evidence. It must
not edit source, edit `Cargo.toml`, install bridge crates, run `cargo metadata`,
execute commands inside the inspected project, perform network lookups, or claim
that any bridge is required.

The command is a bridge-reading aid:

```text
"Here is local evidence around bridge conversions, the current supported bridge,
and the manual checks before using a bridge."
```

It is not a dependency recommendation engine, compatibility oracle, or automatic
bridge inserter.

---

## 2. Background

The current RFC-054 local tool supports:

```text
inspect <path>
report <path> [--output <path>]
suggest --target <target> <path>
explain-api <api-name>
list-targets
--help
```

RFC-054 still lists `check-bridges` as a possible later command. `rewrite` and
`apply` remain out of scope unless a later RFC explicitly authorizes automatic
rewriting.

The bridge-policy docs currently authorize exactly one bridge:

```text
matten-ndarray: Tensor <-> ndarray::ArrayD<f64>
```

No `matten-nalgebra`, `matten-polars`, `matten-candle`, or Python bridge exists.
The command must say that clearly when those ecosystems appear.

---

## 3. Command Shape

Implement only:

```text
matten-migrate check-bridges <path>
```

Accepted input policy should match `inspect`, `report`, and `suggest`:

```text
local file or directory path
no symlink root
skip symlink children
scan Cargo.toml and .rs files only
respect MAX_SCAN_BYTES
no command execution inside the inspected project
```

Reject unsupported forms with readable errors:

```text
matten-migrate check-bridges
matten-migrate check-bridges <path> <extra>
matten-migrate check-bridges --json <path>
matten-migrate check-bridges --output <path> <input>
matten-migrate check-bridges --fix <path>
matten-migrate check-bridges --target ndarray <path>
```

Do not implement in this slice:

```text
matten-migrate rewrite
matten-migrate apply
matten-migrate check-bridges --json
matten-migrate check-bridges --output
matten-migrate check-bridges --fix
matten-migrate check-bridges --target
Cargo.toml mutation
dependency insertion
cargo metadata integration
generated patches
```

No source or output file should be written by `check-bridges` in this slice.

---

## 4. Detection Boundary

Reuse the existing heuristic scanner where possible. Small scanner additions are
allowed only for direct dependency terms needed by this command.

Allowed signal categories:

```text
detected crates/features
matten-ndarray bridge
core Tensor construction
shape operations
reductions
linear algebra
dynamic ingestion
matten-data
dataframe pressure
direct target-library dependency
```

Direct target-library dependency detection may include:

```text
ndarray
nalgebra
polars
candle-core / candle-nn / candle-transformers
```

Keep this dependency scan lexical and conservative:

```text
Cargo.toml only
ignore commented lines
match dependency keys, not substrings
anchor on the key at line start after trimming
the dependency key is the name immediately before =, ., or whitespace
do not resolve dependency graphs
do not inspect lockfiles
do not infer transitive dependencies
```

Do not implement direct target-library detection with a raw substring such as
`contains("ndarray =")`. That would misclassify a bridge dependency line such as:

```toml
matten-ndarray = "0.30.0"
```

as a direct `ndarray` dependency. The detector must first classify the dependency
key itself.

Precedence rule:

```text
matten-ndarray / matten_ndarray key
  -> matten-ndarray bridge evidence
  -> that same Cargo.toml line must not also count as direct ndarray evidence
```

Direct target-library families:

```text
ndarray key -> ndarray
nalgebra key -> nalgebra
polars or polars-* key -> Polars
candle-core / candle-nn / candle-transformers key -> Candle
```

Group family aliases into one target-family signal rather than double-counting
multiple dependency names as separate ecosystems.

Bridge-check logic must distinguish these cases:

```text
matten-ndarray dependency or to_arrayd/from_arrayd source use
  -> existing reference bridge evidence

matten + ndarray direct dependency, but no matten-ndarray evidence
  -> possible bridge-contract reading candidate, not a required missing dependency

matten-data try_numeric/to_tensor evidence
  -> table-to-Tensor on-ramp, not a bridge crate

nalgebra / polars / candle evidence
  -> no approved bridge crate today; read playbook/manual conversion docs

no bridge evidence
  -> no strong bridge signal detected by this heuristic scan
```

Do not suggest a future bridge crate as though it already exists.

---

## 5. Output Shape

Print deterministic Markdown to stdout:

```text
# matten Bridge Check

> advisory disclaimer
> detection-limits disclaimer

Project: `<project>`.

## Bridge evidence
## Current bridge candidates
## Ecosystems without approved bridges
## Manual checks
## Risks
## Next steps
```

Required tone:

```text
may be a bridge-contract reading candidate
manual review should confirm
convert once at boundaries
current approved bridge
no approved bridge exists today
profile before moving hot paths
```

Forbidden tone:

```text
must add
missing dependency
best bridge
guaranteed compatible
automatic conversion
safe to rewrite
fix available
```

The output must include the same advisory and heuristic-detection disclaimers
used by `report` and `suggest`.

---

## 6. Bridge Rules

`matten-ndarray`:

```text
positive evidence:
  matten-ndarray Cargo.toml dependency
  matten_ndarray source use
  to_arrayd / from_arrayd source use
notes:
  current approved bridge
  copies both directions
  rejects dynamic tensors
  preserves logical row-major order
manual checks:
  convert once at boundaries
  avoid conversions inside hot loops
  make dynamic tensors numeric before conversion
docs:
  docs/src/migration/bridge-contracts.md
  docs/src/migration/bridge-crate-policy.md
  docs/src/migration/playbooks/ndarray.md
```

Direct `ndarray` without `matten-ndarray`:

```text
positive evidence:
  matten usage plus direct ndarray dependency or ndarray source terms
notes:
  may be worth reading the matten-ndarray bridge contract
  not enough evidence to require adding the bridge
manual checks:
  existing conversion path
  whether data crosses Tensor <-> ArrayD boundaries
  copy cost
```

`matten-data`:

```text
positive evidence:
  Table / try_numeric / to_tensor
notes:
  this is a table-to-Tensor on-ramp, not a bridge crate
  Polars/Pandas apply only when dataframe analytics are real requirements
manual checks:
  missing values
  non-numeric cells
  whether table work remains ingestion-only
```

Unavailable bridges:

```text
nalgebra:
  no matten-nalgebra bridge exists today
  read nalgebra playbook for manual conversion/redesign

polars/pandas:
  no bridge crate exists today
  enter dataframe tooling at the data-source/table boundary

candle:
  no bridge crate exists today
  f64 -> f32/device/model boundaries require manual design
```

---

## 7. Tests

Extend the existing `tools/matten-migrate` test suite.

Required tests:

```text
[ ] exact output for a fixture with matten-ndarray bridge evidence
[ ] matten-ndarray fixture is bridge evidence only, with no direct ndarray dependency evidence
[ ] direct ndarray dependency without matten-ndarray does not say "missing dependency"
[ ] matten-data to_tensor evidence is described as an on-ramp, not a bridge crate
[ ] nalgebra evidence states no approved bridge exists today
[ ] no bridge evidence produces "no strong bridge signal" style output
[ ] missing path fails clearly
[ ] extra path/argument fails clearly
[ ] --json/--output/--fix/--target forms are rejected
[ ] rewrite/apply remain rejected
[ ] existing inspect/report/suggest/explain-api/list-targets tests still pass
```

Exact-output tests must assert advisory wording, detection-limits wording, and
absence of forbidden phrases.

Add or reuse fixtures as needed:

```text
tools/matten-migrate/fixtures/ndarray-bridge-project
  existing matten-ndarray bridge fixture; must not produce direct ndarray evidence

tools/matten-migrate/fixtures/direct-ndarray-project
  new fixture with matten + direct ndarray dependency and no matten-ndarray dependency

tools/matten-migrate/fixtures/nalgebra-project
  new fixture with matten + nalgebra evidence and no approved bridge crate

tools/matten-migrate/fixtures/data-project
  existing matten-data fixture may be reused for table-to-Tensor on-ramp checks
```

---

## 8. Documentation

Update:

```text
tools/matten-migrate/README.md
docs/src/contributing/release-checklist.md
.github/workflows/test.yaml
```

User-facing migration docs do not need updates unless the implementation changes
the public story. If updated, keep wording short:

```text
local bridge-readiness note
advisory-only
heuristic
not a dependency installer
not a source rewriter
```

---

## 9. Verification

Required checks for implementation review:

```text
cargo fmt --all
cargo check --manifest-path tools/matten-migrate/Cargo.toml
cargo test --manifest-path tools/matten-migrate/Cargo.toml
cargo clippy --manifest-path tools/matten-migrate/Cargo.toml -- -D warnings
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- check-bridges tools/matten-migrate/fixtures/ndarray-bridge-project
cargo run --manifest-path tools/matten-migrate/Cargo.toml -- check-bridges tools/matten-migrate/fixtures/simple-core-project
bash scripts/check-release-docs.sh
git diff --check
```

Optional if user-facing docs are changed beyond the local-tool README/checklist:

```text
mdbook build docs
```

---

## 10. Acceptance Criteria

The implementation is accepted when:

```text
[ ] check-bridges <path> works deterministically
[ ] output is advisory and scoped to bridge-readiness evidence
[ ] unsupported options/forms fail clearly
[ ] no source/output mutation exists
[ ] no network/telemetry/source-upload behavior exists
[ ] no Cargo.toml/dependency mutation exists
[ ] current approved bridge is limited to matten-ndarray
[ ] unavailable bridges are named as unavailable, not suggested as crates to add
[ ] direct ndarray evidence is framed as a reading candidate, not a required dependency
[ ] matten-data to_tensor is described as an on-ramp, not a bridge crate
[ ] existing inspect/report/suggest/explain-api/list-targets behavior is not regressed
[ ] exact-output tests cover disclaimers and forbidden wording
[ ] CI and release checklist include check-bridges smoke commands
```

---

## 11. Still Deferred

Still deferred after this slice:

```text
rewrite/apply
Cargo.toml mutation
automatic bridge dependency insertion
generated patches
AST parser
cargo metadata / cargo check integration
ML/code-transformation assistance
remote analysis
public matten-migrate crate
stable report-format promise
check-bridges --json
check-bridges --output
check-bridges --fix
configuration file support
new bridge crates
```
