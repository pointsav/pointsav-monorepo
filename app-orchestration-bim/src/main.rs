// app-orchestration-bim v0.0.2
// BIM Object Library — Woodfine server-rendered catalog
// Reads DTCG token files from $BIM_DESIGN_SYSTEM_DIR/tokens/bim/
// Reads research markdown from $BIM_VAULT_DIR/research/
// Serves static assets from $BIM_STATIC_DIR

use axum::{
    body::Body,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Response},
    routing::get,
    Json, Router,
};
use pulldown_cmark::{html as md_html, Options, Parser};
use serde_json::{json, Value};
use std::{collections::HashMap, env, fs, io::Cursor, path::PathBuf, sync::Arc};
use tower_http::services::ServeDir;

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    vault_dir: PathBuf,
    library_dir: PathBuf,
    design_system_dir: PathBuf,
    static_dir: PathBuf,
    #[allow(dead_code)]
    tenant: String,
    public_url: String,
    tokens: Arc<HashMap<String, Value>>,
    token_count: usize,
    components_count: usize,
    research_count: usize,
}

// ---------------------------------------------------------------------------
// Category metadata — known registry
// ---------------------------------------------------------------------------

struct CatMeta {
    display_name: &'static str,
    ifc_anchor: &'static str,
    uniclass: &'static str,
    ifc_hierarchy: &'static str,
    intro: &'static str,
    elements: &'static str,
    card_desc: &'static str,
    property_sets: &'static [(&'static str, &'static str, &'static str)],
}

const SIDEBAR_ORDER: &[(&str, &str)] = &[
    ("spatial", "Spatial"),
    ("elements", "Elements"),
    ("systems", "Systems"),
    ("materials", "Materials"),
    ("assemblies", "Assemblies"),
    ("performance", "Performance"),
    ("identity-codes", "Identity + Codes"),
    ("relationships", "Relationships"),
    ("key-plans", "Key Plans"),
];

fn known_categories() -> HashMap<&'static str, CatMeta> {
    let mut m = HashMap::new();

    m.insert("spatial", CatMeta {
        display_name: "Spatial",
        ifc_anchor: "IfcSpatialElement",
        uniclass: "SL",
        ifc_hierarchy: "IfcRoot → IfcObjectDefinition → IfcObject → IfcSpatialElement",
        intro: "Spatial elements define the hierarchy of a building's geography: site, building, storeys, and individual spaces. They are the containers that built elements occupy, and the entities that jurisdictional and climate zone constraints apply to at a zone level.",
        elements: "IfcSite · IfcBuilding · IfcBuildingStorey · IfcSpace · IfcZone",
        card_desc: "Spaces, levels (IfcBuildingStorey), buildings, sites, and zones",
        property_sets: &[
            ("Pset_SpaceCommon", "IsExternal", "BOOLEAN"),
            ("Pset_SpaceCommon", "NetFloorArea", "IfcAreaMeasure"),
            ("Pset_BuildingCommon", "NumberOfStoreys", "INTEGER"),
            ("Pset_SiteCommon", "BuildableArea", "IfcAreaMeasure"),
        ],
    });

    m.insert("elements", CatMeta {
        display_name: "Elements",
        ifc_anchor: "IfcBuiltElement",
        uniclass: "EE",
        ifc_hierarchy: "IfcRoot → IfcObjectDefinition → IfcObject → IfcElement → IfcBuiltElement",
        intro: "Built elements are the physical components of a building: walls, slabs, columns, beams, doors, windows, roofs, and stairs. They carry fire rating, structural, and performance constraints.",
        elements: "IfcWall · IfcSlab · IfcColumn · IfcBeam · IfcDoor · IfcWindow · IfcRoof · IfcStair",
        card_desc: "Walls, slabs, columns, beams, doors, windows, and other built elements",
        property_sets: &[
            ("Pset_WallCommon", "FireRating", "IfcLabel"),
            ("Pset_SlabCommon", "LoadBearing", "BOOLEAN"),
            ("Pset_DoorCommon", "IsFireExit", "BOOLEAN"),
            ("Pset_WindowCommon", "ThermalTransmittance", "IfcThermalTransmittanceMeasure"),
        ],
    });

    m.insert("systems", CatMeta {
        display_name: "Systems",
        ifc_anchor: "IfcDistributionElement",
        uniclass: "SS",
        ifc_hierarchy: "IfcRoot → IfcObjectDefinition → IfcObject → IfcElement → IfcDistributionElement",
        intro: "Distribution elements are mechanical, electrical, and plumbing (MEP) systems: ducts, pipes, conduits, outlets, and equipment.",
        elements: "IfcDuctSegment · IfcPipeSegment · IfcCableSegment · IfcAirTerminal · IfcFan · IfcPump",
        card_desc: "HVAC, plumbing, electrical distribution, and fire protection systems",
        property_sets: &[
            ("Pset_DuctSegmentTypeCommon", "NominalDiameter", "IfcPositiveLengthMeasure"),
            ("Pset_ElectricMotorTypeCommon", "PowerNominal", "IfcPowerMeasure"),
        ],
    });

    m.insert("materials", CatMeta {
        display_name: "Materials",
        ifc_anchor: "IfcMaterial",
        uniclass: "Pr",
        ifc_hierarchy: "IfcMaterial",
        intro: "Material BIM Objects carry thermal, structural, acoustic, and environmental properties anchored to bSDD URI references and Pset_Material* property sets.",
        elements: "IfcMaterial · IfcMaterialLayer · IfcMaterialProfile · IfcMaterialConstituent",
        card_desc: "Material definitions with bSDD URI references and Pset_Material* property sets",
        property_sets: &[
            ("Pset_MaterialCommon", "MassDensity", "IfcMassDensityMeasure"),
            ("Pset_MaterialOptical", "VisibleTransmittance", "IfcNormalisedRatioMeasure"),
            ("Pset_MaterialThermal", "ThermalConductivity", "IfcThermalConductivityMeasure"),
        ],
    });

    m.insert("assemblies", CatMeta {
        display_name: "Assemblies",
        ifc_anchor: "IfcElementAssembly",
        uniclass: "Co",
        ifc_hierarchy: "IfcRoot → IfcObjectDefinition → IfcObject → IfcElement → IfcElementAssembly",
        intro: "Assemblies are hierarchical compositions of elements that function as a unit: curtain walls, stairs with landings, structural frames, and prefabricated panels.",
        elements: "IfcCurtainWall · IfcStairFlight · IfcRamp · IfcTruss · IfcElementAssembly",
        card_desc: "Composite element assemblies — curtain walls, stair assemblies, roof systems",
        property_sets: &[
            ("Pset_ElementAssemblyCommon", "AssemblyPlace", "IfcAssemblyPlaceEnum"),
        ],
    });

    m.insert("performance", CatMeta {
        display_name: "Performance",
        ifc_anchor: "IfcPropertySet",
        uniclass: "—",
        ifc_hierarchy: "IfcPropertySet · IfcQuantitySet",
        intro: "Performance tokens carry energy, thermal, acoustic, and fire properties as IfcPropertySet and IfcQuantitySet entries. These are the specification values that drive compliance checking.",
        elements: "Pset_ThermalLoad · Pset_SpaceThermalDesign · Pset_ZoneCommon · IfcQuantityArea",
        card_desc: "Property sets expressing thermal, acoustic, structural, and fire performance",
        property_sets: &[
            ("Pset_SpaceThermalDesign", "HeatingDesignLoad", "IfcPowerMeasure"),
            ("Pset_SpaceThermalDesign", "CoolingDesignLoad", "IfcPowerMeasure"),
            ("Pset_ZoneCommon", "IsExternal", "BOOLEAN"),
        ],
    });

    m.insert("identity-codes", CatMeta {
        display_name: "Identity + Codes",
        ifc_anchor: "IfcClassificationReference",
        uniclass: "—",
        ifc_hierarchy: "IfcClassificationReference · IfcConstraint",
        intro: "Identity and classification tokens anchor BIM Objects to external classification systems (Uniclass 2015, OmniClass, CAWS) and jurisdictional code references.",
        elements: "IfcClassificationReference · IfcClassification · IfcConstraint · IfcMetric",
        card_desc: "Uniclass, OmniClass, MasterFormat, and bSDD classification references",
        property_sets: &[],
    });

    m.insert("relationships", CatMeta {
        display_name: "Relationships",
        ifc_anchor: "IfcRel*",
        uniclass: "—",
        ifc_hierarchy: "IfcRelationship",
        intro: "Relationship tokens define how building elements connect, contain, aggregate, and interact with each other through the IFC IfcRel* relationship entity family.",
        elements: "IfcRelContainedInSpatialStructure · IfcRelAggregates · IfcRelConnects · IfcRelAssociates",
        card_desc: "Aggregation, containment, nesting, and constraint relationship templates",
        property_sets: &[],
    });

    m.insert("key-plans", CatMeta {
        display_name: "Key Plans",
        ifc_anchor: "IfcSpace",
        uniclass: "SL_25",
        ifc_hierarchy: "IfcRoot → IfcObjectDefinition → IfcObject → IfcSpatialElement → IfcSpace",
        intro: "Key Plans are the smallest BIM Object unit — spatial programs defined by real furniture placement, a three-zone cross-section (Zone 1 Habitat / Zone 2 Magazine / Zone 3 Corridor), net leasable area, and accessibility compliance. Authored by architects from Woodfine equipment programs; the tool-buildingwidth engine nests them into Tiles and Floor Plates.",
        elements: "Private Office · Medical · Business · Laboratory · Academic · Civic · Corporate Office",
        card_desc: "Leasable spatial programs with zone depths, furniture programs, and compliance data",
        property_sets: &[
            ("Pset_SpaceCommon", "NetFloorArea", "IfcAreaMeasure"),
            ("Pset_SpaceCommon", "IsExternal", "BOOLEAN"),
            ("Pset_OccupancyRequirements", "OccupancyNumber", "INTEGER"),
        ],
    });

    m
}

// ---------------------------------------------------------------------------
// HTML helpers
// ---------------------------------------------------------------------------

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn page_shell(title: &str, current_path: &str, content: &str, state: &AppState) -> String {
    let full_title = if title.is_empty() {
        "BIM Object Library — Woodfine".to_string()
    } else {
        format!("{} — BIM Object Library", title)
    };
    let sidebar = render_sidebar(current_path, state);
    let footer = render_footer(state);
    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>{title}</title>
  <meta name="description" content="Building specifications that enforce compliance at placement, not inspection after the fact. Open-standard IFC 4.3 BIM Object catalog.">
  <meta property="og:title" content="{title}">
  <meta property="og:description" content="Building specifications that enforce compliance at placement, not inspection after the fact.">
  <meta property="og:url" content="{public_url}{path}">
  <meta property="og:type" content="website">
  <link rel="icon" href="data:,">
  <link rel="stylesheet" href="/static/css/fonts.css">
  <link rel="stylesheet" href="/static/css/tokens.css">
  <link rel="stylesheet" href="/static/css/base.css">
  <link rel="stylesheet" href="/static/css/layout.css">
  <link rel="stylesheet" href="/static/css/components.css">
</head>
<body>
<header class="bim-topbar">
  <div class="bim-topbar__inner">
    <a class="bim-topbar__brand" href="/">
      <span class="bim-topbar__org">Woodfine</span>
      <span class="bim-topbar__product">BIM Object Library</span>
    </a>
    <span class="bim-topbar__platform">app-orchestration-bim</span>
  </div>
</header>
<div class="bim-shell">
  <aside class="bim-sidebar" aria-label="Primary">{sidebar}</aside>
  <main class="bim-main">
    <div class="bim-main__inner">
      {content}
    </div>
  </main>
</div>
{footer}
{INLINE_JS}
</body>
</html>"#,
        title = esc(&full_title),
        public_url = esc(&state.public_url),
        path = esc(current_path),
        sidebar = sidebar,
        content = content,
        footer = footer,
        INLINE_JS = INLINE_JS,
    )
}

