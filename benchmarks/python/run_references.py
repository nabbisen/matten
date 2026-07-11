#!/usr/bin/env python3
"""RFC-049 Phase 3 Python reference report helper.

The default output is code-shape evidence: ELOC, dependency footprint, versions,
and concise notes. Runtime collection is intentionally not implemented here; if
it is added later, the report must collect matching matten and Python rows on the
same machine and label them as context.
"""

from __future__ import annotations

import argparse
import importlib.metadata as metadata
import importlib.util
import os
import platform
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PYTHON_DIR = ROOT / "benchmarks" / "python"
TASKS_DIR = PYTHON_DIR / "tasks"


@dataclass(frozen=True)
class Task:
    key: str
    title: str
    module: str
    rust_snippet_path: str
    rust_example_path: str
    python_deps: tuple[str, ...]
    note: str


TASKS: tuple[Task, ...] = (
    Task(
        "numpy-cosine-similarity",
        "Cosine similarity",
        "numpy_cosine_similarity.py",
        "benchmarks/python/matten/cosine_similarity.rs",
        "crates/matten/examples/26_cosine_similarity.rs",
        ("numpy",),
        "NumPy keeps vector math compact through array primitives and linalg helpers.",
    ),
    Task(
        "numpy-markov-chain",
        "Markov chain step",
        "numpy_markov_chain.py",
        "benchmarks/python/matten/markov_chain.rs",
        "crates/matten/examples/33_markov_chain_weather.rs",
        ("numpy",),
        "Both versions expose the transition-matrix shape directly.",
    ),
    Task(
        "numpy-tiny-pagerank",
        "Tiny PageRank step",
        "numpy_tiny_pagerank.py",
        "benchmarks/python/matten/tiny_pagerank.rs",
        "crates/matten/examples/34_tiny_pagerank.rs",
        ("numpy",),
        "NumPy expresses the dense update tersely; matten keeps the same shape story in Rust.",
    ),
    Task(
        "numpy-linear-regression-gd",
        "Linear regression GD step",
        "numpy_linear_regression_gd.py",
        "benchmarks/python/matten/linear_regression_gd.rs",
        "crates/matten/examples/35_linear_regression_gradient_descent.rs",
        ("numpy",),
        "NumPy is compact for matrix algebra; matten shows the loop in one Rust binary.",
    ),
    Task(
        "numpy-heat-equation-1d",
        "Heat equation 1D step",
        "numpy_heat_equation_1d.py",
        "benchmarks/python/matten/heat_equation_1d.rs",
        "crates/matten/examples/36_heat_equation_1d.rs",
        ("numpy",),
        "Both versions are dense-operator references, not optimized stencil kernels.",
    ),
    Task(
        "pandas-csv-to-matrix",
        "CSV to numeric matrix",
        "pandas_csv_to_matrix.py",
        "benchmarks/python/matten/csv_to_matrix.rs",
        "crates/matten-data/examples/data_00_quickstart.rs",
        ("pandas",),
        "Pandas owns broader dataframe ergonomics; matten-data stays explicit and narrow.",
    ),
)


def is_import_line(path: Path, line: str) -> bool:
    stripped = line.strip()
    if path.suffix == ".py":
        return stripped.startswith("import ") or stripped.startswith("from ")
    if path.suffix == ".rs":
        return stripped.startswith("use ") or stripped.startswith("extern crate ")
    return False


def eloc(path: Path) -> tuple[int, int]:
    with path.open(encoding="utf-8") as f:
        lines = f.readlines()
    without_imports = 0
    with_imports = 0
    in_block_comment = False
    in_py_docstring = False
    for raw in lines:
        stripped = raw.strip()
        if not stripped:
            continue
        if path.suffix == ".rs":
            if in_block_comment:
                if "*/" in stripped:
                    in_block_comment = False
                continue
            if stripped.startswith("/*") or stripped.startswith("//!"):
                if stripped.startswith("/*") and "*/" not in stripped:
                    in_block_comment = True
                continue
            if stripped.startswith("//"):
                continue
        elif path.suffix == ".py":
            if in_py_docstring:
                if stripped.endswith('"""') or stripped.endswith("'''"):
                    in_py_docstring = False
                continue
            if stripped.startswith('"""') or stripped.startswith("'''"):
                if not (
                    (stripped.endswith('"""') and len(stripped) > 3)
                    or (stripped.endswith("'''") and len(stripped) > 3)
                ):
                    in_py_docstring = True
                continue
            if stripped.startswith("#"):
                continue
        with_imports += 1
        if not is_import_line(path, stripped):
            without_imports += 1
    return without_imports, with_imports


