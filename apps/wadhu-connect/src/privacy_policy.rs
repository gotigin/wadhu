//! Wadhu legal Markdown — one file each under `docs/legal/`.
//!
//! Source of truth (edit only those files):
//! - `docs/legal/PRIVACY.md`
//! - `docs/legal/TERMS.md`
//! - `docs/legal/DATA_DELETION.md`
//! - `docs/legal/SUPPORT.md`
//! - `docs/legal/ABOUT.md`
//!
//! Site: pandoc via `sites/wadhu/scripts/present-legal.sh`
//! Connect: this module embeds + serves formatted HTML.

pub const PRIVACY_MD: &str = include_str!("../../../docs/legal/PRIVACY.md");
pub const TERMS_MD: &str = include_str!("../../../docs/legal/TERMS.md");
pub const DATA_DELETION_MD: &str = include_str!("../../../docs/legal/DATA_DELETION.md");
pub const SUPPORT_MD: &str = include_str!("../../../docs/legal/SUPPORT.md");
pub const ABOUT_MD: &str = include_str!("../../../docs/legal/ABOUT.md");

pub fn privacy_html() -> String {
    present_page(
        "Privacy — Wadhu",
        "How Wadhu (by Gotigin) treats your data.",
        "https://wadhu.com/privacy",
        PRIVACY_MD,
    )
}

pub fn terms_html() -> String {
    present_page(
        "Terms — Wadhu",
        "Terms of use for Wadhu services.",
        "https://wadhu.com/terms",
        TERMS_MD,
    )
}

pub fn data_deletion_html() -> String {
    present_page(
        "Data deletion — Wadhu",
        "How to delete Wadhu SMS Gateway and Connect data.",
        "https://wadhu.com/data-deletion",
        DATA_DELETION_MD,
    )
}

pub fn support_html() -> String {
    present_page(
        "Support — Wadhu",
        "Contact and support for Wadhu products.",
        "https://wadhu.com/support",
        SUPPORT_MD,
    )
}

pub fn about_html() -> String {
    present_page(
        "About — Wadhu",
        "About Wadhu and Gotigin.",
        "https://wadhu.com/about",
        ABOUT_MD,
    )
}

fn present_page(title: &str, description: &str, canonical: &str, md: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>{title}</title>
  <meta name="description" content="{description}" />
  <link rel="canonical" href="{canonical}" />
  <link rel="stylesheet" href="/styles.css" />
  <style>
    .legal {{ max-width: 42rem; margin: 0 auto; padding: 1.5rem 1.25rem 3rem; line-height: 1.55; }}
    .legal .site-back {{ margin: 0 0 1.25rem; }}
    .legal h1 {{ font-size: 1.75rem; line-height: 1.25; margin: 0 0 0.75rem; }}
    .legal h2 {{ font-size: 1.2rem; margin: 1.75rem 0 0.5rem; }}
    .legal h3 {{ font-size: 1.05rem; margin: 1.25rem 0 0.4rem; }}
    .legal p, .legal ul, .legal ol {{ margin: 0.55rem 0; }}
    .legal ul, .legal ol {{ padding-left: 1.35rem; }}
    .legal li {{ margin: 0.25rem 0; }}
    .legal table {{ border-collapse: collapse; width: 100%; margin: 0.75rem 0; font-size: 0.95rem; }}
    .legal th, .legal td {{ border: 1px solid rgba(0,0,0,0.12); padding: 0.35rem 0.5rem; text-align: left; vertical-align: top; }}
    .legal code {{
      font-family: ui-monospace, "Liberation Mono", monospace;
      font-size: 0.92em;
      background: rgba(0,0,0,0.06);
      padding: 0.1em 0.35em;
      border-radius: 4px;
    }}
    .legal-nav {{ margin-top: 2rem; font-size: 0.95rem; }}
  </style>
</head>
<body>
  <main class="legal">
    <p class="site-back"><a href="/">← Wadhu Connect</a></p>
{body}
    <p class="legal-nav">
      <a href="/privacy">Privacy</a> ·
      <a href="/terms">Terms</a> ·
      <a href="/data-deletion">Data deletion</a> ·
      <a href="/support">Support</a> ·
      <a href="/about">About</a>
    </p>
  </main>
</body>
</html>
"#,
        title = esc(title),
        description = esc(description),
        canonical = esc(canonical),
        body = md_to_html(md),
    )
}

