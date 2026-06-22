# RFC-037 Developer Handoff: Deferred Streaming and Large CSV Policy

**Project:** `matten`  
**RFC:** RFC-037  
**Handoff Kind:** Policy / Future Reopening Handoff  
**Implementation Level:** Deferral enforcement required  
**Status:** Draft handoff for developer review  
**Applies To:** v0.20+ planning and implementation sequence  

---

## 0. Handoff Summary

This document turns RFC-037 into developer-executable work. It is not a replacement for the RFC. The RFC remains the design authority; this handoff translates it into implementation phases, PR boundaries, checks, and acceptance criteria.

## 1. Implementation Handoff

RFC-037 is a deferral RFC. Its implementation is mostly negative: prevent streaming from entering v0.20.

Developers should add documentation and release checks that make the deferral visible.

---

## 2. Internal Design

No streaming internal design should be implemented.

Add placeholders only as comments/docs, not public APIs.

Forbidden public APIs:

```rust
Table::stream_csv(...)
CsvStream
BatchReader
AsyncCsvReader
Tensor::stream_csv(...)
```

---

## 3. Task Breakdown / PR Plan

### PR-037-1: Documentation deferral

Add docs stating:

```text
Streaming and large CSV support are deferred.
Use dedicated CSV/dataframe/streaming crates for large data.
```

Acceptance:

```text
[ ] README does not imply large-data support
[ ] mdBook defers streaming
[ ] no streaming example exists
```

### PR-037-2: Release-doc guard

Add check that public examples do not contain:

```text
stream_csv
CsvStream
BatchReader
AsyncCsvReader
large_csv
big_csv
```

Acceptance:

```text
[ ] check fails if streaming example appears
[ ] check allows RFC-037 docs to mention streaming as deferred
```

### PR-037-3: Future RFC template

Add a template or section in docs for future streaming RFC requirements:

- batch model;
- schema policy;
- malformed row policy;
- memory budget;
- sync/async decision;
- crate placement;
- threat model update.

Acceptance:

```text
[ ] future reopening criteria recorded
[ ] no implementation code added
```

---

## 4. Acceptance / QA Checklist

```text
[ ] no streaming APIs exported
[ ] no streaming modules publicly reachable
[ ] no async runtime dependency
[ ] no large-data claims in README
[ ] future reopening criteria documented
[ ] security risks listed
```

CI:

```bash
cargo check --workspace --all-features
bash scripts/check-release-docs.sh
bash scripts/check-core-dependency-boundary.sh
```

---

## 5. Future Reopening Checklist

A future streaming RFC must answer:

```text
[ ] batch model
[ ] schema drift policy
[ ] malformed row policy
[ ] memory budget
[ ] sync vs async
[ ] crate location
[ ] error model
[ ] cancellation/partial failure behavior
[ ] threat model update
```

Until then, the correct answer to streaming requests is:

```text
defer
```