def read_requirements() -> dict[str, str]:
    reqs: dict[str, str] = {}
    for line in (PYTHON_DIR / "requirements.txt").read_text(encoding="utf-8").splitlines():
        stripped = line.strip()
        if not stripped or stripped.startswith("#"):
            continue
        if "==" not in stripped:
            raise SystemExit(f"requirements.txt entry is not exactly pinned: {stripped}")
        name, version = stripped.split("==", 1)
        reqs[name.lower()] = version
    return reqs


def dependency_row(deps: tuple[str, ...], requirements: dict[str, str]) -> str:
    parts = []
    for dep in deps:
        pin = requirements.get(dep)
        installed = installed_version(dep)
        installed_text = installed if installed is not None else "not installed"
        pin_text = pin if pin is not None else "not pinned"
        transitive = transitive_count(dep) if installed is not None else "n/a"
        parts.append(f"{dep} pin {pin_text}; installed {installed_text}; transitive {transitive}")
    return "; ".join(parts)


def installed_version(package: str) -> str | None:
    try:
        return metadata.version(package)
    except metadata.PackageNotFoundError:
        return None


def requirement_name(req: str) -> str | None:
    if "extra ==" in req:
        return None
    name = req.split(";", 1)[0].split("[", 1)[0].strip()
    if not name:
        return None
    return name.split()[0].lower().replace("_", "-")


def runtime_dependency_names(package: str, seen: set[str] | None = None) -> set[str]:
    if seen is None:
        seen = set()
    try:
        dist = metadata.distribution(package)
    except metadata.PackageNotFoundError:
        return seen
    current = package.lower().replace("_", "-")
    names = set()
    for req in dist.requires or []:
        name = requirement_name(req)
        if name and name not in seen and name != current:
            names.add(name)
    for name in names:
        seen.add(name)
        runtime_dependency_names(name, seen)
    return seen


def transitive_count(package: str) -> int | str:
    if installed_version(package) is None:
        return "n/a"
    return len(runtime_dependency_names(package))


def load_task(task: Task):
    path = TASKS_DIR / task.module
    spec = importlib.util.spec_from_file_location(task.key.replace("-", "_"), path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"could not load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verify_task_imports(task: Task) -> str:
    missing = [dep for dep in task.python_deps if installed_version(dep) is None]
    if missing:
        return "missing " + ", ".join(missing)
    try:
        load_task(task)
    except Exception as exc:  # noqa: BLE001 - CLI should report import/runtime failures clearly.
        return f"import failed: {exc}"
    return "ok"


def task_rows(selected: list[Task]) -> str:
    requirements = read_requirements()
    rows = [
        "| Task | Python ELOC | matten ELOC | Dependencies | Import check | Code-shape note |",
        "|---|---:|---:|---|---|---|",
    ]
    for task in selected:
        py_without, py_with = eloc(TASKS_DIR / task.module)
        rust_without, rust_with = eloc(ROOT / task.rust_snippet_path)
        rows.append(
            "| {title} | {py_no}/{py_with} | {rs_no}/{rs_with} | {deps} | {check} | {note} |".format(
                title=task.title,
                py_no=py_without,
                py_with=py_with,
                rs_no=rust_without,
                rs_with=rust_with,
                deps=dependency_row(task.python_deps, requirements),
                check=verify_task_imports(task),
                note=task.note,
            )
        )
    return "\n".join(rows)


def environment_block() -> str:
    requirements = read_requirements()
    rows = [
        "| Field | Value |",
        "|---|---|",
        f"| Python | {platform.python_version()} |",
        f"| Platform | {platform.platform()} |",
        f"| Machine | {platform.machine()} |",
        f"| NumPy pin | {requirements.get('numpy', 'not pinned')} |",
        f"| NumPy installed | {installed_version('numpy') or 'not installed'} |",
        f"| Pandas pin | {requirements.get('pandas', 'not pinned')} |",
        f"| Pandas installed | {installed_version('pandas') or 'not installed'} |",
        f"| Working directory | {os.getcwd()} |",
    ]
    return "\n".join(rows)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    choices = [task.key for task in TASKS]
    parser.add_argument("--task", choices=choices, help="emit one task row")
    parser.add_argument("--all", action="store_true", help="emit every task row")
    parser.add_argument("--environment", action="store_true", help="emit environment/version rows")
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    if args.environment:
        print(environment_block())
        return 0
    if args.task:
        selected = [task for task in TASKS if task.key == args.task]
        print(task_rows(selected))
        return 0
    if args.all:
        print(task_rows(list(TASKS)))
        return 0
    print("No action selected. Use --help, --environment, --task, or --all.", file=sys.stderr)
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
