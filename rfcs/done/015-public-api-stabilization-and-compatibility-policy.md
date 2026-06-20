# RFC-015: Public API Stabilization and Compatibility Policy

**Status:** Implemented (v0.13.3)  
**Target:** v0.13.3  
**Theme:** Sedan-first stabilization  
**Depends on:** RFC-002, RFC-003, RFC-005, RFC-008, RFC-013  
**Related handoff:** `015-public-api-stabilization-and-compatibility-policy-handoff.md`

## 1. Summary

This RFC defines the public API stabilization policy for `matten` after v0.13.2.

The project is now useful enough that accidental public API drift becomes more dangerous than missing features. `matten` must remain a simple, DX-first numeric tensor crate with a carefully bounded dynamic on-ramp. This RFC freezes the core public surface rules, clarifies root exports, introduces public API review gates, and defines how hidden helper traits such as slice conversion plumbing are allowed to exist without becoming user-facing commitments.

## 2. Goals

- Keep `matten` easy to understand and hard to misuse.
- Prevent accidental root exports and public trait leakage.
- Keep the default user story centered on `Tensor`.
- Clarify how hidden/public-but-not-user-facing API items are handled.
- Establish release gates for public API changes.
- Preserve SemVer-compatible evolution during pre-1.0 development.

## 3. Non-goals

- This RFC does not freeze `matten` as stable 1.0.
- This RFC does not add new mathematical operations.
- This RFC does not promote dynamic tensors to a peer computation engine.
- This RFC does not introduce dataframe, ML, or bridge-crate APIs.

## 4. External design

### 4.1 User-facing model

The normal user should still begin with:

```rust
use matten::Tensor;
```

Advanced users may also use:

```rust
use matten::{DataFormat, MattenError};

#[cfg(feature = "dynamic")]
use matten::Element;
```

The public API should feel narrow, direct, and discoverable.

### 4.2 Root export policy

Allowed root exports:

```rust
pub use crate::error::{DataFormat, MattenError};
pub use crate::slice::SliceBuilder;
pub use crate::tensor::Tensor;

#[cfg(feature = "dynamic")]
pub use crate::dynamic::Element;
```

Conditionally allowed hidden exports:

```rust
#[doc(hidden)]
pub use crate::slice::{IntoSliceRange, SliceConvert, SliceSpecRepr};
```

These hidden exports are permitted only if they are required by public method signatures such as:

```rust
pub fn range<R: IntoSliceRange>(self, range: R) -> Self;
```

If the builder API is redesigned to avoid such generic public bounds, hidden slice exports should be removed.

### 4.3 Public API categories

| Category | Example | Stability policy |
|---|---|---|
| Primary API | `Tensor`, `Tensor::new`, `Tensor::matmul` | Strongly protected |
| Boundary API | `load_json`, `load_csv`, `MattenError` | Strongly protected |
| Dynamic on-ramp API | `Element`, `try_numeric`, `fill_none` | Protected but feature-gated |
| Builder API | `SliceBuilder` | Protected |
| Hidden plumbing | `IntoSliceRange`, `SliceConvert` | Allowed only as `#[doc(hidden)]` |
| Internal API | storage modules, parser helpers | Not public |

## 5. Data model impact

No storage layout changes are required.

This RFC affects the public contract around the existing data model:

- numeric `Tensor` remains the default computation object;
- dynamic storage remains feature-gated;
- `Element` remains public only with `dynamic`;
- conversion and parsing errors remain represented through `MattenError`.

## 6. Data lifecycle impact

No lifecycle change is introduced.

However, the lifecycle boundary must be reflected in the public API:

```text
external input
  -> boundary API returning Result<Tensor, MattenError>
  -> numeric Tensor computation
  -> optional serialization / conversion
```

For dynamic:

```text
messy input
  -> dynamic Tensor
  -> inspect / fill / clean
  -> explicit try_numeric()
  -> numeric Tensor computation
```

The public API must not imply that dynamic tensors support all numeric computation directly.

## 7. Events and observable behavior

This is a library crate, so there is no runtime event bus. Observable public API events are:

- a new public item appears;
- a public item disappears;
- a public item changes signature;
- a hidden item becomes visible in docs;
- an error variant or field changes;
- a feature-gated API changes availability;
- an example begins relying on an unaccepted API.

