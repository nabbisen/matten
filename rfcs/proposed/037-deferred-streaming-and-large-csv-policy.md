# RFC-037: Deferred Streaming and Large CSV Policy

**Status:** Proposed  
**Target Release:** v0.20.0 policy; implementation deferred  
**Related:** RFC-033, RFC-034, RFC-035, RFC-036  
**Scope:** Explicit deferral and reopening criteria for streaming / large CSV

---

## 1. Summary

This RFC states that streaming and large CSV processing are **not** part of v0.20 `matten-data` implementation.

They remain design-only until the project answers:

- batch lifecycle;
- schema drift;
- malformed-row policy;
- sync vs async;
- memory budget;
- relationship to `matten-data`;
- relationship to a possible future `matten-stream`.

---

## 2. Motivation

Streaming is attractive but high risk.

Once a crate supports streaming, users expect:

- large file support;
- memory guarantees;
- partial failure policy;
- backpressure or async behavior;
- chunked conversion;
- schema stability across batches;
- resumability or progress reporting.

That is far beyond the v0.20 goal of small table-to-Tensor conversion.

---

## 3. Decision

v0.20 `matten-data` MUST NOT implement:

```rust
Table::stream_csv(...)
Tensor::stream_csv(...)
CsvStream
BatchReader
AsyncCsvReader
```

v0.20 may mention streaming only as a future non-goal.

---

## 4. Reopening Criteria

Streaming may be reopened only after an RFC answers all of the following.

### 4.1 Batch Model

What is a batch?

```text
fixed row count
fixed byte count
schema-driven chunk
caller-pulled iterator
async stream item
```

### 4.2 Schema Policy

What happens when later rows do not match earlier inferred schema?

Options:

```text
fail fast
promote column kind
store mixed values
return row-level errors
require explicit schema upfront
```

### 4.3 Malformed Row Policy

Options:

```text
fail entire stream
skip row with error callback
collect row errors
stop after error limit
```

v0.20 avoids this entirely by requiring in-memory rectangular input.

### 4.4 Memory Budget

A streaming RFC must define:

- per-batch memory bound;
- maximum row width policy;
- string allocation policy;
- whether output is Tensor per batch or full Tensor at end.

### 4.5 Sync vs Async

Do not add async casually.

Questions:

- Is sync iterator enough?
- Does async add dependency and runtime complexity?
- Can async be implemented without forcing Tokio?
- Should async live in a different crate?

### 4.6 Crate Placement

Options:

```text
matten-data streaming module
matten-stream separate crate
not in matten family
```

Recommendation: if streaming becomes real, consider a separate `matten-stream` companion.

---

## 5. Security / Reliability

Streaming has additional risks:

- denial of service by huge rows;
- unbounded string growth;
- partial conversion inconsistency;
- silent row skipping;
- schema drift bugs;
- async cancellation problems.

These require threat-model review before implementation.

---

## 6. Documentation Rule

Docs may say:

> Streaming and large CSV support are deferred. Use a dedicated CSV/dataframe/streaming crate for large data workflows.

Docs must not imply:

```text
matten-data can process arbitrary huge CSVs.
```

---

## 7. Acceptance Criteria

This RFC is accepted when:

```text
[ ] v0.20 docs explicitly defer streaming
[ ] no streaming APIs exist in matten-data
[ ] future reopening criteria are recorded
[ ] large-data claims are absent from README/examples
[ ] use another crate guidance is documented
```

---

## 8. Future RFC Template

A future streaming RFC must include:

```text
1. batch model
2. schema policy
3. malformed row policy
4. memory budget
5. sync/async choice
6. crate placement
7. error model
8. examples
9. non-goals
10. threat model update
```
