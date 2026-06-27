# Migration readiness checklist

This checklist turns the vague question "should I leave `matten`?" into a set of concrete
**pressure signals**. Work through it for the part of your workflow under question, mark each
signal, and follow the mapping to a [playbook](./playbooks/index.md). It is an **advisory
self-assessment** — there is no tool that scans your code, and a high score does not by itself
mean you must migrate.

How to read it: each signal is a yes/no probe. The more signals you mark "yes" — especially
the first six — the stronger the case to move *that hot part* of the workflow. If almost
everything is "no", staying with `matten` is the right answer.

## Pressure signals → target

| # | Signal | You feel it when… | If yes, consider |
|---|---|---|---|
| 1 | **Data-size pressure** | arrays are large enough that `matten`'s copy-on-reshape/slice shows up, or memory is tight | [`ndarray`](./playbooks/ndarray.md) |
| 2 | **Runtime pressure** | a dense kernel (matmul, matrix–vector, operator application) is a *measured* hot path | [`ndarray`](./playbooks/ndarray.md) / [`nalgebra`](./playbooks/nalgebra.md) |
| 3 | **Axis-reduction pressure** | sums/means over axes at scale are a bottleneck | [`ndarray`](./playbooks/ndarray.md) |
| 4 | **Linear-algebra pressure** | you need LU/QR/SVD, solvers, or eigenvalues (`matten` has none) | [`nalgebra`](./playbooks/nalgebra.md) |
| 5 | **Dataframe pressure** | you need group-by, joins, pivots, or query expressions | [Polars / Pandas](./playbooks/polars-and-pandas.md) |
| 6 | **ML / device pressure** | you need autodiff, training loops, or GPU/device execution | [Candle](./playbooks/candle.md) |
| 7 | **Dynamic-ingestion pressure** | you lean heavily on the `dynamic` feature beyond a one-time messy-data on-ramp | resolve to numeric, then 1–4 as they apply |
| 8 | **Dependency policy** | you *cannot* add heavy dependencies (binary size, audit, embedded) | **stay with `matten`** |
| 9 | **Target ecosystem preference** | the surrounding system is Rust, or specifically Python | Rust → 1–4; Python → [NumPy](./playbooks/python-numpy.md) |
| 10 | **Team language preference** | the team works in Python and wants the numeric code there | [NumPy](./playbooks/python-numpy.md) / [Pandas](./playbooks/polars-and-pandas.md) |

## Reading the result

- **Signals 1–3 dominate** → a dense-array hot path: [`ndarray`](./playbooks/ndarray.md).
- **Signal 4 is present** → you need capability `matten` lacks: [`nalgebra`](./playbooks/nalgebra.md).
- **Signal 5 dominates** → the work is tabular, not array math: Polars/Pandas. Remember
  `matten-data` will not grow these.
- **Signal 6 is present** → you have crossed into ML: Candle or another framework.
- **Signals 9–10 point to Python** → NumPy/Pandas, with `matten` as an upstream producer.
- **Signal 8 is "yes", or almost everything is "no"** → **stay with `matten`.** Adding a
  dependency would cost simplicity for no measured gain.

Migration is usually partial: move the signalled hot kernel, keep `matten` for construction,
ingestion, and glue. When you are ready to write the result down, use the
[readiness report template](./readiness-report.md); a filled-in example is in
[examples/](./examples/linear-regression-gd-readiness.md).
