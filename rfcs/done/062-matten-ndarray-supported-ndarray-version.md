# RFC-062: `matten-ndarray` Supported `ndarray` Version — 0.16 → 0.17

**Status:** Implemented in v0.28.0 (architect-accepted Option B, review 2026-06-27): `ndarray = ">=0.16.1, <0.18"`, compatibility-matrix CI verifying `0.16.1` and `0.17.2`; unchanged bridge confirmed against both minors with no version-conditional code.
**Target Release:** v0.28.0 (family minor — a supported-dependency change to the bridge's public surface)
**Related:** RFC-025 §6 (`ndarray` version policy — minor bump is a compatibility event), RFC-027 (`matten-ndarray`), RFC-030 (lock-step family versioning), RFC-057 (`matten-ndarray` production-ready), `crates/matten-ndarray/README.md` (Compatibility + conversion contract)
**Scope:** Decide which `ndarray` minor(s) `matten-ndarray` supports, now that `ndarray 0.17` is available. Dependency/compatibility and docs only — **no bridge API, signature, behavior, or zero-copy change**, and core `matten` still carries no `ndarray` dependency.

---

## 1. Summary

`matten-ndarray` pins `ndarray = "0.16"` (locked `0.16.1`); `ndarray 0.17.2` is now the latest. Because
`to_arrayd`/`from_arrayd` carry `ndarray::ArrayD<f64>` in their signatures, the `ndarray` version is
part of the bridge's **public API**, so moving it is a compatibility event (RFC-025 §6), not a
routine `cargo update`. This RFC records the verified risk picture and proposes **widening the
requirement to a range (`>=0.16, <0.18`)**, CI-tested against both minors, as the option that best
fits a bridge whose whole job is interop — with a straight bump to `0.17.2` as the fallback.

## 2. Background — why this is a compatibility event

The bridge re-exposes an `ndarray` type. A given `matten-ndarray` build resolves to exactly one
`ndarray` minor, and `0.16`/`0.17` are Cargo-incompatible, so a consumer must use a matching minor:
a project still on `ndarray 0.16` cannot interoperate with a `matten-ndarray` built against `0.17`
(the two `ArrayD<f64>` types differ). Under lock-step versioning (RFC-030) the change also ripples
to a whole-family minor, even though only the bridge is affected.

## 3. Verified findings

- **MSRV is a non-issue.** `ndarray 0.17.2` declares `rust-version = 1.64` (same as `0.16`), well
  under `matten`'s 1.85 floor. (Contrast `nalgebra 0.35`, which requires Rust 1.89 — *that* would
  force an MSRV decision; `ndarray 0.17` does not.)
- **0.17 is additive / backwards-compatible.** Its headline is new array *reference* types
  (`ArrayRef`/`RawRef`/`LayoutRef`); the `0.17.1` `+ ?Sized` nuance applies only to code generic over
  those new types. The bridge uses concrete `ArrayD<f64>`, so `to_arrayd`/`from_arrayd` are expected
  to compile essentially unchanged — to be confirmed by CI, not assumed.
- **Target `0.17.2`, never `0.17.0`.** `0.17.0` was yanked (a use-after-free in the new reference
  types), fixed in `0.17.1`; `0.17.2` is a further ArrayRef patch with a reduced packaging footprint.
- **No functional pull.** The bridge uses only basic `ArrayD<f64>` construction/access; it needs no
  `0.17` feature. The only driver is ecosystem currency — letting consumers already on `0.17`
  interoperate.

Net severity: **low and well-bounded** — no MSRV hit, minimal code risk. The only real weight is the
user-facing SemVer/interop question (§2) plus the lock-step family ripple.

## 4. Options

### Option A — single bump `0.16` → `0.17.2`

Set `ndarray = "0.17"`. Simple; the RFC-025 §6 default. **Cost:** consumers still on `ndarray 0.16`
can no longer use the bridge until they move to `0.17`.

### Option B — version range `>=0.16, <0.18` (recommended)

Let Cargo resolve `ndarray` to whatever the **consumer** already uses: `0.16` projects get `0.16`,
`0.17` projects get `0.17`, and a project with no other `ndarray` dependency gets the latest in range
(`0.17.2`). This avoids stranding either group and directly fulfills the bridge README's existing
promise — *"broad `ndarray` version compatibility is not promised until CI tests it."* **Cost:** the
bridge must compile cleanly against both minors, CI must cover both, and the bridge's public
`ArrayD<f64>` type becomes the consumer's resolved version rather than a single fixed one.

