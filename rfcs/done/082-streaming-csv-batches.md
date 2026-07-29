# RFC-082: Streaming CSV Batches for `matten-data`

**Status:** Implemented — reviewed and accepted; `CsvBatchReader` added to `matten-data` behind the
off-by-default `streaming` feature. `scripts/check-streaming-scope.sh` confirmed passing unmodified
against the real implementation, not just a scratch fixture. No version bump or release
**Target:** v0 feature work on the `0.x` line; release family undecided
**Theme:** Reopen streaming/large-CSV — deferred since RFC-026/RFC-037 — by answering the
questions RFC-037 made its precondition, and shipping the smallest useful slice
**Depends on:** RFC-022, RFC-026, RFC-030, RFC-032, RFC-033, RFC-037, RFC-042, RFC-059
**Related:** RFC-040, RFC-067, RFC-074, RFC-078, RFC-081

---

## 1. Summary

Add batched CSV reading to `matten-data`, behind a `streaming` feature flag:

```rust
pub struct CsvBatchReader { /* private */ }

impl CsvBatchReader {
    pub fn open(path: &Path, batch_rows: usize) -> Result<Self, MattenDataError>;
    pub fn next_batch(&mut self) -> Result<Option<Table>, MattenDataError>;
}
```

That is the whole public surface of the first slice: open a CSV, pull `Table` batches of a caller-chosen
row count until exhausted. Synchronous, single-pass, no resumability, no backpressure.

**RFC-037 deferred streaming until its §4 criteria were answered.** §4 below answers §4.1-§4.6. §5 states what is
deliberately excluded. This RFC is where the "should we?" question was already settled — RFC-037 said
*design-only until answered*, not *never* — so this is the answering, not a re-litigation.

**No release, version bump, tag, or publish is authorized by this RFC.**

## 2. Motivation

`matten-data` reads an entire CSV into memory. For the "boring step between a CSV and a `Tensor`" that is
usually right, and RFC-033/RFC-042 deliberately locked the scope there.

The gap it leaves is real and long-recorded: a file larger than available memory cannot be processed at all,
even when the *work* is streamable — computing column statistics, standardizing in chunks, or feeding a
train/test split. Today the answer is "use a different library," which for a file that merely doesn't fit is
a steep cliff.

This is the largest remaining consumer-visible gap in a published crate, and it has sat on the
remaining-themes list since RFC-026.

## 3. Placement decision — `matten-data`, feature-gated (not a new crate)

RFC-037 §4.6 lists three placement options — a `matten-data` streaming module, a separate `matten-stream`
crate, or not in the family — and closes with a standing recommendation:

> *"Recommendation: if streaming becomes real, consider a separate `matten-stream` companion."*

**This RFC does not follow that recommendation, and says so rather than quietly diverging.** RFC-037 wrote
it before `matten-data`'s type surface existed in its current form and before RFC-032/RFC-078 hardened the
companion-dependency rules. Those two developments are what change the answer.

**Decision: `matten-data`, behind an off-by-default `streaming` feature. No `matten-stream` crate.**

The decisive objection to `matten-stream` is structural, not economic:

**A `matten-stream` crate would have to return `Table`.** That is the type the whole family already uses for
tabular CSV data, and re-deriving batches into anything else would make streaming output incompatible with
every existing `matten-data` consumer. But `Table` lives in `matten-data`, so `matten-stream` would need a
**companion-to-companion dependency** — precisely what RFC-078 §6 ruled out when it specified
`matten-stats`'s boundary (*"no companion-to-companion dependency (no matten-mlprep, no matten-data)"*).
The alternatives are worse: duplicate `Table`/`CellValue`/`MattenDataError` into a second crate, or invent
a third shared type that both depend on. Neither is justified by one reader type.

| Option | Assessment |
|---|---|
| **New `matten-stream` crate** | **Rejected on structure.** Returning `Table` requires depending on `matten-data`, violating the companion-to-companion rule (RFC-078 §6); the alternatives duplicate types or add a third crate. Secondarily, RFC-078 showed a fifth crate's real cost — another `cargo package` target, guard scripts to teach, a maturity rung to argue — but that is supporting evidence, not the reason |
| **`matten-data`, unconditional** | Rejected. It would put a streaming API in front of every user of a crate whose whole selling point is being small |
| **`matten-data`, feature-gated** | **Chosen.** Zero cost when off; reuses `Table`, `CellValue`, `MattenDataError` rather than duplicating them; keeps CSV knowledge in the crate that already owns CSV; no new crate boundary to police |

**If streaming later outgrows one reader type** — async, multiple formats, backpressure — the
`matten-stream` question genuinely reopens, and RFC-037 §4.6's recommendation should be re-read at that
point rather than treated as settled by this RFC.

**Does this violate RFC-042's scope lock?** No. RFC-042 forbids `matten-data` becoming a *dataframe library* —
its guard names `groupby`, `join`, `merge`, `pivot`, `query`, `rolling`, `series`, `loc`, `iloc`. Batched
reading adds no relational or indexing operation; it changes *how much of a CSV is resident*, not what the
crate can do with it. The scope lock stays in force unchanged.

