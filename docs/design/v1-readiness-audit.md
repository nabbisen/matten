# v1.0 Readiness Audit

**Project:** `matten`
**Related RFC:** RFC-066: v1.0 Readiness Audit and Release Decision Gate
**Document kind:** Readiness audit report
**Status:** Accepted audit; BF-1 remediation update pending review
**Scope:** Audit report only; no v1.0 release authorization

---

## Summary

The current `matten` family is close enough to justify a maintainer-level v1.0
readiness discussion, but it is **not ready for v1.0 release preparation until
the companion-maturity policy decision is made explicitly**.

The original audit found:

```text
resolved blocking finding:
  BF-1: public API snapshot had an inconsistent dynamic serde statement.
        Remediated in docs/src/reference/public-api-snapshot.md.

maintainer-decision finding:
  MD-1: lock-step v1.0 with production-ready-candidate companions must be
        decided explicitly.

non-blocking findings:
  NF-1: companion snapshot-equivalent docs are adequate, but matten-data would
        benefit from a more explicit public API block before v1.0.
  NF-2: cargo public-api remains a required future snapshot step, but it is not
        wired as a project gate in this audit slice.
```

Recommendation:

```text
Do not start v1.0 release preparation yet.
BF-1 has been remediated; MD-1 remains required.
After MD-1, maintainers may decide whether to draft a separate v1.0 release RFC.
```

This report does not authorize a v1.0 release.

## Decision Boundary

This report is evidence for a later maintainer decision. It does not authorize:

```text
v1.0 release
version bump
tag
publish
API change
dependency change
companion promotion
new public tooling crate
```

If a later decision proposes any of those actions, it needs a separate release
RFC, release-policy decision, or accepted implementation slice.

## Public API Review

Reviewed inputs:

```text
docs/src/reference/public-api-snapshot.md
docs/src/reference/compatibility.md
crates/matten/src/lib.rs
crates/matten-ndarray/src/lib.rs
crates/matten-mlprep/src/lib.rs
crates/matten-data/src/lib.rs
crate READMEs
```

The core crate root matches the documented root surface:

```text
Tensor
MattenError
DataFormat
MattenLimits
SliceBuilder
Element                 # feature = "dynamic"
NumericPolicy           # feature = "dynamic"
```

The extra core exports remain hidden compiler-visibility plumbing:

```text
IntoSliceRange          # #[doc(hidden)]
SliceConvert            # #[doc(hidden)]
SliceSpecRepr           # #[doc(hidden)]
```

The crate root does not expose public modules. Source review found only private
modules plus explicit `pub use` exports.

The public snapshot documents `MattenError` and `DataFormat` as
`#[non_exhaustive]`, which is the right compatibility posture for future
variants. `MattenError` is also documented as match-by-variant, not
`PartialEq`-comparable, because it embeds `std::io::Error`.

Companion surfaces are small and source-visible:

```text
matten-ndarray:
  from_arrayd
  to_arrayd
  MattenNdarrayError

matten-mlprep:
  standardize_columns
  minmax_scale_columns
  add_bias_column
  train_test_split
  MattenMlprepError

matten-data:
  Table
  NumericTable
  ColumnKind
  ColumnSummary
  SchemaSummary
  CellValue
  MattenDataError
```

`matten-ndarray` and `matten-mlprep` have clear README/rustdoc
snapshot-equivalent public API sections. `matten-data` documents the workflow,
status, scope, dependency direction, and exported crate root, but its README is
less explicit as a public API snapshot than the other companions. That is
recorded as NF-1, not as a current release blocker.

Original finding BF-1: `docs/src/reference/public-api-snapshot.md` was
internally inconsistent about dynamic serde behavior. The dynamic behavior table
said `Serialize` returns a serde error, while the later boundary/serde table
said `Serialize` panics on dynamic. Source review of `crates/matten/src/ser.rs`
shows that dynamic serialization returns `Err(serde::ser::Error::custom(...))`.
The snapshot now matches source behavior: dynamic serialization returns a serde
error.

## Panic/Result Boundary Review

Reviewed inputs:

```text
docs/src/reference/compatibility.md
docs/src/reference/error-model.md
docs/src/reference/boundary.md
docs/src/reference/public-api-snapshot.md
docs/src/reference/dynamic.md
```

The project has a stable and well-explained two-zone policy:

```text
Panic zone:
  local, trusted, developer-authored convenience APIs

Result zone:
  external boundaries: parsing, files, user shapes, conversion boundaries
```

The boundary pages consistently teach `try_*` APIs for user-provided shapes and
file/parse APIs returning `Result<Tensor, MattenError>`. Dynamic tensors are also
guarded: numeric operations reject them until the user explicitly calls
`try_numeric()` or `try_numeric_with(policy)`.