All such events must be reviewed before release.

## 8. Store access

No persistent store is introduced.

“Store access” in this RFC means access to internal storage:

- numeric storage access via `as_slice`, `to_vec`, `into_vec`;
- dynamic storage access via `to_elements`;
- conversion from dynamic to numeric via `try_numeric`.

Public API stabilization requires that storage accessors be explicit about dynamic behavior. Numeric accessors must reject dynamic tensors rather than expose empty internal numeric buffers.

## 9. Public API requirements

### REQ-015-001: Root export allowlist

The root module must export only approved user-facing items.

Mandatory root exports:

```rust
Tensor
MattenError
DataFormat
SliceBuilder
```

Feature-gated root export:

```rust
Element // only with feature = "dynamic"
```

Hidden root exports may exist only for public signature compatibility.

### REQ-015-002: Hidden slice traits

If `SliceBuilder::range` uses `IntoSliceRange`, the trait must not be private in a public method signature.

Acceptable options:

1. Root-export as `#[doc(hidden)]`.
2. Implement a true sealed trait pattern.
3. Redesign builder methods to avoid public generic trait bounds.

### REQ-015-003: Public API snapshot

The repository must include a public API snapshot or equivalent documented public surface.

The snapshot must be updated when:

- a public item is added;
- a public item is removed;
- a public signature changes;
- a feature gate changes public availability.

### REQ-015-004: Examples must not invent API

Examples may demonstrate accepted public API only. If an example needs new API, it must first link to an accepted RFC.

### REQ-015-005: Versioned docs must avoid stale exact versions

User-facing docs should prefer:

```text
active pre-1.0 development
```

over hard-coded patch status strings.

Cargo snippets may use the current minor version, but must be updated as part of release preparation.

## 10. Cargo feature impact

This RFC does not add features.

The current matrix remains:

```toml
[features]
default = ["serde", "json", "csv"]
serde = ["dep:serde"]
json = ["serde", "dep:serde_json"]
csv = ["dep:csv"]
dynamic = []
```

Public API snapshots must be checked at least for:

```bash
cargo check --no-default-features
cargo check
cargo check --all-features
```

## 11. Internal design

### 11.1 Sealed trait recommendation

If `IntoSliceRange` remains public-hidden, it should be sealed:

```rust
mod sealed {
    pub trait Sealed {}
}

pub trait IntoSliceRange: sealed::Sealed {
    #[doc(hidden)]
    fn into_repr(self) -> SliceSpecRepr;
}
```

Downstream users should not be able to implement slice plumbing traits accidentally.

### 11.2 Public API audit script

Add an `xtask` or documented command to audit public exports.

Minimum acceptable gate:

```bash
cargo check --all-targets --all-features
cargo clippy --all-targets --all-features -- -D warnings
```

Recommended optional gate:

```bash
cargo public-api --all-features
```

if the project accepts that dependency/tool.

## 12. Examples and documentation

Update:

- README public positioning;
- mdBook public API snapshot;
- slice builder docs;
- compatibility docs;
- changelog wording.

No new example is required by this RFC, but all existing examples must be checked for unaccepted public APIs.

## 13. Acceptance criteria

- Root exports match the allowlist.
- `DataFormat` remains explicitly sanctioned.
- Hidden slice traits do not trigger `private_bounds` under strict warnings.
- Public API snapshot matches actual exports.
- README does not overclaim dynamic completeness.
- Examples compile against accepted APIs only.
- No stale exact version status remains in prominent docs.

## 14. QA checklist

- [ ] `cargo fmt --all --check`
- [ ] `cargo clippy --all-targets --all-features -- -D warnings`
- [ ] `cargo test --all-targets`
- [ ] `cargo test --doc --all-features`
- [ ] Public API snapshot reviewed
- [ ] README and mdBook version/status scan completed
- [ ] Examples checked for hidden API invention

## 15. Open questions

1. Should `cargo public-api` become mandatory CI, or remain a maintainer tool?
2. Should hidden slice traits be sealed immediately, or only documented as hidden plumbing in v0.13.3?
3. Should `SliceBuilder` be simplified later to avoid helper traits entirely?
