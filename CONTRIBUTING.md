# Contributing

`matten` follows an RFC-driven process: design decisions are tracked in [`rfcs/`](./rfcs/README.md)
before implementation.

- **Contributor documentation** (QA commands, reviewer checklist, definition of done, architecture,
  release checklist) lives in the book's [Contributing](https://nabbisen.github.io/matten/contributing.html)
  section.
- **RFC lifecycle and current design decisions** are in [`rfcs/README.md`](./rfcs/README.md).
- **Nine guard scripts** under [`scripts/`](./scripts/) enforce release-documentation truth,
  dependency boundaries, and companion-crate scope; run them before proposing a change that touches
  documentation, dependencies, or crate structure.

For security issues, see [`SECURITY.md`](./SECURITY.md) instead of opening a public issue.
