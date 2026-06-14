use serde_json::Value;

// Phase 4: lift full furniture placement from app-orchestration-bim/src/main.rs lines 446-1201.
// For Phase 1/2/3 this renders accurate zone proportions without furniture overlay.

pub fn render_kp_zone_svg_from_value(val: &Value) -> String {
    let z1 = val.get("zone1_depth_m").and_then(|v| v.as_f64()).unwrap_or(6.0);
    let z2 = val.get("zone2_depth_m").and_then(|v| v.as_f64()).unwrap_or(0.0);
    let z3 = val
        .get("zone3_depth_m")
        .and_then(|v| v.as_f64())
        .filter(|v| *v > 0.0);
    let category = val.get("category").and_then(|v| v.as_str()).unwrap_or("private-office");
    let area_m2 = val.get("area_m2").and_then(|v| v.as_f64());
    render_kp_zone_svg(z1, z2, z3, category, area_m2)
}

pub fn render_kp_zone_svg(
    z1: f64,
    z2: f64,
    z3: Option<f64>,
    category: &str,
    _area_m2: Option<f64>,
) -> String {
    // Drawing area: x0=22, y0=10, max_w=153, h=94, viewBox 180×112
    let x0: f64 = 22.0;
    let y0: f64 = 10.0;
    let max_w: f64 = 153.0;
    let draw_h: f64 = 94.0;

    let z3v = z3.unwrap_or(0.0);
    let total = z1 + z2 + z3v;
    if total <= 0.0 {
        return "<svg viewBox=\"0 0 180 112\" xmlns=\"http://www.w3.org/2000/svg\"></svg>".into();
    }

    let accent = category_accent(category);
    let z1_w = (z1 / total) * max_w;
    let z2_w = (z2 / total) * max_w;
    let z3_w = (z3v / total) * max_w;

    let y1 = y0 + draw_h; // bottom y coordinate

    let mut out = String::with_capacity(1024);
    out.push_str("<svg viewBox=\"0 0 180 112\" xmlns=\"http://www.w3.org/2000/svg\" role=\"img\" aria-label=\"Zone diagram\">");

    // Zone 1 — Habitat (primary accent, full opacity)
    out.push_str(&format!(
        "<rect x=\"{x}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\" fill=\"{a}\" opacity=\"0.85\"/>",
        x = x0, y0 = y0, w = z1_w, h = draw_h, a = accent
    ));

    // Zone 2 — Magazine (half opacity)
    if z2 > 0.0 {
        out.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\" fill=\"{a}\" opacity=\"0.45\"/>",
            x = x0 + z1_w, y0 = y0, w = z2_w, h = draw_h, a = accent
        ));
    }

    // Zone 3 — Corridor (light, 20% opacity)
    if z3v > 0.0 {
        out.push_str(&format!(
            "<rect x=\"{x}\" y=\"{y0}\" width=\"{w}\" height=\"{h}\" fill=\"{a}\" opacity=\"0.20\"/>",
            x = x0 + z1_w + z2_w, y0 = y0, w = z3_w, h = draw_h, a = accent
        ));
    }

    // Zone divider lines (white)
    if z2 > 0.0 {
        let bx = x0 + z1_w;
        out.push_str(&format!(
            "<line x1=\"{bx}\" y1=\"{ya}\" x2=\"{bx}\" y2=\"{yb}\" stroke=\"white\" stroke-width=\"1\" opacity=\"0.6\"/>",
            bx = bx, ya = y0, yb = y1
        ));
    }
    if z3v > 0.0 {
        let bx = x0 + z1_w + z2_w;
        out.push_str(&format!(
            "<line x1=\"{bx}\" y1=\"{ya}\" x2=\"{bx}\" y2=\"{yb}\" stroke=\"white\" stroke-width=\"1\" opacity=\"0.6\"/>",
            bx = bx, ya = y0, yb = y1
        ));
    }

    // Zone labels
    let label_y = y0 + draw_h - 6.0;
    out.push_str(&format!(
        "<text x=\"{x}\" y=\"{y}\" font-family=\"IBM Plex Sans,sans-serif\" font-size=\"8\" fill=\"white\" opacity=\"0.9\">Z1 {z}m</text>",
        x = x0 + z1_w / 2.0 - 10.0,
        y = label_y,
        z = fmt_m(z1),
    ));
    if z2 > 0.0 {
        out.push_str(&format!(
            "<text x=\"{x}\" y=\"{y}\" font-family=\"IBM Plex Sans,sans-serif\" font-size=\"8\" fill=\"{a}\" opacity=\"0.85\">Z2 {z}m</text>",
            x = x0 + z1_w + z2_w / 2.0 - 10.0,
            y = label_y,
            z = fmt_m(z2),
            a = accent,
        ));
    }
    if z3v > 0.0 {
        out.push_str(&format!(
            "<text x=\"{x}\" y=\"{y}\" font-family=\"IBM Plex Sans,sans-serif\" font-size=\"8\" fill=\"{a}\" opacity=\"0.6\">Z3 {z}m</text>",
            x = x0 + z1_w + z2_w + z3_w / 2.0 - 10.0,
            y = label_y,
            z = fmt_m(z3v),
            a = accent,
        ));
    }

    out.push_str("</svg>");
    out
}

fn category_accent(category: &str) -> &'static str {
    match category {
        "private-office" | "PO" => "#164679",
        "medical" | "M" => "#006b3c",
        "business" | "B" => "#5a2d82",
        "laboratory" | "L" | "lab" => "#d46b08",
        "academic" | "A" => "#1a6b5e",
        "civic" | "C" => "#7b2333",
        "corporate-office" | "CO" => "#1c4e80",
        _ => "#164679",
    }
}

fn fmt_m(v: f64) -> String {
    if (v - v.round()).abs() < 0.001 {
        format!("{:.0}", v)
    } else {
        format!("{:.2}", v)
    }
}
