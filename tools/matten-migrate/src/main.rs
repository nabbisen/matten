use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::error::Error;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const MAX_SCAN_BYTES: u64 = 256 * 1024;

const DISCLAIMER: &str = "This report is advisory. It does not prove production readiness, does not guarantee a target library is better, and does not perform automatic conversion.";
const DETECTION_LIMITS: &str = "Detection is a heuristic text/dependency scan. It may miss real matten usage and may over-report source-like text as usage. It has not been validated against real downstream projects; treat results as a starting point for manual review.";

const DATAFRAME_TERMS: &[&str] = &[
    "groupby(",
    "group_by(",
    "pivot(",
    "rolling(",
    "dataframe",
    "data_frame",
];

const TARGETS: &[(&str, &str)] = &[
    (
        "ndarray",
        "general Rust N-D arrays and dense numeric hot paths",
    ),
    (
        "nalgebra",
        "dense linear algebra, decompositions, and solvers",
    ),
    (
        "Polars / Pandas",
        "dataframe analytics such as joins, group-by, pivot, and query",
    ),
    ("Candle", "ML tensors, training, and device execution"),
    ("NumPy", "Python scientific ecosystem hand-off"),
    ("stay with matten", "small work, ingestion, glue, and learning"),
];

#[derive(Debug)]
enum Command {
    Inspect { path: PathBuf },
    Report { path: PathBuf, output: Option<PathBuf> },
    Suggest { target: Target, path: PathBuf },
    ListTargets,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Ndarray,
    Nalgebra,
    PolarsPandas,
    Candle,
    Numpy,
    StayWithMatten,
}

impl Target {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "ndarray" => Some(Self::Ndarray),
            "nalgebra" => Some(Self::Nalgebra),
            "polars-pandas" | "polars" | "pandas" => Some(Self::PolarsPandas),
            "candle" => Some(Self::Candle),
            "numpy" => Some(Self::Numpy),
            "stay-with-matten" | "stay" | "matten" => Some(Self::StayWithMatten),
            _ => None,
        }
    }

    fn slug(self) -> &'static str {
        match self {
            Self::Ndarray => "ndarray",
            Self::Nalgebra => "nalgebra",
            Self::PolarsPandas => "polars-pandas",
            Self::Candle => "candle",
            Self::Numpy => "numpy",
            Self::StayWithMatten => "stay-with-matten",
        }
    }

    fn display(self) -> &'static str {
        match self {
            Self::Ndarray => "ndarray",
            Self::Nalgebra => "nalgebra",
            Self::PolarsPandas => "Polars / Pandas",
            Self::Candle => "Candle",
            Self::Numpy => "NumPy",
            Self::StayWithMatten => "stay with matten",
        }
    }
}

#[derive(Debug, Default)]
struct Analysis {
    project_name: String,
    detected_crates: BTreeSet<String>,
    signals: BTreeMap<&'static str, BTreeSet<String>>,
    warnings: Vec<String>,
    files_scanned: usize,
    cargo_files_scanned: usize,
    rust_files_scanned: usize,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("matten-migrate error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    match parse_args(env::args().skip(1))? {
        Command::Inspect { path } => {
            let analysis = analyze_path(&path)?;
            print!("{}", render_inspect(&analysis));
        }
        Command::Report { path, output } => {
            let analysis = analyze_path(&path)?;
            let report = render_report(&analysis);
            if let Some(output) = output {
                fs::write(output, report)?;
            } else {
                print!("{report}");
            }
        }
        Command::Suggest { target, path } => {
            let analysis = analyze_path(&path)?;
            print!("{}", render_suggest(&analysis, target));
        }
        Command::ListTargets => print!("{}", render_targets()),
        Command::Help => print!("{}", usage()),
    }
    Ok(())
}

fn parse_args<I>(args: I) -> Result<Command, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args: Vec<String> = args.into_iter().collect();
    if args.is_empty() {
        return Err(usage());
    }
    if args.len() == 1 && matches!(args[0].as_str(), "--help" | "-h" | "help") {
        return Ok(Command::Help);
    }

    let command = args.remove(0);
    match command.as_str() {
        "inspect" => {
            if args.len() != 1 {
                return Err(format!("inspect expects exactly one path\n\n{}", usage()));
            }
            Ok(Command::Inspect {
                path: PathBuf::from(&args[0]),
            })
        }
        "report" => parse_report_args(args),
        "suggest" => parse_suggest_args(args),
        "list-targets" => {
            if !args.is_empty() {
                return Err(format!("list-targets does not accept arguments\n\n{}", usage()));
            }
            Ok(Command::ListTargets)
        }
        "rewrite" | "apply" | "explain-api" | "check-bridges" => Err(format!(
            "{command:?} is not supported in this local advisory tool\n\n{}",
            usage()
        )),
        other => Err(format!("unknown command: {other}\n\n{}", usage())),
    }
}

