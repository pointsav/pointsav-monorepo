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
        (Some(a), Some(b)) if b > a => style[a + OVR_OPEN.len()..b].trim_matches('\n').to_string(),
        _ => String::new(),
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
    let t = std::fs::read_to_string(root.join("precedent/manifest.csv")).ok()?;
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
            std::fs::read_to_string(root.join("templates/components").join(format!("{id}.css")))
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

/// apply <html>: replace the first <style> block with family token + fenced components + the file's own overrides.
pub fn apply(target: &str, root: &Path, args: &[String]) -> ExitCode {
    let html = match std::fs::read_to_string(target) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let stem = Path::new(target)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();
    let inst = instrument_of(&stem);
    let Some((family, ids)) = manifest(root, &inst) else {
        eprintln!("error: {inst} is not in precedent/manifest.csv");
        return ExitCode::FAILURE;
    };
    let token = match std::fs::read_to_string(root.join("templates").join(format!("{family}.css")))
    {
        Ok(t) => t,
        Err(e) => {
            eprintln!("error: family token {family}: {e}");
            return ExitCode::FAILURE;
        }
    };
    let comps = match load_components(root, &ids) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let Some((a, b)) = style_range(&html) else {
        eprintln!("error: no <style> block");
        return ExitCode::FAILURE;
    };
    let ovr = overrides(&html[a..b]);
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
    if let Err(e) = std::fs::write(target, out) {
        eprintln!("error: {e}");
        return ExitCode::FAILURE;
    }
    println!(
        "applied family '{family}' with {} components to {target}",
        comps.len()
    );
    ExitCode::SUCCESS
}
