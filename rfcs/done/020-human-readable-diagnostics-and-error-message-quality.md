# RFC-020: Human-Readable Diagnostics and Error Message Quality

**Status:** Implemented (v0.13.3)  
**Target:** v0.13.3  
**Theme:** Diagnostics polish  
**Depends on:** RFC-001, RFC-005, RFC-013  
**Related handoff:** `020-human-readable-diagnostics-and-error-message-quality-handoff.md`

## 1. Summary

This RFC defines a diagnostics quality standard for panic messages, `MattenError` display messages, parser errors, and dynamic unsupported-operation messages.

`matten` promises a DX-first experience. That promise is broken if users see vague, malformed, or spacing-damaged messages. This RFC turns error-message quality into an explicit release gate.

## 2. Goals

- Make errors readable and actionable.
- Normalize panic message style.
- Remove embedded spacing caused by wrapped strings.
- Add tests for representative diagnostics.
- Keep messages friendly without hiding technical facts.
- Preserve the panic-zone / Result-zone distinction.

## 3. Non-goals

- No localization/i18n.
- No rich diagnostic framework.
- No colored terminal output.
- No backtrace customization.
- No structured logging system.

## 4. External design

### 4.1 Message format

Panic-zone messages should follow:

```text
matten <category> error in <operation>: <what happened>; <what to do>
```

Examples:

```text
matten shape error in reshape: cannot reshape 6 elements into shape [4]
matten unsupported error in matmul: dynamic tensors do not support numeric matrix multiplication; call try_numeric() first
```

Boundary errors use `MattenError` and `Display`.

### 4.2 Message requirements

Every user-facing message should answer:

1. What operation failed?
2. Why did it fail?
3. What should the user do next, if obvious?

## 5. Data model

No data model change.

`MattenError` remains:

```rust
pub enum MattenError {
    Shape { operation: &'static str, message: String },
    Broadcast { left: Vec<usize>, right: Vec<usize> },
    Allocation { requested_elements: usize, message: String },
    Slice { input: Option<String>, message: String },
    Parse { format: DataFormat, message: String },
    Io { path: PathBuf, source: std::io::Error },
    Unsupported { operation: &'static str, message: String },
}
```

## 6. Data lifecycle

Diagnostics occur at:

- construction validation;
- reshape validation;
- broadcasting;
- slicing;
- numeric/dynamic boundary;
- JSON/CSV parsing;
- file loading;
- conversion to numeric.

Messages must be consistent across lifecycle stages.

## 7. Events and observable behavior

Observable diagnostic events:

- panic with message;
- `MattenError::Display`;
- `Debug` output for tensor;
- example failure output.

This RFC mainly controls text quality, not control flow.

## 8. Store access

No store changes.

However, store-access failures must be clear:

```text
matten unsupported error in as_slice: dynamic tensors do not expose numeric storage; call try_numeric() first
```

## 9. Public API requirements

No new public API is required.

Optional helper:

```rust
pub(crate) fn unsupported_dynamic(operation: &'static str) -> !;
```

Internal-only helpers may reduce message drift.

## 10. Cargo feature impact

No feature change.

Diagnostics must be tested under:

- default;
- no-default-features;
- all-features;
- dynamic.

## 11. Internal design

### 11.1 Avoid embedded indentation spaces

Bad:

```rust
"dynamic tensors cannot be serialized with the default serde                  implementation"
```

Good:

```rust
concat!(
    "dynamic tensors cannot be serialized with the default serde implementation; ",
    "call try_numeric() first"
)
```

or:

```rust
"dynamic tensors cannot be serialized with the default serde implementation; \
 call try_numeric() first"
```

but only if formatting does not introduce unwanted whitespace.

### 11.2 Snapshot-style tests

Add tests that assert important substrings, not necessarily entire messages.

Example:

```rust
let err = format!("{err}");
assert!(err.contains("parse"));
assert!(err.contains("csv"));
```

For panics:

```rust
let panic = catch_unwind(|| t.as_slice()).unwrap_err();
let message = panic_message(panic);
assert!(message.contains("dynamic tensors"));
assert!(message.contains("try_numeric"));
```

## 12. Examples

Modify examples:

- `12_boundary_error_handling.rs` should demonstrate clear `MattenError`.
- Dynamic examples should show one unsupported numeric call only if it helps teach the boundary.
- Avoid examples that normalize panics as routine control flow.

Add optional example:

```text
examples/14_readable_errors.rs
```

## 13. Acceptance criteria

- No obvious embedded spacing in user-facing messages.
- Dynamic unsupported messages consistently mention `try_numeric()`.
- Parser errors include format context.
- Shape errors include operation context.
- Boundary example demonstrates `Result`.
- Tests cover representative messages.

## 14. QA checklist

- [ ] grep for multi-space suspicious strings
- [ ] panic message tests
- [ ] `MattenError::Display` tests
- [ ] JSON parse error test
- [ ] CSV parse error test
- [ ] dynamic unsupported tests
- [ ] example output reviewed

## 15. Open questions

1. Should exact message text be part of public compatibility?
2. Should messages be snapshot-tested or substring-tested?
3. Should Japanese/other-language messages ever be supported? Current answer: no.