fn render_sidebar(current_path: &str, _state: &AppState) -> String {
    let mut cat_links = String::new();
    let cats = known_categories();
    for (slug, display) in SIDEBAR_ORDER {
        if !cats.contains_key(slug) {
            continue;
        }
        // Only show link if token file is actually loaded
        let href = format!("/tokens/{}.dtcg", slug);
        let active = if current_path == href {
            r#" bim-sidenav__link--active" aria-current="page"#
        } else {
            r#""#
        };
        cat_links.push_str(&format!(
            "<li><a class=\"bim-sidenav__link{active}\" href=\"{href}\">{name}</a></li>\n",
            active = active,
            href = esc(&href),
            name = esc(display),
        ));
    }

    let overview_links = [
        ("/", "What are BIM Objects?"),
        ("/tokens", "Browse All BIM Objects"),
        ("/about", "About BIM Objects"),
    ];
    let mut overview = String::new();
    for (href, label) in &overview_links {
        let active = if current_path == *href {
            r#" bim-sidenav__link--active" aria-current="page"#
        } else {
            r#""#
        };
        overview.push_str(&format!(
            "<li><a class=\"bim-sidenav__link{active}\" href=\"{href}\">{label}</a></li>\n",
            active = active,
            href = esc(href),
            label = esc(label),
        ));
    }

    format!(
        r#"<nav class="bim-sidenav">
<div class="bim-sidenav__section">
  <div class="bim-sidenav__heading">Overview</div>
  <ul class="bim-sidenav__list">{overview}</ul>
</div>
<div class="bim-sidenav__section">
  <div class="bim-sidenav__heading">BIM Objects</div>
  <ul class="bim-sidenav__list">{cat_links}</ul>
</div>
<div class="bim-sidenav__section">
  <div class="bim-sidenav__heading">Key Plan Tools</div>
  <ul class="bim-sidenav__list">
    <li><a class="bim-sidenav__link{furn_active}" href="/furniture">Private Office Furniture</a></li>
  </ul>
</div>
<div class="bim-sidenav__section">
  <div class="bim-sidenav__heading">Regulatory Overlays</div>
  <ul class="bim-sidenav__list">
    <li><a class="bim-sidenav__link" href="/tokens/elements.dtcg">IBC 2024 — Built Elements</a></li>
  </ul>
</div>
</nav>"#,
        overview = overview,
        cat_links = cat_links,
        furn_active = if current_path == "/furniture" {
            r#" bim-sidenav__link--active" aria-current="page"#
        } else {
            r#""#
        },
    )
}

fn render_footer(state: &AppState) -> String {
    format!(
        r#"<footer class="bim-footer">
  <div class="bim-footer__inner">
    <div>
      <p class="bim-footer__heading">Woodfine BIM Object Library</p>
      <ul class="bim-footer__list">
        <li>Specification BIM Objects for the built environment</li>
        <li>{tc} BIM Object categories &middot; {cc} components &middot; {rc} research entries</li>
        <li>IFC&nbsp;4.3 (ISO&nbsp;16739-1:2024) &middot; Uniclass&nbsp;2015 &middot; IDS&nbsp;1.0 &middot; bSDD</li>
      </ul>
    </div>
    <div>
      <p class="bim-footer__heading">Machine-readable surface</p>
      <ul class="bim-footer__list">
        <li><a href="/tokens.json">/tokens.json</a> &mdash; full DTCG bundle</li>
        <li><a href="/components">/components</a> &mdash; component recipes</li>
        <li><a href="/research">/research</a> &mdash; research backplane</li>
        <li><a href="/healthz">/healthz</a> &middot; <a href="/readyz">/readyz</a></li>
      </ul>
    </div>
    <div>
      <p class="bim-footer__heading">Platform</p>
      <ul class="bim-footer__list">
        <li>Open-source &middot; Apache-2.0</li>
        <li>Powered by <strong>PointSav Digital Systems</strong></li>
        <li><a href="https://pointsav.com">pointsav.com</a></li>
      </ul>
    </div>
  </div>
  <div class="bim-footer__base">&copy; 2026 Woodfine Capital Projects Inc. &middot; {url}</div>
</footer>"#,
        tc = state.token_count,
        cc = state.components_count,
        rc = state.research_count,
        url = esc(&state.public_url),
    )
}

fn breadcrumbs(crumbs: &[(&str, &str)]) -> String {
    let items: String = crumbs
        .iter()
        .map(|(href, label)| {
            format!(
                "<li class=\"bim-crumbs__item\"><a class=\"bim-crumbs__link\" href=\"{href}\">{label}</a></li>",
                href = esc(href),
                label = esc(label),
            )
        })
        .collect();
    format!(
        "<nav class=\"bim-crumbs\" aria-label=\"Breadcrumb\"><ol class=\"bim-crumbs__list\">{}</ol></nav>",
        items
    )
}

// ---------------------------------------------------------------------------
// Key Plan entry extraction
// ---------------------------------------------------------------------------

struct KpEntry {
    display_name: String,
    internal_code: String,
    area_m2: Option<f64>,
    area_sf: Option<u64>,
    zone1: Option<f64>,
    zone2: Option<f64>,
    zone3: Option<f64>,
    #[allow(dead_code)]
    status: String,
    note: String,
    category: String,
}

fn size_order(name: &str) -> u8 {
    if name.contains("Small") || name.contains("1/8") {
        0
    } else if name.contains("Medium") || name.contains("1/4") {
        1
    } else if name.contains("Large") || name.contains("1/3") {
        2
    } else if name.contains("1/2") {
        3
    } else if name.contains("Full") {
        4
    } else {
        5
    }
}

fn cat_order(cat: &str) -> u8 {
    match cat {
        "private-office" => 0,
        "medical" => 1,
        "business" => 2,
        "laboratory" => 3,
        "academic" => 4,
        "civic" => 5,
        "corporate-office" => 6,
        _ => 7,
    }
}

