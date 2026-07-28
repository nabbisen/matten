# RFC-082 Streaming CSV Batches: Implementation Handoff

**Project:** `matten`
**Related RFC:** RFC-082 (design authority)
**Document kind:** Detailed implementation handoff
**Status:** Accepted and implemented. `CsvBatchReader::{open, next_batch}` added to `matten-data` behind
the off-by-default `streaming` feature; 13 new tests including the equivalence, batch-boundary
line-number-parity, trailing-newline, and blank-line tests; the streaming guard confirmed passing
unmodified against the real code. No version bump or release
**Date:** 2026-07-28

---

## 1. Purpose

Add `CsvBatchReader` to `matten-data` behind a `streaming` feature, as one reviewable slice.
RFC-082 is the design authority.

**No release.** Ends at a reviewed implementation commit on the `0.x` line.

## 2. Preconditions

```text
RFC-082 and this handoff accepted
working tree clean; version stays 0.39.0
no v1 activity — RFC-076 stays deferred and untouched
```

## 3. Files

```text
crates/matten-data/Cargo.toml            add the `streaming` feature + one [[example]]
crates/matten-data/src/lib.rs            `pub mod stream;` (cfg-gated) + pub use + crate-doc mention
crates/matten-data/src/stream.rs         CsvBatchReader
crates/matten-data/src/error.rs          new variant(s) if needed — see §5.3
crates/matten-data/tests/streaming.rs    all tests for this feature
crates/matten-data/examples/csv_batches.rs   one example
crates/matten-data/README.md             ## Public API entry, feature-gated
```

**No new dependency.** `csv` is already an optional dependency and is in `default`. The `streaming`
feature must imply it:

```toml
[features]
default   = ["csv"]
csv       = ["dep:csv"]
streaming = ["csv"]     # off by default; batched CSV reading (RFC-082)
```

## 4. Feature gating — get this right first

Everything new is behind `#[cfg(feature = "streaming")]`. With the feature off, `matten-data` must compile
to exactly what it is today — same public surface, same docs, same everything.

```rust
// lib.rs
#[cfg(feature = "streaming")]
pub mod stream;
#[cfg(feature = "streaming")]
pub use stream::CsvBatchReader;
```

Verify by building both ways and diffing the public surface (§8).

## 5. Implementation

### 5.1 The type

```rust
pub struct CsvBatchReader {
    reader: csv::Reader<std::fs::File>,
    headers: Vec<String>,
    batch_rows: usize,
    line: usize,        // one-based CSV line number, header = 1
    done: bool,
}
```

`done` is what makes "every subsequent call returns `Ok(None)`" (RFC-082 §4.1) and "the reader is left
unusable after an error" (§4.3) both true — set it on EOF *and* on error.

### 5.2 `open`

```text
reject batch_rows == 0 with an explicit error before opening the file
open the file; read the header row once; store headers
line starts at 1 (the header)
```

### 5.3 `next_batch`

```text
if done -> Ok(None)
collect up to batch_rows records:
  - field count != headers.len()  -> set done, return Err (same variant and
                                      one-based line number Table::from_csv_path uses)
  - a csv parse error             -> set done, return Err(MattenDataError::Csv { .. })
  - an I/O error                  -> set done, return Err(MattenDataError::Io { .. })
if zero records collected and EOF -> set done, Ok(None)
otherwise build a Table from headers + collected rows and return Ok(Some(table))
```

**Reuse the existing error variants.** `error.rs` already documents *"one-based CSV line numbers (the
header is line 1)"* and carries row-length/parse variants. Add a new variant only for
`batch_rows == 0`, which has no existing equivalent — and if you add one, document it in the same style
as its siblings.

**Do not invent a second malformed-row story.** A user moving from `from_csv_path` to `CsvBatchReader`
must get the same error type, the same line numbering, and the same fail-fast behaviour.

### 5.4 Building the `Table`

Construct batches through the same path `from_csv_str`/`from_csv_path` already use, so `CellValue`
parsing, missing-value handling, and column typing are identical. **Do not hand-roll cell parsing** — a
batch must be indistinguishable from the corresponding slice of a fully-loaded `Table`.

## 6. The streaming guard needs no edit — verify, don't modify

An earlier draft of this handoff told you to narrow `scripts/check-streaming-scope.sh`, on the belief that
`BatchReader` being a substring of `CsvBatchReader` would cause a false positive. **That was wrong**, and it
was wrong because the check behind it tested the bare alternation instead of the guard's real pattern.

The guard anchors the forbidden names immediately after `pub struct`/`enum`/`type`:

```text
pub[[:space:]]+(struct|enum|type)[[:space:]]+(CsvStream|BatchReader|AsyncCsvReader)\b
```

`CsvBatchReader` begins `Csv`, so the alternative never lines up with the anchor. Verified end-to-end
against a fixture containing the exact proposed surface: **guard passes, unmodified.** The example name
`data_csv_batches` also clears `EXAMPLE_ALT`'s `(^|_)(...)(_|$)` anchoring.

