# Python reference comparison scripts

Optional RFC-049 Phase 3 reference scripts for NumPy/Pandas. These scripts are
for code-shape, ELOC, and dependency-footprint context, not for ranking `matten`
against Python ecosystems.

Setup may contact PyPI:

```bash
python3 -m venv .venv-bench-python
. .venv-bench-python/bin/activate
python3 -m pip install -r benchmarks/python/requirements.txt
```

Reference runs themselves must not access the network:

```bash
python3 benchmarks/python/run_references.py --help
python3 benchmarks/python/run_references.py --all
```

Runtime context is intentionally off by default. If it is enabled later, collect
matching `matten` and Python rows on the same machine and report them as context,
not ranking.

The `matten/` snippets are minimal task-equivalent Rust references used only for
ELOC comparison. They are not workspace examples; update them whenever the
mirrored examples or public APIs they use change.
