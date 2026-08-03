# Reports

`matten` includes a local development tool, `matten-report`, that renders small fixed
demonstrations of shape reasoning, dynamic-tensor readiness, and preprocessing over a handful of
built-in scenarios. The five pages under this section are its Markdown output, generated once and
committed so they are readable here without a checkout.

## What these are

- **Fixed demos, not a live tool.** Each page is the output of one `matten-report --demo <kind>`
  invocation against data baked into the tool. Nothing here runs in your browser, and nothing
  here reads data you provide — see the [Playground](../playground.md) for the page that does
  compute live, on shapes you enter.
- **Not automatic expression tracing.** Every page says so in its own `## Input` section. These
  demonstrate specific, hand-chosen operations — they do not observe or replay arbitrary code.

## What `matten-report` is — and is not

`matten-report` is a local development tool: `workspace-excluded`, `publish = false`, and never
published to crates.io. It is not a `matten` public API, and using it does not require depending
on anything beyond the crates you already use.

The tool can also render **HTML** and **JSON**, and can run against a CSV file you supply
(`--input <path> --kind data-readiness`). Neither the HTML/JSON output nor that live-input mode is
published here, or anywhere public — RFC-070 declined a public reporting or visualization
surface, and generating these five Markdown pages does not reopen that decision (RFC-097 §3). What
you are reading is rendered *output*, not an interface anything can build against.

## Running it yourself

```bash
# Any of the five fixed demos, Markdown to stdout:
cargo run --manifest-path tools/matten-report/Cargo.toml -- --demo shape-flow

# Against your own CSV:
cargo run --manifest-path tools/matten-report/Cargo.toml -- \
  --input your-data.csv --kind data-readiness --select column_a,column_b
```

## The five demos

- [`shape-flow`](./shape-flow.md) — broadcasting, reshape, axis reductions, and `matmul`, the
  same operations as the [Playground](../playground.md), shown as fixed output.
- [`educational-path`](./educational-path.md) — a longer walk through shape reasoning, dynamic
  readiness, and standardization in one report.
- [`mlprep-standardization`](./mlprep-standardization.md) — before/after column standardization.
- [`data-readiness`](./data-readiness.md) — CSV column selection, missing-value counts, and
  strict numeric conversion.
- [`dynamic-readiness`](./dynamic-readiness.md) — a mixed-type dynamic tensor, its readiness
  masks, and two conversion policies.

## Staying accurate

These pages are generated, not hand-maintained — regenerating them from the commands above must
reproduce them byte for byte. `scripts/check-report-demos.sh` enforces that in CI; if the tool's
output ever changes, these pages are regenerated and recommitted in the same change, never edited
by hand.
