// Building Portfolio V2 — Four-DHS extrapolation + Development Sites layer.
//
// V1 (`d1_dev_classes_v2.rs`) models ONE portfolio (the Canada / PCLP 1 portfolio).
// V2 extrapolates that geometric model across all four Direct-Hold Solutions
// (Canada, US, Spain, Mexico), introduces a "Development Sites" abstraction worked
// BACKWARDS from the fixed building prototypes, and shows the build-out growing
// horizontally Y1→Y10. See the approved plan:
//   .claude/plans/compliance-mcorp-2026-06-04-proforma-bui-delightful-pelican.md
//
// Model (locked with operator, this session):
//   • Per-1×-DHS leasable area pinned to the WCP/PCLP1 ramp: 3,906,855 sf.
//     US = 2× (size_factor); Canada/Spain/Mexico = 1×. Launch lags CA Y1 / US Y2 /
//     ES Y2 / MX Y3 from the WCP revenue generator.
//   • Legal building geometry (HARD bounds): Professional Centres 3–5 floors
//     (21,000 sf/floor); Suburban Office 6–9 floors (19,000 sf/floor); Retail Select
//     4,500/6,700/7,700 sf; Tech Industrial 7,200/8,400 sf (always paired). A
//     mix-weight profile per class is the geometric-distribution lever.
//   • Backwards site model: buildings are primary; sites are sized to hold them.
//       Woodfine Campus  (≤30% — business-plan minority): 1 PC + 1 SO + 1 RS pair
//                         (2 RS) + 2 TI pairs (4 TI) = 8 structures.
//       Anchor + pair    (~75% of remainder): 1 PC or 1 SO + 1 pair of RS or TI = 3.
//       Standalone       (remainder): 1 PC or SO = 1 structure.
//     sites = pinned_GLA / avg_site_GLA → derive class counts → financials.
//   • Per-building math is the V1 calibration: rent = cost × 10.5% dev yield;
//     NOI = rent × 57%; asset value = NOI ÷ 6.25%; depreciation = cost ÷ 40 yr.
//   • Time: program Y1→Y10 absolute (matches WCP). Physical delivery paced over each
//     DHS's own ~10-year build life from launch (Option B); late DHSs truncate at Y10,
//     leaving a "Remaining Development Sites" tail. AUM (CAD) = gross asset value of the
//     delivered portfolio, US weighted 2×.

use serde::Serialize;

// ── Calibration constants (shared with V1 d1_dev_classes_v2) ────────────────
const TARGET_DEV_YIELD: f64 = 0.105;
const TARGET_CAP_RATE: f64 = 0.0625;
const OPEX_RATE: f64 = 0.43;
const DEPRECIATION_YRS: f64 = 40.0;
const COST_OFFICE: f64 = 310.0; // PC + SO ($/sf GLA)
const COST_FLEX: f64 = 260.0; // RS + TI ($/sf GLA)

/// Per-1×-DHS leasable area, pinned to the WCP / PCLP1 revenue-generator ramp.
pub const PORTFOLIO_SQFT_1X: f64 = 3_906_855.0;
const PROGRAM_YEARS: u32 = 10; // Y1..Y10 absolute window (matches WCP tables)
const BUILD_LIFE_YEARS: f64 = 10.0; // Option B: each DHS builds over ~10 yrs from launch

// ── Legal building geometry + default "Top-heavy, mixed" weights ────────────
// Floor plates: Professional Centres 21,000 sf/floor; Suburban Office 19,000 sf/floor.

#[derive(Clone, Copy, Serialize)]
pub struct Variant {
    pub label: &'static str,
    /// Floor count (PC 3–5, SO 6–9); 1 for single-storey flex (RS/TI).
    pub floors: u32,
    pub gla: f64,
    /// Share of this variant within its class (weights sum to 1.0).
    pub weight: f64,
}

const PC_VARIANTS: [Variant; 3] = [
    Variant {
        label: "3-floor",
        floors: 3,
        gla: 63_000.0,
        weight: 0.05,
    },
    Variant {
        label: "4-floor",
        floors: 4,
        gla: 84_000.0,
        weight: 0.25,
    },
    Variant {
        label: "5-floor (reference)",
        floors: 5,
        gla: 105_000.0,
        weight: 0.70,
    },
];
const SO_VARIANTS: [Variant; 4] = [
    Variant {
        label: "6-floor",
        floors: 6,
        gla: 114_000.0,
        weight: 0.10,
    },
    Variant {
        label: "7-floor",
        floors: 7,
        gla: 133_000.0,
        weight: 0.20,
    },
    Variant {
        label: "8-floor (reference)",
        floors: 8,
        gla: 152_000.0,
        weight: 0.35,
    },
    Variant {
        label: "9-floor",
        floors: 9,
        gla: 171_000.0,
        weight: 0.35,
    },
];
const RS_VARIANTS: [Variant; 3] = [
    Variant {
        label: "4,500 sf",
        floors: 1,
        gla: 4_500.0,
        weight: 0.15,
    },
    Variant {
        label: "6,700 sf (reference)",
        floors: 1,
        gla: 6_700.0,
        weight: 0.45,
    },
    Variant {
        label: "7,700 sf",
        floors: 1,
        gla: 7_700.0,
        weight: 0.40,
    },
];
const TI_VARIANTS: [Variant; 2] = [
    Variant {
        label: "7,200 sf pair-half",
        floors: 1,
        gla: 7_200.0,
        weight: 0.40,
    },
    Variant {
        label: "8,400 sf pair-half (reference)",
        floors: 1,
        gla: 8_400.0,
        weight: 0.60,
    },
];

#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Class {
    ProfessionalCentres,
    SuburbanOffice,
    RetailSelect,
    TechIndustrial,
}

