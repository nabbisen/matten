# RFC-026: Large CSV and Streaming Data Policy

**Status:** Proposed  
**Target:** v0.16+ design, v0.20+ exploration  
**Theme:** Large data boundary  
**Depends on:** RFC-018, RFC-022, RFC-023  
**Related handoff:** `026-large-csv-and-streaming-data-policy-handoff.md`

## 1. Summary

This RFC defines the policy for large CSV and streaming data support.

`matten` core is not currently a large-data engine. It should not claim to handle huge datasets “without breaking a sweat” until there is an explicit memory and streaming design. This RFC keeps that future open while preventing premature scope expansion.

## 2. Goals

- Define what large-data support would require.
- Prevent misleading core claims.
- Decide whether streaming belongs in core or companion crate.
- Align with resource safety limits.
- Preserve the simple numeric tensor API.

## 3. Non-goals

- No immediate streaming implementation.
- No out-of-core tensor store.
- No parquet/arrow support in this RFC.
- No async file pipeline.
- No distributed processing.
- No replacement for polars/datafusion.

## 4. External design

Current honest statement:

```text
matten is intended for small-to-medium PoC workloads.
For large datasets, use explicit limits and consider future companion crates.
```

Future possible API in companion crate:

```rust
use matten_stream::CsvBatches;

for batch in CsvBatches::open("large.csv")?.batch_size(1024) {
    let tensor = batch?.try_numeric()?;
    process(tensor);
}
```

This is illustrative only.

## 5. Data model

Core tensor remains in-memory and contiguous for numeric data.

Streaming support would use batch tensors:

```text
large input
  -> batch 1 Tensor
  -> batch 2 Tensor
  -> ...
```

No single huge logical tensor is promised.

## 6. Data lifecycle

Current:

```text
CSV input -> parse all -> Tensor
```

Future streaming:

```text
CSV input -> parse batch -> Tensor -> user processing -> next batch
```

Batching changes lifecycle substantially, so it should not be hidden inside current `load_csv`.

## 7. Events and observable behavior

Streaming events would include:

- open file;
- read batch;
- parse row;
- convert batch;
- handle parse error;
- finish stream.

These are outside current core.

## 8. Store access

Core store remains in-memory.

Future streaming crate should not access core internals. It should construct normal tensors per batch.

## 9. Public API policy

`matten` core should not add:

```rust
Tensor::stream_csv(...)
```

without a dedicated implementation RFC.

Acceptable in core:

- resource limits;
- clear docs;
- small CSV loaders;
- examples showing limits.

Future companion:

```text
matten-stream
matten-data streaming mode
```

## 10. Cargo feature impact

No core feature now.

Avoid:

```toml
streaming = [...]
```

in core until design is accepted.

## 11. Internal design considerations for future

### 11.1 Batch size

Must be explicit.

### 11.2 Error policy

Streaming parse errors need policy:

- fail fast;
- skip bad rows;
- collect errors.

This belongs in companion design, not core.

### 11.3 Schema consistency

Streaming batches need consistent shape/schema. This is table-like and likely belongs with `matten-data`.

## 12. Examples

No core runnable examples until streaming exists.

Core docs may include warning:

```text
For huge CSV files, prefer external streaming tools or future companion crates.
```

Future companion examples:

```text
examples/stream_numeric_csv_batches.rs
examples/stream_dynamic_cleanup_batches.rs
```

## 13. Acceptance criteria for future design

- Explicit memory budget model.
- Explicit batch lifecycle.
- No hidden huge allocation.
- No core dependency pollution.
- Clear relationship to `matten-data`.
- Honest docs about limitations.

## 14. QA checklist for future implementation

- [ ] large synthetic CSV smoke test
- [ ] bounded RSS check
- [ ] malformed row policy tests
- [ ] schema drift tests
- [ ] batch shape tests
- [ ] no core API pollution

## 15. Open questions

1. Should streaming live in `matten-data` or `matten-stream`?
2. Should streaming be sync-only first?
3. Should schema inference require reading the whole file, or use a prefix sample?