fn parse_report_args(args: Vec<String>) -> Result<Command, String> {
    if args.is_empty() {
        return Err(format!("report expects a path\n\n{}", usage()));
    }

    let mut path: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--output" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--output requires a path".to_string())?;
                output = Some(PathBuf::from(value));
            }
            "--help" | "-h" => return Ok(Command::Help),
            value if value.starts_with("--") => {
                return Err(format!("unknown report argument: {value}\n\n{}", usage()));
            }
            value => {
                if path.is_some() {
                    return Err(format!("report accepts only one input path\n\n{}", usage()));
                }
                path = Some(PathBuf::from(value));
            }
        }
    }

    let path = path.ok_or_else(|| format!("report expects a path\n\n{}", usage()))?;
    Ok(Command::Report { path, output })
}

fn parse_suggest_args(args: Vec<String>) -> Result<Command, String> {
    if args.is_empty() {
        return Err(format!(
            "suggest expects --target <target> and one path\n\n{}",
            usage()
        ));
    }

    let mut target: Option<Target> = None;
    let mut path: Option<PathBuf> = None;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--target" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--target requires a target".to_string())?;
                target = Some(Target::parse(&value).ok_or_else(|| {
                    format!("unsupported target: {value}\n\n{}", supported_targets())
                })?);
            }
            "--all" => return Err("suggest --all is not supported in this slice".to_string()),
            "--output" => return Err("suggest --output is not supported in this slice".to_string()),
            "--edit" => return Err("suggest --edit is not supported in this slice".to_string()),
            "--json" => return Err("suggest --json is not supported in this slice".to_string()),
            "--help" | "-h" => return Ok(Command::Help),
            value if value.starts_with("--") => {
                return Err(format!("unknown suggest argument: {value}\n\n{}", usage()));
            }
            value => {
                if path.is_some() {
                    return Err(format!("suggest accepts only one input path\n\n{}", usage()));
                }
                path = Some(PathBuf::from(value));
            }
        }
    }

    let target = target.ok_or_else(|| format!("suggest requires --target <target>\n\n{}", usage()))?;
    let path = path.ok_or_else(|| format!("suggest expects a path\n\n{}", usage()))?;
    Ok(Command::Suggest { target, path })
}

fn supported_targets() -> String {
    "\
Supported targets:
  ndarray
  nalgebra
  polars-pandas
  candle
  numpy
  stay-with-matten

Aliases:
  polars, pandas, stay, matten
"
    .to_string()
}

fn usage() -> String {
    "\
Usage:
  matten-migrate inspect <path>
  matten-migrate report <path> [--output <path>]
  matten-migrate suggest --target <target> <path>
  matten-migrate list-targets

This local tool is advisory and non-mutating except for report --output.
It does not support rewrite/apply/explain-api/check-bridges.
"
    .to_string()
}

fn analyze_path(path: &Path) -> Result<Analysis, Box<dyn Error>> {
    let metadata = fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err("input path must not be a symlink".into());
    }

    let root = if metadata.is_dir() {
        path.to_path_buf()
    } else {
        path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf()
    };
    let canonical_root = fs::canonicalize(&root)?;
    let mut analysis = Analysis {
        project_name: project_name(path),
        ..Analysis::default()
    };

    if metadata.is_file() {
        scan_file(path, &canonical_root, &mut analysis)?;
    } else {
        let mut files = Vec::new();
        collect_files(path, &canonical_root, &mut files, &mut analysis)?;
        files.sort();
        for file in files {
            scan_file(&file, &canonical_root, &mut analysis)?;
        }
    }

    Ok(analysis)
}

fn project_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or(".")
        .to_string()
}

fn collect_files(
    dir: &Path,
    canonical_root: &Path,
    files: &mut Vec<PathBuf>,
    analysis: &mut Analysis,
) -> Result<(), Box<dyn Error>> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(dir)? {
        entries.push(entry?);
    }
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        let meta = fs::symlink_metadata(&path)?;
        if meta.file_type().is_symlink() {
            analysis.warnings.push(format!(
                "skipped symlink: {}",
                relative_display(canonical_root, &path)
            ));
            continue;
        }
        if meta.is_dir() {
            let canonical = fs::canonicalize(&path)?;
            if !canonical.starts_with(canonical_root) {
                analysis.warnings.push(format!(
                    "skipped directory outside inspected path: {}",
                    path.display()
                ));
                continue;
            }
            collect_files(&path, canonical_root, files, analysis)?;
        } else if meta.is_file() && is_scannable_file(&path) {
            files.push(path);
        }
    }
    Ok(())
}

fn is_scannable_file(path: &Path) -> bool {
    path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml")
        || path.extension().and_then(|ext| ext.to_str()) == Some("rs")
}