impl Class {
    fn label(self) -> &'static str {
        match self {
            Class::ProfessionalCentres => "Professional Centres",
            Class::SuburbanOffice => "Suburban Office",
            Class::RetailSelect => "Retail Select",
            Class::TechIndustrial => "Tech Industrial",
        }
    }
    fn variants(self) -> &'static [Variant] {
        match self {
            Class::ProfessionalCentres => &PC_VARIANTS,
            Class::SuburbanOffice => &SO_VARIANTS,
            Class::RetailSelect => &RS_VARIANTS,
            Class::TechIndustrial => &TI_VARIANTS,
        }
    }
    fn cost_per_sf(self) -> f64 {
        match self {
            Class::ProfessionalCentres | Class::SuburbanOffice => COST_OFFICE,
            Class::RetailSelect | Class::TechIndustrial => COST_FLEX,
        }
    }
    /// Weighted-average GLA per building for this class under its mix profile.
    fn avg_gla(self) -> f64 {
        self.variants().iter().map(|v| v.gla * v.weight).sum()
    }
}

const ALL_CLASSES: [Class; 4] = [
    Class::ProfessionalCentres,
    Class::SuburbanOffice,
    Class::RetailSelect,
    Class::TechIndustrial,
];

// ── Development-site dials ──────────────────────────────────────────────────

#[derive(Clone, Copy, Serialize)]
pub struct SiteParams {
    /// Share of ALL sites that are Woodfine Campuses. Business-plan ceiling 0.30.
    pub campus_share: f64,
    /// Share of the non-campus remainder that are anchor+pair sites (rest standalone).
    pub pair_share: f64,
    /// Share of non-campus anchors that are Suburban Office (rest Professional Centres).
    pub anchor_so_share: f64,
    /// Share of pair sites whose pair is Retail Select (rest Tech Industrial).
    pub pair_rs_share: f64,
}

impl Default for SiteParams {
    fn default() -> Self {
        // Recommended V2 operating point (≈7 sites/country/3-yr cycle).
        SiteParams {
            campus_share: 0.25,
            pair_share: 0.75,
            anchor_so_share: 0.60,
            pair_rs_share: 0.50,
        }
    }
}

/// Hard ceiling on the campus share — exceeding it would change the business plan.
pub const CAMPUS_SHARE_CEILING: f64 = 0.30;

// ── Direct-Hold Solutions (four building DHS; seeded from WCP lp_definitions) ─

#[derive(Clone, Copy, Serialize)]
pub struct Dhs {
    pub short: &'static str,
    pub name: &'static str,
    pub currency: &'static str,
    pub size_factor: f64,
    pub launch_year: u32,
}

const DHS_LIST: [Dhs; 4] = [
    Dhs {
        short: "Canada",
        name: "Professional Centres Canada LP",
        currency: "CAD",
        size_factor: 1.0,
        launch_year: 1,
    },
    Dhs {
        short: "United States",
        name: "Professional Centres United States LP",
        currency: "USD",
        size_factor: 2.0,
        launch_year: 2,
    },
    Dhs {
        short: "Spain",
        name: "Professional Centres Spain SOCIMI",
        currency: "EUR",
        size_factor: 1.0,
        launch_year: 2,
    },
    Dhs {
        short: "Mexico",
        name: "Professional Centres Mexico FIBRA",
        currency: "MXN",
        size_factor: 1.0,
        launch_year: 3,
    },
];

// ── Rollups ─────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Default, Serialize)]
pub struct ClassRollup {
    pub count: f64,
    pub gla: f64,
    pub cost: f64,
    pub rent: f64,
    pub noi: f64,
    pub asset_value: f64,
    pub depreciation: f64,
}

impl ClassRollup {
    fn for_class(class: Class, count: f64) -> Self {
        let gla = count * class.avg_gla();
        let cost = gla * class.cost_per_sf();
        let rent = cost * TARGET_DEV_YIELD;
        let noi = rent * (1.0 - OPEX_RATE);
        ClassRollup {
            count,
            gla,
            cost,
            rent,
            noi,
            asset_value: noi / TARGET_CAP_RATE,
            depreciation: cost / DEPRECIATION_YRS,
        }
    }
}

/// One 1× DHS solved backwards from the prototypes under a `SiteParams` dial set.
#[derive(Clone, Serialize)]
pub struct Base1x {
    pub params: SiteParams,
    pub pc: ClassRollup,
    pub so: ClassRollup,
    pub rs: ClassRollup,
    pub ti: ClassRollup,
    pub sites_total: f64,
    pub sites_campus: f64,
    pub sites_pair: f64,
    pub sites_solo: f64,
}

impl Base1x {
    pub fn buildings(&self) -> f64 {
        self.pc.count + self.so.count + self.rs.count + self.ti.count
    }
    pub fn gla(&self) -> f64 {
        self.pc.gla + self.so.gla + self.rs.gla + self.ti.gla
    }
    pub fn cost(&self) -> f64 {
        self.pc.cost + self.so.cost + self.rs.cost + self.ti.cost
    }
    pub fn noi(&self) -> f64 {
        self.pc.noi + self.so.noi + self.rs.noi + self.ti.noi
    }
    pub fn asset_value(&self) -> f64 {
        self.pc.asset_value + self.so.asset_value + self.rs.asset_value + self.ti.asset_value
    }
    fn class(&self, c: Class) -> &ClassRollup {
        match c {
            Class::ProfessionalCentres => &self.pc,
            Class::SuburbanOffice => &self.so,
            Class::RetailSelect => &self.rs,
            Class::TechIndustrial => &self.ti,
        }
    }
}

