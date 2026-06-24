# Benchmark results

This directory holds **small, curated** result artifacts only — not bulky raw
histories (RFC-049 Q3 ruling).

Commit, when useful:

- a small sample schema (for example `internal-baseline.sample.json`);
- a single `latest-summary.json` if a curated summary becomes useful.

Do **not** commit:

- `target/criterion/` output;
- large machine-specific raw JSON histories;
- an ever-growing archive of past runs.

Curated reports live in `../reports/`. Criterion's own raw output stays under the
(git-ignored) benchmark `target/` directory.
