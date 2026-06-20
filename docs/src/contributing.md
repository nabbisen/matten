# Project layout and process

- **Design before code.** Requirements → external design → RFCs → implementation.
  The `rfcs/` pack and roadmap are the source of truth for design decisions.
- **Milestones.** Development proceeds in small, shippable milestones (M0, M1, …),
  each preserving a working crate with passing tests and a coherent public API.
- **Safety.** The crate is `#![forbid(unsafe_code)]`; any future exception
  requires a dedicated RFC.

## Local development

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```