fn render_kp_zone_svg(
    z1: f64,
    z2: f64,
    z3: Option<f64>,
    category: &str,
    area_m2: Option<f64>,
) -> String {
    let d3 = z3.unwrap_or(0.0);
    let total = z1 + z2 + d3;
    if total <= 0.0 {
        return String::new();
    }

    let accent = match category {
        "private-office" => "#1a3a5c",
        "medical" => "#7a1a1a",
        "laboratory" => "#1a4060",
        "business" => "#7a4a00",
        "academic" => "#4a4800",
        "civic" => "#1a5430",
        _ => "#303040",
    };

    // Drawing area: x=22, y=10, max_w=153, h=94 within 180×112 viewBox.
    // Left 22px reserved for Z1/Z2/Z3 labels.
    let x0: f64 = 22.0;
    let y0: f64 = 10.0;
    let max_dw: f64 = 153.0;
    let dh: f64 = 94.0;

    // Proportional width: frontage = area / depth. Normalise against 6 m reference
    // so a 6 m-frontage plan fills the full drawing width; narrower plans shrink.
    let frontage = area_m2.map(|a| a / total).unwrap_or(total);
    let plan_w = ((frontage / 6.0) * max_dw).clamp(max_dw * 0.30, max_dw);
    let xr: f64 = x0 + plan_w;

    let size_tier: u8 = match (category, area_m2) {
        ("private-office", Some(a)) => {
            if a < 38.0 {
                0
            } else if a < 55.0 {
                1
            } else {
                2
            }
        }
        ("medical", Some(a)) => {
            if a < 270.0 {
                0
            } else if a < 410.0 {
                1
            } else {
                2
            }
        }
        ("laboratory", Some(a)) => {
            if a < 260.0 {
                0
            } else if a < 370.0 {
                1
            } else {
                2
            }
        }
        ("academic", Some(a)) => {
            if a < 175.0 {
                0
            } else if a < 315.0 {
                1
            } else {
                2
            }
        }
        ("business", Some(a)) => {
            if a < 360.0 {
                0
            } else if a < 545.0 {
                1
            } else {
                2
            }
        }
        ("civic", Some(a)) => {
            if a < 420.0 {
                0
            } else if a < 700.0 {
                1
            } else {
                2
            }
        }
        _ => 1,
    };

    let h1 = (z1 / total) * dh;
    let h2 = (z2 / total) * dh;
    let h3 = dh - h1 - h2;
    let y1 = y0 + h1;
    let y2 = y1 + h2;

    let lz1a = y0 + h1 * 0.38;
    let lz1b = y0 + h1 * 0.64;
    let lz2a = y1 + h2 * 0.38;
    let lz2b = y1 + h2 * 0.64;
    let lz3a = y2 + h3 * 0.38;
    let lz3b = y2 + h3 * 0.68;

    let mut s = String::with_capacity(2400);

    // SVG open + blueprint-paper background
    s.push_str("<svg class=\"bim-kp-diagram\" viewBox=\"0 0 180 112\" xmlns=\"http://www.w3.org/2000/svg\" aria-hidden=\"true\">");
    s.push_str("<rect width=\"180\" height=\"112\" fill=\"#f0f4f8\"/>");

    // FACADE label
    s.push_str(&format!(
        "<text x=\"108\" y=\"8.5\" font-size=\"5.5\" fill=\"{}\" font-family=\"sans-serif\" text-anchor=\"middle\" letter-spacing=\"1.2\">FACADE</text>",
        accent
    ));

    // Mullion ticks (4 evenly spaced along facade edge, within plan width)
    let mull_step = plan_w / 5.0;
    for i in 1u8..=4 {
        let mx = x0 + mull_step * i as f64;
        s.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"6\" x2=\"{:.1}\" y2=\"{:.0}\" stroke=\"{}\" stroke-width=\"0.8\"/>",
            mx, mx, y0, accent
        ));
    }

    // Zone fills (very subtle tints — floor plan "paper" look)
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#fff9f4\"/>",
        x0, y0, plan_w, h1
    ));
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#fafae8\"/>",
        x0, y1, plan_w, h2
    ));
    if h3 >= 1.0 {
        s.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#f0f8f2\"/>",
            x0, y2, plan_w, h3
        ));
    }

    // Perimeter (accent colour, drawn over fills)
    s.push_str(&format!(
        "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"none\" stroke=\"{}\" stroke-width=\"1.2\"/>",
        x0, y0, plan_w, dh, accent
    ));

    // Zone boundary dashed lines
    s.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#8a9aaa\" stroke-dasharray=\"3.5,2.5\" stroke-width=\"0.75\"/>",
        x0, y1, xr, y1
    ));
    if h3 >= 1.0 {
        s.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#8a9aaa\" stroke-dasharray=\"3.5,2.5\" stroke-width=\"0.75\"/>",
            x0, y2, xr, y2
        ));
    }

    // Zone labels (left of plan)
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"#6a4820\" font-family=\"sans-serif\" text-anchor=\"end\">Z1</text>",
        lz1a
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"#8a6840\" font-family=\"sans-serif\" text-anchor=\"end\">{:.1}m</text>",
        lz1b, z1
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"#4a5020\" font-family=\"sans-serif\" text-anchor=\"end\">Z2</text>",
        lz2a
    ));
    s.push_str(&format!(
        "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"#6a7040\" font-family=\"sans-serif\" text-anchor=\"end\">{:.1}m</text>",
        lz2b, z2
    ));
    if h3 >= 8.0 {
        s.push_str(&format!(
            "<text x=\"21\" y=\"{:.1}\" font-size=\"5\" fill=\"#205040\" font-family=\"sans-serif\" text-anchor=\"end\">Z3</text>",
            lz3a
        ));
        if h3 >= 14.0 {
            s.push_str(&format!(
                "<text x=\"21\" y=\"{:.1}\" font-size=\"4\" fill=\"#407060\" font-family=\"sans-serif\" text-anchor=\"end\">{:.1}m</text>",
                lz3b, d3
            ));
        }
    }

    // ── Category-specific furniture ────────────────────────────────────────
    // Helper: push a desk rectangle + chair circle below it
    macro_rules! desk {
        ($s:expr, $dx:expr, $dy:expr) => {
            $s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"15\" height=\"9\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.5\" rx=\"0.5\"/>", $dx, $dy));
            $s.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"#b0a080\" stroke=\"#8b6a40\" stroke-width=\"0.4\"/>", ($dx as f64) + 7.5, ($dy as f64) + 13.0));
        };
    }
    // Helper: push a round table + N chairs (max 4) at cardinal positions
    macro_rules! round_table {
        ($s:expr, $cx:expr, $cy:expr, $r:expr, $n:expr) => {{
            let (cx, cy, r) = ($cx as f64, $cy as f64, $r as f64);
            $s.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"#d4c4a0\" stroke=\"#8b6a40\" stroke-width=\"0.5\"/>", cx, cy, r));
            let offsets: &[(f64, f64)] = &[(0.0, -(r+3.5)), (r+3.5, 0.0), (0.0, r+3.5), (-(r+3.5), 0.0)];
            for &(dx, dy) in offsets.iter().take($n) {
                $s.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"#b0a080\" stroke=\"#8b6a40\" stroke-width=\"0.4\"/>", cx + dx, cy + dy));
            }
        }};
    }
    // Helper: door arc symbol
    macro_rules! door {
        ($s:expr, $dx:expr, $dy:expr, $dh:expr) => {{
            let (dx, dy, dh) = ($dx as f64, $dy as f64, $dh as f64);
            $s.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#556677\" stroke-width=\"0.75\"/>", dx, dy, dx, dy + dh));
            $s.push_str(&format!("<path d=\"M{:.1},{:.1} A{:.1},{:.1} 0 0,1 {:.1},{:.1}\" stroke=\"#556677\" stroke-width=\"0.75\" fill=\"none\" stroke-dasharray=\"2,1.5\"/>", dx, dy, dh, dh, dx + dh * 0.87, dy + dh * 0.5));
        }};
    }

    match category {
        // ── Private Office ─────────────────────────────────────────────────
        "private-office" => {
            // Desk count: S=1, M=2, L=3
            let desk_n = size_tier as usize + 1;
            for i in 0..desk_n {
                desk!(s, x0 + 3.0 + 19.0 * i as f64, y0 + 3.0);
            }
            if h1 >= 25.0 {
                let tbl_r = (h1 * 0.18).clamp(7.0, 10.0);
                let tbl_x = (x0 + plan_w * 0.58).min(xr - tbl_r - 12.0);
                round_table!(s, tbl_x, y0 + h1 * 0.72, tbl_r, 3);
            }
            // Filing credenza right Z1 (stays clear of last desk)
            let cred_x = (xr - 17.0).max(x0 + 3.0 + 19.0 * desk_n as f64 + 3.0);
            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"14\" height=\"5\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.4\" rx=\"0.3\"/>", cred_x, y0 + 3.0));
            if h2 >= 10.0 {
                let cw = (plan_w * 0.65).min(85.0);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"5\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.5\" rx=\"0.3\"/>", x0 + 3.0, y1 + 3.0, cw));
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"9\" height=\"9\" fill=\"#b8c8d8\" stroke=\"#5a7898\" stroke-width=\"0.4\" rx=\"0.3\"/>", (x0 + cw + 5.0).min(xr - 12.0), y1 + 2.0));
            }
            if h3 >= 10.0 {
                door!(s, x0 + 4.0, y2, (h3 * 0.85).min(13.0));
            }
        }

        // ── Medical ────────────────────────────────────────────────────────
        "medical" => {
            // Doctor office boxes: S=1, L=2
            let doc_n = if size_tier == 2 { 2usize } else { 1 };
            // Dental chairs in Z1: S=2, M=4, L=6
            let chair_n = match size_tier {
                0 => 2usize,
                1 => 4,
                _ => 6,
            };
            for i in 0..doc_n {
                let ox = x0 + 1.0 + 21.0 * i as f64;
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"19\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.6\"/>", ox, y0, h1, accent));
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"6\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.4\" rx=\"0.3\"/>", ox + 2.0, y0 + h1 - 9.0));
            }
            let ch_x0 = x0 + 2.0 + 21.0 * doc_n as f64;
            let ch_area = (xr - 28.0) - ch_x0;
            if h1 >= 15.0 && ch_area > 0.0 {
                let sp = (ch_area / chair_n as f64).max(11.0);
                let cy = y0 + h1 * 0.35;
                for i in 0..chair_n {
                    let cx = ch_x0 + sp * i as f64;
                    if cx + 10.0 > xr - 28.0 {
                        break;
                    }
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"#d0e4d0\" stroke=\"#5a8a6a\" stroke-width=\"0.5\" rx=\"1.5\"/>", cx, cy));
                    s.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3.5\" fill=\"#e4f0e4\" stroke=\"#5a8a6a\" stroke-width=\"0.4\"/>", cx + 5.0, cy - 4.0));
                }
            }
            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"7\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.5\" rx=\"0.5\"/>", xr - 26.0, y0 + 4.0));
            if h2 >= 10.0 {
                let bw = (plan_w * 0.60).min(100.0);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"6\" fill=\"#c0d8c0\" stroke=\"#5a8a6a\" stroke-width=\"0.5\" rx=\"0.5\"/>", x0 + 4.0, y1 + 3.0, bw));
                let sects = (bw / 18.0) as usize;
                for i in 1..sects {
                    let bx = x0 + 4.0 + 18.0 * i as f64;
                    s.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"#5a8a6a\" stroke-width=\"0.3\"/>", bx, y1 + 3.0, bx, y1 + 9.0));
                }
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"11\" height=\"11\" fill=\"#c8d8e0\" stroke=\"#5a7898\" stroke-width=\"0.5\" rx=\"1\"/>", xr - 15.0, y1 + 2.0));
            }
            if h3 >= 8.0 {
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8f0f8\" stroke=\"#7090a8\" stroke-width=\"0.5\" rx=\"0.5\"/>", xr - 20.0, y2 + 2.0, (h3 - 4.0).max(5.0)));
            }
        }

        // ── Laboratory ─────────────────────────────────────────────────────
        "laboratory" => {
            // Z1: reception + offices (S=1, M/L=2) + lab bench clusters (S=3, M=5, L=7)
            let office_n = if size_tier == 0 { 1usize } else { 2 };
            let bench_n = match size_tier {
                0 => 3usize,
                1 => 5,
                _ => 7,
            };
            let rec_h = (h1 * 0.55).max(15.0).min(h1);
            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.5\"/>", x0 + 1.0, y0, rec_h, accent));
            for i in 0..office_n {
                let ox = x0 + 19.0 + 20.0 * i as f64;
                let off_h = (h1 * 0.65).min(h1);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"18\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.5\"/>", ox, y0, off_h, accent));
                if off_h >= 16.0 {
                    desk!(s, ox + 2.0, y0 + off_h - 13.0);
                }
            }
            let bx0 = x0 + 20.0 + 20.0 * office_n as f64;
            let b_area = xr - 4.0 - bx0;
            if b_area > 0.0 && h1 >= 12.0 {
                let bs = b_area / bench_n as f64;
                for i in 0..bench_n {
                    let bx = bx0 + bs * i as f64;
                    if bx + 11.0 > xr - 2.0 {
                        break;
                    }
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"11\" height=\"6\" fill=\"#c0d0c8\" stroke=\"#5a8a6a\" stroke-width=\"0.5\" rx=\"0.3\"/>", bx, y0 + 4.0));
                    s.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2.5\" fill=\"#a0a8b0\" stroke=\"#607080\" stroke-width=\"0.4\"/>", bx + 5.5, y0 + 14.0));
                }
            }
            if h2 >= 10.0 {
                let sr_w = 30.0f64;
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#e0e8e0\" stroke=\"{}\" stroke-width=\"0.5\"/>", x0 + 1.0, y1, sr_w, h2 * 0.85, accent));
                round_table!(s, x0 + 1.0 + sr_w / 2.0, y1 + h2 * 0.42, 6.0, 4);
                let sb_w = (plan_w - sr_w - 10.0).clamp(0.0, 100.0);
                if sb_w > 0.0 {
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"6\" fill=\"#c8d8c0\" stroke=\"#5a8a6a\" stroke-width=\"0.4\" rx=\"0.3\"/>", x0 + sr_w + 5.0, y1 + 3.0, sb_w));
                }
            }
            if h3 >= 8.0 {
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"32\" height=\"5\" fill=\"#d0e0d8\" stroke=\"#5a8a6a\" stroke-width=\"0.5\" rx=\"0.3\"/>", x0 + 4.0, y2 + 2.0));
            }
        }

        // ── Business ───────────────────────────────────────────────────────
        "business" => {
            // Left Z1: exec offices (S=2, M=3, L=5). Right Z1: workstation grid.
            let office_n: usize = match size_tier {
                0 => 2,
                1 => 3,
                _ => 5,
            };
            let ws_cols: usize = match size_tier {
                0 => 3,
                1 => 4,
                _ => 5,
            };
            let ws_rows: usize = match size_tier {
                0 => 3,
                1 => 4,
                _ => 5,
            };
            let conf_n: usize = if size_tier == 2 { 2 } else { 1 };
            let col_n = if office_n > 3 { 2usize } else { 1 };
            let per_col = office_n.div_ceil(col_n);
            let oh = ((h1 - 6.0) / per_col as f64).min(13.0);
            // Reception counter strip at top of office section
            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"3\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.3\" rx=\"0.3\"/>", x0 + 1.0, y0, 16.0 * col_n as f64 + 2.0 * (col_n - 1) as f64));
            for i in 0..per_col {
                let oy = y0 + 5.0 + oh * i as f64;
                if oy + oh > y0 + h1 - 1.0 {
                    break;
                }
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.5\"/>", x0 + 1.0, oy, oh - 0.5, accent));
            }
            if col_n == 2 {
                for i in 0..(office_n - per_col) {
                    let oy = y0 + 5.0 + oh * i as f64;
                    if oy + oh > y0 + h1 - 1.0 {
                        break;
                    }
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.5\"/>", x0 + 19.0, oy, oh - 0.5, accent));
                }
            }
            let ws_x0 = x0 + 2.0 + 18.0 * col_n as f64;
            let ws_aw = xr - ws_x0 - 3.0;
            let ws_sx = ws_aw / ws_cols as f64;
            let ws_sy = (h1 - 2.0) / ws_rows as f64;
            for row in 0..ws_rows {
                for col in 0..ws_cols {
                    let wx = ws_x0 + ws_sx * col as f64;
                    let wy = y0 + 1.0 + ws_sy * row as f64;
                    let ww = (ws_sx - 2.0).clamp(5.0, 16.0);
                    let wh = (ws_sy - 1.5).clamp(3.0, 10.0);
                    if wx + ww > xr - 2.0 {
                        break;
                    }
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#d0c8e0\" stroke=\"#7060a0\" stroke-width=\"0.4\" rx=\"0.3\"/>", wx, wy, ww, wh));
                }
            }
            if h2 >= 12.0 {
                let cw = (plan_w * 0.38).min(58.0);
                let ch = (h2 * 0.48).clamp(8.0, 16.0);
                for ci in 0..conf_n {
                    let cx_t = x0 + 4.0 + ci as f64 * (cw + 6.0);
                    if cx_t + cw > xr - 26.0 {
                        break;
                    }
                    let cy_t = y1 + (h2 - ch) / 2.0;
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#d4c8a0\" stroke=\"#8b7a40\" stroke-width=\"0.5\" rx=\"1\"/>", cx_t, cy_t, cw, ch));
                    let cc = ((cw / 10.0) as usize).max(2);
                    for j in 0..cc {
                        let chair_x = cx_t + (cw / cc as f64) * (j as f64 + 0.5) - 3.0;
                        s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"6\" height=\"3.5\" fill=\"#b0a880\" stroke=\"#8b7a40\" stroke-width=\"0.3\" rx=\"0.5\"/>", chair_x, cy_t - 4.5));
                        s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"6\" height=\"3.5\" fill=\"#b0a880\" stroke=\"#8b7a40\" stroke-width=\"0.3\" rx=\"0.5\"/>", chair_x, cy_t + ch + 1.0));
                    }
                }
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"22\" height=\"{:.1}\" fill=\"#e0e8e0\" stroke=\"{}\" stroke-width=\"0.4\"/>", xr - 24.0, y1, h2 * 0.75, accent));
            }
            if h3 >= 8.0 {
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8f0f8\" stroke=\"#7090a8\" stroke-width=\"0.5\" rx=\"0.3\"/>", xr - 20.0, y2 + 1.0, (h3 - 3.0).max(4.0)));
            }
        }

        // ── Academic ───────────────────────────────────────────────────────
        "academic" => {
            // A-1: workstation bank left + conference table + 2 round tables
            // A-2: workstation banks left+right + oval table + round table
            // A-3: theater seating left + workstation bank + round tables right
            match size_tier {
                0 => {
                    for row in 0..4usize {
                        for col in 0..2usize {
                            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"#d0c8e0\" stroke=\"#7060a0\" stroke-width=\"0.4\" rx=\"0.3\"/>", x0 + 3.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0));
                        }
                    }
                    let ctw = 42.0f64;
                    let cth = (h1 * 0.45).clamp(14.0, 22.0);
                    let cty = y0 + (h1 - cth) / 2.0;
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#d4c8a0\" stroke=\"#8b7a40\" stroke-width=\"0.5\" rx=\"1\"/>", x0 + 29.0, cty, ctw, cth));
                    round_table!(s, x0 + 86.0, y0 + h1 * 0.28, 8.0, 4);
                    round_table!(s, x0 + 86.0, y0 + h1 * 0.72, 8.0, 4);
                }
                1 => {
                    for row in 0..4usize {
                        for col in 0..2usize {
                            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"#d0c8e0\" stroke=\"#7060a0\" stroke-width=\"0.4\" rx=\"0.3\"/>", x0 + 3.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0));
                            let rx2 = xr - 25.0 + col as f64 * 12.0;
                            if rx2 + 10.0 < xr - 2.0 {
                                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"#d0c8e0\" stroke=\"#7060a0\" stroke-width=\"0.4\" rx=\"0.3\"/>", rx2, y0 + 5.0 + row as f64 * 12.0));
                            }
                        }
                    }
                    let ctw = 58.0f64;
                    let cth = (h1 * 0.50).clamp(18.0, 26.0);
                    let cty = y0 + (h1 - cth) / 2.0;
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#d4c8a0\" stroke=\"#8b7a40\" stroke-width=\"0.5\" rx=\"{:.1}\"/>", x0 + 29.0, cty, ctw, cth, cth / 2.0));
                    round_table!(s, x0 + 104.0, y0 + h1 * 0.5, 8.0, 4);
                }
                _ => {
                    // Theater seating rows left
                    let t_rows = ((h1 - 8.0) / 8.0) as usize;
                    for row in 0..t_rows.min(6) {
                        for col in 0..5usize {
                            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"7\" height=\"5\" fill=\"#b8c8d8\" stroke=\"#5a7898\" stroke-width=\"0.3\" rx=\"0.5\"/>", x0 + 3.0 + col as f64 * 9.0, y0 + 5.0 + row as f64 * 8.0));
                        }
                    }
                    // Workstation bank center
                    for row in 0..4usize {
                        for col in 0..2usize {
                            s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"7\" fill=\"#d0c8e0\" stroke=\"#7060a0\" stroke-width=\"0.4\" rx=\"0.3\"/>", x0 + 52.0 + col as f64 * 12.0, y0 + 5.0 + row as f64 * 12.0));
                        }
                    }
                    round_table!(s, x0 + 98.0, y0 + h1 * 0.28, 9.0, 4);
                    round_table!(s, x0 + 98.0, y0 + h1 * 0.72, 9.0, 4);
                    round_table!(s, x0 + 128.0, y0 + h1 * 0.5, 9.0, 4);
                }
            }
            // Z2: instructor desks + storage strip
            if h2 >= 12.0 {
                desk!(s, x0 + 4.0, y1 + 3.0);
                if size_tier >= 1 {
                    desk!(s, x0 + 24.0, y1 + 3.0);
                }
                let sw = (plan_w * 0.32).min(48.0);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"5\" fill=\"#c8d8b8\" stroke=\"#5a7050\" stroke-width=\"0.4\" rx=\"0.3\"/>", xr - sw - 4.0, y1 + 4.0, sw));
            }
        }

        // ── Civic ──────────────────────────────────────────────────────────
        "civic" => {
            // Z1: offices left (S=2, M=4, L=5) + conf rooms right (S=1, M/L=2)
            // L tier also adds court room with theater seating far right
            let office_n: usize = match size_tier {
                0 => 2,
                1 => 4,
                _ => 5,
            };
            let conf_n: usize = match size_tier {
                0 => 1,
                _ => 2,
            };
            let ocols = if office_n > 3 { 2usize } else { 1 };
            let oper_col = office_n.div_ceil(ocols);
            let oh = ((h1 - 2.0) / oper_col as f64).min(12.0);
            for i in 0..oper_col {
                let oy = y0 + 1.0 + oh * i as f64;
                if oy + oh > y0 + h1 - 1.0 {
                    break;
                }
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"13\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.4\"/>", x0 + 1.0, oy, oh - 0.5, accent));
            }
            if ocols == 2 {
                for i in 0..(office_n - oper_col) {
                    let oy = y0 + 1.0 + oh * i as f64;
                    if oy + oh > y0 + h1 - 1.0 {
                        break;
                    }
                    s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"13\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.4\"/>", x0 + 16.0, oy, oh - 0.5, accent));
                }
            }
            // Conference rooms right of offices
            let court_w = if size_tier == 2 { 36.0f64 } else { 0.0 };
            let conf_zone_x = xr - (conf_n as f64 * 32.0 + court_w + 2.0);
            for ci in 0..conf_n {
                let cx = conf_zone_x + ci as f64 * 32.0;
                if cx < x0 + 34.0 {
                    continue;
                }
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"28\" height=\"{:.1}\" fill=\"#e8e0d0\" stroke=\"{}\" stroke-width=\"0.5\"/>", cx, y0 + 1.0, h1 - 2.0, accent));
                let cth = ((h1 - 2.0) * 0.48).min(12.0);
                let cty = y0 + 1.0 + ((h1 - 2.0) - cth) / 2.0;
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"18\" height=\"{:.1}\" fill=\"#d4c8a0\" stroke=\"#8b7a40\" stroke-width=\"0.4\" rx=\"0.5\"/>", cx + 4.0, cty, cth));
            }
            // Court room (L tier only) — far right Z1 with theater seating
            if size_tier == 2 {
                let crx = xr - 34.0;
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"30\" height=\"{:.1}\" fill=\"#f0e8d8\" stroke=\"{}\" stroke-width=\"0.6\"/>", crx, y0 + 1.0, h1 - 2.0, accent));
                let cr_rows = ((h1 - 8.0) / 7.0) as usize;
                for row in 0..cr_rows.min(4) {
                    for col in 0..3usize {
                        let sy = y0 + 3.0 + row as f64 * 7.0;
                        if sy + 4.0 > y0 + h1 - 3.0 {
                            break;
                        }
                        s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"7\" height=\"4\" fill=\"#b8c8d8\" stroke=\"#5a7898\" stroke-width=\"0.3\" rx=\"0.3\"/>", crx + 2.0 + col as f64 * 9.0, sy));
                    }
                }
            }
            // Reception counter between offices and conf rooms
            let rec_start = x0 + 2.0 + 14.0 * ocols as f64 + 3.0;
            let rec_end = conf_zone_x - 2.0;
            if rec_end - rec_start >= 8.0 {
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"4\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.4\" rx=\"0.3\"/>", rec_start, y0 + 2.0, (rec_end - rec_start).min(28.0)));
            }
            // Z2: staff room + restroom
            if h2 >= 12.0 {
                let sr_w = (plan_w * 0.38).min(58.0);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\" fill=\"#e0e8e0\" stroke=\"{}\" stroke-width=\"0.5\"/>", x0 + 1.0, y1, sr_w, h2 * 0.82, accent));
                round_table!(s, x0 + 1.0 + sr_w / 2.0, y1 + h2 * 0.40, 8.0, 4);
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"16\" height=\"{:.1}\" fill=\"#e8f0f8\" stroke=\"#7090a8\" stroke-width=\"0.5\" rx=\"0.3\"/>", xr - 20.0, y1 + 1.0, (h2 * 0.65).min(h2 - 2.0)));
            }
            // Z3: communal corridor with door + bench
            if h3 >= 8.0 {
                door!(s, x0 + 4.0, y2, (h3 * 0.80).min(14.0));
                s.push_str(&format!("<rect x=\"{:.1}\" y=\"{:.1}\" width=\"32\" height=\"4\" fill=\"#c8b48a\" stroke=\"#8b6a40\" stroke-width=\"0.3\" rx=\"0.3\"/>", x0 + 22.0, y2 + 2.0));
            }
        }

        _ => {}
    }

    // CORE label
    s.push_str(&format!(
        "<text x=\"108\" y=\"110\" font-size=\"5.5\" fill=\"{}\" font-family=\"sans-serif\" text-anchor=\"middle\" letter-spacing=\"1.2\">CORE</text>",
        accent
    ));

    s.push_str("</svg>");
    s
}

