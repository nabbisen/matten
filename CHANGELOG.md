# Changelog — matten workspace

Coordinated release notes for the `matten` workspace. Each crate also keeps its
own changelog (independent SemVer, RFC-022 §7):

- core: [`crates/matten/CHANGELOG.md`](./crates/matten/CHANGELOG.md)
- bridge: [`crates/matten-ndarray/CHANGELOG.md`](./crates/matten-ndarray/CHANGELOG.md)

## v0.17.0 milestone — 2026-06-21

**First companion crate; workspace introduced (RFC-025, RFC-027).**

- **Repository restructured into a Cargo workspace.** Core `matten` moved to
  `crates/matten/`; shared `rfcs/`, `docs/`, `ROADMAP.md`, and CI stay at the
  workspace root. The published `matten` crate's content is unchanged by the
  move; its version remains **0.16.0**.
- **`matten-ndarray` 0.1.0 (experimental)** added: `to_arrayd` / `from_arrayd`
  between `matten::Tensor` and `ndarray::ArrayD<f64>`, with logical-order
  conversion for non-standard-layout inputs, zero-axis rejection, and dynamic
  rejection (no panic). Depends on `matten` (no default features) and
  `ndarray` 0.16; adds **no** dependency to core `matten`.
- **RFC-027** (`matten-ndarray` design) implemented → `done/`. Per-crate
  implementation RFCs continue from RFC-028.
- Tooling made workspace-aware: the dependency-boundary and release-docs scripts
  and the CI matrix now scope core checks to `-p matten` and add a bridge job.

### Security / threat model

`matten-ndarray` is a pure in-process data-structure conversion: no I/O, no
network, no auth, no new external data flow into core. The dependency-boundary
gate proves core `matten` gained no new dependency. RFC-001 threat model
unchanged; existing controls remain valid.