fn scan_file(
    path: &Path,
    canonical_root: &Path,
    analysis: &mut Analysis,
) -> Result<(), Box<dyn Error>> {
    let meta = fs::metadata(path)?;
    if meta.len() > MAX_SCAN_BYTES {
        analysis.warnings.push(format!(
            "skipped large file: {}",
            relative_display(canonical_root, path)
        ));
        return Ok(());
    }

    let text = fs::read_to_string(path)?;
    analysis.files_scanned += 1;
    if path.file_name().and_then(|name| name.to_str()) == Some("Cargo.toml") {
        analysis.cargo_files_scanned += 1;
        scan_cargo_toml(&text, path, canonical_root, analysis);
    } else if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
        analysis.rust_files_scanned += 1;
        scan_rust_source(&text, path, canonical_root, analysis);
    }
    Ok(())
}

fn scan_cargo_toml(text: &str, path: &Path, root: &Path, analysis: &mut Analysis) {
    for (line_idx, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        let evidence = evidence(path, root, line_idx + 1);
        for (crate_name, keys) in [
            ("matten", &["matten ="][..]),
            ("matten-data", &["matten-data =", "matten_data ="][..]),
            (
                "matten-ndarray",
                &["matten-ndarray =", "matten_ndarray ="][..],
            ),
            ("matten-mlprep", &["matten-mlprep =", "matten_mlprep ="][..]),
        ] {
            if keys.iter().any(|key| trimmed.contains(key)) {
                analysis.detected_crates.insert(crate_name.to_string());
                analysis
                    .signals
                    .entry("detected crates/features")
                    .or_default()
                    .insert(format!("{crate_name} dependency ({evidence})"));
            }
        }
        if trimmed.contains("dynamic") && trimmed.contains("features") {
            analysis
                .signals
                .entry("detected crates/features")
                .or_default()
                .insert(format!("dynamic feature mention ({evidence})"));
        }
    }
}

fn scan_rust_source(text: &str, path: &Path, root: &Path, analysis: &mut Analysis) {
    for (line_idx, line) in text.lines().enumerate() {
        if is_comment_line(line) {
            continue;
        }
        let evidence = evidence(path, root, line_idx + 1);

        scan_terms(
            line,
            &[
                "Tensor::new",
                "Tensor::try_new",
                "Tensor::from_vec",
                "Tensor::scalar",
                "Tensor::zeros",
                "Tensor::ones",
            ],
            "core Tensor construction",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &["reshape", "flatten", "transpose", "squeeze", "expand_dims"],
            "shape operations",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &[
                "sum_axis",
                "mean_axis",
                "min_axis",
                "max_axis",
                ".sum(",
                ".mean(",
                ".var(",
            ],
            "reductions",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &["matmul", ".dot(", ".outer(", ".norm(", ".trace("],
            "linear algebra",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &[
                "Element",
                "NumericPolicy",
                "try_numeric",
                "from_json_dynamic",
                "from_csv_dynamic",
            ],
            "dynamic ingestion",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &[
                "matten_data",
                "Table",
                "select_columns",
                "fill_missing",
                "try_numeric",
                "to_tensor",
            ],
            "matten-data",
            &evidence,
            analysis,
        );
        scan_terms(
            line,
            &["matten_ndarray", "to_arrayd", "from_arrayd"],
            "matten-ndarray bridge",
            &evidence,
            analysis,
        );
        scan_terms(line, DATAFRAME_TERMS, "dataframe pressure", &evidence, analysis);
    }
}

fn scan_terms(
    line: &str,
    terms: &[&str],
    category: &'static str,
    evidence: &str,
    analysis: &mut Analysis,
) {
    for term in terms {
        if contains_code_term(line, term) {
            let label = display_term(term);
            analysis
                .signals
                .entry(category)
                .or_default()
                .insert(format!("{label} ({evidence})"));
        }
    }
}

fn display_term(term: &str) -> &str {
    term.trim_start_matches('.').trim_end_matches('(')
}

fn contains_code_term(line: &str, term: &str) -> bool {
    let mut start = 0;
    while let Some(offset) = line[start..].find(term) {
        let idx = start + offset;
        if !inside_string_literal(line, idx) && has_token_boundaries(line, idx, term) {
            return true;
        }
        start = idx + term.len();
    }
    false
}

fn has_token_boundaries(line: &str, idx: usize, term: &str) -> bool {
    let before = line[..idx].chars().next_back();
    let after = line[idx + term.len()..].chars().next();
    let before_matches = term.chars().next().is_some_and(is_term_delimiter) || is_boundary(before);
    let after_matches = term
        .chars()
        .next_back()
        .is_some_and(is_term_delimiter)
        || is_boundary(after);
    before_matches && after_matches
}

fn is_boundary(ch: Option<char>) -> bool {
    match ch {
        None => true,
        Some(ch) => !(ch.is_ascii_alphanumeric() || ch == '_'),
    }
}

fn is_term_delimiter(ch: char) -> bool {
    matches!(ch, '(' | '.' | ':' | '<' | '[')
}

fn inside_string_literal(line: &str, idx: usize) -> bool {
    let mut in_string = false;
    let mut escaped = false;
    for ch in line[..idx].chars() {
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == '"' {
            in_string = !in_string;
        }
    }
    in_string
}

fn is_comment_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*')
}

fn evidence(path: &Path, root: &Path, line: usize) -> String {
    format!("{}:{line}", relative_display(root, path))
}

