# Security Policy

## Reporting a vulnerability

Report privately via GitHub's private vulnerability reporting:

**https://github.com/nabbisen/matten/security/advisories/new**

Please do not open a public issue for a suspected vulnerability.

## Supported versions

Only the current `0.4x` family is supported. `matten` is pre-1.0; there are no backports to older
minor versions.

## Scope

**In scope** — the boundary surfaces where untrusted input reaches the library (RFC-001's threat
model): JSON parsing, CSV parsing, filesystem paths passed to `load_json`/`load_csv`, caller-supplied
shapes and indices, and the slice mini-language (`slice_str`).

**Out of scope** — performance issues, and anything the project's documentation states is
explicitly out of scope.

## Expectations

This is a small, pre-1.0 project maintained by one person. There is no contracted response time.
Reports are read and acted on as time allows.
