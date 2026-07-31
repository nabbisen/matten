# RFC-090: Histogram — Bin-Selection Policy and `matten-stats::histogram`

**Status:** `proposed/` by folder (not yet implemented); **reviewed and accepted 2026-07-31** —
implementation authorized under the handoff. Resolves RFC-040 §8's deferral and adds one function;
amends RFC-078 §5's companion boundary (§5). No release, no version bump
**Target:** Post-`0.41.0`, on the `0.x` line
**Theme:** Close the oldest open policy question in the statistics line, chosen against §1.1's baseline
**Depends on:** RFC-018, RFC-031, RFC-040, RFC-078, RFC-083, RFC-087
**Related:** RFC-002, RFC-005, RFC-082

---

## 1. Summary

Resolve the bin-selection policy RFC-040 §8 left open in v0.21.2, and add the function it was blocking:

```rust
pub struct Histogram { pub counts: Vec<usize>, pub edges: Vec<f64> }

pub fn histogram(x: &Tensor, bins: usize) -> Result<Histogram, MattenStatsError>;
```

**The policy is: there is no automatic bin rule. The caller says how many bins.** §4.1 argues why that
is the answer rather than a dodge.

This is `matten-stats`'s **first non-scalar return**, which amends RFC-078 §5's `Tensor -> f64`
boundary. §5 makes that amendment explicitly rather than letting it happen quietly.

One new error variant (`InvalidBinCount`), additive under `#[non_exhaustive]`. No new dependency.

## 2. Why now

Histogram has been blocked since v0.21.2 — the longest-standing open question in the project — and
every subsequent statistics RFC has deferred to it. RFC-078 §7 and RFC-083 §6 both list it as
*"blocked on RFC-040 §8's unresolved bin-selection policy"*.

It is the cheapest remaining item on §3.1 because **the hard part is a decision, not code**. Binning is
a single pass; the reason it never shipped is that nobody chose a policy.

Under §1.1's baseline it also scores well on its own merits: a histogram is where a learner goes
immediately after computing a mean, and it is the standard first look at a distribution.

## 3. What RFC-040 §8 actually asked

It listed five reasons for deferring, and this RFC must answer each:

```text
bin selection policy    -> §4.1   the substance of this RFC
interpolation policy    -> N/A    that was quantile's; RFC-078 §4.2 settled it (linear)
edge inclusion policy   -> §4.3
sorting/allocation cost -> §4.5   no sort is needed; allocation is guarded
statistical expectations-> §4.1   answered by refusing to embed any
```

## 4. Policy decisions

### 4.1 Bin selection — the caller chooses; no automatic rule

**Decision: `bins: usize` is a required argument. `matten-stats` implements no automatic bin rule —
not Sturges, not Freedman–Diaconis, not Scott, not Doane, and no `"auto"`.**

The alternative on offer is NumPy's `bins='auto'`, which picks the larger of Sturges and
Freedman–Diaconis. Each such rule is a *statistical assumption wearing a default's clothing*: Sturges
assumes approximate normality and is known to under-bin large samples; Freedman–Diaconis is
IQR-based and degenerates when the IQR is zero; Scott assumes normality outright. A learner calling
`histogram(&x)` and receiving 14 bins has silently accepted whichever assumption the library picked,
with nothing in the call site to indicate a choice was made at all.

That is precisely what §1.1's baseline rules out — *"prefer an obvious default with a documented
rationale over a configurable policy"*. Note the direction: a required `bins` argument is **not** a
configurable policy. It is the absence of one. There is no mode to select, no rule to learn, and no
hidden assumption to inherit.

It is also the project's established shape for exactly this situation: RFC-082 made `batch_rows` a
required argument rather than auto-tuning a batch size, for the same reason.

**Consequence, stated plainly:** the caller must think about bin count. That is a real cost, and for
an education-oriented library it is the *point* — bin count is a genuine analytical choice, and a
histogram whose bin count was chosen for you teaches the wrong lesson about histograms.

### 4.2 Range — the data's own minimum and maximum

No `range` parameter. `edges[0]` is `min(x)`, `edges[bins]` is `max(x)`, and the interval is divided
into `bins` equal widths. Matching NumPy's default, and a `range` argument would be the configurability
§4.1 declines.

### 4.3 Edge inclusion — half-open, with the last bin closed

```text
bin i covers  [edges[i], edges[i+1])   for i < bins-1
the last bin  [edges[bins-1], edges[bins]]   -- closed at the top
```

**Match NumPy exactly.** Without the closed last bin the maximum value falls in no bin and vanishes
from the counts — `counts.iter().sum()` would silently be less than `x.len()`. Under RFC-087 §6's
rule, that is a **silent** wrong answer, so the ecosystem convention wins.

The doc comment must state the rule and the reason, because "why is the last bin different" is a
question every reader eventually asks.

### 4.4 Constant input is an error, not an invented range

If `min(x) == max(x)`, NumPy silently widens the range to `(v - 0.5, v + 0.5)`. **`matten-stats`
errors instead**, reusing `MattenStatsError::ZeroVariance`.