The panic/Result split is stable enough for v1.0 discussion after BF-1
remediation. BF-1 was a documentation inconsistency, not an apparent source
behavior defect.

## Serde/Format Review

Reviewed inputs:

```text
docs/src/reference/boundary.md
rfcs/done/009-serde-json-csv-and-boundary-integration.md
crates/matten/examples/10_json_roundtrip.rs
crates/matten/examples/11_csv_numeric_loading.rs
crates/matten/src/ser.rs
```

JSON has a clear canonical object form:

```text
{"shape":[...],"data":[...]}
```

The canonical form is unambiguous for any rank and is used by serde
serialization/deserialization. Rank-1 and rank-2 nested arrays are documented as
convenience input forms, not as the canonical representation.

CSV is correctly framed as ingestion:

```text
rectangular numeric CSV -> Tensor with shape [rows, cols]
```

It is not documented as canonical tensor serialization. Mixed/missing data is
owned by the dynamic path or by `matten-data` table preparation, not by core
numeric CSV.

The feature split is clear:

```text
serde  -> Serialize / Deserialize
json   -> from_json / load_json, implies serde
csv    -> from_csv / load_csv
dynamic -> heterogeneous ingestion and explicit numeric conversion
```

With BF-1 remediated, the serde/format story is stable enough for v1.0
discussion.

## Companion Maturity Review

Reviewed inputs:

```text
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
README.md
crates/*/README.md
crates/*/src/lib.rs
```

The current maturity ladder is explicit:

```text
matten          stable (v0.x)
matten-ndarray production-ready
matten-mlprep  production-ready candidate
matten-data    production-ready candidate
```

RFC-030 separates version compatibility from maturity label. Matching lock-step
versions mean "matched family set"; maturity is the Status label.

This creates the required maintainer decision point:

```text
Can a lock-step v1.0 family include production-ready-candidate companions
with explicit labels, or must all family crates become production-ready first?
```

This audit does not answer that silently. It records MD-1:

```text
MD-1: Before v1.0 release preparation, the maintainer must decide whether the
      v1.0 family may include candidate-labeled companions, or whether
      matten-mlprep and matten-data must first receive separate full
      production-ready reviews.
```

If the answer is "candidate-labeled companions are allowed under lock-step v1,"
then the current maturity labels may be compatible with v1.0, provided the
release notes and README state the distinction clearly.

If the answer is "all v1 family crates must be production-ready," then
`matten-mlprep` and `matten-data` need follow-up maturity RFCs before v1.0
release preparation.

## Deferred-Item Review

Reviewed inputs:

```text
docs/src/reference/compatibility.md
docs/src/philosophy.md
docs/src/migration/
rfcs/README.md
ROADMAP.md
docs/src/examples/
```

Deferred items are explicit enough not to block v1.0 by ambiguity. The project
does not hide future work as accidental current scope.

Examples:

```text
Display for Tensor: deferred formatting contract
public mutation API: deferred
zero-sized tensors: rejected/deferred
negative slice indices: deferred
batched matmul: deferred
serious linalg/decompositions: out of core; migrate
sample variance / quantile / histogram / covariance / correlation: deferred
streaming / large CSV: deferred
shuffled or seeded train_test_split: deferred
public matten-report / matten-viz / matten-migrate crates: deferred
rewrite/apply migration automation: future-owned
autograd / ML framework scope: not core scope
```

The examples program is also scoped: examples demonstrate accepted APIs and are
not a path for adding dataframe, ML, GPU, large-data, or serious-linalg scope.

No deferred item should be converted into implementation work as part of the
v1.0 audit. If a deferred item becomes required for v1.0, that needs a separate
RFC or release-policy decision.

## Release-Gate Review

Reviewed inputs:

```text
docs/src/contributing/release-checklist.md
docs/src/tutorial/start-here.md
docs/src/examples/
docs/src/reference/error-model.md
docs/src/reference/dynamic.md
rfcs/done/022-companion-crate-boundary-policy.md
```

The release checklist has two relevant layers:

```text
normal release gates:
  fmt, boundary scripts, release-docs guard, clippy, tests, doctests,
  feature matrix, examples, tools, MSRV, public API audit

v1.0.0 gate:
  explicit maintainer confirmation plus:
  stable core public API
  clear dynamic on-ramp story
  strong, scoped examples
  reliable diagnostics
  documented companion-crate boundary
  clean feature matrix across all profiles
```

Assessment against the v1.0.0 gate:

| Gate | Assessment |
|---|---|
| stable core public API | Mostly yes. BF-1 has been remediated; the official future public API snapshot step still needs release-prep ownership. |
| clear dynamic on-ramp story | Yes. `docs/src/reference/dynamic.md` and `docs/src/tutorial/start-here.md` teach ingest, inspect, clean, convert, then compute. |
| strong, scoped examples | Yes. Examples are grouped, linked to source, and explicitly constrained to accepted APIs. |
| reliable diagnostics | Mostly yes. `docs/src/reference/error-model.md` documents error variants, panic prefix, and matching guidance. BF-1 has been remediated to avoid serde-boundary ambiguity. |
| documented companion-crate boundary | Yes. README, migration docs, and RFC-022/RFC-030 family policy keep core dependency-light and companions optional. |
| clean feature matrix | The release checklist defines the required feature-matrix commands. This audit did not convert those commands into a new gate. |

The release checklist and compatibility policy are reconciled: both say v1.0
requires explicit maintainer confirmation, public API review, and stable
boundary/format documentation. This report is not that confirmation.

## Blocking Findings

No unresolved blocking source/doc mismatch remains after BF-1 remediation.

### Resolved BF-1: Public API snapshot contradicted itself on dynamic serde behavior

Path:

```text
docs/src/reference/public-api-snapshot.md
crates/matten/src/ser.rs
```

Problem:

```text
The dynamic behavior table says Serialize returns a serde error.
The boundary/serde table said Serialize panics on dynamic.
The source returns a serde error for dynamic tensors.
```

Impact:

```text
The public API snapshot could not be approved for v1.0 while it contained
contradictory behavior for a public serde implementation.
```

Remediation:

```text
The boundary/serde table now says Serialize returns a serde error on dynamic.
Review this correction before treating BF-1 as closed.
```

## Maintainer-Decision Findings

### MD-1: Lock-step v1.0 with candidate-labeled companions

Path:

```text
README.md
rfcs/done/030-workspace-versioning-model-lockstep.md
rfcs/done/057-promote-matten-ndarray-production-ready.md
rfcs/done/058-promote-matten-mlprep-production-ready-candidate.md
rfcs/done/059-promote-matten-data-production-ready-candidate.md
crates/*/README.md
```

Question:

```text
Can the v1.0 family include matten-mlprep and matten-data as
production-ready candidates, or must they first become production-ready?
```

This is not a source defect. It is a release-policy decision created by the
combination of lock-step family versioning and per-crate maturity labels.

## Non-Blocking Findings

### NF-1: matten-data snapshot-equivalent docs are less explicit

`matten-data` has a clear README, crate-level docs, and source-visible root
exports. However, compared with `matten-ndarray` and `matten-mlprep`, its README
is less explicit as a public API snapshot-equivalent.

Before v1.0, consider adding a compact `Public API` block to
`crates/matten-data/README.md` covering:

```text
Table
NumericTable
SchemaSummary / ColumnSummary / ColumnKind
CellValue
MattenDataError
CSV feature requirement
missing-value handling
strict numeric conversion
scope lock: not a dataframe
```

This is non-blocking because the current source and docs are sufficient for this
audit, but the README could be clearer as a v1 review artifact.

### NF-2: cargo public-api remains a future snapshot step

The compatibility policy requires a `cargo public-api` snapshot before v1.0
approval. This audit performed source-level export review and documentation
comparison, but it did not add `cargo public-api` as a dependency or required
project gate.

That is intentional for RFC-066. A later v1.0 release-prep RFC should decide how
to take, store, and review the official public API snapshot.

## Verification Record

Observed during this audit implementation:

```text
cargo fmt --all --check                                      passed
bash scripts/check-core-dependency-boundary.sh                passed
bash scripts/check-published-dependency-isolation.sh          passed
bash scripts/check-matten-data-scope.sh                       passed
bash scripts/check-benchmark-dependency-sync.sh               passed
bash scripts/check-streaming-scope.sh                         passed
bash scripts/check-release-docs.sh                            passed
git diff --check                                              passed
```

Not run in this audit implementation:

```text
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
full feature-matrix test set from docs/src/contributing/release-checklist.md
MSRV build/test commands
```

Reason:

```text
This slice creates an audit report and design-doc index entry only. It changes
no Rust source, public API, dependencies, feature flags, manifests, generated
artifacts, or release versions. The omitted gates remain required before any
actual v1.0 release-prep decision.
```

## Recommendation

Current recommendation:

```text
not ready for v1.0 release preparation until MD-1 is decided
```

Minimum next steps:

```text
1. Review the BF-1 remediation.
2. Make an explicit maintainer decision on MD-1.
3. If MD-1 requires full production-ready companions, draft follow-up maturity
   RFCs for matten-mlprep and matten-data before v1.0 release preparation.
4. If MD-1 allows candidate-labeled companions in a lock-step v1.0 family,
   draft a separate v1.0 release RFC only after that decision is recorded.
```

No v1.0 release is authorized by this report.
