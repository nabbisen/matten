# RFC-032: Companion Dependency and Import Convention

**Status:** Implemented (v0.19.2)
**Target:** Documentation/convention; small README + release-doc-script work in a future patch (no source changes to crate logic)
**Theme:** Companion crate ergonomics and public API identity
**Depends on:** RFC-022 (companion architecture and boundary), RFC-030 (lock-step family versioning), RFC-031 (feature-robust dynamic rejection)
**Relates to:** Architect review request "matten Companion Dependency Style" (v0.1)

---

## 1. Summary

Codify the **user-facing dependency and import convention** for the `matten` crate
family. The canonical style is **explicit dependencies**: users depend on `matten`
*and* each companion crate they use, and always import `Tensor` (and other core
types) from `matten`:

```toml
[dependencies]
matten = "0.19"
matten-ndarray = "0.19"
```

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
```

Companion crates **must not** re-export core `matten` types as if they owned them.
The single-dependency convenience pattern (a whole-crate `pub use matten;`) is
**deliberately deferred** — not adopted now, addable additively later if real demand
appears.

This RFC records an existing practice as policy (the family already ships in this
style) and adds two small guardrails so it does not drift.

---

## 2. Motivation

`matten` is now a family: core `matten` owns `Tensor`; `matten-ndarray` bridges to
`ndarray`; `matten-mlprep` provides preprocessing; a future `matten-data` (RFC-023)
may help users reach `Tensor`. Companions necessarily depend on core because their
APIs operate on `matten::Tensor`.

The open question was never whether companions depend on core (they must), but the
**user-facing style**:

- **Explicit** — users declare `matten` plus companions, import `Tensor` from `matten`.
- **Single-dependency** — users declare only a companion, which re-exports `matten`
  so `Tensor` is reachable through the companion.

Left unspecified, examples and READMEs could drift between styles, and a companion
might accumulate re-exports until it becomes a partial facade over core — eroding the
ownership model, complicating feature selection, and increasing the documentation and
compatibility surface.

---

## 3. Decision

### 3.1 Canonical user-facing style (adopted)

Official documentation, READMEs, rustdoc, mdBook, and examples use **explicit
dependencies** and import core types from `matten`:

```toml
[dependencies]
matten = "0.19"
matten-ndarray = "0.19"   # or matten-mlprep, etc.
```

```rust
use matten::Tensor;
use matten_ndarray::to_arrayd;
use matten_mlprep::standardize_columns;
```

When a downstream needs a core feature, it is enabled on the core dependency directly:

```toml
matten = { version = "0.19", features = ["dynamic"] }
matten-ndarray = "0.19"
```

### 3.2 Forbidden: broad core-type re-exports (adopted)

Companion crates **must not** re-export individual core types:

```rust
// FORBIDDEN in companion crates
pub use matten::Tensor;
pub use matten::MattenError;
pub use matten::Element;
pub use matten::NumericPolicy;
pub use matten::MattenLimits;
pub use matten::DataFormat;
```

These create a partial facade, blur ownership (`use matten_ndarray::Tensor` implies the
companion owns `Tensor`), and impose a synchronized-documentation burden.

### 3.3 Deferred: whole-crate `pub use matten;` convenience (not adopted now)

The *less harmful* single-dependency form is a whole-crate re-export:

```rust
// Companion crate (NOT adopted in this RFC)
pub use matten;
// enabling: use matten_ndarray::matten::Tensor;
```

This is **deferred**, not forbidden in principle, for these reasons:

- The usual justification for re-exporting a dependency is **proc-macro hygiene** (so
  macro-generated code can name the dependency). `matten` ships no macros, so that
  rationale does not apply here.
- It is **permanent public surface**: once shipped it cannot be removed without a
  breaking (minor, under the 0.x rule) bump, so introducing it is a one-way door.
- It forces documentation to perpetually disclaim a second, non-canonical import path.
- For a bridge crate the user needs `Tensor` and its methods regardless, so the
  convenience only benefits trivial pass-through code — marginal value for a permanent
  commitment.

Because it is purely additive, it can be introduced later — by amending this RFC — the
moment there is demonstrated user demand, with zero cost to waiting.

### 3.4 Canonical import for core types

`Tensor` and every other core type are canonically imported from `matten`:

```rust
use matten::Tensor;
```

---

## 4. Rationale

- **Ecosystem precedent.** The closest analogues — `ndarray-stats`, `ndarray-linalg`,
  and `serde` / `serde_json` — all use explicit two-dependency style and do not
  re-export the core's primary types. `use matten::Tensor` is the convention Rust
  users already expect from an extension crate.
- **Feature control (ties to RFC-031).** A companion-only dependency would force each
  companion to forward every core feature (`dynamic`, `json`, `csv`) so users can reach
  them — re-creating the feature-fragility class RFC-031 hardened against. An explicit
  core dependency gives users one unambiguous knob.
- **Ownership clarity.** `Tensor` is owned by `matten`. Keeping its import path on
  `matten` preserves a clean mental model: core owns the type; companions add focused
  workflows around it.
- **Maturity-label clarity.** Under lock-step versioning (RFC-030) crates share one
  version but differ in maturity (core stable pre-1.0; `matten-ndarray`
  production-ready candidate; `matten-mlprep` beta). Per-crate import paths keep each
  crate's maturity attached to its own surface.
- **Surface minimalism (RFC-002).** Not adding `pub use matten;` keeps each companion's
  public API minimal and avoids a one-way commitment.

---

## 5. Scope: what changes vs. what is already true

Already conformant (no change required):

- No companion re-exports `matten` or any core type (`src/lib.rs` exports only each
  crate's own functions and error type).
- All companion examples and both companion READMEs already use
  `use matten::Tensor;` plus `use matten_<companion>::<fn>;`.
- The dependency-boundary documentation already states "companions depend on `matten`;
  `matten` does not depend on companions."

Net-new work introduced by this RFC (small, doc/tooling only):

1. A one-line note in each companion README explaining the explicit-import convention.
2. A release-doc-script guardrail enforcing the convention.
3. This RFC, recording the convention and the deferred-convenience decision.

### 5.1 Scope clarification — workspace-excluded internal tooling (architect ruling, 2026-06-24)

RFC-032 governs the **published, user-facing** `matten` crate family and the companion
crates intended for users. **Workspace-excluded, `publish = false` internal tooling — such
as regression fixtures (`tests/fixtures/dynamic_rejection_unification`) and the RFC-049
benchmark harness (`benchmarks/`) — is outside this convention's packaging scope.** Such
tooling may use path-only dependencies, a placeholder version (`0.0.0`), and a targeted
core-feature passthrough (e.g. `dynamic = ["matten/dynamic"]`) without being treated as a
published companion.

Such tooling should still follow the **ownership-clarity spirit** where practical: do not
re-export core `matten` types (`pub use matten::Tensor;`), and import core types from
`matten` directly (`use matten::Tensor;`). Gratuitous forwarding of *every* core feature is
still discouraged, but a single targeted passthrough used for benchmarking/testing is fine.

The RFC-032 release-doc guard is intentionally **not** extended to scan `benchmarks/` or
`tests/fixtures/`, to keep published-family policy and internal-tooling policy from blurring.
Peer/benchmark dependency containment for those excluded crates is instead proven by the
separate published-crate dependency isolation guard
(`scripts/check-published-dependency-isolation.sh`, RFC-049 §B1).

---

## 6. Compatibility

| Dimension | Impact |
|---|---|
| SemVer | None — documentation/convention only; no public API change |
| Source/logic | No changes to any crate's Rust logic |
| Release label | Rides any future patch (e.g., bundled with the next doc/maintenance release) |
| Downstream | No action required |

---

## 7. Acceptance criteria

- Official examples import `use matten::Tensor;`.
- Companion examples import functions via `use matten_<companion>::<fn>;`.
- No companion broadly re-exports core types (§3.2).
- No whole-crate `pub use matten;` is present (deferred, §3.3).
- Each companion README carries a one-line convention note.
- `scripts/check-release-docs.sh` enforces §3.2 and §3.4 and fails on violations.
- The dependency-boundary documentation continues to state the one-directional rule.
- This RFC moves to `rfcs/done/` (Status: Implemented) when the README note and script
  guard ship.

---

## 8. Handoff to implementer

This is a documentation/tooling change only. **Do not modify any crate's Rust logic,
public exports, or `Cargo.toml` dependency lines.** Follow the project workflow:
complete all edits → run `cargo fmt --all` once (without reviewing the formatted
output) → run the full verification suite.

### 8.1 README convention note (both companions)

Add a short note to `crates/matten-ndarray/README.md` and
`crates/matten-mlprep/README.md`, placed immediately after the Quick Start dependency
snippet. Use this wording (adjust the crate name per file):

> **Dependency style.** This crate depends on `matten`, but official examples import
> `Tensor` (and other core types) from `matten` directly:
>
> ```rust
> use matten::Tensor;
> use matten_ndarray::to_arrayd;
> ```
>
> This keeps ownership and feature selection clear: `Tensor` belongs to `matten`, and
> core features (e.g. `dynamic`) are enabled on the `matten` dependency. Declare both
> `matten` and this crate in your `Cargo.toml`.

For `matten-mlprep`, substitute the import line with a representative function, e.g.
`use matten_mlprep::standardize_columns;`.

Keep READMEs concise per project structure rules; this is a single short subsection, not
a new top-level section.

### 8.2 Release-doc-script guardrails

Extend `scripts/check-release-docs.sh` with two checks. Place them alongside the
existing companion checks. Both must set `FAIL=1` on a hit.

**(a) Forbid core-type re-exports from companions (§3.2 and the deferred §3.3):**

```bash
echo "=== Checking companions do not re-export core matten (RFC-032 §3.2/§3.3) ==="
# Matches `pub use matten;` and `pub use matten::<Item>;`. Whole-crate re-export
# (§3.3) is deferred; introducing it requires amending RFC-032 and relaxing this check.
if grep -rn "pub use matten\b" \
     crates/matten-ndarray/src crates/matten-mlprep/src 2>/dev/null; then
  echo "ERROR: companions must not re-export core matten types/crate (RFC-032)"
  FAIL=1
