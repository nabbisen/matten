# RFC-026 / RFC-037 Large Data and Streaming Policy Closure Handoff

**Project:** `matten`
**Related RFCs:** RFC-026: Large CSV and Streaming Data Policy; RFC-037: Deferred Streaming and Large CSV Policy
**Document kind:** Compact policy-closure handoff
**Status:** Revised draft for rereview; not implementation authority until accepted
**Scope:** Docs/RFC/guard cleanup only; no streaming implementation

---

## 1. Summary

This handoff proposes the next small theme after the `0.30.0` educational
visualization release: close the old large-CSV / streaming policy debt without
adding streaming support.

Before this slice, the repo had two overlapping proposed RFCs:

```text
rfcs/proposed/026-large-csv-and-streaming-data-policy.md
rfcs/proposed/037-deferred-streaming-and-large-csv-policy.md
```

Both say the same durable thing:

```text
streaming and large CSV support are deferred
core matten must not expose streaming APIs
matten-data remains small in-memory table-to-Tensor preparation
large-data/table analytics belong in dedicated tools such as Polars, DataFusion, or Pandas
```

The proposed `0.31.0` theme is to make that policy explicit, current, and
guarded, then move the resolved policy records out of `proposed/`. RFC-037
should become the canonical resolved deferral-policy record; RFC-026 should be
retained as superseded/consolidated into RFC-037 rather than left as a duplicate
active policy.

---

## 2. Reviewer Background

`matten` now has a stronger educational/data-understanding story:

```text
0.29.0: RFC-063 visual-understanding docs/examples/local report tool
0.30.0: RFC-065 educational tensor-learning path
```

That makes one boundary more important: educational data understanding must not
be misread as large-data or streaming support.

Current user-facing docs already state this boundary in several places:

```text
README.md: matten-data is small CSV-to-Tensor ingestion, not a dataframe library
docs/src/examples/data.md: large/streaming data should use dataframe/query tools
crates/matten-data/README.md: no large-data streaming
crates/matten-data/src/lib.rs: no large-data streaming
```

The remaining problem is lifecycle debt, not missing runtime behavior:

```text
RFC-026 and RFC-037 still sit in proposed/
their content overlaps
the guard story is dataframe-focused but not explicitly streaming-focused
the project has no single current policy-closure note for large/streaming data
```

---

## 3. Proposed Implementation Slice

Keep the slice documentation/policy-only:

```text
1. Audit public docs and examples for large-data / streaming claims.
2. Add or tighten a small guard that rejects new public streaming API/example names.
3. Mark RFC-037 as the canonical resolved deferral-policy record.
4. Move RFC-026 and RFC-037 from proposed/ to done/, with RFC-026 explicitly
   marked as superseded by / consolidated into RFC-037.
5. Update rfcs/README.md and handoff index links.
6. Add a short CHANGELOG entry only when release prep happens, not in this slice.
```

The preferred result is:

```text
no streaming APIs exist
no streaming examples exist
large-data guidance remains honest and external-tool-oriented
old RFC lifecycle state no longer suggests active near-term streaming implementation
future streaming work still requires a new implementation RFC
```

---

## 4. Non-Goals

This handoff does not authorize:

```text
[ ] Table::stream_csv
[ ] Tensor::stream_csv
[ ] CsvStream / BatchReader / AsyncCsvReader
[ ] matten-stream crate
[ ] streaming mode in matten-data
[ ] async runtime dependency
[ ] large-file benchmark suite
[ ] memory-budgeted batch reader
[ ] public API additions
[ ] new published dependencies
[ ] version bump or release prep
```

If streaming becomes desirable later, it must start from a new implementation
RFC that answers the batch model, schema policy, malformed-row policy,
memory-budget, sync/async, crate-placement, and error-model questions preserved
by RFC-026/RFC-037.

---

## 5. Candidate Guard

Add a focused script check:

```text
scripts/check-streaming-scope.sh
```

Do not overload `scripts/check-matten-data-scope.sh`. The RFC-042 dataframe
guard is intentionally scoped to `matten-data`, while the streaming boundary
also protects core `matten` from accidental APIs such as `Tensor::stream_csv`.

The new guard should cover all published crates:

```text
crates/*/src
crates/*/examples
```

The guard must be definition-scoped and filename-scoped, not prose-scoped. It
should reject public definitions that imply streaming support, for example:

```text
pub (async )?fn (stream_csv|large_csv_streaming)[[:space:]]*[(<]
pub (struct|enum|type) (CsvStream|BatchReader|AsyncCsvReader)\b
```

It should also reject published-crate example filenames with streaming-shaped
tokens, for example:

```text
stream_csv
csv_stream
batch_reader
async_csv
large_csv_streaming
```

The guard should not scan arbitrary Markdown or Rust comments for these words.
That keeps honest deferral docs from becoming false positives.

Allowed places for prose references:

```text
RFC records
handoffs
docs that explicitly say streaming is deferred or unsupported
guard script text
release notes describing the deferral
```

Do not reject every word `streaming`; the docs need to say "streaming is
deferred" honestly.

Wire the new guard into the same release path as the existing scope guard:

```text
.github/workflows/test.yaml
docs/src/contributing/release-checklist.md
```

---

## 6. Files Likely In Scope

RFC and handoff state:

```text
rfcs/done/026-large-csv-and-streaming-data-policy.md
rfcs/done/037-deferred-streaming-and-large-csv-policy.md
rfcs/README.md
rfcs/handoffs/README.md
```

Scope guard:

```text
scripts/check-streaming-scope.sh
scripts/check-release-docs.sh
.github/workflows/test.yaml
docs/src/contributing/release-checklist.md
```

Docs only if the audit finds a real gap:

```text
README.md
crates/matten-data/README.md
crates/matten-data/src/lib.rs
docs/src/examples/data.md
docs/src/reference/dynamic.md
docs/src/migration/playbooks/polars-and-pandas.md
```

---

## 7. Acceptance Criteria

The implementation is accepted when:

```text
[ ] reviewer agrees RFC-026/RFC-037 are policy-closure work, not streaming implementation
[ ] no public API or runtime behavior is added
[ ] no new published dependency is added
[ ] docs still direct large/streaming table workloads to dedicated tools
[ ] RFC-026 and RFC-037 lifecycle state is no longer misleading
[ ] RFC-037 is canonical for the resolved deferral policy
[ ] RFC-026 is explicitly marked superseded/consolidated into RFC-037
[ ] a definition-scoped guard catches new streaming API names across all published crates
[ ] a filename-scoped guard catches new streaming example names across all published crates
[ ] guard allows explicit deferral wording in docs/RFCs
[ ] release checklist or CI invokes the guard
[ ] mdBook/release-doc checks pass
```

Suggested verification:

```text
bash scripts/check-streaming-scope.sh
bash scripts/check-release-docs.sh
mdbook build docs
git diff --check
```

Run broader Rust checks only if source or manifest files change beyond guard
scripts/docs.

---

## 8. Review Questions

Please review:

```text
[ ] Is this the right next theme after 0.30.0?
[ ] Should RFC-037 become the canonical resolved deferral-policy record?
[ ] Should RFC-026 be moved to done as superseded/consolidated into RFC-037?
[ ] Is a conservative guard enough, or should this wait for a fuller streaming RFC rewrite?
[ ] Is the definition-scoped / filename-scoped guard mechanism precise enough?
[ ] Is all-published-crates scope the right guard boundary?
[ ] Does this preserve future streaming optionality without implying near-term support?
```