/// Solve one 1× DHS backwards from the building prototypes.
pub fn compute_base(sp: SiteParams) -> Base1x {
    let pc_avg = Class::ProfessionalCentres.avg_gla();
    let so_avg = Class::SuburbanOffice.avg_gla();
    let rs_avg = Class::RetailSelect.avg_gla();
    let ti_avg = Class::TechIndustrial.avg_gla();

    let rs_pair = 2.0 * rs_avg;
    let ti_pair = 2.0 * ti_avg;

    // Per-site GLA by type.
    let campus_gla = pc_avg + so_avg + rs_pair + 2.0 * ti_pair; // 1 PC + 1 SO + 1 RS pair + 2 TI pairs
    let anchor_avg = sp.anchor_so_share * so_avg + (1.0 - sp.anchor_so_share) * pc_avg;
    let pair_avg = sp.pair_rs_share * rs_pair + (1.0 - sp.pair_rs_share) * ti_pair;
    let avg_site = sp.campus_share * campus_gla
        + (1.0 - sp.campus_share)
            * (sp.pair_share * (anchor_avg + pair_avg) + (1.0 - sp.pair_share) * anchor_avg);

    // Backwards solve: total leasable area pinned ⇒ number of sites.
    let n = PORTFOLIO_SQFT_1X / avg_site;
    let n_campus = sp.campus_share * n;
    let n_pair = (1.0 - sp.campus_share) * sp.pair_share * n;
    let n_solo = (1.0 - sp.campus_share) * (1.0 - sp.pair_share) * n;
    let non_campus = n_pair + n_solo;

    // Building counts by class.
    let pc_count = n_campus + non_campus * (1.0 - sp.anchor_so_share);
    let so_count = n_campus + non_campus * sp.anchor_so_share;
    let rs_count = n_campus * 2.0 + n_pair * sp.pair_rs_share * 2.0;
    let ti_count = n_campus * 4.0 + n_pair * (1.0 - sp.pair_rs_share) * 2.0;

    Base1x {
        params: sp,
        pc: ClassRollup::for_class(Class::ProfessionalCentres, pc_count),
        so: ClassRollup::for_class(Class::SuburbanOffice, so_count),
        rs: ClassRollup::for_class(Class::RetailSelect, rs_count),
        ti: ClassRollup::for_class(Class::TechIndustrial, ti_count),
        sites_total: n,
        sites_campus: n_campus,
        sites_pair: n_pair,
        sites_solo: n_solo,
    }
}

/// Build-life completion fraction for a DHS at absolute program year `y` (Option B).
fn build_fraction(launch: u32, y: u32) -> f64 {
    if y < launch {
        return 0.0;
    }
    (((y - launch) as f64 + 1.0) / BUILD_LIFE_YEARS).min(1.0)
}

// ── Formatting helpers ──────────────────────────────────────────────────────

fn comma_f(x: f64, dec: usize) -> String {
    let s = format!("{:.*}", dec, x);
    let (int_part, frac_part) = match s.split_once('.') {
        Some((a, b)) => (a.to_string(), Some(b.to_string())),
        None => (s, None),
    };
    let neg = int_part.starts_with('-');
    let digits = int_part.trim_start_matches('-');
    let bytes = digits.as_bytes();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            grouped.push(',');
        }
        grouped.push(*b as char);
    }
    let mut out = String::new();
    if neg {
        out.push('-');
    }
    out.push_str(&grouped);
    if let Some(f) = frac_part {
        out.push('.');
        out.push_str(&f);
    }
    out
}

fn fmt_int(v: f64) -> String {
    comma_f(v, 0)
}
fn fmt_sqft(v: f64) -> String {
    format!("{} sf", comma_f(v, 0))
}
fn fmt_money_m(v: f64) -> String {
    format!("${}M", comma_f(v / 1_000_000.0, 1))
}
fn fmt_money_m_yr(v: f64) -> String {
    format!("${}M/yr", comma_f(v / 1_000_000.0, 1))
}
fn fmt_b(v: f64) -> String {
    format!("${}B", comma_f(v / 1_000_000_000.0, 2))
}
fn fmt_pct(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}

// ── HTML head — financial-report-layout DESIGN token (verbatim) ─────────────

