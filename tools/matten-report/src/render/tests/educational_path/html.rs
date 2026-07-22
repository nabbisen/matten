use super::*;

#[test]
fn educational_path_html_report_matches_expected_html() {
    let data = educational_path_data();
    let report =
        render_educational_path_html_report(&data).expect("educational-path HTML should render");

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
    let data = educational_path_data();
    let report =
        render_educational_path_html_report(&data).expect("educational-path HTML should render");

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
