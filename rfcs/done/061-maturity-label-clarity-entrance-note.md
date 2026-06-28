# RFC-061: Maturity-Label Clarity — Keep "Production-Ready", Add an Entrance Note

**Status:** Implemented (v0.27.1); maintainer-authorized (docs-only — not an architect-ruling cycle)
**Target Release:** a future docs-only release (deferred per maintainer; may bundle with RFC-060)
**Related:** RFC-029 (maturity labels), RFC-057 / RFC-058 / RFC-059 (companion-maturity decisions), ROADMAP §12 (maturity ladder), `docs/src/philosophy.md` (positioning)
**Scope:** Reduce the risk that the maturity label "production-ready" is misread as a performance/scale claim — **without** renaming any rung and **without** qualifying the term at every occurrence. Documentation only.

---

## 1. Summary

`matten`'s maturity ladder uses the standard term **production-ready**, defined (ROADMAP §12) as
"stable enough to recommend as a normal dependency for *its documented scope*" — a maturity/
reliability label whose signals are mature docs, stable API, and compatibility policy, with nothing
about speed. That definition is sound. But the **bare** label, seen in the root README status
column or on a badge without the "for its documented scope" qualifier, can be misread by outsiders
as a performance- or scale-readiness claim — exactly the axis `matten` deliberately does not serve
("optimizes for time to a runnable PoC, not benchmark leadership").

**Decision (maintainer, Option D):** keep the standard term, and add a *small* clarifying note at
the **documentation entrances only** — the root `README.md` and the mdbook **introduction** — rather
than repeating a qualifier at every occurrence. The project already declares its purpose and goals
up front, so an entrance-level note is sufficient; the term does not need re-explaining everywhere.

## 2. Problem

Two things let the bare label over-read:

1. **The scope qualifier is dropped on compact surfaces.** In the root README crate table the
   Status cell reads simply `production-ready`, next to core's `stable (v0.x)`. A reader supplies
   their own default meaning of "production-ready," which in the Rust ecosystem often connotes
   "ready for serious/large/fast production workloads."
2. **The precise definition lives in a maintainer doc.** The scope-bound ladder definition is in
   the ROADMAP (§12), not in the user-facing book, so a book reader sees the label but never the
   "for its documented scope" definition.

For a project whose brand is honest scope ("family car; migrate when you outgrow it"), a bare
"production-ready" risks promising on precisely the dimension the project is candid about lacking.

## 3. Decision: Option D (keep term + entrance note)

This RFC adopts **Option D**, a deliberate narrowing of the earlier Options A/C:

- **Keep the term.** "production-ready" (and "production-ready candidate") stay — they are the
  recognized maturity vocabulary; renaming would discard a signal readers know and churn the
  ladder, every crate label, the guards, and prior RFCs.
- **Note at entrances only.** Add one short clarifying note to:
  - the root `README.md` (near the crate/status table or the overview), and
  - the mdbook introduction (`docs/src/introduction.md`).
- **Do not** add the qualifier to every crate README, table cell, badge, or status line.

Rationale for entrance-only (vs always-binding): the purpose and goals are already declared at the
top of both entrances, so the maturity labels are read *in that context*. A single, well-placed
note removes the misreading at the door without adding repetitive noise downstream.

## 4. Proposed content (small)

A note of roughly one to three sentences at each entrance, e.g. in substance (wording finalized at
implementation):

> Maturity labels here — including *production-ready* — describe **stability and maturity within
> `matten`'s documented scope** (PoC, learning, and small workflows). They are **not** performance
> or scale claims; `matten` optimizes for time to a runnable PoC, not benchmark leadership. For the
> full rung definitions see the maturity ladder (ROADMAP §12).

The root README note may be even terser, since the table is right there. The two notes should be
consistent but need not be identical.

## 5. Acceptance criteria

- [ ] The root README and the mdbook introduction each carry a short note clarifying that maturity
      labels describe maturity-within-scope, not performance/scale.
- [ ] The note is small and does not bloat either entrance; it sits naturally near the existing
      purpose/positioning text.
- [ ] No rung is renamed; the ladder (ROADMAP §12) is unchanged; no per-occurrence qualifier is
      added to crate READMEs, table cells, badges, or status lines.
- [ ] No crate's maturity status changes; no RFC label wording changes.

## 6. Non-goals

- **No rename** of "production-ready", "production-ready candidate", or any other rung.
- **No always-on qualifier** (that was Options A/C, explicitly not chosen).
- No change to the maturity ladder, its definitions, or any crate's current maturity status.
- No code, API, runtime, or dependency change; no versioning-policy change.

## 7. Sequencing

Docs-only and deferred ("proceed later"). May ship in the same docs-only release as RFC-060 (both
improve documentation legibility at the entrances), or separately — neither blocks the other.