fn relative_display(root: &Path, path: &Path) -> String {
    let canonical = fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf());
    canonical
        .strip_prefix(root)
        .unwrap_or(&canonical)
        .to_string_lossy()
        .replace('\\', "/")
}

fn render_inspect(analysis: &Analysis) -> String {
    let mut out = String::new();
    writeln!(out, "matten-migrate inspect").unwrap();
    writeln!(out, "project: {}", analysis.project_name).unwrap();
    writeln!(out, "files scanned: {}", analysis.files_scanned).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "status: {}", status_line(analysis)).unwrap();
    writeln!(out).unwrap();
    write_signal_summary(&mut out, analysis);
    writeln!(out, "limitations:").unwrap();
    writeln!(out, "- {DETECTION_LIMITS}").unwrap();
    out
}

fn render_report(analysis: &Analysis) -> String {
    let mut out = String::new();
    writeln!(out, "# matten Migration Readiness Report").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "> {DISCLAIMER}").unwrap();
    writeln!(out, "> {DETECTION_LIMITS}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Summary").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Project: `{}`.", analysis.project_name).unwrap();
    writeln!(out, "{}", summary_sentence(analysis)).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Detected matten usage").unwrap();
    writeln!(out).unwrap();
    write_signal_summary(&mut out, analysis);
    writeln!(out, "## Production pressure signals").unwrap();
    writeln!(out).unwrap();
    if has_signal(analysis, "dataframe pressure") {
        writeln!(
            out,
            "- Dataframe pressure terms appear with local evidence; manual review should decide whether table analytics belong outside `matten-data`."
        )
        .unwrap();
    }
    if has_signal(analysis, "linear algebra") || has_signal(analysis, "reductions") {
        writeln!(
            out,
            "- Numeric hot-path pressure may exist if these operations dominate real workloads; profile before deciding."
        )
        .unwrap();
    }
    if !has_signal(analysis, "dataframe pressure")
        && !has_signal(analysis, "linear algebra")
        && !has_signal(analysis, "reductions")
    {
        writeln!(
            out,
            "- No strong production pressure detected by this heuristic scan."
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Suggested target playbooks").unwrap();
    writeln!(out).unwrap();
    for suggestion in suggestions(analysis) {
        writeln!(out, "- {suggestion}").unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Direct conversion candidates").unwrap();
    writeln!(out).unwrap();
    write_direct_candidates(&mut out, analysis);
    writeln!(out).unwrap();
    writeln!(out, "## Manual redesign areas").unwrap();
    writeln!(out).unwrap();
    write_manual_redesign(&mut out, analysis);
    writeln!(out).unwrap();
    writeln!(out, "## Bridge crates / tools").unwrap();
    writeln!(out).unwrap();
    write_bridge_tools(&mut out, analysis);
    writeln!(out).unwrap();
    writeln!(out, "## Risks").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "- Heuristic detection can miss usage or report source-like text; verify every finding manually.").unwrap();
    writeln!(out, "- Avoid converting inside hot loops; convert once at boundaries when using bridge crates.").unwrap();
    writeln!(out, "- Treat target playbooks as reading suggestions, not migration decisions.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Next steps").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "1. Review the detected usage manually.").unwrap();
    writeln!(out, "2. Read the suggested playbook sections.").unwrap();
    writeln!(out, "3. Profile real workloads before moving any hot path.").unwrap();
    writeln!(out, "4. Keep `matten` where small, readable glue is still enough.").unwrap();
    if !analysis.warnings.is_empty() {
        writeln!(out).unwrap();
        writeln!(out, "## Scan warnings").unwrap();
        writeln!(out).unwrap();
        for warning in &analysis.warnings {
            writeln!(out, "- {warning}").unwrap();
        }
    }
    out
}

fn render_suggest(analysis: &Analysis, target: Target) -> String {
    let mut out = String::new();
    writeln!(out, "# matten Migration Target Suggestion").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "> {DISCLAIMER}").unwrap();
    writeln!(out, "> {DETECTION_LIMITS}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "Target: `{}`.", target.slug()).unwrap();
    writeln!(out, "Project: `{}`.", analysis.project_name).unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Local evidence").unwrap();
    writeln!(out).unwrap();
    write_target_evidence(&mut out, analysis, target);
    writeln!(out).unwrap();
    writeln!(out, "## Target fit notes").unwrap();
    writeln!(out).unwrap();
    write_target_fit_notes(&mut out, analysis, target);
    writeln!(out).unwrap();
    writeln!(out, "## Manual checks").unwrap();
    writeln!(out).unwrap();
    write_manual_checks_for_target(&mut out, target);
    writeln!(out).unwrap();
    writeln!(out, "## Risks").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "- Treat this as a reading aid for the `{}` playbook, not a migration decision.", target.display()).unwrap();
    writeln!(out, "- Heuristic detection can miss usage or report source-like text; verify every finding manually.").unwrap();
    writeln!(out, "- Profile before moving hot paths or adding bridge conversions.").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Next steps").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "1. Review the local evidence manually.").unwrap();
    writeln!(out, "2. Read the `{}` playbook if the notes match real requirements.", target.display()).unwrap();
    writeln!(out, "3. Keep `matten` where small, readable glue is enough.").unwrap();
    out
}