const HEAD: &str = r#"<!DOCTYPE html><html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Woodfine Direct-Hold Solutions — Building Portfolio Proforma V2</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1400px}
h1{font-size:1.25rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.25rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#555}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0;font-size:0.76rem}
th,td{border:1px solid #ccc;padding:3px 6px;text-align:right;white-space:nowrap}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.lbl,th.lbl{text-align:left;min-width:230px}
table.wide{width:100%;table-layout:fixed}
table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}
td.lnum,th.lnum{width:32px;min-width:32px;font-family:'Courier New',monospace;font-size:9px;color:#aaa;text-align:right!important;background:white!important;font-weight:normal!important;border-right:2px solid #d0d0d0;padding:2px 5px 2px 2px;white-space:nowrap}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
tr.section-banner td{background:#e3edf7;font-weight:700;font-size:0.74rem;text-transform:uppercase;letter-spacing:.3px;color:#1a2a44;text-align:left}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@page{size:letter landscape;margin:1.5cm 2cm 1.5cm 1.5cm}
@media print{
  body{margin:0;font-size:11px;max-width:none}
  table{break-inside:avoid;page-break-inside:avoid}
  h2,h3{break-after:avoid;page-break-after:avoid}
  td.lnum,th.lnum{-webkit-print-color-adjust:exact;print-color-adjust:exact;color:#bbb!important;border-right-color:#ccc!important}
  table.wide{table-layout:fixed;font-size:10px}
  table.wide td,table.wide th{padding:3px 6px}
  table.wide td.lbl,table.wide th.lbl{width:25%;white-space:normal;overflow-wrap:break-word}
}
</style></head>
"#;

const LNUM_SCRIPT: &str = r#"<script>
(function(){
  var n=1;
  document.querySelectorAll('table').forEach(function(tbl){
    tbl.querySelectorAll('tr').forEach(function(row){
      var allTh=Array.from(row.children).every(function(c){return c.tagName==='TH';});
      var cell=document.createElement(allTh?'th':'td');
      cell.className='lnum';
      cell.textContent=n++;
      row.insertBefore(cell,row.firstChild);
    });
  });
})();
</script>
"#;

const DISCLAIMER: &str = "<p class=\"footer\"><strong>Forward-Looking Information — Notice under applicable securities legislation including the British Columbia Securities Commission (BCSC) and NI 51-102.</strong> This document contains forward-looking information. All building counts, development-site counts, square footage, cost estimates, NOI figures, asset valuations, assets-under-management figures, and yield targets are management estimates based on planning assumptions as of the date of this document and are subject to material change. Development-site composition (Woodfine Campus / anchor-plus-pair / standalone shares) and the geometric distribution of floor counts are planning parameters, not commitments. Actual results may differ materially. This document is prepared for internal planning purposes and does not constitute an offering memorandum, financial advice, or an offer to sell or solicitation to buy any security. Readers should not place undue reliance on forward-looking information.</p>\n";

// ── Renderer ────────────────────────────────────────────────────────────────

/// Full V2 proforma HTML (8 sections, leading with the dev-yield proof).
pub fn render_proforma() -> String {
    let base = compute_base(SiteParams::default());
    let mut s = String::new();
    s.push_str(HEAD);
    s.push_str("<body>\n");
    s.push_str("<h1>Woodfine Direct-Hold Solutions — Building Portfolio Proforma V2</h1>\n");
    s.push_str("<p>Four-DHS extrapolation (Canada · United States · Spain · Mexico) with Development-Sites layer, worked backwards from the fixed Woodfine building prototypes. Supersedes <code>COMPLIANCE_MCorp_2026_06_04_Proforma_BuildingPortfolio_V1.html</code> (single Canada / PCLP 1 portfolio).<br>\n");
    s.push_str("DRAFT — 2026-06-13 — V2<br>\n");
    s.push_str("All amounts CAD — Development yield 10.5% · cap rate 6.25% · NOI 57% of rent — Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture.</p>\n");

    s.push_str(&render_class_financials());
    s.push_str(&render_geometry_distribution(&base));
    s.push_str(&render_site_composition(&base));
    s.push_str(&render_per_dhs(&base));
    s.push_str(&render_buildout(&base));
    s.push_str(&render_scale_reconciliation(&base));
    s.push_str(&render_sensitivity());
    s.push_str(&render_methodology(&base));

    s.push_str(DISCLAIMER);
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// Section 1 — Development Class Financial Statements (FOUNDATION; proves 10.5% yield).
fn render_class_financials() -> String {
    let mut s = String::new();
    s.push_str("<h2>1 · Development Class Financial Statements</h2>\n");
    s.push_str("<p class=\"note\">Foundation of the portfolio. Each of the four Development Classes is calibrated so its construction cost earns a 10.5% development yield; everything below scales from these per-building economics. Legal floor geometry: Professional Centres 3–5 floors (21,000 sf/floor); Suburban Office 6–9 floors (19,000 sf/floor); Retail Select and Tech Industrial single-storey. Tech Industrial is always built in pairs.</p>\n");
    for c in ALL_CLASSES {
        s.push_str(&render_one_class_statement(c));
    }
    s
}

fn render_one_class_statement(c: Class) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "<h3>{} — ${}/sf GLA</h3>\n",
        c.label(),
        comma_f(c.cost_per_sf(), 0)
    ));
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Variant</th><th>GLA</th><th>Construction cost</th><th>Rent (10.5%)</th><th>NOI (57%)</th><th>Asset value (6.25%)</th><th>Depreciation (40yr)</th><th>Dev yield</th></tr>\n");
    for v in c.variants() {
        let gla = v.gla;
        let cost = gla * c.cost_per_sf();
        let rent = cost * TARGET_DEV_YIELD;
        let noi = rent * (1.0 - OPEX_RATE);
        let av = noi / TARGET_CAP_RATE;
        let depr = cost / DEPRECIATION_YRS;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            v.label,
            fmt_sqft(gla),
            fmt_money_m(cost),
            fmt_money_m_yr(rent),
            fmt_money_m_yr(noi),
            fmt_money_m(av),
            fmt_money_m_yr(depr),
            fmt_pct(TARGET_DEV_YIELD),
        ));
    }
    s.push_str("</table>\n");
    s
}

// Section 2 — Building geometry distribution (the legal "good mix").
fn render_geometry_distribution(base: &Base1x) -> String {
    let mut s = String::new();
    s.push_str("<h2>2 · Building Geometry Distribution (per 1× DHS)</h2>\n");
    s.push_str("<p class=\"note\">A realistic spread of floor counts and plate sizes within the legal bounds — no single maxed-out prototype repeated. The skew toward taller anchors is the lever that keeps the development-site count digestible. Counts are per 1× DHS (Canada / Spain / Mexico each; United States is 2×).</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Class · Variant</th><th>GLA / building</th><th>Mix weight</th><th>Buildings (per 1× DHS)</th></tr>\n");
    for c in ALL_CLASSES {
        let class_count = base.class(c).count;
        for v in c.variants() {
            s.push_str(&format!(
                "<tr><td class=\"lbl\">{} · {}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
                c.label(),
                v.label,
                fmt_sqft(v.gla),
                fmt_pct(v.weight),
                fmt_int(class_count * v.weight),
            ));
        }
    }
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total buildings (per 1× DHS)</td><td>{}</td><td>—</td><td>{}</td></tr>\n",
        fmt_sqft(base.gla()),
        fmt_int(base.buildings()),
    ));
    s.push_str("</table>\n");
    s
}

