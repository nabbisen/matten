use std::error::Error;
use std::fmt::Write as _;

pub(crate) fn render<F>(title: &str, note: &str, write_body: F) -> Result<String, Box<dyn Error>>
where
    F: FnOnce(&mut String) -> Result<(), std::fmt::Error>,
{
    let mut report = String::new();
    write_start(&mut report, title, note)?;
    write_body(&mut report)?;
    write_end(&mut report)?;
    Ok(report)
}

pub(crate) fn write_shape_flow_table(
    report: &mut String,
    rows: &[(&str, String)],
) -> Result<(), std::fmt::Error> {
    writeln!(report, "<table>")?;
    writeln!(
        report,
        "<thead><tr><th>{}</th><th>{}</th></tr></thead>",
        escape("item"),
        escape("shape / value")
    )?;
    writeln!(report, "<tbody>")?;
    for (label, value) in rows {
        writeln!(
            report,
            "<tr><td>{}</td><td><span class=\"shape\">{}</span></td></tr>",
            escape(label),
            escape(value)
        )?;
    }
    writeln!(report, "</tbody>")?;
    writeln!(report, "</table>")
}

pub(crate) fn write_pre(report: &mut String, value: &str) -> Result<(), std::fmt::Error> {
    writeln!(report, "<pre><code>{}</code></pre>", escape(value))
}

pub(crate) fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&#39;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn write_start(report: &mut String, title: &str, note: &str) -> Result<(), std::fmt::Error> {
    writeln!(report, "<!doctype html>")?;
    writeln!(report, "<html lang=\"en\">")?;
    writeln!(report, "<head>")?;
    writeln!(report, "  <meta charset=\"utf-8\">")?;
    writeln!(report, "  <title>{}</title>", escape(title))?;
    writeln!(report, "  <style>")?;
    writeln!(
        report,
        "    :root {{ color-scheme: light; font-family: system-ui, sans-serif; }}"
    )?;
    writeln!(
        report,
        "    body {{ margin: 2rem auto; max-width: 920px; color: #17202a; background: #ffffff; line-height: 1.5; }}"
    )?;
    writeln!(
        report,
        "    h1, h2 {{ color: #14324a; }} section {{ border-top: 1px solid #d6dde5; padding: 1rem 0; }}"
    )?;
    writeln!(
        report,
        "    table {{ width: 100%; border-collapse: collapse; margin: 0.75rem 0; }} th, td {{ border: 1px solid #d6dde5; padding: 0.45rem 0.6rem; text-align: left; vertical-align: top; }}"
    )?;
    writeln!(
        report,
        "    th {{ background: #eef4f8; }} code, .shape {{ font-family: ui-monospace, SFMono-Regular, Menlo, Consolas, monospace; }}"
    )?;
    writeln!(
        report,
        "    .note {{ background: #f6f8fa; border-left: 4px solid #5b8fb9; padding: 0.75rem 1rem; }}"
    )?;
    writeln!(
        report,
        "    .shape {{ display: inline-block; background: #eef4f8; border: 1px solid #cbd8e3; border-radius: 4px; padding: 0.1rem 0.35rem; }}"
    )?;
    writeln!(report, "  </style>")?;
    writeln!(report, "</head>")?;
    writeln!(report, "<body>")?;
    writeln!(report, "<main>")?;
    writeln!(report, "<h1>{}</h1>", escape(title))?;
    writeln!(report, "<p class=\"note\">{}</p>", escape(note))
}

fn write_end(report: &mut String) -> Result<(), std::fmt::Error> {
    writeln!(report, "</main>")?;
    writeln!(report, "</body>")?;
    writeln!(report, "</html>")
}