fn write_target_evidence(out: &mut String, analysis: &Analysis, target: Target) {
    let categories = evidence_categories(target);
    let mut wrote = false;
    for category in categories {
        if let Some(signals) = analysis.signals.get(category) {
            if signals.is_empty() {
                continue;
            }
            wrote = true;
            writeln!(out, "- {category}:").unwrap();
            for signal in signals {
                writeln!(out, "  - {signal}").unwrap();
            }
        }
    }

    if !wrote {
        writeln!(
            out,
            "- no strong local evidence for `{}` in this heuristic scan",
            target.slug()
        )
        .unwrap();
    }
}

fn evidence_categories(target: Target) -> &'static [&'static str] {
    match target {
        Target::Ndarray => &[
            "shape operations",
            "reductions",
            "matten-ndarray bridge",
            "core Tensor construction",
        ],
        Target::Nalgebra => &["linear algebra"],
        Target::PolarsPandas => &["matten-data", "dataframe pressure"],
        Target::Candle => &[],
        Target::Numpy => &[],
        Target::StayWithMatten => &[
            "core Tensor construction",
            "dynamic ingestion",
            "matten-data",
            "detected crates/features",
        ],
    }
}

fn write_target_fit_notes(out: &mut String, analysis: &Analysis, target: Target) {
    match target {
        Target::Ndarray => {
            if has_signal(analysis, "shape operations")
                || has_signal(analysis, "reductions")
                || has_signal(analysis, "matten-ndarray bridge")
            {
                writeln!(out, "- `ndarray` may be relevant when dense N-D operations become measured hot paths.").unwrap();
            } else {
                writeln!(out, "- No strong local `ndarray` pressure was detected by this scan.").unwrap();
            }
            writeln!(out, "- If bridge evidence appears, review copy boundaries and the `f64` conversion contract.").unwrap();
        }
        Target::Nalgebra => {
            if has_signal(analysis, "linear algebra") {
                writeln!(out, "- `nalgebra` may be relevant for dense matrix/vector redesigns and solver needs.").unwrap();
            } else {
                writeln!(out, "- No strong local `nalgebra` pressure was detected by this scan.").unwrap();
            }
            writeln!(out, "- Manual review should decide whether fixed or dynamic dimensions matter.").unwrap();
        }
        Target::PolarsPandas => {
            if has_signal(analysis, "matten-data") && has_signal(analysis, "dataframe pressure") {
                writeln!(out, "- `Polars / Pandas` may be relevant when table analytics such as group-by, join, pivot, or query are real requirements.").unwrap();
            } else if has_signal(analysis, "matten-data") {
                writeln!(out, "- `matten-data` appears, but dataframe pressure was not detected; do not treat ingestion alone as a Polars / Pandas reason.").unwrap();
            } else {
                writeln!(out, "- No strong local Polars / Pandas pressure was detected by this scan.").unwrap();
            }
            writeln!(out, "- Manual review should decide whether table work belongs outside `matten-data`.").unwrap();
        }
        Target::Candle => {
            writeln!(out, "- No strong local `Candle` evidence is detected in this slice unless explicit training, device, or model terms are added by a reviewed scanner refinement.").unwrap();
            writeln!(out, "- Do not treat a single `matmul` or `dot` occurrence as ML pressure.").unwrap();
        }
        Target::Numpy => {
            writeln!(out, "- No strong local `NumPy` evidence is detected unless the project explicitly shows a Python ecosystem handoff.").unwrap();
            writeln!(out, "- Manual review should decide whether downstream work belongs next to Python data or science tooling.").unwrap();
        }
        Target::StayWithMatten => {
            if !has_signal(analysis, "dataframe pressure")
                && !has_signal(analysis, "linear algebra")
                && !has_signal(analysis, "reductions")
            {
                writeln!(out, "- Staying with `matten` may be relevant when no strong production pressure appears.").unwrap();
            } else {
                writeln!(out, "- Staying with `matten` can still be valid if the work remains small, readable, and not a measured hot path.").unwrap();
            }
            writeln!(out, "- Dynamic ingestion can remain useful as cleanup before numeric conversion.").unwrap();
        }
    }
}

fn write_manual_checks_for_target(out: &mut String, target: Target) {
    let checks = match target {
        Target::Ndarray => &[
            "rank and shape assumptions",
            "conversion boundary frequency",
            "whether N-D operations dominate real workloads",
        ][..],
        Target::Nalgebra => &[
            "fixed versus dynamic dimensions",
            "solver or decomposition needs",
            "whether matrix/vector semantics are clearer outside core matten",
        ],
        Target::PolarsPandas => &[
            "whether table work is more than simple ingestion",
            "whether group-by, join, pivot, or query are real requirements",
            "whether `matten-data` should stay as a table-to-Tensor boundary",
        ],
        Target::Candle => &[
            "training loop requirements",
            "device execution requirements",
            "model serialization requirements",
        ],
        Target::Numpy => &[
            "whether downstream work belongs in Python",
            "data interchange boundaries",
            "team and deployment constraints",
        ],
        Target::StayWithMatten => &[
            "readability",
            "workload size",
            "whether profiling shows real hot paths",
        ],
    };

    for check in checks {
        writeln!(out, "- {check}").unwrap();
    }
}