// Section 3 — Development-Site composition.
fn render_site_composition(base: &Base1x) -> String {
    let sp = base.params;
    let mut s = String::new();
    s.push_str("<h2>3 · Development-Site Composition</h2>\n");
    s.push_str(&format!(
        "<p class=\"note\">The business plan is to build individual Woodfine Buildings; the Woodfine Campus — a larger site in a denser market that accommodates all four classes together — is the exception and is held to a minority (ceiling {}). Sites are worked backwards from the prototypes so the class building counts reconcile exactly to Section 2.</p>\n",
        fmt_pct(CAMPUS_SHARE_CEILING),
    ));
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Site type</th><th>Composition</th><th>Structures/site</th><th>Share of sites</th><th>Sites (per 1× DHS)</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Woodfine Campus</td><td>1 PC + 1 SO + 1 RS pair + 2 TI pairs</td><td>8</td><td>{}</td><td>{}</td></tr>\n",
        fmt_pct(sp.campus_share),
        fmt_int(base.sites_campus),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Anchor + pair</td><td>1 PC or SO + 1 pair (RS or TI)</td><td>3</td><td>{} of remainder</td><td>{}</td></tr>\n",
        fmt_pct(sp.pair_share),
        fmt_int(base.sites_pair),
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Standalone</td><td>1 PC or SO</td><td>1</td><td>{} of remainder</td><td>{}</td></tr>\n",
        fmt_pct(1.0 - sp.pair_share),
        fmt_int(base.sites_solo),
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Total development sites (per 1× DHS)</td><td>—</td><td>{} avg</td><td>100%</td><td>{}</td></tr>\n",
        comma_f(base.buildings() / base.sites_total, 1),
        fmt_int(base.sites_total),
    ));
    s.push_str("</table>\n");
    s.push_str(&format!(
        "<p class=\"note\">Anchor split {} Suburban Office / {} Professional Centres; pair split {} Retail Select / {} Tech Industrial. ~{} sites per country per three-year cycle.</p>\n",
        fmt_pct(sp.anchor_so_share),
        fmt_pct(1.0 - sp.anchor_so_share),
        fmt_pct(sp.pair_rs_share),
        fmt_pct(1.0 - sp.pair_rs_share),
        comma_f(base.sites_total / 3.0, 1),
    ));
    s
}

// Section 4 — Per-DHS rollup.
fn render_per_dhs(base: &Base1x) -> String {
    let mut s = String::new();
    s.push_str("<h2>4 · Per-DHS Rollup (full modeled build-out)</h2>\n");
    s.push_str("<p class=\"note\">Each Direct-Hold Solution replicates the 1× building portfolio scaled by its size factor (United States 2×). Figures are the fully-modeled portfolio; Section 5 shows how much is delivered inside the 10-year window. All amounts CAD.</p>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Direct-Hold Solution</th><th>Ccy</th><th>Size</th><th>Launch</th><th>Sites</th><th>Buildings</th><th>GLA</th><th>Cost</th><th>NOI</th><th>Asset value</th></tr>\n");
    let mut t_sites = 0.0;
    let mut t_bld = 0.0;
    let mut t_gla = 0.0;
    let mut t_cost = 0.0;
    let mut t_noi = 0.0;
    let mut t_av = 0.0;
    for d in DHS_LIST {
        let f = d.size_factor;
        t_sites += base.sites_total * f;
        t_bld += base.buildings() * f;
        t_gla += base.gla() * f;
        t_cost += base.cost() * f;
        t_noi += base.noi() * f;
        t_av += base.asset_value() * f;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}×</td><td>Y{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            d.name,
            d.currency,
            comma_f(d.size_factor, 0),
            d.launch_year,
            fmt_int(base.sites_total * f),
            fmt_int(base.buildings() * f),
            fmt_sqft(base.gla() * f),
            fmt_money_m(base.cost() * f),
            fmt_money_m_yr(base.noi() * f),
            fmt_money_m(base.asset_value() * f),
        ));
    }
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Four-DHS system</td><td>CAD</td><td>5×</td><td>—</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        fmt_int(t_sites),
        fmt_int(t_bld),
        fmt_sqft(t_gla),
        fmt_money_m(t_cost),
        fmt_money_m_yr(t_noi),
        fmt_money_m(t_av),
    ));
    s.push_str("</table>\n");
    s
}

// Section 5 — Horizontal build-out Y1→Y10 (+ AUM growth row).
fn render_buildout(base: &Base1x) -> String {
    let mut s = String::new();
    s.push_str("<h2>5 · Build-Out Y1 → Y10 (program window, matches WCP)</h2>\n");
    s.push_str("<p class=\"note\">Physical delivery is paced over each DHS's own ~10-year build life from its launch year (capital commits faster than concrete pours; delivery trails the WCP capital ramp). Development sites and Woodfine Buildings shown cumulative-delivered; later DHSs do not reach full build-out inside the window (see Section 6). AUM = gross asset value of the delivered portfolio, CAD.</p>\n");

    s.push_str("<table class=\"wide\">\n<tr><th class=\"lbl\">Line</th>");
    for y in 1..=PROGRAM_YEARS {
        s.push_str(&format!("<th>Y{}</th>", y));
    }
    s.push_str("</tr>\n");

    // Development sites delivered (cumulative), per DHS + total.
    s.push_str(&banner_row("DEVELOPMENT SITES — DELIVERED (CUMULATIVE)"));
    for d in DHS_LIST {
        s.push_str(&year_row(d.short, |y| {
            fmt_int(base.sites_total * d.size_factor * build_fraction(d.launch_year, y))
        }));
    }
    s.push_str(&year_row_class(
        "All-DHS sites delivered",
        "subtotal",
        |y| {
            let v: f64 = DHS_LIST
                .iter()
                .map(|d| base.sites_total * d.size_factor * build_fraction(d.launch_year, y))
                .sum();
            fmt_int(v)
        },
    ));

    // Woodfine Buildings delivered (cumulative), total.
    s.push_str(&banner_row("WOODFINE BUILDINGS — DELIVERED (CUMULATIVE)"));
    for d in DHS_LIST {
        s.push_str(&year_row(d.short, |y| {
            fmt_int(base.buildings() * d.size_factor * build_fraction(d.launch_year, y))
        }));
    }
    s.push_str(&year_row_class(
        "All-DHS buildings delivered",
        "subtotal",
        |y| {
            let v: f64 = DHS_LIST
                .iter()
                .map(|d| base.buildings() * d.size_factor * build_fraction(d.launch_year, y))
                .sum();
            fmt_int(v)
        },
    ));

    // Sites under construction (in-year flow), all-DHS.
    s.push_str(&banner_row("ALL-DHS SITES UNDER CONSTRUCTION (IN-YEAR)"));
    s.push_str(&year_row("Sites added this year", |y| {
        let v: f64 = DHS_LIST
            .iter()
            .map(|d| {
                let prev = if y > 1 {
                    build_fraction(d.launch_year, y - 1)
                } else {
                    0.0
                };
                base.sites_total * d.size_factor * (build_fraction(d.launch_year, y) - prev)
            })
            .sum();
        fmt_int(v)
    }));

    // AUM growth (CAD).
    s.push_str(&banner_row("ASSETS UNDER MANAGEMENT (CAD)"));
    s.push_str(&year_row_class(
        "AUM — delivered portfolio",
        "total",
        |y| {
            let v: f64 = DHS_LIST
                .iter()
                .map(|d| base.asset_value() * d.size_factor * build_fraction(d.launch_year, y))
                .sum();
            fmt_b(v)
        },
    ));

    s.push_str("</table>\n");
    s
}