fi
```

> Implementer note: the companions currently export only `pub use crate::...`, so this
> check passes today. The `\b` word boundary prevents false matches on other crate
> names; verify it does not match `pub use crate::` (it must not).

**(b) Forbid importing `Tensor` from a companion in examples/docs (§3.4):**

```bash
echo "=== Checking Tensor is imported from matten, not a companion (RFC-032 §3.4) ==="
if grep -rn "use matten_ndarray::[^;]*Tensor\|use matten_mlprep::[^;]*Tensor" \
     crates/matten-ndarray/examples crates/matten-mlprep/examples \
     crates/matten-ndarray/README.md crates/matten-mlprep/README.md \
     docs/src 2>/dev/null; then
  echo "ERROR: import Tensor from matten, not a companion (RFC-032 §3.4)"
  FAIL=1
fi
```

> This guard is currently *self-enforcing by the compiler*: because no companion
> re-exports `Tensor`, `use matten_ndarray::Tensor;` does not compile today. The check
> is a regression tripwire that becomes load-bearing only if a future change adds a
> re-export. Keep it regardless — it is cheap and pairs with the existing cfg-guard and
> status checks.

### 8.3 RFC lifecycle

When 8.1 and 8.2 ship:

- Change this RFC's Status to `Implemented (<version>)`.
- Move `rfcs/proposed/032-companion-dependency-and-import-convention.md` to `rfcs/done/`.
- Add a CHANGELOG entry under the shipping version noting the convention and guardrails.
- If a `pub use matten;` convenience is ever added later, do it as a separate amendment
  to this RFC (do not silently relax §3.3).

### 8.4 Verification

After `cargo fmt --all`, run:

```bash
bash scripts/check-release-docs.sh          # new guards must pass
bash scripts/check-core-dependency-boundary.sh
cargo test --workspace --all-features        # READMEs are not doctested, but examples are checked
cargo check --workspace --examples --all-features
```

No new unit tests are required (this RFC changes no logic). The script guards are the
executable specification for §3.2 and §3.4, consistent with the project's
"tests validate design specifications" principle.

### 8.5 Explicitly out of scope (do NOT do)

- Do **not** add `pub use matten;` or any `pub use matten::<Item>;` to a companion.
- Do **not** add feature-forwarding flags to companions for core features.
- Do **not** alter companion `Cargo.toml` dependency declarations.
- Do **not** change the canonical `use matten::Tensor;` style anywhere.

---

## 9. Non-goals

- Changing the internal dependency direction (companions → core is unchanged).
- Introducing a single-dependency convenience path now (deferred, §3.3).
- Any change to feature flags, error types, or crate logic.
- A facade/umbrella crate that re-exports the whole family.

---

## 10. Open questions / future revisit conditions

- **When to introduce `pub use matten;`?** Revisit only if there is concrete user
  demand (issues/requests) for a single-dependency path. If introduced, it must be a
  whole-crate re-export reachable as `matten_<companion>::matten::Tensor`, documented as
  a convenience path, never as the canonical one — and §3.2's prohibition on broad item
  re-exports remains.
- **Future companions (`matten-data`, RFC-023).** This convention applies to them by
  default: depend on core explicitly, import `Tensor` from `matten`, no core-type
  re-exports.

---

## 11. Document history

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-06-21 | Initial proposal. Adopts explicit-dependency convention (Option A); forbids broad core-type re-exports; defers whole-crate `pub use matten;`; specifies README note and release-doc guardrails with implementer handoff. |
| 0.1.1 | 2026-06-21 | Implemented in v0.19.2: README convention notes added to both companions; release-doc guardrails (§3.2/§3.3 and §3.4) added; moved to `rfcs/done/`. |
