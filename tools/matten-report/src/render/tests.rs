use super::*;

mod shape_flow;

const SMALL_CSV: &str = include_str!("../../fixtures/small.csv");
const MISSING_CSV: &str = include_str!("../../fixtures/missing.csv");
const NON_NUMERIC_CSV: &str = include_str!("../../fixtures/non_numeric.csv");

fn selected(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn render_fixture_report(label: &str, csv: &str, values: &[&str]) -> String {
    let table = Table::from_csv_str(csv).expect("fixture CSV should parse");
    render_table_report(label, &table, &selected(values)).expect("report should render")
}

#[test]
fn fixed_demo_json_is_deterministic() {
    let first = render_fixed_demo_json_report(KIND_EDUCATIONAL_PATH)
        .expect("educational-path JSON should render");
    let second = render_fixed_demo_json_report(KIND_EDUCATIONAL_PATH)
        .expect("educational-path JSON should render again");

    assert_eq!(first, second);
}

#[test]
fn data_readiness_json_report_matches_expected_snapshot() {
    let report = render_fixed_demo_json_report(KIND_DATA_READINESS)
        .expect("data-readiness JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "data-readiness",
  "input_mode": "demo",
  "data": {
    "input_label": "demo: data-readiness",
    "source_columns": [
      "region",
      "sales",
      "cost",
      "note"
    ],
    "selected_columns": [
      "sales",
      "cost"
    ],
    "left_out_columns": [
      "region",
      "note"
    ],
    "missing_counts": [
      {
        "column": "sales",
        "missing": 0
      },
      {
        "column": "cost",
        "missing": 0
      }
    ],
    "numeric_conversion": {
      "status": "success",
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          100.0,
          40.0,
          150.0,
          45.0,
          120.0,
          55.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      }
    }
  }
}
"#
    );
}

#[test]
fn dynamic_readiness_json_report_matches_expected_snapshot() {
    let report = render_fixed_demo_json_report(KIND_DYNAMIC_READINESS)
        .expect("dynamic-readiness JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "dynamic-readiness",
  "input_mode": "demo",
  "data": {
    "shape": [
      2,
      3
    ],
    "values": [
      {
        "row": 0,
        "column": 0,
        "element": "Float(1.0)"
      },
      {
        "row": 0,
        "column": 1,
        "element": "Text(\"2.5\")"
      },
      {
        "row": 0,
        "column": 2,
        "element": "None"
      },
      {
        "row": 1,
        "column": 0,
        "element": "Int(4)"
      },
      {
        "row": 1,
        "column": 1,
        "element": "Text(\"6.0\")"
      },
      {
        "row": 1,
        "column": 2,
        "element": "Float(8.0)"
      }
    ],
    "schema_summary": [
      {
        "label": "Float",
        "count": 2
      },
      {
        "label": "Int",
        "count": 1
      },
      {
        "label": "Text",
        "count": 2
      },
      {
        "label": "None",
        "count": 1
      }
    ],
    "readiness_masks": {
      "none_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          0.0,
          0.0,
          1.0,
          0.0,
          0.0,
          0.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "numeric_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          0.0,
          0.0,
          1.0,
          0.0,
          1.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "strict_numeric_ready": false
    },
    "strict_conversion": {
      "status": "error",
      "message": "error: strict conversion rejects Text and None values"
    },
    "explicit_policy_conversion": {
      "policy": "none_as(0.0) + allow_text_parse()",
      "tensor": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          2.5,
          0.0,
          4.0,
          6.0,
          8.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      }
    }
  }
}
"#
    );
}

