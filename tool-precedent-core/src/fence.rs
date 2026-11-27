// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! Hash-fenced CSS (family token plus components), same fence format and FNV-1a 64 hash as every tool-precedent-* sibling.
use std::path::Path;
use std::process::ExitCode;

pub fn hash(s: &str) -> String {
    let mut h: u64 = 0xcbf29ce484222325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("{h:016x}")
}
pub fn region(id: &str, css: &str) -> String {
    let c = css.trim_end();
    format!("/* @component {} {} */\n{}\n/* @end */\n", id, hash(c), c)
}
pub const OVR_OPEN: &str =
    "/* === PER-DOCUMENT OVERRIDES (preserved by apply: edit here, not above) === */";
pub const OVR_CLOSE: &str = "/* === END PER-DOCUMENT OVERRIDES === */";

/// Existing per-document overrides in a style block (text between the markers), if any.
pub fn overrides(style: &str) -> String {
    match (style.find(OVR_OPEN), style.find(OVR_CLOSE)) {
        (Some(a), Some(b)) if b >= a + OVR_OPEN.len() => {
            style[a + OVR_OPEN.len()..b].trim_matches('\n').to_string()
        }
        _ => String::new(),
    }
}
/// Like `overrides`, but refuses a style block whose markers are unbalanced (one missing, or the
/// close before the open), which `overrides` would read as "no overrides" and so drop the CSS.
pub fn overrides_checked(style: &str) -> Result<String, String> {
    match (style.find(OVR_OPEN), style.find(OVR_CLOSE)) {
        (None, None) => Ok(String::new()),
        (Some(a), Some(b)) if b >= a + OVR_OPEN.len() => {
            Ok(style[a + OVR_OPEN.len()..b].trim_matches('\n').to_string())
        }
        _ => Err("the per-document override markers are unbalanced (one missing, or the end before the start); fix them by hand, nothing was changed".to_string()),
    }
}
pub fn compose(token: &str, comps: &[(String, String)], ovr: &str) -> String {
    let mut s = String::new();
    s.push_str(token.trim_end());
    s.push('\n');
    for (id, css) in comps {
        s.push_str(&region(id, css));
    }
    s.push_str(OVR_OPEN);
    s.push('\n');
    if !ovr.trim().is_empty() {
        s.push_str(ovr.trim_matches('\n'));
        s.push('\n');
    }
    s.push_str(OVR_CLOSE);
    s.push('\n');
    s
}
/// Components named in precedent/manifest.csv for an instrument ("instrument,family,components").
pub fn manifest(root: &Path, instrument: &str) -> Option<(String, Vec<String>)> {
    let t = crate::csv::read_text(&root.join("precedent/manifest.csv")).ok()?;
    for r in crate::csv::parse(&t).into_iter().skip(1) {
        if r.len() >= 3 && r[0] == instrument {
            return Some((
                r[1].clone(),
                r[2].split(';')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect(),
            ));
        }
    }
    None
}
pub fn instrument_of(stem: &str) -> String {
    match stem.rfind("_JW") {
        Some(i) if stem[i + 3..].chars().all(|c| c.is_ascii_digit()) && i + 3 < stem.len() => {
            stem[..i].to_string()
        }
        _ => stem.to_string(),
    }
}
pub fn load_components(root: &Path, ids: &[String]) -> Result<Vec<(String, String)>, String> {
    ids.iter()
        .map(|id| {
            crate::csv::read_text(&root.join("templates/components").join(format!("{id}.css")))
                .map(|c| (id.clone(), c))
                .map_err(|e| format!("component {id}: {e}"))
        })
        .collect()
}
pub fn style_range(html: &str) -> Option<(usize, usize)> {
    let a = html.find("<style>")? + 7;
    let b = html[a..].find("</style>")? + a;
    Some((a, b))
}

/// What `apply` needs besides the page: the family name, its token CSS and its components.
type ApplyInputs = (String, String, Vec<(String, String)>);

fn apply_inputs(target: &str, root: &Path) -> Result<ApplyInputs, String> {
    let stem = Path::new(target)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let inst = instrument_of(&stem);
    let Some((family, ids)) = manifest(root, &inst) else {
        return Err(format!("{inst} is not in precedent/manifest.csv"));
    };
    // The family and component names become file names under `templates/`: plain names only.
    let plain = |n: &str| !n.is_empty() && n != "." && n != ".." && !n.contains(['/', '\\']);
    if let Some(bad) = std::iter::once(&family)
        .chain(ids.iter())
        .find(|n| !plain(n))
    {
        return Err(format!(
            "precedent/manifest.csv: '{bad}' is not a plain name"
        ));
    }
    let token = crate::csv::read_text(&root.join("templates").join(format!("{family}.css")))
        .map_err(|e| format!("family token {family}: {e}"))?;
    let comps = load_components(root, &ids)?;
    Ok((family, token, comps))
}

/// apply <html>: replace the first <style> block with family token + fenced components + the file's own overrides.
pub fn apply(target: &str, root: &Path, args: &[String]) -> ExitCode {
    let html = match crate::csv::read_text(Path::new(target)) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let (family, token, comps) = match apply_inputs(target, root) {
        Ok(i) => i,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let Some((a, b)) = style_range(&html) else {
        eprintln!("error: no <style> block");
        return ExitCode::FAILURE;
    };
    let ovr = match overrides_checked(&html[a..b]) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let new_style = format!("\n{}", compose(&token, &comps, &ovr));
    let out = format!("{}{}{}", &html[..a], new_style, &html[b..]);
    if args.iter().any(|x| x == "--check") {
        println!(
            "{}",
            if out == html {
                "ok"
            } else {
                "style block differs from the family token and components"
            }
        );
        return if out == html {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        };
    }
    if let Err(e) = crate::csv::write_file_atomic(Path::new(target), &out) {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    println!(
        "applied family '{family}' with {} components to {target}",
        comps.len()
    );
    ExitCode::SUCCESS
}
