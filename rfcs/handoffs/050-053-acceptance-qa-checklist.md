# Acceptance and QA Checklist — RFC-050–053

**Project:** `matten`  
**Document kind:** Acceptance / QA checklist  
**Scope:** RFC-050, RFC-051, RFC-052, RFC-053  

---

## 1. Global Acceptance

```text
[ ] No new dependency in core matten.
[ ] No new runtime behavior in core matten.
[ ] No new bridge crate unless separately approved.
[ ] No CLI/tooling implementation.
[ ] mdBook builds.
[ ] All new docs are linked from SUMMARY.md.
[ ] No broken internal links.
[ ] Current docs avoid marketing claims.
[ ] Current docs avoid automatic conversion claims.
```

---

## 2. RFC-050 Checklist

```text
[ ] docs/src/migration/index.md exists.
[ ] docs/src/migration/when-to-migrate.md exists.
[ ] docs/src/migration/target-selection.md exists.
[ ] docs/src/migration/common-pitfalls.md exists.
[ ] "stay with matten" guidance exists.
[ ] "migrate from matten" guidance exists.
[ ] target-selection matrix exists.
[ ] "outgrowing matten is a successful PoC outcome" or equivalent wording exists.
[ ] docs state core remains dependency-light.
[ ] docs state migration support is not automatic conversion.
```

---

## 3. RFC-051 Checklist

```text
[ ] docs/src/migration/bridge-contracts.md exists.
[ ] docs/src/migration/bridge-crate-policy.md exists.
[ ] bridge contract template includes copy/view behavior.
[ ] bridge contract template includes shape/rank policy.
[ ] bridge contract template includes dynamic tensor policy.
[ ] bridge contract template includes missing/value policy.
[ ] bridge contract template includes error behavior.
[ ] matten-ndarray README is audited against the template.
[ ] docs state bridge crates must not re-export Tensor.
[ ] docs state bridge dependencies stay out of core matten.
```

---

## 4. RFC-052 Checklist

```text
[ ] playbooks/index.md exists.
[ ] ndarray playbook exists.
[ ] nalgebra playbook exists.
[ ] Polars/Pandas playbook or boundary page exists.
[ ] Candle playbook or stub exists.
[ ] NumPy/Python playbook exists.
[ ] each playbook includes "choose when" and "do not choose when".
[ ] at least three matten examples are mapped to target choices.
[ ] dataframe needs point to Polars/Pandas, not matten-data expansion.
[ ] ML tensor/model needs point to Candle, not core matten expansion.
```

---

## 5. RFC-053 Checklist

```text
[ ] readiness report template exists.
[ ] readiness checklist exists.
[ ] checklist maps signals to targets.
[ ] one worked example report exists.
[ ] report includes production pressure signals.
[ ] report includes direct conversion candidates.
[ ] report includes manual redesign areas.
[ ] report includes risks and next steps.
[ ] report clearly says advisory, not automatic conversion.
```

---

## 6. Benchmark Evidence Checklist

```text
[ ] migration docs do not cite sandbox numbers.
[ ] migration docs do not cite unfilled peer-comparison template.
[ ] migration docs use official peer results only after maintainer-run numbers are accepted.
[ ] migration docs include workload-specific limitation language.
[ ] no "faster than X" claim.
[ ] no competitor ranking.
```

---

## 7. Review Questions Before Merge

```text
[ ] Would a beginner understand when to stay with matten?
[ ] Would a production developer understand when to migrate?
[ ] Does the guide avoid making matten sound deprecated?
[ ] Does the guide avoid pretending matten is production-super-library?
[ ] Are target recommendations specific, not universal?
[ ] Are all future/tooling items clearly marked deferred?
```