## 4. Answering RFC-037 §4.1-§4.6

**Numbering note.** This section's subsections do not line up one-to-one with RFC-037's, so the mapping is
stated explicitly rather than left to be inferred:

| RFC-037 §4 criterion | Answered in |
|---|---|
| §4.1 Batch Model | §4.1 below |
| §4.2 Schema Policy | §4.2 |
| §4.3 Malformed Row Policy | §4.3 |
| **§4.4 Memory Budget** | **§4.5** (all four of its sub-questions) |
| **§4.5 Sync vs Async** | **§4.4** |
| §4.6 Crate Placement | §3, plus §4.6 and §4.7 |

RFC-037's §1 bullet list splits placement into two items ("relationship to `matten-data`", "relationship to
a possible future `matten-stream`") while its §4.6 treats it as one; §4.6 and §4.7 below answer both
framings. Every criterion is covered exactly once.

### 4.1 Batch lifecycle

```text
one CsvBatchReader owns one open file handle for its lifetime
next_batch() reads up to batch_rows data rows and returns Some(Table)
at end of file it returns Ok(None), and every subsequent call also returns Ok(None)
the header row is read once at open() and applied to every batch
dropping the reader closes the file; there is no explicit close()
batches are independent Tables — the reader holds no cross-batch state beyond
  the file position and the header
```

A batch is an ordinary `Table`. Everything that already works on a `Table` works on a batch, unchanged.

### 4.2 Schema drift

```text
the header read at open() is authoritative for the whole file
a data row with a different field count is a malformed row (§4.3), not a schema change
column names and count never change mid-file
no schema re-inference, no union-of-schemas, no promotion
```

Streaming does not introduce schema evolution. That is deliberate: schema drift is a data-modelling
question, and answering it here would smuggle dataframe semantics past RFC-042.

### 4.3 Malformed-row policy

```text
default: fail fast — next_batch() returns Err(MattenDataError) naming the CSV line number,
  matching how Table::from_csv_path already reports malformed input
the failing batch is not returned; the reader is left unusable after an error
no skip-and-continue, no error-collection mode, no partial batches on error
```

Matching the existing non-streaming behaviour is the point: a user who moves from `from_csv_path` to
`CsvBatchReader` should not silently acquire different error semantics. A lenient mode is a separate
decision (§5).

