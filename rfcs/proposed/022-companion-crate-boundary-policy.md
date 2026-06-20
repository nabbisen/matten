# RFC-022: Companion Crate Boundary Policy

**Status:** Proposed  
**Target:** v0.16+  
**Theme:** Ecosystem extension boundary  
**Depends on:** RFC-015, RFC-016, RFC-021  
**Related handoff:** `022-companion-crate-boundary-policy-handoff.md`

## 1. Summary

This RFC defines the boundary between `matten` core and future `matten-*` companion crates.

The core crate should remain Sedan-first: a simple numeric tensor library with an optional dynamic ingestion on-ramp. Heavier SUV-like workflows should grow as companion crates rather than contaminating the core API.

## 2. Goals

- Protect `matten` core simplicity.
- Provide a path for ecosystem expansion.
- Define what belongs in core vs companion crates.
- Prevent dataframe/ML/bridge dependencies from entering core prematurely.
- Establish naming and dependency rules for companion crates.

## 3. Non-goals

- This RFC does not implement companion crates.
- This RFC does not commit all future crates to release.
- This RFC does not add table semantics to core.
- This RFC does not add external dependencies to core.

## 4. External design

### 4.1 Core rule

```text
If a feature helps construct, inspect, clean, or explicitly convert a Tensor,
it may belong in matten.

If it introduces table semantics, domain semantics, pipeline semantics, or
external framework dependencies, it belongs in a companion crate.
```

### 4.2 Candidate companion crates

```text
matten-data
matten-mlprep
matten-ndarray
matten-nalgebra
matten-candle
matten-stream
```

These names are provisional.

## 5. Data model

Core data model remains:

```text
Tensor
Element
MattenError
DataFormat
```

Companion crates should consume and produce `matten::Tensor`, not duplicate storage.

## 6. Data lifecycle

Core:

```text
input -> Tensor -> computation
```

Dynamic on-ramp:

```text
messy input -> dynamic Tensor -> cleanup -> try_numeric -> Tensor
```

Companion:

```text
domain-specific workflow -> Tensor boundary -> matten computation
```

## 7. Events and observable behavior

Companion crates may release independently, but must not require breaking core changes without an RFC.

## 8. Store access

Companion crates should use public APIs only.

They must not rely on internal storage layout.

If they need efficient access, the core should expose a small accepted public API rather than leaking internals.

## 9. Public API requirements

No core API change required.

Future companion crates should depend on `matten` through normal public API.

## 10. Cargo feature impact

Core `matten` must not gain features like:

```text
dataframe
mlprep
ndarray
nalgebra
candle
streaming
```

unless a later RFC proves that in-core feature is small, optional, and does not harm compile time.

## 11. Internal design

### 11.1 Workspace strategy

Future repository layout may be:

```text
crates/
  matten/
  matten-data/
  matten-mlprep/
  matten-ndarray/
  matten-nalgebra/
  matten-candle/
```

But separate repositories are also acceptable if release cadence differs.

### 11.2 Dependency direction

Allowed:

```text
matten-data -> matten
matten-mlprep -> matten
matten-candle -> matten
```

Not allowed:

```text
matten -> matten-data
matten -> candle
matten -> nalgebra
```

## 12. Examples

Core examples should not use nonexistent companion crates.

When companion crates exist, examples may live in those crates.

Core may include docs saying:

```text
For table-like workflows, see future matten-data.
```

but should avoid broken code examples.

## 13. Acceptance criteria

- Companion boundary is documented.
- Core non-goals include dataframe/ML/bridge scope.
- No heavy companion dependency is added to core.
- Future RFCs reference this boundary.
- README positioning remains Sedan-first.

## 14. QA checklist

- [ ] Core Cargo.toml dependency scan
- [ ] README scope check
- [ ] mdBook scope check
- [ ] Examples do not reference nonexistent crates
- [ ] Future crate names marked provisional

## 15. Open questions

1. Should companion crates share a workspace immediately?
2. Should `matten-data` be first, or should bridge crates come first?
3. Should companion crates use the same RFC lifecycle?
