# Design Documents

This directory contains tracked design-governance records and historical snapshots. It is outside
the mdBook source tree; do not add `docs/design/**` files to `docs/src/SUMMARY.md`.

## Ownership Rule

Current project authority is split across four planes:

```text
rfcs/                = canonical normative decisions: data model, error model,
                       boundaries, feature scope, threat model, versioning,
                       companion contracts, and lifecycle policy
docs/src/            = distilled evergreen user contract, tutorials, reference pages,
                       examples, philosophy, and positioning
ROADMAP.md           = forward schedule and milestone history
docs/design/history/ = dated design snapshots; historical only, never cited as current
```

Editing heuristic:

```text
If it is a decision, write or update an RFC.
If it is what a user must know to use matten, write or update docs/src.
If it is schedule or milestone history, update ROADMAP.md.
The archived specs are never cited as current.
```

## Governance-Disposition Decision

The ownership rule is recorded here as a tracked design-governance note, not as RFC-066.

Rationale:

```text
The rule describes where existing authority lives after the v0.19.0 snapshots were superseded.
It does not define new public API, dependency policy, runtime behavior, release scope, or user
contract. RFC-000 remains the normative lifecycle policy for RFC states; this note is the
repository map that keeps archived design snapshots from being mistaken for current authority.
```

If this ownership model later needs normative force beyond repository navigation, it should be
promoted by a separate RFC.

## Supersession Summary

The v0.19.0 requirements, external-design, and roadmap snapshots are archived for traceability only.
They self-declare the RFC corpus and `ROADMAP.md` as forward authority, and they predate major
current work: `matten-data`, v0.20-v0.30 API/companion development, RFC-049 benchmarking closure,
RFC-054 local migration tooling, RFC-063 visual-understanding work, and RFC-065 educational
visualization.

Use these current owners instead of citing archived snapshots as current:

```text
Core API/data/error/shape/feature decisions -> rfcs/done/
User-facing usage/reference/tutorials       -> docs/src/
Forward schedule and release history        -> ROADMAP.md
Historical snapshot traceability            -> docs/design/history/
Coverage-gap decisions before archival      -> docs/design/coverage-gap-resolution.md
v1.0 readiness audit report                 -> docs/design/v1-readiness-audit.md
```

The archived snapshots retain retired vocabulary inside historical text. That vocabulary must not be
copied into user-facing docs.

The legacy tracked `docs/design/external-design.md` file predates the v0.19.0 snapshots and is also
historical only. It remains in place with a superseded banner so existing local links do not silently
break during this archival slice.