fn md_to_html(md: &str) -> String {
    let mut out = String::new();
    let mut paras: Vec<String> = Vec::new();
    let mut list: Option<&'static str> = None;
    let mut table_rows: Vec<Vec<String>> = Vec::new();
    let mut in_table = false;

    let flush_paras = |out: &mut String, paras: &mut Vec<String>| {
        if paras.is_empty() {
            return;
        }
        let joined = paras.join(" ");
        paras.clear();
        out.push_str("<p>");
        out.push_str(&inline(&joined));
        out.push_str("</p>\n");
    };
    let close_list = |out: &mut String, list: &mut Option<&'static str>| {
        if let Some(tag) = list.take() {
            out.push_str("</");
            out.push_str(tag);
            out.push_str(">\n");
        }
    };
    let flush_table = |out: &mut String, rows: &mut Vec<Vec<String>>, in_table: &mut bool| {
        if rows.is_empty() {
            *in_table = false;
            return;
        }
        out.push_str("<table>\n");
        for (i, row) in rows.iter().enumerate() {
            // Skip markdown separator rows like ---|---
            if row.iter().all(|c| c.chars().all(|ch| ch == '-' || ch == ':' || ch.is_whitespace())) {
                continue;
            }
            out.push_str("<tr>");
            let tag = if i == 0 { "th" } else { "td" };
            for cell in row {
                out.push('<');
                out.push_str(tag);
                out.push('>');
                out.push_str(&inline(cell.trim()));
                out.push_str("</");
                out.push_str(tag);
                out.push('>');
            }
            out.push_str("</tr>\n");
        }
        out.push_str("</table>\n");
        rows.clear();
        *in_table = false;
    };

    for raw in md.lines() {
        let line = raw.trim_end();
        if line.trim().is_empty() {
            flush_paras(&mut out, &mut paras);
            close_list(&mut out, &mut list);
            flush_table(&mut out, &mut table_rows, &mut in_table);
            continue;
        }
        if line.trim_start().starts_with('|') {
            flush_paras(&mut out, &mut paras);
            close_list(&mut out, &mut list);
            in_table = true;
            let cells: Vec<String> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|s| s.trim().to_string())
                .collect();
            table_rows.push(cells);
            continue;
        }
        if in_table {
            flush_table(&mut out, &mut table_rows, &mut in_table);
        }
        if let Some(rest) = line.strip_prefix("# ") {
            flush_paras(&mut out, &mut paras);
            close_list(&mut out, &mut list);
            out.push_str("<h1>");
            out.push_str(&inline(rest.trim()));
            out.push_str("</h1>\n");
            continue;
        }
        if let Some(rest) = line.strip_prefix("## ") {
            flush_paras(&mut out, &mut paras);
            close_list(&mut out, &mut list);
            out.push_str("<h2>");
            out.push_str(&inline(rest.trim()));
            out.push_str("</h2>\n");
            continue;
        }
        if let Some(rest) = line.strip_prefix("### ") {
            flush_paras(&mut out, &mut paras);
            close_list(&mut out, &mut list);
            out.push_str("<h3>");
            out.push_str(&inline(rest.trim()));
            out.push_str("</h3>\n");
            continue;
        }
        if let Some(rest) = line.strip_prefix("- ") {
            flush_paras(&mut out, &mut paras);
            if list != Some("ul") {
                close_list(&mut out, &mut list);
                out.push_str("<ul>\n");
                list = Some("ul");
            }
            out.push_str("<li>");
            out.push_str(&inline(rest.trim()));
            out.push_str("</li>\n");
            continue;
        }
        close_list(&mut out, &mut list);
        paras.push(line.trim().to_string());
    }
    flush_paras(&mut out, &mut paras);
    close_list(&mut out, &mut list);
    flush_table(&mut out, &mut table_rows, &mut in_table);
    out
}