fn banner_row(label: &str) -> String {
    format!(
        "<tr class=\"section-banner\"><td colspan=\"{}\">{}</td></tr>\n",
        PROGRAM_YEARS + 1,
        label
    )
}

fn year_row(label: &str, f: impl Fn(u32) -> String) -> String {
    year_row_class(label, "", f)
}

fn year_row_class(label: &str, class: &str, f: impl Fn(u32) -> String) -> String {
    let tr = if class.is_empty() {
        "<tr>".to_string()
    } else {
        format!("<tr class=\"{}\">", class)
    };
    let mut s = format!("{tr}<td class=\"lbl\">{label}</td>");
    for y in 1..=PROGRAM_YEARS {
        s.push_str(&format!("<td>{}</td>", f(y)));
    }
    s.push_str("</tr>\n");
    s
}

// Section 6 — System scale summary + Remaining-Sites reconciliation.
fn render_scale_reconciliation(base: &Base1x) -> String {
    let mut s = String::new();
    s.push_str("<h2>6 · System Scale &amp; Remaining-Sites Reconciliation</h2>\n");
    s.push_str("<p class=\"note\">Built within the 10-year window vs the total fully-modeled portfolio. The remaining Development Sites complete after Y10 to reach the full modeled leasable area — wording support for the prospectus and investor materials.</p>\n");

    // Per-DHS reconciliation.
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Direct-Hold Solution</th><th>Total sites</th><th>Built by Y10</th><th>Remaining</th><th>% built</th></tr>\n");
    let (mut tot, mut built) = (0.0, 0.0);
    for d in DHS_LIST {
        let total = base.sites_total * d.size_factor;
        let b = total * build_fraction(d.launch_year, PROGRAM_YEARS);
        tot += total;
        built += b;
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            d.short,
            fmt_int(total),
            fmt_int(b),
            fmt_int(total - b),
            fmt_pct(b / total),
        ));
    }
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Four-DHS system</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
        fmt_int(tot),
        fmt_int(built),
        fmt_int(tot - built),
        fmt_pct(built / tot),
    ));
    s.push_str("</table>\n");

    // System totals across metrics: within-window vs full vs remaining.
    let frac10 = |d: &Dhs| build_fraction(d.launch_year, PROGRAM_YEARS);
    let within = |per1x: f64| -> f64 {
        DHS_LIST
            .iter()
            .map(|d| per1x * d.size_factor * frac10(d))
            .sum()
    };
    let full = |per1x: f64| per1x * 5.0;
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric (four-DHS system)</th><th>Built within Y10</th><th>Total fully modeled</th><th>Remaining post-Y10</th></tr>\n");
    let rows: [(&str, f64, bool); 5] = [
        ("Development sites", base.sites_total, false),
        ("Woodfine Buildings", base.buildings(), false),
        ("Gross leasable area", base.gla(), false),
        ("Net operating income (CAD/yr)", base.noi(), true),
        ("Assets under management (CAD)", base.asset_value(), true),
    ];
    for (label, per1x, _money) in rows {
        let w = within(per1x);
        let fl = full(per1x);
        let fmt = |v: f64| -> String {
            match label {
                "Gross leasable area" => fmt_sqft(v),
                "Net operating income (CAD/yr)" => fmt_money_m_yr(v),
                "Assets under management (CAD)" => fmt_b(v),
                _ => fmt_int(v),
            }
        };
        let cls = if label.starts_with("Assets") {
            " class=\"total\""
        } else {
            ""
        };
        s.push_str(&format!(
            "<tr{}><td class=\"lbl\">{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            cls,
            label,
            fmt(w),
            fmt(fl),
            fmt(fl - w),
        ));
    }
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">AUM crosses ~CAD 4 billion around Year 8 — consistent with the prospectus target of approximately CAD 4 billion of assets under management over six to eight years.</p>\n");
    s
}

