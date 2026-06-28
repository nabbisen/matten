# RFC-060: Surface Benchmark Evidence in the Rendered Documentation

**Status:** Implemented (v0.27.1); maintainer-authorized (docs-only — not an architect-ruling cycle)
**Target Release:** a future docs-only release (deferred per maintainer; sequence after the companion-maturity line, RFC-057–059)
**Related:** RFC-049 (benchmarking, complexity metrics & positioning), `docs/src/benchmarks/` (index + methodology), `benchmarks/reports/` (accepted reports)
**Scope:** Make the *accepted* benchmark results readable from inside the rendered mdbook. Documentation only — no new benchmarks, no harness or dependency change, no change to the RFC-049 program or its honesty rules.

---

## 1. Summary

The mdbook Benchmarks section explains the *method* and asserts that the Phase 1 internal baseline
and Phase 2 Rust peer comparison are implemented and accepted — but it contains **no actual
numbers**. The accepted results live in `benchmarks/reports/internal-baseline-v0.1.md` and
`benchmarks/reports/peer-comparison-v0.1.md`, which sit outside `docs/src`, so mdbook never renders
them; the book references them only as bare repo paths. A reader of the book therefore cannot
*confirm* the results without leaving the book and opening files in the repository.

This RFC proposes surfacing a **curated results summary** inside the book, preserving every RFC-049
honesty caveat, so the evidence is legible where the positioning claims are made.

## 2. Problem

`matten`'s benchmark program exists "to make that position legible" (RFC-049). Today the legibility
stops at methodology:

- `docs/src/benchmarks/index.md` + `methodology.md` describe what is measured and the rules, and
  state Phase 1/2 are "accepted."
- The numbers themselves — and the architect-accepted Baseline/Report IDs
  (`matten-rfc049-internal-baseline-v0.1`, `matten-rfc049-rust-peer-comparison-v0.1`) — are only in
  `benchmarks/reports/*.md`, outside the book.
- The book's references to those reports are plain paths, not links a book reader can follow.

So the book says "we measured honestly and it was accepted" without showing the evidence. For a
project whose brand is honest, evidence-backed positioning, that is a real gap.

## 3. Constraints (must not break)

- **Harness isolation.** `benchmarks/` stays out of the workspace and `publish = false`; nothing
  here may pull `criterion`/`nalgebra` into the book build, the workspace, or any published crate's
  dependency graph.
- **RFC-049 honesty rules.** Results are a positioning and regression-visibility tool — *not* a
  ranking and *not* a "faster than X" claim. Every surfaced number must keep the
  workload-specific / environment-specific caveat and the machine-class + commit context.
- **Single source of truth.** The curated reports in `benchmarks/reports/` remain authoritative;
  the book must not fork a second, drifting set of numbers.
- **No raw histories in docs** (RFC-049 Q3 ruling): summaries only, never bulky criterion output.

## 4. Options

- **Option 1 — curated summary page in the book (recommended).** Add
  `docs/src/benchmarks/results.md` to the SUMMARY, containing a short, hand-curated digest of the
  accepted Phase 1 baseline and Phase 2 peer comparison: a small results table per phase, the
  honest framing ("where matten is slower but acceptable"), the machine-class/commit context, and
  the Baseline/Report IDs, with a pointer to the full reports. Lowest machinery; preserves
  isolation; keeps `benchmarks/reports/` as the source of truth.
- **Option 2 — render the reports directly.** Move/copy `benchmarks/reports/*.md` into `docs/src`
  so mdbook renders them verbatim. Rejected: it separates the report from its harness/regeneration
  instructions, or duplicates files that then drift.
- **Option 3 — generate the page from the reports at build time.** A script extracts the summary
  tables into the book during `mdbook build`. More machinery and a new build step; deferred unless
  drift becomes a problem.

## 5. Recommended design (Option 1)

- New page `docs/src/benchmarks/results.md`, linked under the existing `# Benchmarks` section after
  Methodology.
- Content: a compact per-phase summary (a few representative tasks, not the full matrix), each
  table headed with the workload/environment caveat; the architect-accepted Baseline/Report IDs and
  acceptance dates; the machine class and commit recorded in the reports; an explicit "these are not
  a ranking or a faster-than claim" line; and a link to the full reports in `benchmarks/reports/`
  for the complete numbers and regeneration steps.
- **Freshness guard (light).** Extend `check-release-docs.sh` (or add a small check) so the
  Baseline/Report IDs cited on the results page must match the IDs in the corresponding report
  files — preventing the book summary from outliving the reports it cites. The guard checks IDs and
  presence, not the numeric values (humans curate those).

## 6. Acceptance criteria

- [ ] A book reader can read the accepted Phase 1 and Phase 2 results, with caveats, without
      leaving the rendered documentation.
- [ ] The page carries the workload/environment caveat, machine-class + commit context, the
      Baseline/Report IDs, and the "not a ranking / not a faster-than claim" framing.
- [ ] `benchmarks/reports/` remains the single source of truth; the book is an explicitly-curated
      summary that links to it.
- [ ] No `criterion`/`nalgebra` (or any benchmark-only dependency) enters the book build, the
      workspace, or any published crate's dependency graph.
- [ ] The freshness guard ties the page's cited IDs to the report files.

## 7. Non-goals

- No new benchmarks, workloads, or measurements; no re-running on new hardware as part of this RFC.
- No Phase 3 (NumPy/Pandas) or Phase 4 (regression gates) work — both stay deferred (RFC-049).
- No ranking, marketing, or "faster than X" language; no public-API change to make any number look
  better.
- No change to harness isolation or to the lock-step family versioning.