fn render_kp_fraction_svg(display_name: &str) -> String {
    let fraction = if display_name.contains("1/8") {
        0.125
    } else if display_name.contains("1/4") {
        0.25
    } else if display_name.contains("1/3") {
        1.0 / 3.0
    } else if display_name.contains("1/2") {
        0.5
    } else {
        1.0
    };
    let fill_w = (164.0 * fraction) as u32;
    let label = if display_name.contains("1/8") {
        "1/8 Floor"
    } else if display_name.contains("1/4") {
        "1/4 Floor"
    } else if display_name.contains("1/3") {
        "1/3 Floor"
    } else if display_name.contains("1/2") {
        "1/2 Floor"
    } else {
        "Full Floor"
    };
    format!(
        r##"<svg class="bim-kp-diagram" viewBox="0 0 180 112" xmlns="http://www.w3.org/2000/svg" aria-hidden="true">
  <text x="90" y="8" font-size="7" fill="#888" font-family="sans-serif" text-anchor="middle" letter-spacing="1.5">FLOOR PLATE</text>
  <rect x="8" y="12" width="164" height="88" fill="#ebebeb" stroke="#ccc" stroke-width="0.5"/>
  <rect x="8" y="12" width="{fw}" height="88" fill="#c8d8e8" stroke="#a0b8cc" stroke-width="0.5"/>
  <text x="90" y="62" font-size="14" fill="#5a7898" font-family="sans-serif" text-anchor="middle" font-weight="600">{lbl}</text>
  <text x="90" y="80" font-size="8" fill="#888" font-family="sans-serif" text-anchor="middle">of net leasable area</text>
  <text x="90" y="110" font-size="7" fill="#888" font-family="sans-serif" text-anchor="middle" letter-spacing="1.5">SIZED AGAINST FLOOR PLATE</text>
</svg>"##,
        fw = fill_w,
        lbl = label
    )
}

fn collect_key_plan_entries(token: &Value) -> Vec<KpEntry> {
    let mut entries = Vec::new();
    let Some(bim) = token.get("bim").and_then(|b| b.get("key-plan")) else {
        return entries;
    };
    let Some(categories) = bim.as_object() else {
        return entries;
    };
    for (cat_slug, cat_val) in categories {
        if cat_slug.starts_with('$') {
            continue;
        }
        let Some(sizes) = cat_val.as_object() else {
            continue;
        };
        for (size_slug, entry_val) in sizes {
            if size_slug.starts_with('$') {
                continue;
            }
            let val = match entry_val.get("$value") {
                Some(v) => v,
                None => continue,
            };
            let display_name = val
                .get("display_name")
                .and_then(|v| v.as_str())
                .unwrap_or(size_slug)
                .to_string();
            let internal_code = val
                .get("internal_code")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let area_m2 = val.get("area_m2").and_then(|v| v.as_f64());
            let area_sf = val.get("area_sf").and_then(|v| v.as_u64());
            let zone1 = val.get("zone1_depth_m").and_then(|v| v.as_f64());
            let zone2 = val.get("zone2_depth_m").and_then(|v| v.as_f64());
            let zone3 = val.get("zone3_depth_m").and_then(|v| v.as_f64());
            let status = val
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            let note = val
                .get("design_notes")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            entries.push(KpEntry {
                display_name,
                internal_code,
                area_m2,
                area_sf,
                zone1,
                zone2,
                zone3,
                status,
                note,
                category: cat_slug.clone(),
            });
        }
    }
    // Sort: by category group, then Small → Medium → Large within each group
    entries.sort_by(|a, b| {
        cat_order(&a.category)
            .cmp(&cat_order(&b.category))
            .then(size_order(&a.display_name).cmp(&size_order(&b.display_name)))
            .then(a.display_name.cmp(&b.display_name))
    });
    entries
}

