# Bridge-crate policy

Bridge crates are how `matten` connects to specific external ecosystems without dragging
their dependencies into core. This page states the rules every bridge crate follows, and the
checklist a *future* bridge crate must satisfy.

## Why bridges are separate crates

Core `matten` owns the `Tensor` type and stays small and dependency-light. It must **not**
gain a dependency on `ndarray`, `nalgebra`, Polars, Candle, or any other target library.
Each target-specific conversion therefore lives in its own crate (for example
`matten-ndarray`), which is the only place that target's dependency appears.

```text
core matten      →  owns Tensor; no target-library dependency
matten-ndarray   →  owns the ndarray dependency; converts Tensor ↔ ArrayD<f64>
(future bridges) →  own their target's dependency; same pattern
```

This boundary is **CI-enforced**: `scripts/check-published-dependency-isolation.sh` proves
that the published core and companion crates do not pull in target/benchmark dependencies,
with `matten-ndarray → ndarray` as the one allowed, documented exception.

## Rules every bridge crate follows

- **Own the target dependency.** The bridge crate is the only published crate that depends on
  its target library.
- **Do not add a dependency to core `matten`.** A bridge never causes core to gain a
  target-library dependency.
- **Do not re-export core `Tensor`.** A bridge takes and returns `matten::Tensor`, but users
  import `Tensor` from `matten`, not from the bridge. (For example, `matten-ndarray` exports
  only `to_arrayd`, `from_arrayd`, and `MattenNdarrayError`.)
- **Return `Result`, never panic** on rejected input. Document the rejection cases.
- **Publish a [conversion contract](./bridge-contracts.md).** Fill in every dimension of the
  contract template.
- **Name conversions `to_<target>` / `from_<target>`.** This follows the `to_arrayd` /
  `from_arrayd` precedent (e.g. `to_dmatrix` / `from_dmatrix`, `to_dvector` / `from_dvector`).
  Deviate only if the target ecosystem has a stronger idiom, and justify it in that bridge's
  RFC.

## Current bridges

- **`matten-ndarray`** — the reference bridge (`Tensor` ↔ `ndarray::ArrayD<f64>`). Its
  contract is documented in [bridge contracts](./bridge-contracts.md).

There is **no `matten-nalgebra` bridge** today; the [`nalgebra` playbook](./playbooks/nalgebra.md)
documents the manual conversion path, and a dedicated bridge is only a possible future
direction, not a commitment.

## Future bridge-crate checklist

Before a new bridge crate is created (which requires separate approval — see below):

- [ ] The target library has a clear, recurring conversion need that does not fit an existing
      bridge.
- [ ] The crate owns the target dependency; core `matten` gains nothing.
- [ ] Conversions are `to_<target>` / `from_<target>` and return `Result`.
- [ ] The crate does not re-export `Tensor`.
- [ ] A full [conversion contract](./bridge-contracts.md) is filled in (copy/shape/memory
      order/dynamic/NaN/missing/dtype/error/performance).
- [ ] `scripts/check-published-dependency-isolation.sh` is extended so the new crate's
      allowed/forbidden dependencies are enforced.
- [ ] The dynamic-tensor policy is explicit (reject, or document the numeric-first step).

## No new bridge crate without approval

This policy page does **not** authorize creating new bridge crates. A new bridge (such as a
hypothetical `matten-nalgebra`, `matten-polars`, or `matten-candle`) requires its own RFC and
explicit approval. Documenting the *pattern* here does not pre-approve any specific crate.