### Recommendation — **ratified: Option B** (architect ruling 2026-06-27)

**Option B with the requirement `ndarray = ">=0.16.1, <0.18"`** (preferring `0.16.1` as the floor,
the previous known-good point and the named test target). Conditional on CI confirming the bridge
compiles and its tests/doctests/examples pass against **both** `0.16.1` and `0.17.2`.

**Hard rule (P1): no version-conditional bridge code.** No `#[cfg(...)]` branches and no
`ndarray-016`/`ndarray-017` feature flags. The bridge is intentionally tiny; version-conditional
code would be more complexity than the range is worth.

**Fallback (P1):** if the unchanged bridge cannot compile/pass against *either* minor, **reject the
range and fall back to Option A** (`ndarray = "0.17"`, CI targeting `0.17.2`) rather than branch.

## 5. Proposed implementation

- `[workspace.dependencies] ndarray`: set to `">=0.16.1, <0.18"`.
- **CI compatibility matrix** (`.github/workflows/test.yaml`): a `bridge-ndarray-compat` job running
  the bridge against each pinned minor via `cargo update -p ndarray --precise <ver>` in a fresh
  checkout (its `Cargo.lock` changes are not committed):

  ```bash
  # for ver in 0.16.1, 0.17.2:
  cargo update -p ndarray --precise <ver>
  cargo test -p matten-ndarray
  cargo test -p matten-ndarray --doc
  cargo run  -p matten-ndarray --example to_arrayd
  cargo run  -p matten-ndarray --example from_arrayd
  ```

  The existing `bridge` and `smoke` jobs stay (default resolution = latest in range, `0.17.2`).
- **Docs** — README Compatibility + conversion-contract table. State that `matten-ndarray` supports
  `ndarray` 0.16 and 0.17 (requirement `>=0.16.1, <0.18`; CI-verified at `0.16.1` and `0.17.2`), and
  that because `to_arrayd`/`from_arrayd` use `ndarray::ArrayD<f64>`, **the resolved `ndarray` minor
  is part of the public type identity** — a consumer on `ndarray 0.16` receives `0.16`'s `ArrayD`, a
  consumer on `0.17` receives `0.17`'s. Add the **yanked caveat**: `ndarray 0.17.0` is yanked and is
  **not** a tested target; use a non-yanked patch in the supported minor. Add the **docs.rs caveat**:
  docs.rs renders one resolved minor (likely `0.17.2`) even though CI verifies both. Update the
  contract from "supports the 0.16 minor" to "supports 0.16 and 0.17, tested at 0.16.1 and 0.17.2".
- **Guards:** `check-published-dependency-isolation` must still confirm core `matten` has **no**
  `ndarray` dependency; no other dependency expansion.
- **Lockfile:** the committed root `Cargo.lock` resolves `ndarray` to the latest in range (`0.17.2`);
  the matrix job's `--precise` pins are job-local and not committed.

## 6. Acceptance criteria

- [ ] Reviewer rules on Option A vs Option B.
- [ ] `matten-ndarray` tests, doctests, and both examples pass against the supported `ndarray`
      version(s) in CI (both `0.16.1` and `0.17.2` for B; `0.17.2` for A).
- [ ] No change to bridge signatures, behavior, copy semantics, dynamic rejection, or error
      variants; no zero-copy work; no version-conditional bridge code (if B is infeasible without it,
      fall back to A).
- [ ] Core `matten` still carries no `ndarray` dependency (isolation guard green).
- [ ] README Compatibility + conversion contract state the supported `ndarray` version(s); `0.17.0`
      noted as yanked.

## 7. Non-goals

- No bridge API/signature/behavior/error change; no zero-copy; no new conversion paths.
- No `ndarray` *feature* adoption (the new `0.17` reference types are not used).
- No MSRV change (stays 1.85); no change to lock-step family versioning.
- No re-run of the RFC-049 peer benchmark here — the accepted Phase 2 report is a snapshot at
  `ndarray 0.16.1`; refreshing it against `0.17` is a separate future benchmark task (Phase 2 v0.2),
  not part of this compatibility decision.
- No new bridge crates.

## 8. Versioning

Whichever option, this is a **family minor** (proposed **v0.28.0**) because the bridge's public
dependency surface changes. Under lock-step versioning the bump applies to the whole family even
though only `matten-ndarray` is materially affected — an accepted trade-off of RFC-030.
