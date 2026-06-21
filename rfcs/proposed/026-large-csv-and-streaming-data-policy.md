# RFC-026: Large CSV and Streaming Data Policy

**Status:** Proposed  
**Target:** design spike no earlier than v0.20+; implementation later only after separate approval  
**Theme:** Large data and streaming boundary  
**Depends on:** RFC-018, RFC-022, RFC-023

---

## 1. Summary

This RFC defines the policy for large CSV and streaming data support.

Streaming remains design-only until the project has explicit answers for batch lifecycle, schema drift, malformed rows, memory budget, and sync-vs-async strategy.

Streaming must not enter core `matten` as `Tensor::stream_csv`.

---

## 2. Goals

- Prevent misleading large-data claims.
- Keep core `matten` in-memory and simple.
- Identify what a future streaming design must prove.
- Clarify relationship to `matten-data` or a possible `matten-stream` crate.

---

## 3. Non-goals

- No implementation in v0.16/v0.17/v0.18.
- No streaming API in core `matten`.
- No out-of-core tensor store.
- No async pipeline until sync semantics are proven.
- No replacement for Polars/DataFusion.

---

## 4. Current honest statement

`matten` is intended for small-to-medium PoC workloads.

For huge CSV/table workloads, users should use appropriate external tools, then convert to `Tensor` when a small numeric matrix is needed.

---

## 5. Future design questions

A future implementation RFC must answer:

- What is a batch?
- What happens on schema drift?
- Are malformed rows fail-fast, skipped, or collected?
- How is memory budget enforced?
- Is the API sync-only first?
- Does streaming belong in `matten-data` or `matten-stream`?
- How are dynamic cleanup and numeric conversion applied per batch?

---

## 6. Possible future API

Illustrative only:

```rust
use matten_stream::CsvBatches;

for batch in CsvBatches::open("large.csv")?.batch_size(1024) {
    let x = batch?.try_numeric()?;
    process(x);
}
```

This must not be added before a dedicated implementation RFC.

---

## 7. Relationship to `matten-data`

Streaming may eventually belong in:

```text
matten-data streaming mode
```

or:

```text
matten-stream
```

The decision is deferred. Do not force it into v0.16.

---

## 8. Acceptance criteria for any future implementation

- Explicit memory budget.
- Explicit batch lifecycle.
- Explicit malformed-row policy.
- Explicit schema consistency policy.
- Tests for bounded memory behavior.
- No core dependency pollution.
- No claim that core `matten` is a large-data engine.