// Section 7 — Simulation / sensitivity (campus share, capped at the ceiling).
fn render_sensitivity() -> String {
    let mut s = String::new();
    s.push_str("<h2>7 · Simulation — Sensitivity to the Campus Share</h2>\n");
    s.push_str(&format!(
        "<p class=\"note\">Site count is near-invariant to the dials; the campus share (capped at the business-plan ceiling {}) and the geometry skew trade Retail/Tech density against the development-site count. Per 1× DHS at the default pair / anchor splits.</p>\n",
        fmt_pct(CAMPUS_SHARE_CEILING),
    ));
    s.push_str("<table>\n");
    s.push_str("<tr><th class=\"lbl\">Campus share</th><th>Sites (1× DHS)</th><th>Sites/cycle</th><th>Buildings (1× DHS)</th><th>System sites</th><th>System buildings</th></tr>\n");
    for cs in [0.20_f64, 0.25, 0.30] {
        let sp = SiteParams {
            campus_share: cs,
            ..SiteParams::default()
        };
        let b = compute_base(sp);
        s.push_str(&format!(
            "<tr><td class=\"lbl\">{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>\n",
            fmt_pct(cs),
            fmt_int(b.sites_total),
            comma_f(b.sites_total / 3.0, 1),
            fmt_int(b.buildings()),
            fmt_int(b.sites_total * 5.0),
            fmt_int(b.buildings() * 5.0),
        ));
    }
    s.push_str("</table>\n");
    s
}

// Section 8 — Methodology + assumptions.
fn render_methodology(base: &Base1x) -> String {
    let mut s = String::new();
    s.push_str("<h2>8 · Methodology &amp; Assumptions</h2>\n");
    s.push_str(&format!(
        "<p>Per-1×-DHS leasable area is pinned to the WCP / PCLP 1 revenue-generator ramp at {} ({} per Development Class building × the geometric mix). The United States DHS is 2× (size factor); Canada, Spain and Mexico are 1×. Launch years (Canada Y1, United States Y2, Spain Y2, Mexico Y3) follow the WCP revenue generator.</p>\n",
        fmt_sqft(PORTFOLIO_SQFT_1X),
        fmt_sqft(base.gla() / base.buildings()),
    ));
    s.push_str("<p>Development sites are worked backwards from the fixed building prototypes: total leasable area ÷ average site area = number of sites. Construction cost per Development Class: Professional Centres and Suburban Office at $310/sf GLA; Retail Select and Tech Industrial at $260/sf GLA. Development yield = rent ÷ cost = 10.5%; NOI = rent × 57% (43% operating-cost ratio); asset value = NOI ÷ 6.25% cap rate; depreciation = cost ÷ 40 years (building component only). Amounts CAD.</p>\n");
    s.push_str("<p><strong>Legal geometry:</strong> Professional Centres 3–5 floors (21,000 sf/floor); Suburban Office 6–9 floors (19,000 sf/floor); Retail Select 4,500 / 6,700 / 7,700 sf; Tech Industrial 7,200 / 8,400 sf, always paired. <strong>Time:</strong> program Y1–Y10 absolute (matching the WCP proforma tables); physical delivery paced over each DHS's ~10-year build life from launch.</p>\n");
    s
}

// ── Single-page summary ─────────────────────────────────────────────────────

pub fn render_summary() -> String {
    let base = compute_base(SiteParams::default());
    let within = |per1x: f64| -> f64 {
        DHS_LIST
            .iter()
            .map(|d| per1x * d.size_factor * build_fraction(d.launch_year, PROGRAM_YEARS))
            .sum()
    };
    let mut s = String::new();
    s.push_str(HEAD);
    s.push_str("<body>\n");
    s.push_str("<h1>Woodfine Direct-Hold Solutions — Building Portfolio Summary V2</h1>\n");
    s.push_str("<p>DRAFT — 2026-06-13 — V2<br>\n");
    s.push_str("Companion: <code>COMPLIANCE_MCorp_2026_06_04_Proforma_BuildingPortfolio_V2.html</code> (full proforma)<br>\n");
    s.push_str(
        "All amounts CAD — Forward-looking projections; BCSC continuous-disclosure posture.</p>\n",
    );

    s.push_str("<h2>Four-DHS Scale</h2>\n<table>\n");
    s.push_str("<tr><th class=\"lbl\">Metric</th><th>Built within Y10</th><th>Total fully modeled</th></tr>\n");
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Development sites</td><td>{}</td><td>{}</td></tr>\n",
        fmt_int(within(base.sites_total)),
        fmt_int(base.sites_total * 5.0)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Woodfine Buildings</td><td>{}</td><td>{}</td></tr>\n",
        fmt_int(within(base.buildings())),
        fmt_int(base.buildings() * 5.0)
    ));
    s.push_str(&format!(
        "<tr><td class=\"lbl\">Gross leasable area</td><td>{}</td><td>{}</td></tr>\n",
        fmt_sqft(within(base.gla())),
        fmt_sqft(base.gla() * 5.0)
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td class=\"lbl\">Assets under management (CAD)</td><td>{}</td><td>{}</td></tr>\n",
        fmt_b(within(base.asset_value())),
        fmt_b(base.asset_value() * 5.0)
    ));
    s.push_str("</table>\n");
    s.push_str("<p class=\"note\">Canada / Spain / Mexico each replicate the 1× building portfolio; the United States is 2×. ~7 development sites per country per three-year cycle.</p>\n");
    s.push_str(DISCLAIMER);
    s.push_str(LNUM_SCRIPT);
    s.push_str("</body></html>\n");
    s
}

// ── JSON dump ───────────────────────────────────────────────────────────────

