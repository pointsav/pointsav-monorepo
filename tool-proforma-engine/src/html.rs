use pulldown_cmark::{html as cm_html, Options, Parser};

/// Convert a markdown string to a self-contained HTML page.
pub fn render(markdown: &str, title: &str) -> String {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(markdown, opts);
    let mut body = String::new();
    cm_html::push_html(&mut body, parser);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>{title}</title>
<style>
  *, *::before, *::after {{ box-sizing: border-box; }}
  body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    font-size: 14px;
    line-height: 1.5;
    color: #1a1a1a;
    background: #fff;
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem 1.5rem 4rem;
  }}
  h1 {{ font-size: 1.5rem; margin: 0 0 0.25rem; }}
  h2 {{ font-size: 1.1rem; margin: 2rem 0 0.5rem; border-bottom: 1px solid #e0e0e0; padding-bottom: 0.25rem; }}
  h3 {{ font-size: 0.95rem; margin: 1.25rem 0 0.4rem; color: #444; }}
  p  {{ margin: 0.4rem 0 0.75rem; }}
  blockquote {{
    margin: 0.75rem 0;
    padding: 0.4rem 0.8rem;
    border-left: 3px solid #ccc;
    color: #555;
    background: #fafafa;
  }}
  table {{
    border-collapse: collapse;
    width: 100%;
    margin: 0.5rem 0 1.25rem;
    font-size: 13px;
    font-variant-numeric: tabular-nums;
  }}
  th, td {{
    border: 1px solid #d8d8d8;
    padding: 4px 10px;
    white-space: nowrap;
  }}
  th {{
    background: #f5f5f5;
    font-weight: 600;
    text-align: center;
  }}
  td:first-child {{ text-align: left; }}
  td:not(:first-child) {{ text-align: right; }}
  tr:nth-child(even) td {{ background: #fafafa; }}
  strong {{ font-weight: 700; }}
  hr {{ border: none; border-top: 1px solid #e0e0e0; margin: 2rem 0; }}
  code {{ font-family: "SFMono-Regular", Consolas, monospace; font-size: 12px; }}
</style>
</head>
<body>
{body}
</body>
</html>
"#
    )
}
