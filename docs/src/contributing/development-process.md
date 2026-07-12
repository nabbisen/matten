# Development process

This page distils the workflow that applies to every PR in `matten`. It is
drawn from the common sections that appear across all implementation handoffs
(RFC-002 through RFC-008).

## Required QA commands

Run these before requesting review unless the PR is explicitly
documentation-only:

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

When the PR touches feature-gated behaviour, also run:

```bash
cargo test --all-targets --no-default-features
cargo test --all-targets --features serde
cargo test --all-targets --features json
cargo test --all-targets --features csv
cargo test --all-targets --features dynamic
cargo test --all-features
```

When the PR touches parser, JSON, CSV, indexing, or shape arithmetic, add at
least one targeted invariant or regression test. Property tests and fuzz targets
are future hardening candidates rather than current release gates; if a fuzz
crate is introduced for a slice, its targets should compile even when they do
not run on every PR.

## Reviewer checklist

Every PR is reviewed against this list regardless of which RFC it implements:

- [ ] The implementation keeps the `matten::Tensor` public surface simple.
- [ ] No public lifetime, storage, or dimension generic leaks into user-facing
      examples.
- [ ] Panic-zone APIs have actionable `matten … error` messages.
- [ ] Result-zone APIs do not panic for malformed external input.
- [ ] Shape product and allocation-sensitive paths use checked helpers.
- [ ] Documentation examples compile and match implemented behaviour.
- [ ] Any deferred work is listed explicitly rather than hidden in TODO
      comments.

## Definition of done

A milestone or RFC is complete when:

- all planned PRs are merged;
- acceptance criteria in the RFC are satisfied;
- all QA commands pass;
- README and rustdoc examples for the affected API surface compile and are
  accurate.

## File-size guideline

- Consider splitting a `.rs` file if it exceeds **300 effective lines of
  code (ELOC)** (non-blank, non-comment-only lines).
- Splitting is strongly recommended above **500 ELOC**.
- Test code within `src/` lives in `tests.rs` (a sibling file) or in a
  `tests/` subdirectory; use the 2018+ module style (`foo.rs` + `foo/`
  coexistence, no `mod.rs`).

## Design-before-code sequence

```text
Requirements → External design → RFC → Implementation → Tests
```

Do not widen the public API beyond what an accepted RFC specifies without
a follow-up RFC or maintainer approval.
