# RFC-085: Promote `matten-data` to Production-Ready

**Status:** Implemented — commit *"Promote matten-data to production-ready (RFC-085)"*; implementation
reviewed and approved 2026-07-30 over two rounds (guard-coverage and regex-precision corrections; no
change to the promotion decision itself). `matten-data` is **production-ready**; RFC-059 §6's deferred
full-production review is closed. Label/docs/guard only — no code, API, version, or release change; the
RFC-042 scope lock is untouched. See §5 for the substantive question and the residual risk accepted
with it
**Target:** Post-`0.39.0`, on the `0.x` line
**Theme:** Close RFC-059 §6's deferred full-production review, audited against the RFC-057 bar as
RFC-057, RFC-080 and RFC-084 were
**Depends on:** RFC-030, RFC-033, RFC-042, RFC-057, RFC-059, RFC-067, RFC-082
**Related:** RFC-023, RFC-035, RFC-036, RFC-058, RFC-076, RFC-080, RFC-084

---

## 1. Summary

Promote `matten-data` from **production-ready candidate** to **production-ready**, closing the
review RFC-059 §6 deferred.

One substantive question has to be answered first, and §5 answers it explicitly rather than in
passing: `matten-data` gained a **new public type days ago** — `CsvBatchReader`, RFC-082 — behind the
off-by-default `streaming` feature, and RFC-082 §5 defers nine further streaming items. "Stable API"
is a signal on the full-production bar, so a crate that just grew one deserves scrutiny.

Label, documentation and guard changes only. No code, API, version, or release change.

## 2. Why now — RFC-059 named three concerns and two have moved

RFC-059 §6 deferred full production-ready with a specific, quotable reason:

> **Architect ruling: full production-ready is deferred** and requires a *separate future review*
> after at least one candidate cycle — `matten-data` is the newest companion, CSV/table ingestion has
> a wide edge-case surface, large/streaming CSV is explicitly deferred, and it is deliberately an
> on-ramp, not a dataframe engine.

Four clauses. Taking each on current evidence:

