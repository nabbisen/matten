use super::*;

#[test]
fn dynamic_readiness_html_report_matches_expected_html() {
    let data = dynamic_readiness_data();
    let report =
        render_dynamic_readiness_html_report(&data).expect("dynamic-readiness HTML should render");

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
    let data = dynamic_readiness_data();
    let report =
        render_dynamic_readiness_html_report(&data).expect("dynamic-readiness HTML should render");

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