fn inline(s: &str) -> String {
    let mut out = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == '[' {
            if let Some((label, href, next)) = parse_link(&chars, i) {
                out.push_str("<a href=\"");
                out.push_str(&esc(&href));
                out.push_str("\">");
                out.push_str(&inline(&label));
                out.push_str("</a>");
                i = next;
                continue;
            }
        }
        if chars[i] == '`' {
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == '`') {
                let code: String = chars[i + 1..i + 1 + end].iter().collect();
                out.push_str("<code>");
                out.push_str(&esc(&code));
                out.push_str("</code>");
                i = i + 2 + end;
                continue;
            }
        }
        if chars[i] == '*' && i + 1 < chars.len() && chars[i + 1] == '*' {
            if let Some(end) = find_close(&chars, i + 2, "**") {
                let inner: String = chars[i + 2..end].iter().collect();
                out.push_str("<strong>");
                out.push_str(&inline(&inner));
                out.push_str("</strong>");
                i = end + 2;
                continue;
            }
        }
        if chars[i] == '_' || (chars[i] == '*' && (i + 1 >= chars.len() || chars[i + 1] != '*'))
        {
            let delim = chars[i];
            if let Some(end) = chars[i + 1..].iter().position(|&c| c == delim) {
                let abs = i + 1 + end;
                let inner: String = chars[i + 1..abs].iter().collect();
                if !inner.is_empty() {
                    out.push_str("<em>");
                    out.push_str(&inline(&inner));
                    out.push_str("</em>");
                    i = abs + 1;
                    continue;
                }
            }
        }
        out.push_str(&esc_char(chars[i]));
        i += 1;
    }
    out
}

fn parse_link(chars: &[char], start: usize) -> Option<(String, String, usize)> {
    let mut i = start + 1;
    let mut label = String::new();
    while i < chars.len() && chars[i] != ']' {
        label.push(chars[i]);
        i += 1;
    }
    if i >= chars.len() || chars[i] != ']' {
        return None;
    }
    i += 1;
    if i >= chars.len() || chars[i] != '(' {
        return None;
    }
    i += 1;
    let mut href = String::new();
    while i < chars.len() && chars[i] != ')' {
        href.push(chars[i]);
        i += 1;
    }
    if i >= chars.len() || chars[i] != ')' {
        return None;
    }
    Some((label, href, i + 1))
}

fn find_close(chars: &[char], from: usize, delim: &str) -> Option<usize> {
    let d: Vec<char> = delim.chars().collect();
    let mut i = from;
    while i + d.len() <= chars.len() {
        if chars[i..i + d.len()] == d[..] {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn esc(s: &str) -> String {
    s.chars().map(esc_char).collect::<String>()
}

fn esc_char(c: char) -> String {
    match c {
        '&' => "&amp;".into(),
        '<' => "&lt;".into(),
        '>' => "&gt;".into(),
        '"' => "&quot;".into(),
        _ => c.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn privacy_embeds_and_formats() {
        assert!(PRIVACY_MD.contains("Wadhu privacy policy"));
        assert!(PRIVACY_MD.contains("com.wadhu.smsgateway"));
        assert!(PRIVACY_MD.contains("U62011GJ2026PTC179660"));
        let html = privacy_html();
        assert!(html.contains("<h1>"));
        assert!(html.contains("<strong>one</strong>") || html.contains("one</strong>"));
        assert!(html.contains("<table>"));
    }

    #[test]
    fn all_legal_docs_embed() {
        assert!(TERMS_MD.contains("Wadhu terms of use"));
        assert!(DATA_DELETION_MD.contains("data deletion"));
        assert!(SUPPORT_MD.contains("mearaj@gotigin.com"));
        assert!(ABOUT_MD.contains("GOTIGIN SOFTWARE"));
        assert!(data_deletion_html().contains("<h1>"));
        assert!(support_html().contains("<h1>"));
        assert!(about_html().contains("<h1>"));
        assert!(terms_html().contains("<h1>"));
    }
}