**Do not modify this script.** Just run it (§9) and confirm it passes. If it ever *does* fail on
`CsvBatchReader`, that means someone widened the pattern — investigate rather than narrowing it back.

## 7. Tests — `crates/matten-data/tests/streaming.rs`

```text
[ ] exact batching: 10-row file, batch_rows = 3 -> 3,3,3,1 then Ok(None)
[ ] file smaller than one batch -> single batch, then Ok(None)
[ ] empty data (header only) -> Ok(None) immediately
[ ] repeated calls after exhaustion keep returning Ok(None)
[ ] EQUIVALENCE: concatenating all batches equals Table::from_csv_path on the same
    file — same columns, same cell values, same order. This is the test that proves
    batching is a memory strategy and not a different parser
[ ] malformed row (wrong field count) -> Err with the SAME variant and one-based line
    number Table::from_csv_path gives for the same file
[ ] reader is unusable after an error: the next call does not resume or panic
[ ] batch_rows = 0 -> Err at open()
[ ] missing file -> Err at open()
[ ] headers are applied to every batch, not just the first
[ ] LINE-NUMBER PARITY AT A BATCH BOUNDARY: malformed row in a LATER batch
    (e.g. bad row 7 with batch_rows = 3) reports the same one-based line number
    as Table::from_csv_path on the identical file. Compare the two errors against
    EACH OTHER, not against a hard-coded number — that is what catches an
    off-by-one that both paths would otherwise have to share
[ ] trailing-newline file: concatenated batches == from_csv_path
[ ] empty last row: concatenated batches == from_csv_path
```

The last three exist because the first-batch case can pass while an incremental parser diverges from a
whole-file one at a boundary or on a file-end edge case.

The equivalence test is the important one. Everything else can pass while the parser silently diverges.

## 8. Feature-off verification

```bash
cargo build -p matten-data --no-default-features
cargo build -p matten-data                              # default: csv on, streaming off
cargo build -p matten-data --features streaming
cargo test  -p matten-data --features streaming
cargo test  -p matten-data                              # streaming tests must not run
# public surface must be unchanged with the feature off:
grep -n "^pub use\|^pub mod" crates/matten-data/src/lib.rs
```

With `streaming` off, `CsvBatchReader` must not appear in the public API at all.

## 9. Full verification

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --no-default-features -- -D warnings
cargo test --all-targets
cargo test --doc --all-features
cargo +1.85.0 build && cargo +1.85.0 test --all-features
cargo run -p matten-data --example data_csv_batches --features streaming
bash scripts/check-streaming-scope.sh          # UNMODIFIED — must pass as-is; see §6
bash scripts/check-matten-data-scope.sh        # RFC-042 lock must still pass
bash scripts/check-core-dependency-boundary.sh
bash scripts/check-published-dependency-isolation.sh
bash scripts/check-release-docs.sh
bash tools/matten-report/tests/process-boundary.sh    # anchors untouched
git diff --check
```

Scope confirmation:

```bash
git diff --name-only -- crates/matten crates/matten-mlprep crates/matten-ndarray crates/matten-stats
# expect EMPTY
grep -c "^version" Cargo.toml && grep -m1 "^version" Cargo.toml   # still 0.39.0
```

## 10. What the review request must report

```text
[ ] the equivalence test result (batches concatenated == from_csv_path)
[ ] malformed-row parity: same variant, same one-based line number, shown side by side
[ ] scripts/check-streaming-scope.sh passes UNMODIFIED, and confirmation the script is not in the diff
[ ] feature-off build + public-surface check
[ ] confirmation no new dependency (Cargo.toml diff)
[ ] RFC-042 scope guard still passing
[ ] git diff showing the other four crates untouched
[ ] full gate set incl. MSRV
[ ] confirmation of no version bump, CHANGELOG, or release action
```

## 11. Known pitfalls

1. **Editing the streaming guard.** It needs no change (§6). An earlier draft said otherwise; that was a
   verification error, not a real constraint.
2. **Hand-rolling cell parsing** instead of reusing the `Table` construction path — produces batches that
   subtly differ from `from_csv_path`. The equivalence test exists to catch it.
3. **Different malformed-row semantics** from the non-streaming path. Reuse the variants.
4. **Forgetting `done` on the error path**, leaving a reader that resumes mid-file after a failure.
5. **Leaking the API when the feature is off** — a missed `#[cfg]` on the `pub use`.
6. **Adding a dependency.** `csv` is already there; `streaming` just implies it.
7. **Touching the streaming guard at all** — narrowing, widening, or deleting. It already permits this slice and still forbids everything RFC-082 §5 defers (§6).
8. **Bumping the version** — no release in this slice.

## 12. Review stop

Acceptance makes this a commit point. It authorizes no release, version bump, tag, publish, async work, or
`matten-stream` crate.