#[test]
fn mlprep_standardization_json_report_matches_expected_snapshot() {
    let report = render_fixed_demo_json_report(KIND_MLPREP_STANDARDIZATION)
        .expect("mlprep-standardization JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "mlprep-standardization",
  "input_mode": "demo",
  "data": {
    "selected_columns": [
      "feature_0",
      "feature_1"
    ],
    "operation": "standardize_columns(input)",
    "before": {
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          8.0,
          80.0,
          10.0,
          100.0,
          12.0,
          120.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "column_mean": [
        10.0,
        100.0
      ],
      "column_population_std": [
        1.632993161855452,
        16.32993161855452
      ]
    },
    "after": {
      "tensor": {
        "shape": [
          3,
          2
        ],
        "values": [
          -1.224744871391589,
          -1.224744871391589,
          0.0,
          0.0,
          1.224744871391589,
          1.224744871391589
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "column_mean": [
        0.0,
        0.0
      ],
      "column_population_std": [
        0.9999999999999999,
        0.9999999999999999
      ]
    }
  }
}
"#
    );
}

#[test]
fn educational_path_json_report_matches_expected_snapshot() {
    let report = render_fixed_demo_json_report(KIND_EDUCATIONAL_PATH)
        .expect("educational-path JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "educational-path",
  "input_mode": "demo",
  "data": {
    "reading_steps": [
      "ask what shape each input has",
      "ask which axes align, disappear, or remain",
      "read the output shape before reading values",
      "convert dynamic data before numeric computation"
    ],
    "broadcasting": {
      "left_shape": [
        3,
        1
      ],
      "right_shape": [
        1,
        4
      ],
      "result": {
        "shape": [
          3,
          4
        ],
        "values": [
          11.0,
          21.0,
          31.0,
          41.0,
          12.0,
          22.0,
          32.0,
          42.0,
          13.0,
          23.0,
          33.0,
          43.0
        ],
        "truncated": false,
        "shown_values": 12,
        "total_values": 12,
        "limit": 12
      },
      "axis_1_meaning": "left repeats across 4 columns",
      "axis_0_meaning": "right repeats across 3 rows"
    },
    "reshape_and_transpose": {
      "input_shape": [
        2,
        3
      ],
      "reshape": {
        "shape": [
          3,
          2
        ],
        "values": [
          1.0,
          2.0,
          3.0,
          4.0,
          5.0,
          6.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "transpose": {
        "shape": [
          3,
          2
        ],
        "values": [
          1.0,
          4.0,
          2.0,
          5.0,
          3.0,
          6.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "meaning": "reshape changes grouping; transpose changes coordinate meaning"
    },
    "axis_reductions": {
      "input_shape": [
        2,
        3
      ],
      "mean_axis_0": {
        "shape": [
          3
        ],
        "values": [
          2.5,
          3.5,
          4.5
        ],
        "truncated": false,
        "shown_values": 3,
        "total_values": 3,
        "limit": 12
      },
      "mean_axis_1": {
        "shape": [
          2
        ],
        "values": [
          2.0,
          5.0
        ],
        "truncated": false,
        "shown_values": 2,
        "total_values": 2,
        "limit": 12
      }
    },
    "matmul": {
      "left_shape": [
        2,
        3
      ],
      "right_shape": [
        3,
        4
      ],
      "shared_inner_dimension": 3,
      "result": {
        "shape": [
          2,
          4
        ],
        "values": [
          38.0,
          44.0,
          50.0,
          56.0,
          83.0,
          98.0,
          113.0,
          128.0
        ],
        "truncated": false,
        "shown_values": 8,
        "total_values": 8,
        "limit": 12
      }
    },
    "dynamic_readiness": {
      "shape": [
        2,
        3
      ],
      "none_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          0.0,
          0.0,
          1.0,
          0.0,
          0.0,
          0.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "numeric_mask": {
        "shape": [
          2,
          3
        ],
        "values": [
          1.0,
          0.0,
          0.0,
          1.0,
          0.0,
          1.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      },
      "note": "Text values are not numeric-ready under the strict mask",
      "next_step": "clean values, then call try_numeric()"
    },
    "standardization": {
      "operation": "standardize_columns(input)",
      "input_shape": [
        3,
        2
      ],
      "output_shape": [
        3,
        2
      ],
      "before_mean": [
        10.0,
        100.0
      ],
      "before_population_std": [
        1.632993161855452,
        16.32993161855452
      ],
      "after_mean": [
        0.0,
        0.0
      ],
      "after_population_std": [
        0.9999999999999999,
        0.9999999999999999
      ]
    },
    "non_goals": [
      "not a public API",
      "not source scanning",
      "not a renderer",
      "not model-quality analysis"
    ]
  }
}
"#
    );
}

#[test]
fn dynamic_readiness_report_matches_expected_markdown() {
    let report = render_dynamic_readiness_report().expect("dynamic-readiness report should render");

    assert_eq!(
        report,
        "\
# matten dynamic-readiness report

## Input
demo: dynamic-readiness
note: fixed demo report, not automatic data profiling

## Dynamic values
shape: [2, 3]
row-major values:
- [0, 0] Float(1.0)
- [0, 1] Text(\"2.5\")
- [0, 2] None
- [1, 0] Int(4)
- [1, 1] Text(\"6.0\")
- [1, 2] Float(8.0)
schema summary:
- Float: 2
- Int: 1
- Text: 2
- None: 1

## Readiness masks
none mask: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
numeric mask: strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]
strict numeric-ready: false

## Strict conversion
result: error: strict conversion rejects Text and None values

## Explicit policy conversion
policy: none_as(0.0) + allow_text_parse()
converted shape: [2, 3]
converted row-major values: [1.0, 2.5, 0.0, 4.0, 6.0, 8.0]
"
    );
}

#[test]
fn dynamic_readiness_html_report_matches_expected_html() {
    let report =
        render_dynamic_readiness_html_report().expect("dynamic-readiness HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten dynamic-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten dynamic-readiness report</h1>
<p class=\"note\">Fixed demo report, not automatic data profiling.</p>
<section>
<h2>Dynamic values</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
</tbody>
</table>
<table>
<thead><tr><th>row</th><th>column</th><th>value</th></tr></thead>
<tbody>
<tr><td>0</td><td>0</td><td><span class=\"shape\">Float(1.0)</span></td></tr>
<tr><td>0</td><td>1</td><td><span class=\"shape\">Text(&quot;2.5&quot;)</span></td></tr>
<tr><td>0</td><td>2</td><td><span class=\"shape\">None</span></td></tr>
<tr><td>1</td><td>0</td><td><span class=\"shape\">Int(4)</span></td></tr>
<tr><td>1</td><td>1</td><td><span class=\"shape\">Text(&quot;6.0&quot;)</span></td></tr>
<tr><td>1</td><td>2</td><td><span class=\"shape\">Float(8.0)</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Schema summary</h2>
<table>
<thead><tr><th>element kind</th><th>count</th></tr></thead>
<tbody>
<tr><td>Float</td><td><span class=\"shape\">2</span></td></tr>
<tr><td>Int</td><td><span class=\"shape\">1</span></td></tr>
<tr><td>Text</td><td><span class=\"shape\">2</span></td></tr>
<tr><td>None</td><td><span class=\"shape\">1</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Readiness masks</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>none mask</td><td><span class=\"shape\">[0.0, 0.0, 1.0, 0.0, 0.0, 0.0]</span></td></tr>
<tr><td>numeric mask</td><td><span class=\"shape\">strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]</span></td></tr>
<tr><td>strict numeric-ready</td><td><span class=\"shape\">false</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Strict conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>result</td><td><span class=\"shape\">error: strict conversion rejects Text and None values</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Explicit policy conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>policy</td><td><span class=\"shape\">none_as(0.0) + allow_text_parse()</span></td></tr>
<tr><td>converted shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>converted row-major values</td><td><span class=\"shape\">[1.0, 2.5, 0.0, 4.0, 6.0, 8.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn dynamic_readiness_html_report_is_static_and_self_contained() {
    let report =
        render_dynamic_readiness_html_report().expect("dynamic-readiness HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten dynamic-readiness report</title>"));
    assert!(report.contains("<h1>matten dynamic-readiness report</h1>"));
    assert!(report.contains("<h2>Dynamic values</h2>"));
    assert!(report.contains("<span class=\"shape\">Text(&quot;2.5&quot;)</span>"));
    assert!(report.contains("<h2>Readiness masks</h2>"));
    assert!(report.contains("strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]"));
    assert!(report.contains("<h2>Strict conversion</h2>"));
    assert!(report.contains("error: strict conversion rejects Text and None values"));
    assert!(report.contains("<h2>Explicit policy conversion</h2>"));
    assert!(report.contains("[1.0, 2.5, 0.0, 4.0, 6.0, 8.0]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn mlprep_standardization_report_matches_expected_markdown() {
    let report = render_mlprep_standardization_report()
        .expect("mlprep-standardization report should render");

    assert_eq!(
        report,
        "\
# matten mlprep-standardization report

## Input
demo: mlprep-standardization
note: fixed demo report, not automatic model-quality analysis

## Operation
operation: standardize_columns(input)
meaning: each column is centered to mean 0 and population standard deviation 1

## Before
shape: [3, 2]
row-major values: [8.000, 80.000, 10.000, 100.000, 12.000, 120.000]
column mean: [10.000, 100.000]
column population std: [1.633, 16.330]

## After
shape: [3, 2]
row-major values: [-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]
column mean: [0.000, 0.000]
column population std: [1.000, 1.000]

## Shape meaning
shape flow: [3, 2] -> [3, 2]
rows: samples unchanged
columns: features unchanged
"
    );
}

#[test]
fn mlprep_standardization_html_report_matches_expected_html() {
    let report = render_mlprep_standardization_html_report()
        .expect("mlprep-standardization HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten mlprep-standardization report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten mlprep-standardization report</h1>
<p class=\"note\">Fixed demo report, not automatic model-quality analysis.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>demo</td><td><span class=\"shape\">mlprep-standardization</span></td></tr>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Operation</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>operation</td><td><span class=\"shape\">standardize_columns(input)</span></td></tr>
<tr><td>meaning</td><td><span class=\"shape\">each column is centered to mean 0 and population standard deviation 1</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Before</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[8.000, 80.000, 10.000, 100.000, 12.000, 120.000]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[10.000, 100.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.633, 16.330]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>After</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]</span></td></tr>
<tr><td>column mean</td><td><span class=\"shape\">[0.000, 0.000]</span></td></tr>
<tr><td>column population std</td><td><span class=\"shape\">[1.000, 1.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Shape meaning</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape flow</td><td><span class=\"shape\">[3, 2] -&gt; [3, 2]</span></td></tr>
<tr><td>rows</td><td><span class=\"shape\">samples unchanged</span></td></tr>
<tr><td>columns</td><td><span class=\"shape\">features unchanged</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn mlprep_standardization_html_report_is_static_and_self_contained() {
    let report = render_mlprep_standardization_html_report()
        .expect("mlprep-standardization HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten mlprep-standardization report</title>"));
    assert!(report.contains("<h1>matten mlprep-standardization report</h1>"));
    assert!(report.contains("not automatic model-quality analysis"));
    assert!(report.contains("<h2>Input</h2>"));
    assert!(report.contains("<h2>Operation</h2>"));
    assert!(report.contains("standardize_columns(input)"));
    assert!(report.contains("<h2>Before</h2>"));
    assert!(report.contains("[10.000, 100.000]"));
    assert!(report.contains("[1.633, 16.330]"));
    assert!(report.contains("<h2>After</h2>"));
    assert!(report.contains("[-1.225, -1.225, 0.000, 0.000, 1.225, 1.225]"));
    assert!(report.contains("[0.000, 0.000]"));
    assert!(report.contains("[1.000, 1.000]"));
    assert!(report.contains("<h2>Shape meaning</h2>"));
    assert!(report.contains("[3, 2] -&gt; [3, 2]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn educational_path_report_matches_expected_markdown() {
    let report = render_educational_path_report().expect("educational-path report should render");

    assert_eq!(
        report,
        "\
# matten educational-path report

## Input
demo: educational-path
note: fixed educational demo report, not automatic expression tracing

## How to read shapes first
1. ask what shape each input has
2. ask which axes align, disappear, or remain
3. read the output shape before reading values
4. convert dynamic data before numeric computation

## Broadcasting
shape flow: [3, 1] + [1, 4] -> [3, 4]
axis 1: left repeats across 4 columns
axis 0: right repeats across 3 rows
result values: [11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0]

## Reshape and transpose
reshape: [2, 3] -> [3, 2]
reshape values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
transpose: [2, 3] -> [3, 2]
transpose values: [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]
meaning: reshape changes grouping; transpose changes coordinate meaning

## Axis reductions
mean_axis(0): [2, 3] -> [3]
mean_axis(0) keeps columns: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) keeps rows: [2.0, 5.0]

## Matrix multiplication
shape flow: [2, 3] @ [3, 4] -> [2, 4]
shared inner dimension: 3
result values: [38.0, 44.0, 50.0, 56.0, 83.0, 98.0, 113.0, 128.0]

## Dynamic readiness
dynamic shape: [2, 3]
none mask: [0.0, 0.0, 1.0, 0.0, 0.0, 0.0]
numeric mask: strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]
Text values are not numeric-ready under the strict mask
next step: clean values, then call try_numeric()

## Standardization
operation: standardize_columns(input)
shape flow: [3, 2] -> [3, 2]
before column mean: [10.000, 100.000]
before column population std: [1.633, 16.330]
after column mean: [0.000, 0.000]
after column population std: [1.000, 1.000]

## What this report is not
- not a public API
- not source scanning
- not a renderer
- not model-quality analysis
"
    );
}

#[test]
fn educational_path_html_report_matches_expected_html() {
    let report =
        render_educational_path_html_report().expect("educational-path HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten educational-path report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten educational-path report</h1>
<p class=\"note\">Fixed educational demo report, not automatic expression tracing.</p>
<section>
<h2>How to read shapes first</h2>
<ol>
<li>ask what shape each input has</li>
<li>ask which axes align, disappear, or remain</li>
<li>read the output shape before reading values</li>
<li>convert dynamic data before numeric computation</li>
</ol>
</section>
<section>
<h2>Broadcasting</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[3, 1]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[1, 4]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[3, 4]</span></td></tr>
</tbody>
</table>
<p>axis 1: left repeats across 4 columns; axis 0: right repeats across 3 rows</p>
<pre><code>result values: [11.0, 21.0, 31.0, 41.0, 12.0, 22.0, 32.0, 42.0, 13.0, 23.0, 33.0, 43.0]</code></pre>
</section>
<section>
<h2>Reshape and transpose</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>reshape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>transpose</td><td><span class=\"shape\">[3, 2]</span></td></tr>
</tbody>
</table>
<pre><code>reshape values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
transpose values: [1.0, 4.0, 2.0, 5.0, 3.0, 6.0]</code></pre>
<p>reshape changes grouping; transpose changes coordinate meaning</p>
</section>
<section>
<h2>Axis reductions</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>mean_axis(0)</td><td><span class=\"shape\">[2, 3] -&gt; [3]</span></td></tr>
<tr><td>mean_axis(1)</td><td><span class=\"shape\">[2, 3] -&gt; [2]</span></td></tr>
</tbody>
</table>
<pre><code>mean_axis(0) keeps columns: [2.5, 3.5, 4.5]
mean_axis(1) keeps rows: [2.0, 5.0]</code></pre>
</section>
<section>
<h2>Matrix multiplication</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[3, 4]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 4]</span></td></tr>
</tbody>
</table>
<p>shared inner dimension: 3</p>
<pre><code>result values: [38.0, 44.0, 50.0, 56.0, 83.0, 98.0, 113.0, 128.0]</code></pre>
</section>
<section>
<h2>Dynamic readiness</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>dynamic shape</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>none mask</td><td><span class=\"shape\">[0.0, 0.0, 1.0, 0.0, 0.0, 0.0]</span></td></tr>
<tr><td>numeric mask</td><td><span class=\"shape\">strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]</span></td></tr>
</tbody>
</table>
<p>Text values are not numeric-ready under the strict mask; clean values, then call try_numeric().</p>
</section>
<section>
<h2>Standardization</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape flow</td><td><span class=\"shape\">[3, 2] -&gt; [3, 2]</span></td></tr>
<tr><td>before mean</td><td><span class=\"shape\">[10.000, 100.000]</span></td></tr>
<tr><td>before population std</td><td><span class=\"shape\">[1.633, 16.330]</span></td></tr>
<tr><td>after mean</td><td><span class=\"shape\">[0.000, 0.000]</span></td></tr>
<tr><td>after population std</td><td><span class=\"shape\">[1.000, 1.000]</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>What this report is not</h2>
<ul>
<li>not a public API</li>
<li>not source scanning</li>
<li>not a renderer</li>
<li>not model-quality analysis</li>
</ul>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn educational_path_html_report_is_static_and_self_contained() {
    let report =
        render_educational_path_html_report().expect("educational-path HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten educational-path report</title>"));
    assert!(report.contains("<h1>matten educational-path report</h1>"));
    assert!(report.contains("<h2>Broadcasting</h2>"));
    assert!(report.contains("<span class=\"shape\">[3, 1]</span>"));
    assert!(report.contains("<h2>Dynamic readiness</h2>"));
    assert!(report.contains("strict policy readiness [1.0, 0.0, 0.0, 1.0, 0.0, 1.0]"));
    assert!(report.contains("<h2>Standardization</h2>"));
    assert!(report.contains("after population std"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn data_readiness_report_still_matches_expected_markdown() {
    let report = render_fixture_report("fixture: small.csv", SMALL_CSV, &["sales", "cost"]);

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: small.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: success

## Tensor preview
shape: [3, 2]
row-major values: [100.0, 40.0, 150.0, 45.0, 120.0, 55.0]
"
    );
}
#[test]
fn data_readiness_html_report_matches_expected_html() {
    let report = render_data_readiness_html_report().expect("data-readiness HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Fixed demo report, not arbitrary CSV profiling.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">demo: data-readiness</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn data_readiness_html_report_is_static_and_self_contained() {
    let report = render_data_readiness_html_report().expect("data-readiness HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten data-readiness report</title>"));
    assert!(report.contains("<h1>matten data-readiness report</h1>"));
    assert!(report.contains("Fixed demo report, not arbitrary CSV profiling."));
    assert!(report.contains("<h2>Columns</h2>"));
    assert!(report.contains("region, sales, cost, note"));
    assert!(report.contains("sales, cost"));
    assert!(report.contains("region, note"));
    assert!(report.contains("<h2>Missing values</h2>"));
    assert!(report.contains("<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>"));
    assert!(report.contains("<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>"));
    assert!(report.contains("<h2>Numeric conversion</h2>"));
    assert!(report.contains("<span class=\"shape\">success</span>"));
    assert!(report.contains("<h2>Tensor preview</h2>"));
    assert!(report.contains("<span class=\"shape\">[3, 2]</span>"));
    assert!(report.contains("[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn input_data_readiness_html_success_matches_expected_html() {
    let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: tools/matten-report/fixtures/small.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input-mode data-readiness HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten data-readiness report</title>
  <style>
    :root { color-scheme: light; font-family: system-ui, sans-serif; }
    body { margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }
    h1, h2 { color: #14324a; } section { border-top: 1px solid #d6dde5; padding: 1rem 0; }
    table { width: 100%; border-collapse: collapse; margin: 0.75rem 0; } th, td { border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }
    th { background: #eef4f8; } code, .shape { font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }
    .note { background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }
    .shape { display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }
  </style>
</head>
<body>
<main>
<h1>matten data-readiness report</h1>
<p class=\"note\">Bounded summary of the provided CSV file; not a full raw table rendering.</p>
<section>
<h2>Input</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">path: tools/matten-report/fixtures/small.csv</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Columns</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>source columns</td><td><span class=\"shape\">region, sales, cost, note</span></td></tr>
<tr><td>selected columns</td><td><span class=\"shape\">sales, cost</span></td></tr>
<tr><td>columns left out</td><td><span class=\"shape\">region, note</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Missing values</h2>
<table>
<thead><tr><th>column</th><th>missing</th></tr></thead>
<tbody>
<tr><td>sales</td><td><span class=\"shape\">0</span></td></tr>
<tr><td>cost</td><td><span class=\"shape\">0</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Numeric conversion</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>strict conversion</td><td><span class=\"shape\">success</span></td></tr>
</tbody>
</table>
</section>
<section>
<h2>Tensor preview</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>shape</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>row-major values</td><td><span class=\"shape\">[100.0, 40.0, 150.0, 45.0, 120.0, 55.0]</span></td></tr>
</tbody>
</table>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn input_data_readiness_html_error_is_bounded_summary() {
    let table = Table::from_csv_str(NON_NUMERIC_CSV).expect("fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: tools/matten-report/fixtures/non_numeric.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("input-mode data-readiness error HTML should render");

    assert!(report.contains("<h1>matten data-readiness report</h1>"));
    assert!(report.contains("Bounded summary of the provided CSV file"));
    assert!(report.contains("<h2>Numeric conversion</h2>"));
    assert!(report.contains("<span class=\"shape\">error</span>"));
    assert!(
        report
            .contains("non-numeric value &quot;oops&quot; in column &quot;sales&quot;, CSV line 3")
    );
    assert!(!report.contains("<h2>Tensor preview</h2>"));
    assert!(!report.contains("Fixed demo report, not arbitrary CSV profiling."));
}

#[test]
fn input_data_readiness_html_is_static_self_contained_and_escaped() {
    let csv = "\
region,<script>alert(1)</script>,cost,note
north,<b>oops</b>,40,ok
";
    let table = Table::from_csv_str(csv).expect("hostile fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: <script>/tmp/hostile.csv</script>",
        &table,
        &selected(&["<script>alert(1)</script>", "cost"]),
    )
    .expect("hostile input HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("path: &lt;script&gt;/tmp/hostile.csv&lt;/script&gt;"));
    assert!(report.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(report.contains("&lt;b&gt;oops&lt;/b&gt;"));
    assert!(!report.contains("<script>alert(1)</script>"));
    assert!(!report.contains("<b>oops</b>"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}

#[test]
fn input_data_readiness_html_bounds_wide_and_long_fields() {
    let headers: Vec<String> = (0..15)
        .map(|index| {
            if index == 1 {
                format!("selected_{}", "x".repeat(180))
            } else {
                format!("col{index}")
            }
        })
        .collect();
    let values: Vec<String> = (0..15).map(|index| index.to_string()).collect();
    let csv = format!("{}\n{}\n", headers.join(","), values.join(","));
    let table = Table::from_csv_str(&csv).expect("wide fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        &format!("path: {}", "p".repeat(180)),
        &table,
        &selected(&[&headers[1], "col2"]),
    )
    .expect("wide input HTML should render");

    assert!(report.contains("... 3 more"));
    assert!(report.contains("path: ppppp"));
    assert!(report.contains("...</span>"));
    assert!(report.contains("selected_xxxxxxxxx"));
    assert!(!report.contains(&"p".repeat(180)));
    assert!(!report.contains(&headers[1]));
}

#[test]
fn input_data_readiness_html_bounds_tensor_preview_values() {
    let csv = "\
sales,cost
1,2
3,4
5,6
7,8
9,10
11,12
13,14
";
    let table = Table::from_csv_str(csv).expect("long numeric fixture CSV should parse");
    let report = render_input_data_readiness_html_report(
        "path: long.csv",
        &table,
        &selected(&["sales", "cost"]),
    )
    .expect("long numeric input HTML should render");

    assert!(
        report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, ... 2 more]"
        )
    );
    assert!(
        !report.contains(
            "[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0]"
        )
    );
}

#[test]
fn missing_value_report_matches_expected_markdown() {
    let report = render_fixture_report("fixture: missing.csv", MISSING_CSV, &["sales", "cost"]);

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: missing.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 1 |

## Numeric conversion
strict conversion: error: missing value in column \"cost\", CSV line 3
"
    );
}

#[test]
fn non_numeric_report_matches_expected_markdown() {
    let report = render_fixture_report(
        "fixture: non_numeric.csv",
        NON_NUMERIC_CSV,
        &["sales", "cost"],
    );

    assert_eq!(
        report,
        "\
# matten data-readiness report

## Input
fixture: non_numeric.csv

## Source columns
- region
- sales
- cost
- note

## Selected columns
- sales
- cost

## Columns left out
- region
- note

## Missing values
| column | missing |
|---|---:|
| sales | 0 |
| cost | 0 |

## Numeric conversion
strict conversion: error: non-numeric value \"oops\" in column \"sales\", CSV line 3
"
    );
}

#[test]
fn selected_column_errors_are_readable() {
    let table = Table::from_csv_str(SMALL_CSV).expect("fixture CSV should parse");

    let missing = render_table_report("fixture: small.csv", &table, &selected(&["profit"]))
        .unwrap_err()
        .to_string();
    assert!(missing.contains("column \"profit\" does not exist"));

    let duplicate =
        render_table_report("fixture: small.csv", &table, &selected(&["sales", "sales"]))
            .unwrap_err()
            .to_string();
    assert!(duplicate.contains("column \"sales\" was selected more than once"));
}
