# Migrating to Polars / Pandas (dataframes)

[Polars](https://pola.rs) (Rust, with Python bindings) and
[Pandas](https://pandas.pydata.org) (Python) are **dataframe** libraries: labeled columns,
heterogeneous types, group-by, joins, pivots, and query expressions. They solve a different
problem than `matten`, which is a numeric `Tensor`. Reach for them when your real need is
**tabular analytics**, not array math.

## The boundary you are crossing

`matten-data` is an **ingestion on-ramp**: it reads CSV/table data and hands you a numeric
`Tensor`. It is intentionally minimal and **will not grow** group-by, joins, pivots, or
query expressions. If you find yourself wanting any of those, that is the signal to move the
tabular layer to a dataframe library — it is not a missing `matten-data` feature.

```text
matten-data →  CSV/table → numeric Tensor   (on-ramp only)
Polars/Pandas →  group-by / join / pivot / query / labeled columns
```

## Choose this target when

- You need **group-by, joins, pivots, windowing, or query-style** selection.
- Your data is **tabular and heterogeneous** (mixed column types, labels, nulls as a
  first-class concept).
- You want to explore/clean tabular data interactively before any numeric step.

## Do not choose this target when

- Your data is already a clean numeric array and you only need array math → `ndarray` or
  stay with `matten`.
- You need decompositions/solvers → `nalgebra`.
- The tabular step is a one-time CSV-to-numeric on-ramp → `matten-data` already covers it.

## Concept mapping

| `matten` / `matten-data` | Polars / Pandas |
|---|---|
| `Table::from_csv_str(..)` (on-ramp) | `pl.read_csv(..)` / `pd.read_csv(..)` |
| numeric `Tensor` (homogeneous `f64`) | a `DataFrame` of typed, labeled columns |
| select columns then `to_tensor()` | `df.select([..])`, then to ndarray/NumPy if needed |
| *(not available)* group-by / join / pivot | `df.group_by(..)`, `df.join(..)`, pivots |

## Example migrations

- `data_00_quickstart` → if the next step is group-by/join/pivot rather than array math,
  read the CSV straight into Polars/Pandas instead of `matten-data`.
- A CSV → clean → single numeric pass with no tabular analytics → **stay with
  `matten-data`**; it is the right size for that.

## Conversion path

The usual pattern is **not** to convert a `matten` tensor into a dataframe, but to **enter
the dataframe library at the data source**:

```text
have a CSV and need tabular analytics?  → read it directly into Polars/Pandas
already have a numeric matten Tensor?    → export its columns if you must, but prefer
                                           doing tabular work upstream of matten
```

If you genuinely need to move a numeric `Tensor` into a dataframe, export its data
(`tensor.to_vec()` / `tensor.shape()`) and build columns in the dataframe library; the exact
constructor depends on the library and version, so follow its current docs.

## Common pitfalls

- **Don't wait for `matten-data` to grow tabular features.** It will not. Recognize the
  boundary early.
- **Round-tripping is usually wrong.** If tabular work is the point, do it in the dataframe
  library from the start rather than converting back and forth with `matten`.

## Performance / positioning notes

There is **no `matten`-vs-dataframe benchmark**: a numeric tensor and a dataframe are
different paradigms, and a cross-library/cross-language ecosystem comparison would be
RFC-049 Phase 3, which is **not authorized**. Choose by **capability and ecosystem fit**
(do you need tabular operations?), not by a measured speed comparison.

## Minimal checklist

- [ ] Your real need is tabular (group-by/join/pivot/query), not array math.
- [ ] You enter the dataframe library at the data source rather than round-tripping.
- [ ] You are not waiting for `matten-data` to grow dataframe features.
