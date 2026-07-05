# RFC-064: Workspace Core Dependency Requirement Maintenance Policy

**Status:** Implemented (post-v0.29.0 repository policy)
**Target:** Documentation and workspace dependency policy
**Theme:** Companion-to-core dependency maintenance under lock-step releases
**Depends on:** RFC-030, RFC-032

---

## 1. Summary

The `matten` workspace keeps lock-step family releases (RFC-030): all published
crates share one release version, and user-facing examples show explicit matched
family pins.

Internally, companion crates inherit their dependency on core `matten` from
`[workspace.dependencies]` with a broad pre-1.0 requirement:

```toml
matten = { version = "0", path = "crates/matten", default-features = false }
```

This is a maintenance policy, not a change to the family release model. The path
keeps local workspace development exact; the version requirement controls the
published manifest after packaging.

## 2. Motivation

Under the earlier RFC-030 wording, companion crates were expected to depend on
core as `matten = "X.Y"` for each family minor. That made the release identity
obvious, but it also required a dependency-requirement edit every family release,
even though companion crates and core are already versioned and released together.

The workspace now has a single inherited dependency entry. Keeping that entry as
`version = "0"` reduces repetitive release churn while still allowing Cargo to
resolve any compatible pre-1.0 core `matten` release.

## 3. Decision

- Workspace member crates inherit `matten` through `[workspace.dependencies]`.
- The inherited core requirement is broad pre-1.0: `version = "0"`.
- The inherited core dependency keeps `default-features = false`; companions opt
  into core features only when explicitly designed.
- User-facing docs continue to show explicit matched pins, for example
  `matten = "0.29.0"` and `matten-ndarray = "0.29.0"`.
- Lock-step family versioning still governs release identity, changelog entries,
  maturity labels, and public communication.

## 4. Rationale

This separates two concerns:

- **Published crate compatibility requirement:** the broad `0` requirement says a
  companion can build against the pre-1.0 core line unless a future change proves
  otherwise.
- **Recommended release set:** docs and examples still tell users to choose the
  matched family, because that is what the project tests, documents, and releases
  together.

The policy keeps maintenance simple without introducing an umbrella crate,
re-exporting core types, or changing the companion import convention from
RFC-032.

## 5. Release and packaging implications

Core `matten` is still the first crate to publish. Companion package dry-runs may
depend on crates.io seeing the core release. If a companion dry-run cannot resolve
the just-released core before core is published, record that as a publish-order
caveat, not as a policy failure.

Routine releases should not change the workspace core dependency requirement. If
the project needs to narrow it later, that is a policy change and must update this
RFC, RFC-030, companion compatibility notes, and the release checklist together.

## 6. Compatibility

| Dimension | Impact |
|---|---|
| Public Rust API | None |
| Runtime behavior | None |
| Feature flags | None |
| Published dependency graph | The companion requirement on core is broader (`0`) but still points only to core `matten` |
| User install guidance | Still explicit matched family pins |

## 7. Acceptance criteria

- `[workspace.dependencies].matten` uses `version = "0"` with the local path and
  `default-features = false`.
- RFC-030 records the amendment to its mechanics.
- RFC-032 clarifies that explicit matched pins are the user-facing convention,
  not the internal member manifest requirement.
- Companion README compatibility sections mention the maintenance policy without
  changing quick-start snippets away from matched pins.
- The release checklist records the package dry-run sequencing caveat.

## 8. Non-goals

- No change to lock-step family releases.
- No single-dependency convenience path.
- No `pub use matten;` or core-type re-export from companions.
- No new public API, feature, runtime behavior, or external dependency.

## 9. Document history

| Version | Date | Change |
|---|---|---|
| 0.1.0 | 2026-07-05 | Initial policy. Documents the broad workspace core requirement and corresponding docs/checklist updates. |