**Two accepted exceptions, found during implementation review and recorded here rather than left
implicit** (both documented in `CsvBatchReader`'s own doc comments, and locked by dedicated tests):

```text
blank-but-not-empty file (NOT merely "whitespace-only" -- see the precise
  boundary below): Table::from_csv_path checks whether the WHOLE input trims
  to empty before parsing and returns EmptyInput. A file containing ONLY line
  terminators (e.g. "\n", "\n\n", "\r\n") trims to empty, so both paths agree:
  EmptyInput. Divergence requires at least one whitespace character other
  than a line terminator somewhere in the file (a space or tab being the
  common cases, but any other Unicode whitespace such as U+00A0 also counts;
  e.g. "   \n", a lone "\t") -- CsvBatchReader has no
  upfront whole-file check -- doing one would require buffering the file
  ahead of time, defeating the point of streaming -- so it parses the first
  line as a header record and reports Csv (empty column name) instead. The
  tempting fix (treat an all-empty header as EmptyInput) was evaluated and
  rejected: it would trade this divergence for a new one (",,\n1,2,3" would
  become EmptyInput where from_csv_path gives Csv). Accepted as-is on a
  degenerate input.

invalid UTF-8: Table::from_csv_path validates UTF-8 for the whole file upfront
  (via read_to_string) and reports invalid UTF-8 as Io, before returning any
  data. CsvBatchReader parses incrementally, so invalid UTF-8 is a mid-stream
  Csv error here, not Io -- and because parsing is incremental, one or more
  valid batches may already have been returned before a later batch hits the
  bad bytes. Csv is judged the more accurate classification for malformed
  content in either case; from_csv_path's Io is an artifact of reading the
  whole file through read_to_string first. Kept as-is; the divergence and its
  timing consequence are both documented on next_batch().
```

### 4.4 Sync vs async

```text
synchronous only
```

Async would pull an executor choice into a crate that currently has none, and would make
`matten-data`'s dependency graph a runtime-ecosystem decision. `std::io` blocking reads are adequate for
file-backed CSV. Async remains available as a future RFC if a real need appears; nothing here forecloses it.

### 4.5 Memory budget

RFC-037 §4.4 asks a streaming RFC to define four things. All four:

**Per-batch memory bound**

```text
the caller sets the bound by choosing batch_rows
resident memory is approximately: one batch of Tables + one line buffer + the header
the reader never buffers the whole file, and never accumulates across batches
batch_rows = 0 is rejected at open() with an explicit error
```

**Explicitly not promised:** a byte-denominated cap. Row width varies, so `batch_rows` bounds rows, not
bytes. Stating that honestly is better than implying a memory guarantee the type cannot keep.

**Maximum row width policy**

```text
none — no cap on field count or line length
```

A single pathological row can exceed any batch bound on its own, because `batch_rows` bounds the *number*
of rows, not their size. A 1-row batch of a 100 MB row is 100 MB resident. Imposing a width cap would mean
rejecting input that `Table::from_csv_path` accepts today, which would make the streaming path *less*
capable than the non-streaming one — the wrong trade. The limitation is stated rather than engineered
around; users with unbounded row widths need a different tool.

**String allocation policy**

```text
per-cell owned allocation, freed when the batch is dropped
no interning, no arena, no buffer reuse across batches
```

`Table`/`CellValue` own their strings today, and batches use the same construction path (§4.1), so
allocation behaviour is identical to `from_csv_path` — just bounded to one batch at a time. Reuse across
batches would require either a shared arena or borrowed cells, both of which would change `Table`'s type
contract for every existing user. Not worth it for a first slice.

**Output shape: `Table` per batch, not a full `Tensor` at the end**

Numeric conversion stays the caller's decision. A batch is an ordinary `Table`; the caller converts each
one, accumulates, or discards as they choose. Streaming conversion is explicitly out of scope (§5).

### 4.6 Relationship to `matten-data`

Answered in §3: it lives there, feature-gated, reusing the existing types.

### 4.7 Relationship to a possible `matten-stream`

Answered in §3: no such crate. If streaming later grows beyond one reader type — async, multiple formats,
backpressure — that would be the moment to revisit, via its own crate-boundary RFC.

## 5. Out of scope

```text
async / futures / executors
resumability, checkpointing, progress reporting
backpressure
parallel or multi-threaded reading
writing CSV in batches
formats other than CSV
skip-malformed / lenient modes / error collection
schema inference, drift handling, or union schemas
streaming numeric conversion (batch -> Table is enough; the caller converts)
a matten-stream crate
any change to Table::from_csv_path's existing behaviour
any dataframe operation (RFC-042's lock is untouched)
version bump, release prep, tag, publish
```

## 6. The `check-streaming-scope.sh` guard needs no change

`scripts/check-streaming-scope.sh` forbids streaming-shaped public API in published crates. It was written
to stop streaming appearing before an RFC authorized it, so it is the obvious thing to check — and an
earlier draft of this RFC wrongly asserted it would need narrowing.

**Tested: it already permits exactly this slice's surface, with no edit.** Its type pattern anchors the
forbidden names immediately after `pub struct`/`enum`/`type`:

```text
pub[[:space:]]+(struct|enum|type)[[:space:]]+(CsvStream|BatchReader|AsyncCsvReader)\b
```

`CsvBatchReader` begins `Csv`, so the `BatchReader` alternative never lines up with the anchor. A bare
`pub struct BatchReader` is still correctly forbidden, as are `CsvStream`, `AsyncCsvReader`, `stream_csv`,
`large_csv_streaming`, and their example-name equivalents — i.e. everything §5 defers stays guarded.

The guard is therefore **not modified by this RFC**. It remains in the verification set, and the reason it
passes is recorded here so a future streaming-shaped addition does not mistake "no change needed" for
"unmaintained."

## 7. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | **Additive, feature-gated** — nothing exists unless `streaming` is enabled |
| Runtime behavior | None with the feature off; `Table::from_csv_path` unchanged either way |
| Dependencies | **None new** — reuses the existing `csv` dependency already behind `matten-data`'s `csv` feature |
| Features | Adds `streaming` (off by default; implies `csv`) |
| MSRV | None (`1.85`) |
| Maturity labels | None — `matten-data` stays `production-ready candidate` |
| RFC-042 scope lock | Unchanged and still enforced |

## 8. Acceptance criteria

```text
[ ] RFC-037 §4.1-§4.6 answered explicitly, not deferred again (incl. all four §4.4 sub-questions)
[ ] the placement decision is argued against the alternatives, not asserted
[ ] the public surface is exactly CsvBatchReader::{open, next_batch}
[ ] no new dependency; streaming implies the existing csv feature
[ ] malformed-row behaviour matches Table::from_csv_path
[ ] batch_rows = 0 rejected at open()
[ ] the streaming guard is UNCHANGED and verified to still forbid everything in §5
[ ] RFC-042's dataframe scope lock is untouched and still passes
[ ] no version bump, release, tag, or publish
```

## 9. Non-goals

```text
[ ] a matten-stream crate
[ ] async support
[ ] making streaming the default path
[ ] promoting matten-data
[ ] relaxing RFC-042
[ ] any 1.0 activity — RFC-076 stays deferred and this RFC does not touch it
```

## 10. Follow-up

If accepted and implemented, the natural next questions are async (needs a real use case, not speculation)
and streaming numeric conversion. Both require their own RFC. The `streaming` feature staying off by default
means neither is urgent.