NumPy's expansion invents data-independent numbers — the `0.5` comes from nowhere in the input — and
produces a plot a learner cannot interpret. The divergence is **visible**, an error naming the
condition, so RFC-087 §6 places it on the "diverge where it surfaces and teaches" side. It is also
consistent with `correlation`, `skewness` and `kurtosis`, which already reject zero variance rather
than return a silent `NaN` (RFC-078 §4.3).

### 4.5 No sorting; allocation is guarded

Unlike `quantile`, a histogram needs no sort — one pass for min/max, one to bin. That answers
RFC-040 §8's "sorting cost" concern outright.

`bins` is caller-supplied and unbounded, so the output must be validated against
`MattenLimits::default().max_elements` before allocating. `MattenLimits` is publicly exported from
core `matten` (`matten::MattenLimits`), so this needs no new dependency — it is, however,
`matten-stats`'s first use of it.

## 5. The boundary amendment — stated, not smuggled

RFC-078 §5 defines the companion split as:

```text
matten-mlprep   transforms tensors      input: Tensor -> output: Tensor
matten-stats    scalar summaries        input: Tensor -> output: f64
```

**A histogram is not a scalar.** It is counts-per-bin plus the edges that define them, and no honest
`f64` return exists. So this RFC amends the boundary:

> `matten-stats` computes **statistical summaries** of a `Tensor`. A summary is returned as `f64`
> where it is scalar, and as a **small owned struct** where it is inherently vector-valued.
> `matten-stats` never returns a `Tensor`.

The last sentence is the load-bearing one. The original `-> f64` rule was not about scalars for their
own sake; it existed to stop `matten-stats` drifting into a tensor-transform crate competing with
`matten-mlprep`. **Returning `Histogram` creates no such risk; returning `Tensor` would.** That is
also why RFC-083 §6 deferred matrix-wide and axis-wise covariance — those return a `Tensor`, and this
amendment does **not** unblock them.

`Histogram` is a plain data struct with public fields and no methods — nothing to grow into.

## 6. API shape

```rust
/// counts.len() == bins ; edges.len() == bins + 1
pub struct Histogram {
    pub counts: Vec<usize>,
    pub edges: Vec<f64>,
}

pub fn histogram(x: &Tensor, bins: usize) -> Result<Histogram, MattenStatsError>;
```

Errors, all mapping to existing variants except one:

```text
bins == 0                 InvalidBinCount   NEW — additive under #[non_exhaustive]
dynamic tensor            DynamicTensor
non-finite value          NonFiniteValue
fewer than 1 element      Empty
min == max                ZeroVariance      §4.4
bins exceeds the limit    Empty is wrong here — use the allocation path (§4.5)
```

`InvalidBinCount` follows `matten-data`'s `InvalidBatchSize` precedent (RFC-082) exactly: a required
count argument that cannot be zero earns its own variant.

**No `try_` pairing.** `matten-stats` has no panicking convenience forms — all six existing functions
return `Result` — and this RFC does not introduce the pattern.

## 7. Scope

```text
IN    crates/matten-stats/src/histogram.rs   new module: Histogram, histogram
      crates/matten-stats/src/lib.rs         mod + pub use + the §5 boundary amendment in crate docs
      crates/matten-stats/src/error.rs       InvalidBinCount variant + Display arm
      crates/matten-stats/tests/statistics.rs
      crates/matten-stats/README.md, docs/src/reference/stats.md — incl. the amended boundary
      one example, stats_* namespaced, with its [[example]] entry and CI smoke-run line

OUT   any automatic bin rule, now or later, without a further RFC (§4.1)
      a `range` parameter (§4.2); density/normalised output; 2-D histograms
      matrix-wide or axis-wise stats — the §5 amendment does NOT unblock them
      any maturity-label change; matten-stats stays production-ready candidate
      version bump, CHANGELOG, tag, publish
```

## 8. Acceptance criteria

```text
[ ] counts.len() == bins and edges.len() == bins + 1, always
[ ] counts.iter().sum::<usize>() == x.len() for every valid input — the invariant
    that proves the closed last bin works (§4.3). Test with a maximum that lands
    exactly on the last edge
[ ] a known input with hand-computed bins matches exactly
[ ] edges are evenly spaced from min to max
[ ] bins == 0 -> InvalidBinCount ; min == max -> ZeroVariance
[ ] dynamic, non-finite, and empty inputs rejected
[ ] a huge `bins` returns the allocation error without attempting the allocation
[ ] the §5 boundary amendment replaces the old framing at ALL FIVE sites across
    lib.rs, README.md and stats.md — not just the first in each file
[ ] existing six functions unchanged; matten-stats still depends only on matten
[ ] full gate set; no version bump, CHANGELOG, tag, or publish
```

## 9. Non-goals

```text
automatic bin selection, in any form
density / probability-mass normalisation
2-D or N-D histograms
plotting or rendering — matten has no visualisation surface (RFC-070)
unblocking matrix-wide or axis-wise statistics (§5)
```