| RFC-059's concern | Status now |
|---|---|
| "at least one candidate cycle" | **Met.** Candidate since 2026-06-27 (RFC-059's acceptance ruling); a full cycle has run |
| "the newest companion" | **No longer true.** `matten-stats` (RFC-078) is newer by two months |
| "large/streaming CSV is explicitly deferred" | **Discharged.** RFC-082 shipped `CsvBatchReader`; streaming is no longer deferred, it is implemented and scoped |
| "wide edge-case surface" | **Evidence now exists.** RFC-082's implementation review ran a 4,000-case randomized differential harness over that exact surface — mixed types, quoting, non-ASCII, blank lines, ragged rows, varied file endings — at full `Debug` fidelity, with **zero mismatches**. That is direct evidence about the edge-case surface RFC-059 was worried about |
| "an on-ramp, not a dataframe engine" | **Permanent and fine.** A documented, CI-enforced scope limit (RFC-042) is not a maturity deficit — RFC-067 distinguishes "an explicit scope or workflow caveat" from "hidden API churn", and RFC-057 promoted `matten-ndarray` on a closed scope |

This is the RFC-080 pattern again: a named exit criterion, discharged by separate work landing first
(there RFC-077 → RFC-080; here RFC-082 → RFC-085).

RFC-059's own two P2 blockers are also long fixed — verified: the `Cargo.toml` description is now
maturity-neutral (*"CSV/table-to-Tensor preparation companion for matten (small PoC datasets)."*) and
eight `[[example]]` entries carry `required-features = ["csv"]`.

## 3. Audit against the full production-ready bar

RFC-057 §3's signals, plus the candidate signals the crate must still hold.

| Signal | Evidence | Verdict |
|---|---|---|
| Strong tests | **54** tests (34 at RFC-059, +17 streaming, +3 others) plus doctests. Still the most-tested companion | ✅ |
| Examples in CI | **9** examples; dedicated `data` job (`check --examples`, tests, doctests, RFC-042 guard) and **all 9 executed** in the smoke job | ✅ |
| Clear error types | `MattenDataError`, `#[non_exhaustive]`, 12 variants, `Display` + `std::error::Error` + `source()` | ✅ |
| Mature docs | Crate rustdoc with Status/Streaming sections, README, a full book page (`docs/src/examples/data.md`), nine examples | ✅ |
| Compatibility + MSRV policy | README §Compatibility; lock-step family (RFC-030); MSRV 1.85 | ✅ |
| Clear release notes | Lock-step family versioning; root `CHANGELOG.md` | ✅ |
| No hidden dependency surprises | `matten` + `csv` (optional, default-on). `streaming` implies `csv` and adds nothing | ✅ |
| No known P0/P1 issues | None. RFC-059's two P2 findings are fixed | ✅ |
| Scope lock intact | RFC-042's three-check guard passes; RFC-033/RFC-042 lock untouched | ✅ |
| **Stable API** | Default surface unchanged since `0.22.0`. But the `streaming` feature added `CsvBatchReader` in RFC-082, days ago | **⚠️ — see §5** |

Ten signals clear. One needs an argument rather than a checkmark.

## 4. The default surface has been stable for a long time

Worth stating precisely, because "the crate gained an API" is easy to over-read. The **default**
public surface is:

```text
Table          from_csv_str, from_csv_path, schema_summary, select_columns,
               fill_missing, try_numeric, row_count, column_count, column_names
NumericTable   to_tensor
SchemaSummary / ColumnSummary / ColumnKind
CellValue
MattenDataError
```

That set has not changed since `0.22.0` (RFC-036) — **38 releases ago**; the CHANGELOG records 39
released versions from `0.22.0` through `0.39.0` inclusive. `MattenDataError` gained one variant in
RFC-082 (`InvalidBatchSize`, taking it to 12), which is additive and non-breaking under
`#[non_exhaustive]`.

Thirty-eight releases without a default-surface change is a stronger stability record than most
crates can demonstrate, and it is the core of the case for this rung.

Everything RFC-082 added is behind `#[cfg(feature = "streaming")]`, off by default. A user on default
features sees exactly the surface they saw at `0.22.0`.

## 5. The substantive question: does the `streaming` feature block full production-ready?

**Stated plainly: RFC-082 §5 defers async, resumability, backpressure, parallel reading,
lenient/skip-malformed modes, schema inference, streaming numeric conversion, CSV writing, other
formats, and a `matten-stream` crate.** That is a lot of unbuilt surface adjacent to a feature this
RFC would cover with a stability promise.

**The argument for promoting anyway.** Deferred work is not the same as unstable API. The question is
not "will more be added?" — it is "will what exists change shape?" On that:

- `CsvBatchReader`'s surface is two methods, `open` and `next_batch`, and RFC-082 §4 fixed their
  semantics deliberately: synchronous, single-pass, fail-fast, `Table` per batch, `Ok(None)` at and
  after EOF, reader unusable after an error. Those are decisions, not placeholders.
- Every deferred item in RFC-082 §5 is **additive** — a new type, a new method, or a new crate. None
  of them requires changing `open` or `next_batch`.
- The one genuine risk is **async**: an async reader might want `next_batch` to become a different
  shape. But RFC-082 §4.4 already decided sync-only as a policy, and an async variant would
  conventionally be a separate type or crate, not a mutation of this one.

**The honest residual risk**, recorded rather than argued away: if resumability or async is later
designed in a way that wants `CsvBatchReader`'s existing signature changed, this promotion will have
made that a breaking change rather than a free one. That is a real cost and this RFC accepts it
knowingly.

**Recommended resolution — promote, and mark the boundary explicitly.** Full production-ready for the
crate, with the `streaming` feature documented as *"stable in what it does; its scope may still grow"*
in the crate docs, README, and `compatibility.md`. That is the accurate statement, and it is a normal
Rust posture for a feature-gated surface.

**Alternative considered — defer until streaming settles.** Rejected: RFC-082 §5's list is open-ended
by design (async and `matten-stream` may never be built), so "until streaming settles" is a deferral
with no trigger. This project has explicitly rejected trigger-less deferrals before.

## 6. Scope

### In scope

```text
scripts/check-release-docs.sh          invert the matten-data block: it currently ASSERTS
                                       "production-ready candidate" (line ~101). Also fix the
                                       blanket `grep -rIni "experimental"` at line 89 — the same
                                       over-broad shape RFC-084 review C1 removed for matten-stats
crates/matten-data/README.md           maturity line (~91) + the streaming-scope note (§5)
crates/matten-data/src/lib.rs          Status section + the streaming-scope note
README.md                              root crate table row
docs/src/reference/compatibility.md    current-status sites only; per-family history stays
docs/src/examples/{data,companions,index}.md   wherever the current label appears
docs/src/contributing/release-checklist.md
ROADMAP.md                             Status prose, §3.1 candidate-theme entry, history row
rfcs/README.md                         remaining-themes row + this RFC's entry
```

### Out of scope

```text
any code, API, test, example, or behaviour change
any change to the RFC-033/RFC-042 scope lock — matten-data stays an on-ramp, not a dataframe engine
promoting matten-stats — it is candidate by RFC-084 and explicitly not near full production
resolving any RFC-082 §5 deferral
version bump, CHANGELOG, release prep, tag, publish
RFC-076 execution — v1.0 is not currently wanted
```

### Must not be touched

```text
rfcs/done/**, rfcs/handoffs/**, CHANGELOG.md, docs/design/history/**, ROADMAP history rows
```

RFC-059 §6's deferral ruling in particular is a correct record of its position and must survive
intact — §2 answers it in argument, it does not get edited away.

## 7. What this does NOT claim

```text
it does not make matten-data a dataframe engine, or weaken RFC-042's scope lock
it does not promise async, resumable, or parallel streaming
it does not claim the streaming feature's SCOPE is finished — only that what exists is stable
it does not promote matten-stats
it does not make the family v1.0-ready, and authorizes no release action
it does not change any API, behaviour, or numeric result
```

## 8. Compatibility

Label, documentation and guard changes only. No API, behaviour, dependency, feature, MSRV, or version
change. Family version stays `0.39.0`.

**Consequence worth naming:** at production-ready, breaking `matten-data`'s default surface becomes a
documented compatibility event rather than an ordinary `0.x` change. That is the point of the rung,
and it is the main thing the owner is agreeing to.

## 9. Acceptance criteria

```text
[ ] label moved at every live site, derived by sweep-and-classify, NOT from §6's list
[ ] the guard's matten-data block inverted (not deleted), asserting the new label
[ ] the line-89 blanket `experimental` grep narrowed to a present-tense claim, matching the
    fix RFC-084 review C1 required for matten-stats — with BOTH directions proven:
      a past-tense history sentence must PASS; a present-tense stale label must FAIL
[ ] the streaming-scope note present in crate docs, README, and compatibility.md (§5)
[ ] RFC-042 scope guard still passes, unmodified
[ ] rfcs/done/**, rfcs/handoffs/**, CHANGELOG.md, docs/design/history/** unchanged
[ ] no matten-data source, test, or example file changed
[ ] full gate set: fmt, clippy, workspace tests, doctests, MSRV, mdbook, all guards
[ ] version still 0.39.0; no CHANGELOG entry, tag, or publish
```

## 10. Non-goals

```text
full production-ready for matten-stats
any RFC-082 §5 streaming work
any dataframe capability
v1.0 preparation or execution
```

## 11. Follow-up

With this, four of five crates are production-ready and one (`matten-stats`) is a candidate. That is
the family's maturity ceiling until `matten-stats` accumulates usage history (RFC-084 §3).

RFC-076 remains deferred; v1.0 timing is the owner's alone (§6.7).
