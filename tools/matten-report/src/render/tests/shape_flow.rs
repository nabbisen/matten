use crate::render::{
    render_shape_flow_html_report, render_shape_flow_json_report, render_shape_flow_report,
};

fn shape_flow_data() -> crate::report::shape_flow::ShapeFlowReportData {
    crate::report::shape_flow::build()
}

#[test]
fn shape_flow_json_report_matches_expected_snapshot() {
    let report =
        render_shape_flow_json_report(&shape_flow_data()).expect("shape-flow JSON should render");

    assert_eq!(
        report,
        r#"{
  "schema_version": 0,
  "schema_status": "private-local",
  "tool": "matten-report",
  "report_kind": "shape-flow",
  "input_mode": "demo",
  "data": {
    "broadcast": {
      "operation": "a + b",
      "input_a_shape": [
        2,
        3
      ],
      "input_b_shape": [
        3
      ],
      "result": {
        "shape": [
          2,
          3
        ],
        "values": [
          11.0,
          22.0,
          33.0,
          14.0,
          25.0,
          36.0
        ],
        "truncated": false,
        "shown_values": 6,
        "total_values": 6,
        "limit": 12
      }
    },
    "reshape": {
      "operation": "reshape([3, 2])",
      "input_shape": [
        2,
        3
      ],
      "result": {
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
      }
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
      "operation": "left.matmul(right)",
      "left_shape": [
        2,
        3
      ],
      "right_shape": [
        3,
        2
      ],
      "result": {
        "shape": [
          2,
          2
        ],
        "values": [
          22.0,
          28.0,
          49.0,
          64.0
        ],
        "truncated": false,
        "shown_values": 4,
        "total_values": 4,
        "limit": 12
      }
    }
  }
}
"#
    );
}
#[test]
fn shape_flow_report_still_matches_expected_markdown() {
    let report =
        render_shape_flow_report(&shape_flow_data()).expect("shape-flow report should render");

    assert_eq!(
        report,
        "\
# matten shape-flow report

## Input
demo: shape-flow
note: fixed demo report, not automatic expression tracing

## Broadcast add
input a: shape [2, 3]
input b: shape [3]
operation: a + b
shape flow: [2, 3] + [3] -> [2, 3]
result values: [11.0, 22.0, 33.0, 14.0, 25.0, 36.0]

## Reshape
input: shape [2, 3]
operation: reshape([3, 2])
shape flow: [2, 3] -> [3, 2]
result values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]

## Axis reductions
input: shape [2, 3]
mean_axis(0): [2, 3] -> [3]
mean_axis(0) values: [2.5, 3.5, 4.5]
mean_axis(1): [2, 3] -> [2]
mean_axis(1) values: [2.0, 5.0]

## Matrix multiplication
left: shape [2, 3]
right: shape [3, 2]
operation: left.matmul(right)
shape flow: [2, 3] @ [3, 2] -> [2, 2]
result values: [22.0, 28.0, 49.0, 64.0]
"
    );
}

#[test]
fn shape_flow_html_report_matches_expected_html() {
    let report =
        render_shape_flow_html_report(&shape_flow_data()).expect("shape-flow HTML should render");

    assert_eq!(
            report,
            "\
<!doctype html>
<html lang=\"en\">
<head>
  <meta charset=\"utf-8\">
  <title>matten shape-flow report</title>
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
<h1>matten shape-flow report</h1>
<p class=\"note\">Fixed demo report, not automatic expression tracing.</p>
<section>
<h2>Broadcast add</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input a</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>input b</td><td><span class=\"shape\">[3]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 3]</span></td></tr>
</tbody>
</table>
<p>operation: a + b</p>
<pre><code>result values: [11.0, 22.0, 33.0, 14.0, 25.0, 36.0]</code></pre>
</section>
<section>
<h2>Reshape</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[3, 2]</span></td></tr>
</tbody>
</table>
<p>operation: reshape([3, 2])</p>
<pre><code>result values: [1.0, 2.0, 3.0, 4.0, 5.0, 6.0]</code></pre>
</section>
<section>
<h2>Axis reductions</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>input</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>mean_axis(0)</td><td><span class=\"shape\">[2, 3] -&gt; [3]</span></td></tr>
<tr><td>mean_axis(1)</td><td><span class=\"shape\">[2, 3] -&gt; [2]</span></td></tr>
</tbody>
</table>
<pre><code>mean_axis(0) values: [2.5, 3.5, 4.5]
mean_axis(1) values: [2.0, 5.0]</code></pre>
</section>
<section>
<h2>Matrix multiplication</h2>
<table>
<thead><tr><th>item</th><th>shape / value</th></tr></thead>
<tbody>
<tr><td>left</td><td><span class=\"shape\">[2, 3]</span></td></tr>
<tr><td>right</td><td><span class=\"shape\">[3, 2]</span></td></tr>
<tr><td>result</td><td><span class=\"shape\">[2, 2]</span></td></tr>
</tbody>
</table>
<p>operation: left.matmul(right)</p>
<pre><code>result values: [22.0, 28.0, 49.0, 64.0]</code></pre>
</section>
</main>
</body>
</html>
"
        );
}

#[test]
fn shape_flow_html_report_is_static_and_self_contained() {
    let report =
        render_shape_flow_html_report(&shape_flow_data()).expect("shape-flow HTML should render");

    assert!(report.starts_with("<!doctype html>\n<html lang=\"en\">"));
    assert!(report.contains("<title>matten shape-flow report</title>"));
    assert!(report.contains("<h1>matten shape-flow report</h1>"));
    assert!(report.contains("<h2>Broadcast add</h2>"));
    assert!(report.contains("<span class=\"shape\">[2, 3]</span>"));
    assert!(report.contains("<h2>Axis reductions</h2>"));
    assert!(report.contains("[2, 3] -&gt; [3]"));
    assert!(report.contains("<h2>Matrix multiplication</h2>"));
    assert!(report.contains("result values: [22.0, 28.0, 49.0, 64.0]"));
    assert!(!report.contains("<script"));
    assert!(!report.contains(" src="));
    assert!(!report.contains(" href="));
    assert!(!report.contains("data:"));
    assert!(!report.contains("<svg"));
}