fn render_kp_entries_tab(token: &Value) -> String {
    let entries = collect_key_plan_entries(token);
    if entries.is_empty() {
        return "<p style=\"color:var(--bim-fg-muted)\">No Key Plan entries found.</p>".to_string();
    }
    let mut cards = String::new();
    for e in &entries {
        let area_str = match (e.area_sf, e.area_m2) {
            (Some(sf), Some(m2)) => format!("{sf} SF  ·  {m2} m²"),
            (Some(sf), None) => format!("{sf} SF"),
            (None, Some(m2)) => format!("{m2} m²"),
            _ => String::new(),
        };
        let diagram = match (e.zone1, e.zone2) {
            (Some(z1), Some(z2)) => render_kp_zone_svg(z1, z2, e.zone3, &e.category, e.area_m2),
            _ => render_kp_fraction_svg(&e.display_name),
        };
        let note_row = if e.note.is_empty() {
            String::new()
        } else {
            format!(r#"<p class="bim-kp-note">{}</p>"#, esc(&e.note))
        };
        let area_row = if area_str.is_empty() {
            String::new()
        } else {
            format!(r#"<p class="bim-kp-area">{}</p>"#, esc(&area_str))
        };
        cards.push_str(&format!(
            r#"<div class="bim-token-card bim-kp-card">
  {diagram}
  <div class="bim-kp-data">
    <div class="bim-kp-header">
      <span class="bim-kp-name">{name}</span>
    </div>
    {area_row}
    {note_row}
    <p class="bim-kp-code">{code}</p>
  </div>
</div>"#,
            diagram = diagram,
            name = esc(&e.display_name),
            area_row = area_row,
            note_row = note_row,
            code = esc(&e.internal_code),
        ));
    }
    format!(
        r#"<style>
.bim-kp-card {{ display:flex; flex-direction:column; text-decoration:none; cursor:default; padding:0; overflow:hidden; }}
.bim-kp-diagram {{ display:block; width:100%; height:auto; border-bottom:1px solid var(--bim-border,#e0e0e0); }}
.bim-kp-data {{ padding:var(--bim-space-3,0.75rem); display:flex; flex-direction:column; gap:0.25rem; flex:1; }}
.bim-kp-header {{ display:flex; align-items:center; gap:0.4rem; flex-wrap:wrap; }}
.bim-kp-name {{ font-weight:600; font-size:0.88rem; line-height:1.2; }}
.bim-kp-badge {{ font-size:0.6rem; font-weight:600; padding:0.1rem 0.4rem; border-radius:2px; white-space:nowrap; letter-spacing:0.03em; }}
.bim-kp-badge--ok {{ background:rgba(74,140,92,0.13); color:#3a7a4a; }}
.bim-kp-badge--tbd {{ background:rgba(128,128,128,0.1); color:var(--bim-fg-muted,#888); }}
.bim-kp-area {{ font-family:var(--bim-font-mono); font-size:0.78rem; font-weight:600; }}
.bim-kp-note {{ font-size:0.72rem; color:var(--bim-fg-muted,#888); font-style:italic; }}
.bim-kp-code {{ font-size:0.85rem; font-weight:700; color:var(--bim-fg-muted,#666); font-family:var(--bim-font-mono); margin-top:auto; padding-top:0.25rem; letter-spacing:0.04em; }}
</style>
<p style="margin-bottom:var(--bim-space-4);color:var(--bim-fg-muted)">{count} Key Plans — confirmed sizes from FIN.xlsx Summary_Key Plans; Corporate Office sized against Floor Plate.</p>
<div class="bim-token-grid" id="token-grid">{cards}</div>"#,
        count = entries.len(),
        cards = cards,
    )
}

// ---------------------------------------------------------------------------
// Token category page renderer
// ---------------------------------------------------------------------------

fn render_token_page(slug: &str, state: &AppState) -> String {
    let cats = known_categories();
    let meta = cats.get(slug);

    let (display_name, ifc_anchor, uniclass, ifc_hierarchy, intro, elements_str) = match meta {
        Some(m) => (
            m.display_name,
            m.ifc_anchor,
            m.uniclass,
            m.ifc_hierarchy,
            m.intro,
            m.elements,
        ),
        None => ("Tokens", "IfcRoot", "—", "IfcRoot", "", ""),
    };

    let token_json = state.tokens.get(slug);

    // Spec table
    let psets_rows = match meta {
        Some(m) if !m.property_sets.is_empty() => {
            m.property_sets
                .iter()
                .map(|(ps, prop, ty)| {
                    format!(
                        "<tr><td><code>{ps}</code></td><td><code>{prop}</code></td><td><code>{ty}</code></td></tr>",
                        ps = esc(ps), prop = esc(prop), ty = esc(ty)
                    )
                })
                .collect::<String>()
        }
        _ => "<tr><td colspan=\"3\" style=\"color:var(--bim-fg-muted)\">No property sets defined for this category</td></tr>".to_string(),
    };

    // DTCG JSON bundle
    let json_bundle = match token_json {
        Some(v) => serde_json::to_string_pretty(v).unwrap_or_default(),
        None => "{}".to_string(),
    };
    let json_escaped = esc(&json_bundle);

    // Key Plans entries tab — only for key-plans category
    let entries_tab = if slug == "key-plans" {
        let entries_html = match token_json {
            Some(v) => render_kp_entries_tab(v),
            None => "<p style=\"color:var(--bim-fg-muted)\">Token file not loaded.</p>".to_string(),
        };
        format!(
            r#"<details class="bim-tab" id="bim-objects">
  <summary class="bim-tab__summary">BIM Objects</summary>
  <div class="bim-tab__panel">{}</div>
</details>"#,
            entries_html
        )
    } else {
        String::new()
    };

    let overlay_count = 0usize; // v0.0.3 will wire IDS regulation overlays

    let content = format!(
        r#"{crumbs}
<header class="bim-page-header">
  <p class="bim-ifc-anchor">{ifc}</p>
  <h1>{name}</h1>
</header>
<div class="bim-chip-row">
  <span class="bim-chip bim-chip--ifc"><span class="bim-chip__label">IFC</span><span class="bim-chip__value">{ifc}</span></span>
  <span class="bim-chip bim-chip--uniclass"><span class="bim-chip__label">Uniclass</span><span class="bim-chip__value">{uni}</span></span>
  <span class="bim-chip bim-chip--codes"><span class="bim-chip__label">regulatory overlays</span><span class="bim-chip__value">{oc} registered</span></span>
</div>
<div class="bim-tab-bar">
<details class="bim-tab" id="specification" open>
  <summary class="bim-tab__summary">Specification</summary>
  <div class="bim-tab__panel">
    <p class="bim-category-intro">{intro}</p>
    <p class="bim-category-elements">{elems}</p>
    <div class="bim-table-wrap"><table><tbody>
      <tr><th>IFC entity</th><td><code>{ifc}</code></td></tr>
      <tr><th>Uniclass 2015</th><td><code>{uni}</code></td></tr>
      <tr><th>bSDD URI</th><td style="color:var(--bim-fg-muted)">pending — v0.0.3</td></tr>
      <tr><th>IFC hierarchy</th><td><code class="bim-ifc-hierarchy">{hier}</code></td></tr>
    </tbody></table></div>
    <h3 class="bim-section-heading" style="margin-top:var(--bim-space-6)">Applicable property sets</h3>
    <div class="bim-table-wrap"><table>
      <thead><tr><th>Property set</th><th>Property</th><th>Type</th></tr></thead>
      <tbody>{psets}</tbody>
    </table></div>
    <details style="margin-top:var(--bim-space-6)">
      <summary style="cursor:pointer;font-size:0.875rem;color:var(--bim-fg-muted)">DTCG JSON bundle (machine consumers)</summary>
      <div class="bim-code-block" style="margin-top:var(--bim-space-3)">
        <button type="button" class="bim-code-block__copy" data-bim-copy>copy</button>
        <pre><code>{json}</code></pre>
      </div>
    </details>
  </div>
</details>
{entries_tab}
<details class="bim-tab" id="regulation">
  <summary class="bim-tab__summary">Regulation</summary>
  <div class="bim-tab__panel">
    <p style="color:var(--bim-fg-muted);margin-bottom:var(--bim-space-4)">Regulatory overlays are static lookup tables — not runtime selections. Each row below is one registered jurisdictional constraint for this element type.</p>
    <div class="bim-table-wrap"><table>
      <thead><tr><th>Jurisdiction</th><th>Standard</th><th>Element</th><th>Constraint</th><th>Required value</th><th>Source</th></tr></thead>
      <tbody><tr><td colspan="6" style="color:var(--bim-fg-muted);font-style:italic">No regulatory overlays registered for <code>{ifc}</code> — BC RS-1 in development (v0.0.3)</td></tr></tbody>
    </table></div>
  </div>
</details>
<details class="bim-tab" id="climate-zone">
  <summary class="bim-tab__summary">Climate Zone</summary>
  <div class="bim-tab__panel">
    <p style="color:var(--bim-fg-muted);margin-bottom:var(--bim-space-4)">Climate zone performance requirements are embedded token data — not runtime selections. All registered zones are shown simultaneously as reference data.</p>
    <div class="bim-table-wrap"><table>
      <thead><tr><th>Zone</th><th>Parameter</th><th>Required value</th><th>Unit</th><th>Source standard</th></tr></thead>
      <tbody>
        <tr><td>Arctic</td><td>Max U-value (ext. wall)</td><td>0.12</td><td>W/m²K</td><td>ASHRAE / NBC 2020</td></tr>
        <tr><td>Temperate</td><td>Max U-value (ext. wall)</td><td>0.18</td><td>W/m²K</td><td>ASHRAE / NBC 2020</td></tr>
      </tbody>
    </table></div>
  </div>
</details>
<details class="bim-tab" id="token-format">
  <summary class="bim-tab__summary">Token Format</summary>
  <div class="bim-tab__panel">
    <p style="color:var(--bim-fg-muted);margin-bottom:var(--bim-space-4)">BIM Objects are stored in W3C Design Token Community Group (DTCG) format JSON. The complete catalog is available at <a href="/tokens.json"><code>GET /tokens.json</code></a>.</p>
    <p style="color:var(--bim-fg-muted);font-size:var(--bim-text-xs);font-family:var(--bim-font-mono)">Schema: dtcg-bim-bundle-v1 &nbsp;&middot;&nbsp; Format: application/json &nbsp;&middot;&nbsp; Endpoint: GET /tokens.json</p>
  </div>
</details>
</div>"#,
        crumbs = breadcrumbs(&[("/", "Home"), ("/tokens", "BIM Objects")]),
        ifc = esc(ifc_anchor),
        name = esc(display_name),
        uni = esc(uniclass),
        oc = overlay_count,
        intro = esc(intro),
        elems = esc(elements_str),
        hier = esc(ifc_hierarchy),
        psets = psets_rows,
        json = json_escaped,
        entries_tab = entries_tab,
    );

    page_shell(
        display_name,
        &format!("/tokens/{}.dtcg", slug),
        &content,
        state,
    )
}

// ---------------------------------------------------------------------------
// Route handlers
// ---------------------------------------------------------------------------

async fn home_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let token_count = state.token_count;
    let content = format!(
        r#"<div class="bim-hero">
  <p class="bim-hero__eyebrow">Woodfine BIM Object Library</p>
  <p class="bim-hero__statline">Building specifications that enforce compliance at placement,<br>not inspection after the fact.</p>
  <p class="bim-hero__lead">The AEC industry has spent twenty years validating BIM models after design is complete. BIM Objects take a different position: if every element in the design library already encodes its regulatory requirements and performance constraints, a non-compliant model cannot be assembled. Compliance is a property of the starting material, not a filter applied at the end.</p>
  <div class="bim-chip-row">
    <span class="bim-chip bim-chip--ifc"><span class="bim-chip__label">categories</span><span class="bim-chip__value">{tc}</span></span>
    <span class="bim-chip bim-chip--ifc"><span class="bim-chip__label">standard</span><span class="bim-chip__value">IFC 4.3 · ISO 16739-1:2024</span></span>
    <span class="bim-chip bim-chip--codes"><span class="bim-chip__label">format</span><span class="bim-chip__value">DTCG</span></span>
  </div>
</div>
<article class="bim-prose bim-article">
  <section>
    <h2>The problem with building specifications</h2>
    <p>Every building project generates thousands of specification decisions — fire ratings, thermal values, structural classifications, material provenance. Those decisions are scattered across incompatible containers: proprietary model files, PDF specification clauses, product data sheets, contractor RFIs, O&amp;M binders. None of them travel reliably between the software tools that design, finance, regulate, and manage buildings.</p>
    <p>The U.S. construction sector loses an estimated $31.3 billion annually to rework caused by data inconsistencies. At project handover, the BIM model that cost hundreds of thousands of dollars to produce is commonly delivered to the owner as a static PDF extract.</p>
  </section>
  <section>
    <h2>BIM Objects as the answer</h2>
    <p>A BIM Object is a machine-readable specification unit stored in W3C DTCG format JSON. Each object carries its IFC 4.3 entity anchor, Uniclass 2015 classification, applicable property sets, and regulatory overlays as structured data — not prose. The object travels with the element through every tool in the AEC stack.</p>
    <p>When an architect places a wall, the BIM Object for that wall already knows its required fire rating, its thermal transmittance range, and which jurisdictional code clause governs it. No post-hoc checking. No separate specification document. The compliance constraint is encoded in the starting material.</p>
  </section>
  <section>
    <h2>Browse the catalog</h2>
    <p>The catalog is organized by IFC 4.3 entity class. <a href="/tokens">Browse all BIM Object categories</a> or navigate by category in the sidebar.</p>
  </section>
</article>"#,
        tc = token_count,
    );
    Html(page_shell("", "/", &content, &state))
}

async fn tokens_index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let cats = known_categories();
    let mut cards = String::new();
    for (slug, display) in SIDEBAR_ORDER {
        let Some(meta) = cats.get(slug) else { continue };
        cards.push_str(&format!(
            r#"<a class="bim-token-card" href="/tokens/{slug}.dtcg">
  <h3 class="bim-token-card__name">{name}</h3>
  <p class="bim-token-card__desc">{desc}</p>
  <p class="bim-token-card__anchor">&rsaquo; {ifc}</p>
</a>"#,
            slug = esc(slug),
            name = esc(display),
            desc = esc(meta.card_desc),
            ifc = esc(meta.ifc_anchor),
        ));
    }
    let content = format!(
        r#"{crumbs}
<header class="bim-page-header">
  <h1>BIM Objects</h1>
</header>
<div style="margin-bottom:var(--bim-space-4)">
  <input type="search" id="token-filter" class="bim-search" placeholder="Filter BIM Object categories…" style="width:100%;max-width:32rem">
</div>
<div class="bim-token-grid" id="token-grid">{cards}</div>"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
        cards = cards,
    );
    Html(page_shell(
        "Browse All BIM Objects",
        "/tokens",
        &content,
        &state,
    ))
}

async fn token_category_handler(
    Path(name): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let slug = name.trim_end_matches(".dtcg");
    Html(render_token_page(slug, &state))
}

async fn about_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let content = format!(
        r#"{crumbs}
<header class="bim-page-header">
  <h1>About BIM Objects</h1>
</header>
<article class="bim-prose bim-article">
  <section>
    <h2>What is a BIM Object?</h2>
    <p>A BIM Object is a machine-readable specification unit stored in W3C Design Token Community Group (DTCG) format JSON. Each object is anchored to an IFC 4.3 entity class and carries three layers of constraint data:</p>
    <ol>
      <li><strong>Specification</strong> — the IFC entity anchor, applicable Uniclass 2015 code, bSDD URI, and property set definitions.</li>
      <li><strong>Regulation</strong> — jurisdictional overlays (building code clauses, fire ratings, energy standards) registered against the element type.</li>
      <li><strong>Climate Zone</strong> — performance requirements that vary by geographic zone (ASHRAE, NBC 2020).</li>
    </ol>
  </section>
  <section>
    <h2>Key Plans</h2>
    <p>Key Plans extend the BIM Object model to the spatial program layer. A Key Plan is the smallest leasable BIM Object: a bounded IfcSpace defined by real furniture placement (Steelcase, Midmark manufacturer SKUs), a three-zone cross-section (Zone 1 Habitat / Zone 2 Magazine / Zone 3 Corridor), net leasable area, and accessibility compliance.</p>
    <p>Key Plans nest into Tiles (climate zone boundaries), which nest into Floor Plates (full building-floor programs). The tool-buildingwidth Rust engine computes remainder-free nesting in both directions.</p>
  </section>
  <section>
    <h2>Standards</h2>
    <ul>
      <li><strong>IFC 4.3</strong> (ISO 16739-1:2024) — entity backbone</li>
      <li><strong>Uniclass 2015</strong> — classification floor</li>
      <li><strong>IDS 1.0</strong> — regulatory overlay constraint format</li>
      <li><strong>bSDD</strong> (buildingSMART Data Dictionary) — URI authority</li>
      <li><strong>DTCG</strong> — W3C Design Token Community Group token format</li>
    </ul>
  </section>
</article>"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
    );
    Html(page_shell("About BIM Objects", "/about", &content, &state))
}

async fn research_index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let research_dir = state.vault_dir.join("research");
    let mut items = String::new();
    if let Ok(entries) = fs::read_dir(&research_dir) {
        let mut files: Vec<_> = entries
            .flatten()
            .filter(|e| e.path().extension().map(|x| x == "md").unwrap_or(false))
            .collect();
        files.sort_by_key(|e| e.file_name());
        for entry in files {
            let path = entry.path();
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            let content = fs::read_to_string(&path).unwrap_or_default();
            let title = content
                .lines()
                .find(|l| l.starts_with("# "))
                .map(|l| l.trim_start_matches("# ").to_string())
                .unwrap_or_else(|| slug.clone());
            items.push_str(&format!(
                r#"<a class="bim-token-card" href="/research/{slug}">
  <h3 class="bim-token-card__name">{title}</h3>
  <p class="bim-token-card__anchor">&rsaquo; /research/{slug}</p>
</a>"#,
                slug = esc(&slug),
                title = esc(&title),
            ));
        }
    }
    let content = format!(
        r#"{crumbs}
<header class="bim-page-header"><h1>Research</h1></header>
<div class="bim-token-grid">{items}</div>"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
        items = items,
    );
    Html(page_shell("Research", "/research", &content, &state))
}

async fn research_item_handler(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let path = state
        .vault_dir
        .join("research")
        .join(format!("{}.md", slug));
    let md = fs::read_to_string(&path).map_err(|_| StatusCode::NOT_FOUND)?;
    let mut html_out = String::new();
    let parser = Parser::new_ext(&md, Options::all());
    md_html::push_html(&mut html_out, parser);
    let content = format!(
        r#"{}<article class="bim-prose bim-article">{}</article>"#,
        breadcrumbs(&[("/", "Home"), ("/research", "Research")]),
        html_out
    );
    // extract title from first h1
    let title = md
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l.trim_start_matches("# ").to_string())
        .unwrap_or_else(|| slug.clone());
    Ok(Html(page_shell(
        &title,
        &format!("/research/{}", slug),
        &content,
        &state,
    )))
}

async fn components_index_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let comp_dir = state.design_system_dir.join("components");
    let mut items = String::new();
    if let Ok(entries) = fs::read_dir(&comp_dir) {
        let mut files: Vec<_> = entries
            .flatten()
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|x| x == "md" || x == "html")
                    .unwrap_or(false)
            })
            .collect();
        files.sort_by_key(|e| e.file_name());
        for entry in files {
            let path = entry.path();
            let slug = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            items.push_str(&format!(
                r#"<a class="bim-token-card" href="/components/{slug}">
  <h3 class="bim-token-card__name">{title}</h3>
  <p class="bim-token-card__anchor">&rsaquo; /components/{slug}</p>
</a>"#,
                slug = esc(&slug),
                title = esc(&slug.replace('-', " ")),
            ));
        }
    }
    let content = format!(
        r#"{crumbs}
<header class="bim-page-header"><h1>Components</h1></header>
<div class="bim-token-grid">{items}</div>"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
        items = items,
    );
    Html(page_shell("Components", "/components", &content, &state))
}

async fn components_item_handler(
    Path(slug): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Html<String>, StatusCode> {
    let base = state.design_system_dir.join("components");
    let md_path = base.join(format!("{}.md", slug));
    let html_path = base.join(format!("{}.html", slug));
    if md_path.exists() {
        let md = fs::read_to_string(&md_path).map_err(|_| StatusCode::NOT_FOUND)?;
        let mut html_out = String::new();
        let parser = Parser::new_ext(&md, Options::all());
        md_html::push_html(&mut html_out, parser);
        let content = format!(
            r#"{crumbs}<article class="bim-prose bim-article">{html_out}</article>"#,
            crumbs = breadcrumbs(&[("/", "Home"), ("/components", "Components")]),
            html_out = html_out
        );
        let title = slug.replace('-', " ");
        Ok(Html(page_shell(
            &title,
            &format!("/components/{}", slug),
            &content,
            &state,
        )))
    } else if html_path.exists() {
        let html_out = fs::read_to_string(&html_path).map_err(|_| StatusCode::NOT_FOUND)?;
        let content = format!(
            r#"{crumbs}{html_out}"#,
            crumbs = breadcrumbs(&[("/", "Home"), ("/components", "Components")]),
            html_out = html_out
        );
        let title = slug.replace('-', " ");
        Ok(Html(page_shell(
            &title,
            &format!("/components/{}", slug),
            &content,
            &state,
        )))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn tokens_json_handler(State(state): State<Arc<AppState>>) -> Json<Value> {
    let bundle: HashMap<&String, &Value> = state.tokens.iter().collect();
    Json(json!(bundle))
}

async fn healthz_handler() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

async fn readyz_handler(State(state): State<Arc<AppState>>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "vault_dir": state.vault_dir.to_string_lossy(),
        "theme": "pointsav-brand",
        "components_count": state.components_count,
        "tokens_count": state.token_count,
        "research_count": state.research_count,
    }))
}

// ---------------------------------------------------------------------------
// Furniture — Private Office BIM Objects
// ---------------------------------------------------------------------------

struct FurnitureItem {
    slug: String,
    sidebar_label: String,
    manufacturer: String,
    product_line: String,
    #[allow(dead_code)]
    model: String,
    sku: Option<String>,
    sku_note: Option<String>,
    w_mm: u32,
    d_mm: u32,
    h_min_mm: u32,
    clearance_front: u32,
    #[allow(dead_code)]
    clearance_side: u32,
    weight_kg: Option<f64>,
    ifc_class: String,
    url: Option<String>,
    description: String,
}

fn furn_sidebar_label(product_slug: &str, model: &str) -> String {
    match product_slug {
        "steelcase-migration-se-58x29" => "Migration SE Desk".to_string(),
        "steelcase-leap-v2" => "Leap V2 Chair".to_string(),
        "steelcase-groupwork-36" => "Groupwork 36\u{22} Table".to_string(),
        "steelcase-currency-credenza-72" => "Currency Credenza".to_string(),
        "steelcase-ts-mobile-pedestal" => "TS Mobile Pedestal".to_string(),
        "steelcase-currency-bookcase-36" => "Currency Bookcase".to_string(),
        "coalesse-wing-ch445" => "Wing Chair CH445".to_string(),
        "generic-coat-rack" => "Coat Rack".to_string(),
        _ => model
            .split(" \u{2014} ")
            .next()
            .unwrap_or(model)
            .to_string(),
    }
}

fn extract_furniture(tokens: &HashMap<String, Value>) -> Vec<FurnitureItem> {
    let mut items = Vec::new();
    let Some(furn_root) = tokens
        .get("interior")
        .and_then(|v| v.get("bim"))
        .and_then(|v| v.get("interior"))
        .and_then(|v| v.get("furniture"))
        .and_then(|v| v.as_object())
    else {
        return items;
    };
    let cat_order = [
        "desk",
        "task-chair",
        "table",
        "credenza",
        "storage",
        "lounge-chair",
        "utility",
    ];
    for cat in cat_order {
        let Some(cat_obj) = furn_root.get(cat).and_then(|v| v.as_object()) else {
            continue;
        };
        for (product_slug, product_val) in cat_obj {
            let Some(val) = product_val.get("$value").and_then(|v| v.as_object()) else {
                continue;
            };
            let gs = |k: &str| {
                val.get(k)
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string()
            };
            let gd = |obj: &serde_json::Map<String, Value>, k: &str| -> u32 {
                obj.get(k).and_then(|v| v.as_f64()).unwrap_or(0.0) as u32
            };
            let dims = val.get("dimensions_mm").and_then(|v| v.as_object());
            let clr = val.get("clearance_mm").and_then(|v| v.as_object());
            let w_mm = dims.map(|d| gd(d, "w")).unwrap_or(0);
            let d_mm = dims.map(|d| gd(d, "d")).unwrap_or(0);
            let h_min_mm = dims.map(|d| gd(d, "h_min")).unwrap_or(0);
            let clearance_front = clr.map(|c| gd(c, "front")).unwrap_or(0);
            let clearance_side = clr.map(|c| gd(c, "sides")).unwrap_or(0);
            let model = gs("model");
            items.push(FurnitureItem {
                slug: format!("{}-{}", cat, product_slug),
                sidebar_label: furn_sidebar_label(product_slug, &model),
                manufacturer: gs("manufacturer"),
                product_line: gs("product_line"),
                model: model.clone(),
                sku: val
                    .get("sku")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                sku_note: val
                    .get("sku_note")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                w_mm,
                d_mm,
                h_min_mm,
                clearance_front,
                clearance_side,
                weight_kg: val.get("weight_kg").and_then(|v| v.as_f64()),
                ifc_class: gs("ifc_class"),
                url: val
                    .get("url")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
                description: product_val
                    .get("$description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
            });
        }
    }
    items
}

async fn furniture_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let items = extract_furniture(&state.tokens);

    let furn_lib = state.library_dir.join("blocks").join("furniture");

    let sidebar_btns: String = items
        .iter()
        .map(|it| {
            format!(
                "<button class=\"furn-btn\" data-slug=\"{}\" type=\"button\">{}</button>",
                esc(&it.slug),
                esc(&it.sidebar_label)
            )
        })
        .collect();

    let docs_panels: String = items
        .iter()
        .enumerate()
        .map(|(i, it)| {
            let vis = if i == 0 { "" } else { " hidden" };
            let sku_row = match (&it.sku, &it.sku_note) {
                (Some(s), _) => format!("<tr><th>SKU</th><td>{}</td></tr>", esc(s)),
                (None, Some(n)) => format!("<tr><th>SKU</th><td><em>{}</em></td></tr>", esc(n)),
                _ => String::new(),
            };
            let wt_row = match it.weight_kg {
                Some(w) => format!("<tr><th>Weight</th><td>{:.1} kg</td></tr>", w),
                None => "<tr><th>Weight</th><td><em>Not published</em></td></tr>".to_string(),
            };
            let mfr_row = match &it.url {
                Some(u) => format!(
                    "<tr><th>Manufacturer</th><td><a href=\"{}\" target=\"_blank\" rel=\"noopener\">{}</a></td></tr>",
                    esc(u),
                    esc(&it.manufacturer)
                ),
                None => String::new(),
            };
            let has_dwg_local = furn_lib.join(format!("{}.dwg", it.slug)).exists();
            let has_rfa_local = furn_lib.join(format!("{}.rfa", it.slug)).exists();
            // DWG and RFA: local file takes priority; fall back to manufacturer page.
            let dwg_btn = if has_dwg_local {
                format!("<a class=\"furn-dl-btn\" href=\"/furniture/download/{}.dwg\">DWG</a>", esc(&it.slug))
            } else if let Some(u) = &it.url {
                format!(
                    "<a class=\"furn-dl-btn furn-dl-ext\" href=\"{}\" target=\"_blank\" \
                     rel=\"noopener\" title=\"Download DWG from manufacturer site\">DWG ↗</a>",
                    esc(u)
                )
            } else {
                "<span class=\"furn-dl-unavail\" title=\"Not yet available\">DWG</span>".to_string()
            };
            let rfa_btn = if has_rfa_local {
                format!("<a class=\"furn-dl-btn\" href=\"/furniture/download/{}.rfa\">RFA</a>", esc(&it.slug))
            } else if let Some(u) = &it.url {
                format!(
                    "<a class=\"furn-dl-btn furn-dl-ext\" href=\"{}\" target=\"_blank\" \
                     rel=\"noopener\" title=\"Download RFA from manufacturer site\">RFA ↗</a>",
                    esc(u)
                )
            } else {
                "<span class=\"furn-dl-unavail\" title=\"Not yet available\">RFA</span>".to_string()
            };
            format!(
                "<div class=\"furn-docs-panel{}\" data-slug=\"{}\">\
                 <h2 class=\"furn-docs-title\">{}</h2>\
                 <p class=\"furn-docs-mfr\">{} &mdash; {}</p>\
                 <table class=\"furn-spec-table\"><tbody>\
                 <tr><th>Width</th><td>{} mm</td></tr>\
                 <tr><th>Depth</th><td>{} mm</td></tr>\
                 <tr><th>Height</th><td>{} mm</td></tr>\
                 <tr><th>Front clearance</th><td>{} mm</td></tr>\
                 {}{}\
                 <tr><th>IFC class</th><td><code>{}</code></td></tr>\
                 {}\
                 </tbody></table>\
                 <div class=\"furn-dl-strip\">\
                 {dwg}{rfa}\
                 <a class=\"furn-dl-btn furn-dl-ifc\" href=\"/furniture/download/{ifc_slug}.ifc\">IFC</a>\
                 </div>\
                 <p class=\"furn-docs-desc\">{desc}</p>\
                 </div>",
                vis,
                esc(&it.slug),
                esc(&it.sidebar_label),
                esc(&it.manufacturer),
                esc(&it.product_line),
                it.w_mm,
                it.d_mm,
                it.h_min_mm,
                it.clearance_front,
                sku_row,
                wt_row,
                esc(&it.ifc_class),
                mfr_row,
                dwg = dwg_btn,
                rfa = rfa_btn,
                ifc_slug = esc(&it.slug),
                desc = esc(&it.description),
            )
        })
        .collect();

    let content = format!(
        r#"{crumbs}
<header class="bim-page-header">
  <h1>Private Office Furniture</h1>
  <p class="bim-page-header__sub">8 BIM Objects &mdash; Steelcase FFE program &mdash; Key Plans PO-1 / PO-2 / PO-3</p>
</header>
<div class="furn-bundle-bar">
  <span>Download all blocks as a ZIP archive</span>
  <a class="furn-bundle-btn" href="/furniture/download/bundle.zip">&#x2b07; Download All (ZIP)</a>
</div>
<div class="furn-layout">
  <div class="furn-col-sidebar">{sidebar}</div>
  <div class="furn-col-docs">{docs}</div>
</div>
{css}{js}"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
        sidebar = sidebar_btns,
        docs = docs_panels,
        css = FURN_CSS,
        js = FURN_JS,
    );

    Html(page_shell(
        "Private Office Furniture",
        "/furniture",
        &content,
        &state,
    ))
}

async fn furniture_download_handler(
    Path(filename): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Response {
    let valid_ext = filename.ends_with(".ifc")
        || filename.ends_with(".dwg")
        || filename.ends_with(".rfa")
        || filename.ends_with(".dxf");
    let valid = valid_ext
        && filename.len() <= 128
        && !filename.contains('/')
        && !filename.contains("..")
        && filename
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if !valid {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("invalid filename"))
            .unwrap();
    }
    let path = state
        .library_dir
        .join("blocks")
        .join("furniture")
        .join(&filename);
    let content_type = if filename.ends_with(".dwg") {
        "application/acad"
    } else if filename.ends_with(".rfa") {
        "application/octet-stream"
    } else if filename.ends_with(".dxf") {
        "application/dxf"
    } else {
        "application/x-step"
    };
    match fs::read(&path) {
        Ok(bytes) => {
            let disp = format!("attachment; filename=\"{}\"", filename);
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", content_type)
                .header("content-disposition", disp)
                .body(Body::from(bytes))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("file not yet available — contact administrator"))
            .unwrap(),
    }
}

async fn furniture_bundle_handler(State(state): State<Arc<AppState>>) -> Response {
    let furn_dir = state.library_dir.join("blocks").join("furniture");
    let mut buf = Cursor::new(Vec::new());
    {
        let mut zip = zip::ZipWriter::new(&mut buf);
        let opts = zip::write::FileOptions::<()>::default()
            .compression_method(zip::CompressionMethod::Deflated);
        let entries = fs::read_dir(&furn_dir)
            .into_iter()
            .flatten()
            .flatten()
            .filter(|e| {
                let p = e.path();
                matches!(
                    p.extension().and_then(|s| s.to_str()),
                    Some("ifc") | Some("dwg") | Some("rfa") | Some("dxf")
                )
            });
        let mut count = 0u32;
        for entry in entries {
            let Ok(bytes) = fs::read(entry.path()) else {
                continue;
            };
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if zip.start_file(name.as_ref(), opts).is_ok() {
                use std::io::Write;
                let _ = zip.write_all(&bytes);
                count += 1;
            }
        }
        if count == 0 {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("no files available"))
                .unwrap();
        }
        let _ = zip.finish();
    }
    Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/zip")
        .header(
            "content-disposition",
            "attachment; filename=\"woodfine-private-office-furniture.zip\"",
        )
        .body(Body::from(buf.into_inner()))
        .unwrap()
}

// ─── Key Plans handlers ───────────────────────────────────────────────────────

async fn key_plans_handler(State(state): State<Arc<AppState>>) -> Html<String> {
    let kp_dir = state.library_dir.join("key-plans");

    // Collect which IFC files are present
    let available: std::collections::HashSet<String> = fs::read_dir(&kp_dir)
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            if p.extension().and_then(|s| s.to_str()) == Some("ifc") {
                p.file_name().map(|n| n.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();

    // Read PO Key Plans from woodfine-bim-library token file directly
    let kp_token_path = state
        .library_dir
        .join("tokens")
        .join("bim")
        .join("key-plans.dtcg.json");
    let kp_token: Option<serde_json::Value> = fs::read_to_string(&kp_token_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok());

    let kp_cards: String = if let Some(po) = kp_token
        .as_ref()
        .and_then(|v| v.get("bim"))
        .and_then(|v| v.get("key-plan"))
        .and_then(|v| v.get("private-office"))
    {
        ["small", "medium", "large"]
            .iter()
            .filter_map(|size| {
                let v = po.get(size)?.get("$value")?;
                let code = v.get("internal_code")?.as_str()?;
                let name = v.get("display_name")?.as_str()?;
                let area_m2 = v.get("area_m2")?.as_f64()?;
                let area_sf = v.get("area_sf")?.as_f64()?;
                let z1 = v.get("zone1_depth_m")?.as_f64()?;
                let z2 = v.get("zone2_depth_m")?.as_f64()?;
                let z3 = v.get("zone3_depth_m")?.as_f64()?;
                let fname = format!("private-office-{}.ifc", &code[code.len() - 1..]);
                let ifc_btn = if available.contains(&fname) {
                    format!(
                        "<a class=\"kp-dl-btn\" href=\"/key-plans/download/{}\">&#x2b07; IFC</a>",
                        esc(&fname)
                    )
                } else {
                    "<span class=\"kp-dl-unavail\">IFC pending</span>".to_string()
                };
                Some(format!(
                    "<div class=\"kp-card\">\
                     <div class=\"kp-card__head\">\
                       <span class=\"kp-code\">{code}</span>\
                       <span class=\"kp-name\">{name}</span>\
                     </div>\
                     <table class=\"kp-spec\"><tbody>\
                       <tr><th>Area</th><td>{area_m2:.1} m² / {area_sf:.0} SF</td></tr>\
                       <tr><th>Zone 1 (Habitat)</th><td>{z1:.1} m</td></tr>\
                       <tr><th>Zone 2 (Magazine)</th><td>{z2:.1} m</td></tr>\
                       <tr><th>Zone 3 (Corridor)</th><td>{z3:.1} m</td></tr>\
                     </tbody></table>\
                     <div class=\"kp-dl-strip\">{ifc_btn}</div>\
                     </div>",
                    code = esc(code),
                    name = esc(name),
                    area_m2 = area_m2,
                    area_sf = area_sf,
                    z1 = z1,
                    z2 = z2,
                    z3 = z3,
                    ifc_btn = ifc_btn,
                ))
            })
            .collect()
    } else {
        "<p>Key Plan token data not loaded.</p>".to_string()
    };

    let content = format!(
        r#"{crumbs}
<header class="bim-page-header">
  <h1>Key Plans</h1>
  <p class="bim-page-header__sub">Private Office series &mdash; IFC4 spatial compositions &mdash; ready for assembly in Bonsai / FreeCAD BIM / IfcOpenShell</p>
</header>
<p class="kp-intro">Each Key Plan is a complete IFC4 file containing an IfcSpace (room boundary) and
IfcFurniture instances placed at 2D coordinates. Download and open in any IFC-compatible
BIM tool. Assemblies are built from the Private Office furniture blocks on the
<a href="/furniture">Furniture</a> page.</p>
<div class="kp-grid">{cards}</div>
{css}"#,
        crumbs = breadcrumbs(&[("/", "Home")]),
        cards = kp_cards,
        css = KP_CSS,
    );

    Html(page_shell("Key Plans", "/key-plans", &content, &state))
}

async fn key_plans_download_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(filename): axum::extract::Path<String>,
) -> Response {
    let valid = filename.len() <= 64
        && filename.ends_with(".ifc")
        && filename
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.');
    if !valid {
        return Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("invalid filename"))
            .unwrap();
    }
    let path = state.library_dir.join("key-plans").join(&filename);
    match fs::read(&path) {
        Ok(bytes) => {
            let disp = format!("attachment; filename=\"{}\"", filename);
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/x-step")
                .header("content-disposition", disp)
                .body(Body::from(bytes))
                .unwrap()
        }
        Err(_) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("file not yet generated — run the pipeline"))
            .unwrap(),
    }
}

const KP_CSS: &str = r#"<style>
.kp-intro{font-size:13px;color:var(--bim-text-muted,#6b7280);margin:.5rem 0 1.25rem;line-height:1.6;max-width:680px}
.kp-grid{display:grid;grid-template-columns:repeat(auto-fill,minmax(260px,1fr));gap:16px;margin-top:.5rem}
.kp-card{border:1px solid var(--bim-border,#dde1e7);border-radius:6px;padding:16px;background:var(--bim-surface-2,#f5f7fa)}
.kp-card__head{display:flex;align-items:baseline;gap:10px;margin-bottom:12px}
.kp-code{font-size:18px;font-weight:700;color:var(--bim-accent,#1a3a5c)}
.kp-name{font-size:13px;color:var(--bim-text-muted,#6b7280)}
.kp-spec{width:100%;border-collapse:collapse;font-size:12px;margin-bottom:14px}
.kp-spec th{text-align:left;font-weight:500;color:var(--bim-text-muted,#6b7280);padding:3px 8px 3px 0;white-space:nowrap;width:50%}
.kp-spec td{color:var(--bim-text,#1a1a2e);padding:3px 0}
.kp-dl-strip{display:flex;gap:8px}
.kp-dl-btn{display:inline-block;padding:6px 16px;background:var(--bim-accent,#1a3a5c);color:#fff;border-radius:4px;font-size:12px;font-weight:600;text-decoration:none}
.kp-dl-btn:hover{opacity:.85}
.kp-dl-unavail{display:inline-block;padding:6px 16px;border:1px solid #ccc;color:#bbb;border-radius:4px;font-size:12px;font-weight:600}
</style>"#;

const FURN_CSS: &str = r#"<style>
.furn-bundle-bar{display:flex;align-items:center;justify-content:space-between;padding:10px 16px;margin-top:1.25rem;background:var(--bim-surface-2,#f5f7fa);border:1px solid var(--bim-border,#dde1e7);border-radius:6px;font-size:12px;color:var(--bim-text-muted,#6b7280)}
.furn-bundle-btn{display:inline-block;padding:6px 16px;background:var(--bim-accent,#1a3a5c);color:#fff;border-radius:4px;font-size:12px;font-weight:600;text-decoration:none;letter-spacing:.03em;white-space:nowrap}
.furn-bundle-btn:hover{opacity:.85}
.furn-layout{display:grid;grid-template-columns:180px 1fr;min-height:540px;border:1px solid var(--bim-border,#dde1e7);border-radius:6px;overflow:hidden;margin-top:.75rem}
.furn-col-sidebar{border-right:1px solid var(--bim-border,#dde1e7);background:var(--bim-surface-2,#f5f7fa);padding:12px 0}
.furn-btn{display:block;width:100%;text-align:left;background:none;border:none;padding:8px 14px;font-size:13px;color:var(--bim-text,#1a1a2e);cursor:pointer;border-left:3px solid transparent;line-height:1.35}
.furn-btn:hover{background:var(--bim-surface-3,#eaecf0)}
.furn-btn.active{border-left-color:var(--bim-accent,#1a3a5c);background:var(--bim-surface-3,#eaecf0);font-weight:600}
.furn-col-viewer{display:flex;align-items:center;justify-content:center;padding:20px;background:#f9f9f7}
.furn-svg-panel{width:100%;max-width:480px;background:#fff;border:1px solid #e0e0e0;border-radius:4px;padding:8px}
.furn-svg-panel svg{width:100%;height:auto;display:block;max-height:480px}
.furn-cad-pending{display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:280px;color:#999;font-size:12px;text-align:center;gap:8px;border:1px dashed #ccc;border-radius:4px;background:#fafafa;padding:24px}
.furn-cad-pending code{font-size:10px;background:#f0f0f0;padding:2px 8px;border-radius:3px;color:#555;font-family:monospace}
.furn-col-docs{border-left:1px solid var(--bim-border,#dde1e7);background:var(--bim-surface-2,#f5f7fa);padding:18px 16px;overflow-y:auto;max-height:620px}
.furn-docs-title{font-size:14px;font-weight:700;margin:0 0 4px;line-height:1.3}
.furn-docs-mfr{font-size:11px;color:var(--bim-text-muted,#6b7280);margin:0 0 14px}
.furn-spec-table{width:100%;border-collapse:collapse;font-size:12px;margin-bottom:14px}
.furn-spec-table th{text-align:left;font-weight:500;color:var(--bim-text-muted,#6b7280);padding:3px 8px 3px 0;white-space:nowrap;width:42%}
.furn-spec-table td{color:var(--bim-text,#1a1a2e);padding:3px 0}
.furn-spec-table code{font-size:10px;background:var(--bim-surface-3,#eaecf0);padding:1px 4px;border-radius:3px}
.furn-dl-strip{display:flex;gap:8px;margin-bottom:14px;flex-wrap:wrap}
.furn-dl-btn{display:inline-block;padding:5px 13px;border:1px solid var(--bim-accent,#1a3a5c);color:var(--bim-accent,#1a3a5c);border-radius:4px;font-size:12px;font-weight:600;text-decoration:none;letter-spacing:.03em}
.furn-dl-btn:hover{background:var(--bim-accent,#1a3a5c);color:#fff}
.furn-dl-ifc{background:var(--bim-accent,#1a3a5c);color:#fff}
.furn-dl-ifc:hover{opacity:.85}
.furn-dl-ext{border-style:dashed;opacity:.85}
.furn-dl-ext:hover{background:var(--bim-accent,#1a3a5c);color:#fff;opacity:1}
.furn-dl-unavail{display:inline-block;padding:5px 13px;border:1px solid #ccc;color:#bbb;border-radius:4px;font-size:12px;font-weight:600;letter-spacing:.03em;cursor:not-allowed;user-select:none}
.furn-docs-desc{font-size:11px;color:var(--bim-text-muted,#6b7280);line-height:1.5;margin:0;border-top:1px solid var(--bim-border,#dde1e7);padding-top:12px}
@media(max-width:900px){.furn-bundle-bar{flex-direction:column;gap:8px;text-align:center}.furn-layout{grid-template-columns:1fr}.furn-col-sidebar{border-right:none;border-bottom:1px solid var(--bim-border,#dde1e7);display:flex;flex-wrap:wrap;gap:4px;padding:10px}.furn-btn{width:auto;border-left:none;border:1px solid var(--bim-border,#dde1e7);padding:5px 10px;font-size:12px;border-radius:4px}.furn-btn.active{border-color:var(--bim-accent,#1a3a5c);background:var(--bim-accent,#1a3a5c);color:#fff}.furn-col-docs{border-left:none;border-top:1px solid var(--bim-border,#dde1e7);max-height:none}}
</style>"#;

const FURN_JS: &str = r#"<script>
(function(){
var btns=document.querySelectorAll('.furn-btn');
var svgs=document.querySelectorAll('.furn-svg-panel');
var docs=document.querySelectorAll('.furn-docs-panel');
function show(slug){
  for(var i=0;i<btns.length;i++) btns[i].classList.toggle('active',btns[i].getAttribute('data-slug')===slug);
  for(var i=0;i<svgs.length;i++) svgs[i].hidden=svgs[i].getAttribute('data-slug')!==slug;
  for(var i=0;i<docs.length;i++) docs[i].hidden=docs[i].getAttribute('data-slug')!==slug;
}
for(var i=0;i<btns.length;i++)(function(b){b.addEventListener('click',function(){show(b.getAttribute('data-slug'));})})(btns[i]);
if(btns.length>0) show(btns[0].getAttribute('data-slug'));
})();
</script>"#;

// ---------------------------------------------------------------------------
// Inline JavaScript (identical behaviour to v0.0.1)
// ---------------------------------------------------------------------------

const INLINE_JS: &str = r#"<script>
(function () {
  'use strict';
  const store = {
    state: { selectedGuid: null, elementData: {} },
    listeners: [],
    subscribe(fn) { this.listeners.push(fn); fn(this.state); },
    notify() { this.listeners.forEach(fn => fn(this.state)); },
    set(patch) { this.state = Object.assign({}, this.state, patch); this.notify(); }
  };
  function bindUI() {
    const brand = document.querySelector('.bim-topbar__brand');
    const sidebar = document.querySelector('.bim-sidebar');
    if (brand && sidebar) {
      brand.addEventListener('click', (e) => {
        if (window.innerWidth <= 960) { e.preventDefault(); sidebar.classList.toggle('bim-sidebar--open'); }
      });
    }
    store.subscribe((state) => {
      const sheet = document.querySelector('.bim-properties-sheet');
      if (!sheet) return;
      if (state.selectedGuid) {
        sheet.classList.add('bim-properties-sheet--peek');
        const guidEl = document.getElementById('sheet-guid');
        if (guidEl) guidEl.textContent = state.selectedGuid;
      } else {
        sheet.classList.remove('bim-properties-sheet--peek', 'bim-properties-sheet--expanded');
      }
    });
  }
  function bindAgnosticComponents() {
    store.subscribe((state) => {
      const guids = document.querySelectorAll('.bim-guid');
      for (let i = 0; i < guids.length; i++) {
        const el = guids[i];
        if (el.id !== 'sheet-guid') el.textContent = state.selectedGuid || 'no-selection';
      }
    });
    const targets = document.querySelectorAll('[data-bim-guid], .bim-hero__svg-wrap');
    for (let i = 0; i < targets.length; i++) {
      const el = targets[i];
      el.addEventListener('click', () => {
        const guid = el.getAttribute('data-bim-guid') || 'IfcSpace_Office_001';
        store.set({ selectedGuid: guid });
      });
    }
  }
  function bindCopy() {
    const btns = document.querySelectorAll('[data-bim-copy]');
    for (let i = 0; i < btns.length; i++) {
      const btn = btns[i];
      btn.addEventListener('click', function () {
        const target = btn.closest('.bim-code-block');
        const code = target.querySelector('pre');
        navigator.clipboard.writeText(code.innerText).then(() => {
          const orig = btn.textContent;
          btn.textContent = 'copied';
          btn.classList.add('bim-code-block__copy--copied');
          setTimeout(() => { btn.textContent = orig; btn.classList.remove('bim-code-block__copy--copied'); }, 1400);
        });
      });
    }
  }
  function bindTabBar() {
    const bars = document.querySelectorAll('.bim-tab-bar');
    for (let i = 0; i < bars.length; i++) {
      const bar = bars[i];
      const tabs = bar.querySelectorAll('details.bim-tab');
      for (let j = 0; j < tabs.length; j++) {
        const tab = tabs[j];
        tab.addEventListener('toggle', function () {
          if (!tab.open) return;
          for (let k = 0; k < tabs.length; k++) { if (tabs[k] !== tab) tabs[k].open = false; }
        });
      }
      if (tabs.length > 0 && !bar.querySelector('details[open]')) tabs[0].open = true;
    }
  }
  function bindFilter() {
    const input = document.getElementById('token-filter');
    const grid = document.getElementById('token-grid');
    if (!input || !grid) return;
    input.addEventListener('input', function () {
      const q = input.value.toLowerCase();
      const cards = grid.querySelectorAll('.bim-token-card');
      for (let i = 0; i < cards.length; i++) {
        const text = cards[i].textContent.toLowerCase();
        cards[i].style.display = q === '' || text.includes(q) ? '' : 'none';
      }
    });
  }
  const init = () => { bindCopy(); bindTabBar(); bindUI(); bindAgnosticComponents(); bindFilter(); };
  if (document.readyState === 'loading') document.addEventListener('DOMContentLoaded', init);
  else init();
  window.BIM_STORE = store;
})();
</script>"#;

// ---------------------------------------------------------------------------
// Startup — load state
// ---------------------------------------------------------------------------

fn load_state() -> AppState {
    let design_system_dir =
        PathBuf::from(env::var("BIM_DESIGN_SYSTEM_DIR").unwrap_or_else(|_| ".".to_string()));
    let vault_dir = PathBuf::from(env::var("BIM_VAULT_DIR").unwrap_or_else(|_| ".".to_string()));
    let library_dir =
        PathBuf::from(env::var("BIM_LIBRARY_DIR").unwrap_or_else(|_| {
            "/srv/foundry/clones/project-bim/woodfine-bim-library".to_string()
        }));
    let static_dir =
        PathBuf::from(env::var("BIM_STATIC_DIR").unwrap_or_else(|_| "./static".to_string()));
    let tenant = env::var("BIM_TENANT").unwrap_or_else(|_| "woodfine".to_string());
    let public_url =
        env::var("BIM_PUBLIC_URL").unwrap_or_else(|_| "https://bim.woodfinegroup.com".to_string());

    // Load DTCG token files
    let tokens_dir = design_system_dir.join("tokens").join("bim");
    let mut tokens: HashMap<String, Value> = HashMap::new();
    if let Ok(entries) = fs::read_dir(&tokens_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                let stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .trim_end_matches(".dtcg")
                    .to_string();
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(v) = serde_json::from_str::<Value>(&content) {
                        tokens.insert(stem, v);
                    }
                }
            }
        }
    }
    let token_count = tokens.len();

    // Count research files
    let research_count = fs::read_dir(vault_dir.join("research"))
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().extension().map(|x| x == "md").unwrap_or(false))
                .count()
        })
        .unwrap_or(0);

    // Count component files
    let components_count = fs::read_dir(design_system_dir.join("components"))
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    e.path()
                        .extension()
                        .map(|x| x == "md" || x == "html")
                        .unwrap_or(false)
                })
                .count()
        })
        .unwrap_or(0);

    AppState {
        vault_dir,
        library_dir,
        design_system_dir,
        static_dir,
        tenant,
        public_url,
        tokens: Arc::new(tokens),
        token_count,
        components_count,
        research_count,
    }
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() {
    let bind = env::var("BIM_BIND").unwrap_or_else(|_| "127.0.0.1:9096".to_string());
    let state = Arc::new(load_state());

    eprintln!(
        "app-orchestration-bim v0.0.2 — {} token categories, {} research, {} components",
        state.token_count, state.research_count, state.components_count
    );
    eprintln!("Listening on {}", bind);

    let static_dir = state.static_dir.clone();
    let app = Router::new()
        .route("/", get(home_handler))
        .route("/tokens", get(tokens_index_handler))
        .route("/tokens/:name", get(token_category_handler))
        .route("/tokens.json", get(tokens_json_handler))
        .route("/about", get(about_handler))
        .route("/research", get(research_index_handler))
        .route("/research/:slug", get(research_item_handler))
        .route("/components", get(components_index_handler))
        .route("/components/:slug", get(components_item_handler))
        .route("/healthz", get(healthz_handler))
        .route("/readyz", get(readyz_handler))
        .route("/furniture", get(furniture_handler))
        .route(
            "/furniture/download/bundle.zip",
            get(furniture_bundle_handler),
        )
        .route(
            "/furniture/download/:filename",
            get(furniture_download_handler),
        )
        .route("/key-plans", get(key_plans_handler))
        .route(
            "/key-plans/download/:filename",
            get(key_plans_download_handler),
        )
        .nest_service("/static", ServeDir::new(static_dir))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind)
        .await
        .expect("failed to bind");
    axum::serve(listener, app).await.expect("server error");
}