fn status_line(analysis: &Analysis) -> &'static str {
    if analysis.signals.is_empty() {
        "no matten usage detected"
    } else {
        "matten usage evidence detected"
    }
}

fn summary_sentence(analysis: &Analysis) -> &'static str {
    if analysis.signals.is_empty() {
        "No `matten` usage was detected. Treat this as a heuristic result, not proof."
    } else {
        "`matten` usage evidence was detected. Read suggested target playbooks as a starting point for manual review."
    }
}

fn write_signal_summary(out: &mut String, analysis: &Analysis) {
    writeln!(out, "detected crates:").unwrap();
    if analysis.detected_crates.is_empty() {
        writeln!(out, "- none").unwrap();
    } else {
        for crate_name in &analysis.detected_crates {
            writeln!(out, "- {crate_name}").unwrap();
        }
    }
    writeln!(out).unwrap();
    writeln!(out, "signals:").unwrap();
    if analysis.signals.is_empty() {
        writeln!(out, "- none").unwrap();
    } else {
        for (category, signals) in &analysis.signals {
            writeln!(out, "- {category}:").unwrap();
            for signal in signals {
                writeln!(out, "  - {signal}").unwrap();
            }
        }
    }
    writeln!(out).unwrap();
}

fn has_signal(analysis: &Analysis, category: &str) -> bool {
    analysis
        .signals
        .get(category)
        .is_some_and(|signals| !signals.is_empty())
}

fn suggestions(analysis: &Analysis) -> Vec<&'static str> {
    let mut out = Vec::new();
    if has_signal(analysis, "matten-ndarray bridge") {
        out.push("ndarray: read the bridge playbook; current code already mentions `matten-ndarray`.");
    } else if has_signal(analysis, "shape operations") || has_signal(analysis, "reductions") {
        out.push("ndarray: consider reading this playbook if dense N-D operations become a measured hot path.");
    }
    if has_signal(analysis, "linear algebra") {
        out.push("nalgebra: consider reading this playbook for dense matrix/vector redesigns and solvers.");
    }
    if has_signal(analysis, "matten-data") && has_signal(analysis, "dataframe pressure") {
        out.push("Polars / Pandas: read this playbook if table analytics such as join/group-by/pivot/query are real requirements.");
    }
    if has_signal(analysis, "dynamic ingestion") {
        out.push("stay with matten: dynamic ingestion may still be appropriate for explicit cleanup before numeric conversion.");
    }
    if out.is_empty() {
        out.push("stay with matten: no strong migration pressure was detected by this heuristic scan.");
    }
    out
}

fn write_direct_candidates(out: &mut String, analysis: &Analysis) {
    if has_signal(analysis, "matten-ndarray bridge") {
        writeln!(out, "- Existing `to_arrayd` / `from_arrayd` mentions are direct bridge candidates.").unwrap();
    }
    if has_signal(analysis, "linear algebra") {
        writeln!(out, "- `matmul` / `dot` / core-lite linalg mentions may map to ndarray or nalgebra after manual review.").unwrap();
    }
    if has_signal(analysis, "matten-data") {
        writeln!(out, "- `matten-data` table-to-Tensor preparation can stay as an ingestion boundary unless dataframe pressure is real.").unwrap();
    }
    if !has_signal(analysis, "matten-ndarray bridge")
        && !has_signal(analysis, "linear algebra")
        && !has_signal(analysis, "matten-data")
    {
        writeln!(out, "- none detected").unwrap();
    }
}

fn write_manual_redesign(out: &mut String, analysis: &Analysis) {
    if has_signal(analysis, "dataframe pressure") {
        writeln!(out, "- Dataframe-like table work is a design move, not a mechanical `matten-data` conversion.").unwrap();
    }
    if has_signal(analysis, "linear algebra") {
        writeln!(out, "- Solver/decomposition needs require manual redesign; core `matten` does not provide those APIs.").unwrap();
    }
    if !has_signal(analysis, "dataframe pressure") && !has_signal(analysis, "linear algebra") {
        writeln!(out, "- none identified by this heuristic scan").unwrap();
    }
}

fn write_bridge_tools(out: &mut String, analysis: &Analysis) {
    if has_signal(analysis, "matten-ndarray bridge") {
        writeln!(out, "- `matten-ndarray`: use `to_arrayd` / `from_arrayd`; copies both ways and stays `f64`.").unwrap();
    }
    if has_signal(analysis, "matten-data") {
        writeln!(out, "- `matten-data`: keep as a small CSV/table-to-Tensor on-ramp, not a dataframe engine.").unwrap();
    }
    if !has_signal(analysis, "matten-ndarray bridge") && !has_signal(analysis, "matten-data") {
        writeln!(out, "- none detected").unwrap();
    }
}