pub fn render_json() -> String {
    let base = compute_base(SiteParams::default());
    let dhs: Vec<serde_json::Value> = DHS_LIST
        .iter()
        .map(|d| {
            let f = d.size_factor;
            let years: Vec<serde_json::Value> = (1..=PROGRAM_YEARS)
                .map(|y| {
                    let frac = build_fraction(d.launch_year, y);
                    serde_json::json!({
                        "year": y,
                        "sites_delivered": base.sites_total * f * frac,
                        "buildings_delivered": base.buildings() * f * frac,
                        "aum_cad": base.asset_value() * f * frac,
                    })
                })
                .collect();
            serde_json::json!({
                "short": d.short,
                "name": d.name,
                "currency": d.currency,
                "size_factor": d.size_factor,
                "launch_year": d.launch_year,
                "sites_total": base.sites_total * f,
                "buildings_total": base.buildings() * f,
                "gla_total": base.gla() * f,
                "built_within_window_pct": build_fraction(d.launch_year, PROGRAM_YEARS),
                "years": years,
            })
        })
        .collect();

    let within = |per1x: f64| -> f64 {
        DHS_LIST
            .iter()
            .map(|d| per1x * d.size_factor * build_fraction(d.launch_year, PROGRAM_YEARS))
            .sum()
    };

    let json = serde_json::json!({
        "entity": "Woodfine Direct-Hold Solutions — Building Portfolio (four DHS)",
        "source": "tool-proforma-engine src/report/building_portfolio_v2 module",
        "version": "V2",
        "generated_at": "2026-06-13",
        "model_constants": {
            "target_dev_yield": TARGET_DEV_YIELD,
            "target_cap_rate": TARGET_CAP_RATE,
            "opex_rate": OPEX_RATE,
            "depreciation_yrs": DEPRECIATION_YRS,
            "portfolio_sqft_1x": PORTFOLIO_SQFT_1X,
            "program_years": PROGRAM_YEARS,
            "build_life_years": BUILD_LIFE_YEARS,
            "campus_share_ceiling": CAMPUS_SHARE_CEILING,
        },
        "site_params": base.params,
        "base_1x": {
            "sites_total": base.sites_total,
            "sites_campus": base.sites_campus,
            "sites_pair": base.sites_pair,
            "sites_solo": base.sites_solo,
            "buildings": base.buildings(),
            "gla": base.gla(),
            "cost": base.cost(),
            "noi": base.noi(),
            "asset_value": base.asset_value(),
            "classes": {
                "professional_centres": base.pc,
                "suburban_office": base.so,
                "retail_select": base.rs,
                "tech_industrial": base.ti,
            }
        },
        "dhs": dhs,
        "system": {
            "sites_total": base.sites_total * 5.0,
            "buildings_total": base.buildings() * 5.0,
            "gla_total": base.gla() * 5.0,
            "asset_value_total": base.asset_value() * 5.0,
            "sites_built_within_window": within(base.sites_total),
            "buildings_built_within_window": within(base.buildings()),
            "aum_within_window": within(base.asset_value()),
            "sites_remaining_post_window": base.sites_total * 5.0 - within(base.sites_total),
        },
        "note": "Four building DHS (DHS-01..04); DHS-05 Vertical Warehouse and DHS-06 Parking Structure are separate asset classes, excluded."
    });
    serde_json::to_string_pretty(&json).expect("Building Portfolio V2 JSON serialization failed")
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_gla_pins_to_portfolio_target() {
        let b = compute_base(SiteParams::default());
        assert!(
            (b.gla() - PORTFOLIO_SQFT_1X).abs() < 1.0,
            "1× GLA must pin to {PORTFOLIO_SQFT_1X}"
        );
    }

    #[test]
    fn class_counts_reconcile_to_total_buildings() {
        let b = compute_base(SiteParams::default());
        let sum = b.pc.count + b.so.count + b.rs.count + b.ti.count;
        assert!((sum - b.buildings()).abs() < 1e-6);
    }

    #[test]
    fn site_breakdown_sums_to_total_sites() {
        let b = compute_base(SiteParams::default());
        let sum = b.sites_campus + b.sites_pair + b.sites_solo;
        assert!((sum - b.sites_total).abs() < 1e-6);
    }

    #[test]
    fn campus_share_within_business_plan_ceiling() {
        assert!(SiteParams::default().campus_share <= CAMPUS_SHARE_CEILING + 1e-9);
    }

    #[test]
    fn legal_floor_bounds() {
        for v in PC_VARIANTS {
            assert!(
                (3..=5).contains(&v.floors),
                "PC floors out of bounds: {}",
                v.floors
            );
        }
        for v in SO_VARIANTS {
            assert!(
                (6..=9).contains(&v.floors),
                "SO floors out of bounds: {}",
                v.floors
            );
        }
    }

    #[test]
    fn mix_weights_sum_to_one() {
        for c in ALL_CLASSES {
            let w: f64 = c.variants().iter().map(|v| v.weight).sum();
            assert!((w - 1.0).abs() < 1e-9, "{} weights sum to {}", c.label(), w);
        }
    }

    #[test]
    fn program_window_is_ten_years() {
        assert_eq!(PROGRAM_YEARS, 10);
    }

    #[test]
    fn build_fraction_truncates_late_dhs() {
        // Canada (Y1) fully built; later DHSs partial within the Y10 window.
        assert!((build_fraction(1, 10) - 1.0).abs() < 1e-9);
        assert!((build_fraction(2, 10) - 0.9).abs() < 1e-9);
        assert!((build_fraction(3, 10) - 0.8).abs() < 1e-9);
        assert!((build_fraction(2, 1) - 0.0).abs() < 1e-9); // before launch
    }

    #[test]
    fn within_window_plus_remaining_equals_total() {
        let b = compute_base(SiteParams::default());
        for d in DHS_LIST {
            let total = b.sites_total * d.size_factor;
            let within = total * build_fraction(d.launch_year, PROGRAM_YEARS);
            let remaining = total - within;
            assert!((within + remaining - total).abs() < 1e-6);
        }
    }

    #[test]
    fn us_dhs_is_double_canada() {
        let us = DHS_LIST[1];
        let ca = DHS_LIST[0];
        assert!((us.size_factor - 2.0 * ca.size_factor).abs() < 1e-9);
    }

    #[test]
    fn system_aum_crosses_4b_by_year_8() {
        let b = compute_base(SiteParams::default());
        let aum = |y: u32| -> f64 {
            DHS_LIST
                .iter()
                .map(|d| b.asset_value() * d.size_factor * build_fraction(d.launch_year, y))
                .sum()
        };
        assert!(aum(8) >= 3.9e9, "AUM at Y8 = {}", aum(8));
        assert!(aum(7) < aum(8) && aum(8) < aum(10));
    }
}