fn render_targets() -> String {
    let mut out = String::new();
    writeln!(out, "matten migration target playbooks").unwrap();
    for (name, description) in TARGETS {
        writeln!(out, "- {name}: {description}").unwrap();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("fixtures")
            .join(name)
    }

    #[test]
    fn simple_core_report_matches_expected_output() {
        let analysis = analyze_path(&fixture("simple-core-project")).unwrap();
        let report = render_report(&analysis);
        let expected = "\
# matten Migration Readiness Report

> This report is advisory. It does not prove production readiness, does not guarantee a target library is better, and does not perform automatic conversion.
> Detection is a heuristic text/dependency scan. It may miss real matten usage and may over-report source-like text as usage. It has not been validated against real downstream projects; treat results as a starting point for manual review.

## Summary

Project: `simple-core-project`.
`matten` usage evidence was detected. Read suggested target playbooks as a starting point for manual review.

## Detected matten usage

detected crates:
- matten

signals:
- core Tensor construction:
  - Tensor::new (src/main.rs:4)
- detected crates/features:
  - matten dependency (Cargo.toml:7)
- reductions:
  - mean_axis (src/main.rs:6)
- shape operations:
  - reshape (src/main.rs:5)

## Production pressure signals

- Numeric hot-path pressure may exist if these operations dominate real workloads; profile before deciding.

## Suggested target playbooks

- ndarray: consider reading this playbook if dense N-D operations become a measured hot path.

## Direct conversion candidates

- none detected

## Manual redesign areas

- none identified by this heuristic scan

## Bridge crates / tools

- none detected

## Risks

- Heuristic detection can miss usage or report source-like text; verify every finding manually.
- Avoid converting inside hot loops; convert once at boundaries when using bridge crates.
- Treat target playbooks as reading suggestions, not migration decisions.

## Next steps

1. Review the detected usage manually.
2. Read the suggested playbook sections.
3. Profile real workloads before moving any hot path.
4. Keep `matten` where small, readable glue is still enough.
";
        assert_eq!(report, expected);
        assert!(report.contains("Detection is a heuristic text/dependency scan"));
        assert!(!report.contains("must migrate"));
        assert!(!report.contains("guaranteed"));
        assert!(!report.contains("faster"));
        assert!(!report.contains("best target"));
    }

    #[test]
    fn no_matten_project_reports_no_usage_not_error() {
        let analysis = analyze_path(&fixture("no-matten-project")).unwrap();
        assert!(analysis.signals.is_empty());
        assert_eq!(status_line(&analysis), "no matten usage detected");
    }

    #[test]
    fn messy_nonproject_comments_and_strings_are_not_high_confidence() {
        let analysis = analyze_path(&fixture("messy-nonproject")).unwrap();
        assert!(analysis.signals.is_empty(), "{:?}", analysis.signals);
    }

    #[test]
    fn data_project_requires_dataframe_pressure_for_polars_pandas() {
        let analysis = analyze_path(&fixture("data-project")).unwrap();
        assert!(has_signal(&analysis, "matten-data"));
        assert!(has_signal(&analysis, "dataframe pressure"));
        assert!(suggestions(&analysis)
            .iter()
            .any(|suggestion| suggestion.starts_with("Polars / Pandas")));
    }

    #[test]
    fn common_rust_terms_do_not_trigger_pressure_signals() {
        let analysis = analyze_path(&fixture("common-rust-collisions-project")).unwrap();
        assert!(has_signal(&analysis, "matten-data"));
        assert!(!has_signal(&analysis, "dataframe pressure"));
        assert!(!has_signal(&analysis, "reductions"));
        assert!(!has_signal(&analysis, "linear algebra"));
        assert!(!suggestions(&analysis)
            .iter()
            .any(|suggestion| suggestion.starts_with("Polars / Pandas")));
    }

    #[test]
    fn receiver_method_calls_trigger_numeric_pressure_signals() {
        let analysis = analyze_path(&fixture("receiver-method-project")).unwrap();
        assert!(has_signal(&analysis, "reductions"));
        assert!(has_signal(&analysis, "linear algebra"));
        assert!(analysis
            .signals
            .get("reductions")
            .unwrap()
            .iter()
            .any(|signal| signal.starts_with("sum ")));
        assert!(analysis
            .signals
            .get("linear algebra")
            .unwrap()
            .iter()
            .any(|signal| signal.starts_with("dot ")));
    }

    #[test]
    fn ndarray_suggestion_matches_expected_output() {
        let analysis = analyze_path(&fixture("receiver-method-project")).unwrap();
        let suggestion = render_suggest(&analysis, Target::Ndarray);
        let expected = "\
# matten Migration Target Suggestion

> This report is advisory. It does not prove production readiness, does not guarantee a target library is better, and does not perform automatic conversion.
> Detection is a heuristic text/dependency scan. It may miss real matten usage and may over-report source-like text as usage. It has not been validated against real downstream projects; treat results as a starting point for manual review.

Target: `ndarray`.
Project: `receiver-method-project`.

## Local evidence

- reductions:
  - mean (src/main.rs:7)
  - sum (src/main.rs:6)
- core Tensor construction:
  - Tensor::new (src/main.rs:4)
  - Tensor::new (src/main.rs:5)

## Target fit notes

- `ndarray` may be relevant when dense N-D operations become measured hot paths.
- If bridge evidence appears, review copy boundaries and the `f64` conversion contract.

## Manual checks

- rank and shape assumptions
- conversion boundary frequency
- whether N-D operations dominate real workloads

## Risks

- Treat this as a reading aid for the `ndarray` playbook, not a migration decision.
- Heuristic detection can miss usage or report source-like text; verify every finding manually.
- Profile before moving hot paths or adding bridge conversions.

## Next steps

1. Review the local evidence manually.
2. Read the `ndarray` playbook if the notes match real requirements.
3. Keep `matten` where small, readable glue is enough.
";
        assert_eq!(suggestion, expected);
        assert!(suggestion.contains("This report is advisory"));
        assert!(!suggestion.contains("must migrate"));
        assert!(!suggestion.contains("best target"));
        assert!(!suggestion.contains("guaranteed"));
        assert!(!suggestion.contains("faster"));
        assert!(!suggestion.contains("drop-in replacement"));
    }

    #[test]
    fn polars_pandas_suggestion_requires_dataframe_pressure() {
        let collision = analyze_path(&fixture("common-rust-collisions-project")).unwrap();
        let collision_suggestion = render_suggest(&collision, Target::PolarsPandas);
        assert!(!has_signal(&collision, "dataframe pressure"));
        assert!(collision_suggestion.contains("do not treat ingestion alone"));
        assert!(!collision_suggestion.contains("may be relevant when table analytics"));

        let data = analyze_path(&fixture("data-project")).unwrap();
        let data_suggestion = render_suggest(&data, Target::PolarsPandas);
        assert!(has_signal(&data, "dataframe pressure"));
        assert!(data_suggestion.contains("may be relevant when table analytics"));
    }

    #[test]
    fn stay_with_matten_suggestion_handles_simple_usage() {
        let analysis = analyze_path(&fixture("simple-core-project")).unwrap();
        let suggestion = render_suggest(&analysis, Target::StayWithMatten);
        assert!(suggestion.contains("Staying with `matten` can still be valid"));
        assert!(suggestion.contains("Keep `matten` where small, readable glue is enough."));
    }

    #[test]
    fn ndarray_bridge_points_to_bridge_not_automatic_migration() {
        let analysis = analyze_path(&fixture("ndarray-bridge-project")).unwrap();
        assert!(has_signal(&analysis, "matten-ndarray bridge"));
        let report = render_report(&analysis);
        assert!(report.contains("Existing `to_arrayd` / `from_arrayd` mentions"));
        assert!(!report.contains("automatic migration"));
    }

    #[test]
    fn list_targets_is_deterministic() {
        let targets = render_targets();
        assert_eq!(
            targets,
            "\
matten migration target playbooks
- ndarray: general Rust N-D arrays and dense numeric hot paths
- nalgebra: dense linear algebra, decompositions, and solvers
- Polars / Pandas: dataframe analytics such as joins, group-by, pivot, and query
- Candle: ML tensors, training, and device execution
- NumPy: Python scientific ecosystem hand-off
- stay with matten: small work, ingestion, glue, and learning
"
        );
    }

    #[test]
    fn parse_rejects_deferred_commands() {
        let err = parse_args(["rewrite".to_string()]).unwrap_err();
        assert!(err.contains("not supported"));
        let err = parse_args(["explain-api".to_string()]).unwrap_err();
        assert!(err.contains("not supported"));
        let err = parse_args(["check-bridges".to_string()]).unwrap_err();
        assert!(err.contains("not supported"));
    }

    #[test]
    fn suggest_rejects_unsupported_forms() {
        let err = parse_args(["suggest".to_string(), "fixtures".to_string()]).unwrap_err();
        assert!(err.contains("requires --target"));

        let err = parse_args([
            "suggest".to_string(),
            "--target".to_string(),
            "unknown".to_string(),
            "fixtures".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("unsupported target"));

        let err = parse_args([
            "suggest".to_string(),
            "--target".to_string(),
            "ndarray".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("expects a path"));

        let err = parse_args([
            "suggest".to_string(),
            "--all".to_string(),
            "fixtures".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("not supported"));

        let err = parse_args([
            "suggest".to_string(),
            "--target".to_string(),
            "ndarray".to_string(),
            "--output".to_string(),
            "target/out.md".to_string(),
            "fixtures".to_string(),
        ])
        .unwrap_err();
        assert!(err.contains("not supported"));
    }
}
