// D1 Development Classes — v2 calibrated model
//
// Per plan dated 2026-06-03:
//   - 4 classes (Professional Centres, Suburban Office, Tech Industrial, Retail Select)
//   - Each class has multiple building variants (floor counts or size variants)
//   - Cost per sf gross is the primary assumption:
//       - Professional Centres: $311.05/sf gross (from Test Site_Report)
//       - Suburban Office:      $273.97/sf gross (from Test Site_Underground_Report)
//       - Tech Industrial:      ~$200/sf gross (extrapolated industrial-blend)
//       - Retail Select:        ~$255/sf gross (extrapolated standalone retail)
//   - Rent is WORKED BACKWARDS from 10.5% dev yield target
//   - GDV at 6.25% cap rate on rent net of 5.5% non-recovery cost
//
// Output: a self-contained HTML page with all four classes side-by-side
// (similar shape to the JW2 SPV Operating Budget HTML).

const TARGET_DEV_YIELD: f64 = 0.105;
const TARGET_CAP_RATE: f64 = 0.0625;
const NON_RECOVERY_RATE: f64 = 0.055;

// ─── Variant + Class definitions ────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct BuildingVariant {
    pub label: &'static str,
    pub floors: u32,
    pub office_sqft_per_floor: f64,
    pub retail_sqft: f64,
    pub count: u32,
}

impl BuildingVariant {
    pub fn nla_per_building(&self) -> f64 {
        self.office_sqft_per_floor * self.floors as f64 + self.retail_sqft
    }

    pub fn class_nla_contribution(&self) -> f64 {
        self.nla_per_building() * self.count as f64
    }
}

#[derive(Debug, Clone)]
pub struct DevClassV2 {
    pub label: &'static str,
    pub variants: &'static [BuildingVariant],
    pub cost_per_sf_gross: f64,
    pub nla_gross_ratio: f64,
    pub brief_class_nla: f64,
    pub cost_source: &'static str,
}

#[derive(Debug, Clone)]
pub struct ClassCalibration {
    pub class_total_nla: f64,
    pub class_total_gross_sf: f64,
    pub class_total_cost: f64,
    pub required_rent_at_10_5_yield: f64,
    pub implied_rent_per_sf_nla: f64,
    pub gdv_at_6_25_cap: f64,
    pub nla_gap_vs_brief: f64,
    pub nla_gap_pct: f64,
}

impl DevClassV2 {
    pub fn class_total_nla(&self) -> f64 {
        self.variants.iter().map(|v| v.class_nla_contribution()).sum()
    }

    pub fn class_total_gross_sf(&self) -> f64 {
        self.class_total_nla() / self.nla_gross_ratio
    }

    pub fn class_total_cost(&self) -> f64 {
        self.class_total_gross_sf() * self.cost_per_sf_gross
    }

    pub fn calibrate(&self) -> ClassCalibration {
        let nla = self.class_total_nla();
        let gross = self.class_total_gross_sf();
        let cost = self.class_total_cost();
        let rent = cost * TARGET_DEV_YIELD;
        let rent_per_sf = if nla > 0.0 { rent / nla } else { 0.0 };
        let net_rent = rent * (1.0 - NON_RECOVERY_RATE);
        let gdv = net_rent / TARGET_CAP_RATE;
        let gap = nla - self.brief_class_nla;
        let gap_pct = if self.brief_class_nla > 0.0 {
            gap / self.brief_class_nla * 100.0
        } else {
            0.0
        };
        ClassCalibration {
            class_total_nla: nla,
            class_total_gross_sf: gross,
            class_total_cost: cost,
            required_rent_at_10_5_yield: rent,
            implied_rent_per_sf_nla: rent_per_sf,
            gdv_at_6_25_cap: gdv,
            nla_gap_vs_brief: gap,
            nla_gap_pct: gap_pct,
        }
    }
}

// ─── Locked dev-class definitions (2026-06-03) ──────────────────────────────

pub const PROFESSIONAL_CENTRES: DevClassV2 = DevClassV2 {
    label: "Professional Centres",
    variants: &[
        BuildingVariant {
            label: "3-floor (Test Site reference)",
            floors: 3,
            office_sqft_per_floor: 21_000.0,
            retail_sqft: 10_600.0,
            count: 5,
        },
        BuildingVariant {
            label: "4-floor",
            floors: 4,
            office_sqft_per_floor: 21_000.0,
            retail_sqft: 10_600.0,
            count: 5,
        },
        BuildingVariant {
            label: "5-floor",
            floors: 5,
            office_sqft_per_floor: 21_000.0,
            retail_sqft: 10_600.0,
            count: 5,
        },
    ],
    cost_per_sf_gross: 311.05,
    nla_gross_ratio: 0.7955,
    brief_class_nla: 919_260.0,
    cost_source: "Test Site_Report row 23 (Excel exact)",
};

pub const SUBURBAN_OFFICE: DevClassV2 = DevClassV2 {
    label: "Suburban Office",
    variants: &[
        BuildingVariant {
            label: "3-floor",
            floors: 3,
            office_sqft_per_floor: 19_000.0,
            retail_sqft: 0.0,
            count: 3,
        },
        BuildingVariant {
            label: "4-floor",
            floors: 4,
            office_sqft_per_floor: 19_000.0,
            retail_sqft: 0.0,
            count: 3,
        },
        BuildingVariant {
            label: "5-floor (Test Site reference)",
            floors: 5,
            office_sqft_per_floor: 19_000.0,
            retail_sqft: 0.0,
            count: 3,
        },
    ],
    cost_per_sf_gross: 273.97,
    nla_gross_ratio: 0.8167,
    brief_class_nla: 689_445.0,
    cost_source: "Test Site_Underground_Report row 23 (Excel exact)",
};

pub const TECH_INDUSTRIAL: DevClassV2 = DevClassV2 {
    label: "Tech Industrial",
    variants: &[
        BuildingVariant {
            label: "Small pair-half (7,200 sf)",
            floors: 1,
            office_sqft_per_floor: 7_200.0,
            retail_sqft: 0.0,
            count: 30,
        },
        BuildingVariant {
            label: "Large pair-half (8,400 sf)",
            floors: 1,
            office_sqft_per_floor: 8_400.0,
            retail_sqft: 0.0,
            count: 30,
        },
    ],
    cost_per_sf_gross: 200.0,
    nla_gross_ratio: 0.95,
    brief_class_nla: 459_630.0,
    cost_source: "Extrapolated (industrial-blend; land $15/sf, construction $150/sf, minimal TI)",
};

pub const RETAIL_SELECT: DevClassV2 = DevClassV2 {
    label: "Retail Select",
    variants: &[
        BuildingVariant {
            label: "Small (4,500 sf, paired)",
            floors: 1,
            office_sqft_per_floor: 4_500.0,
            retail_sqft: 0.0,
            count: 12,
        },
        BuildingVariant {
            label: "Medium (6,700 sf, paired)",
            floors: 1,
            office_sqft_per_floor: 6_700.0,
            retail_sqft: 0.0,
            count: 12,
        },
        BuildingVariant {
            label: "Large (7,700 sf, paired)",
            floors: 1,
            office_sqft_per_floor: 7_700.0,
            retail_sqft: 0.0,
            count: 12,
        },
    ],
    cost_per_sf_gross: 255.0,
    nla_gross_ratio: 0.92,
    brief_class_nla: 229_815.0,
    cost_source: "Extrapolated (standalone retail; land $25/sf, construction $180/sf, TI $15/sf)",
};

pub const ALL_CLASSES: &[&DevClassV2] = &[
    &PROFESSIONAL_CENTRES,
    &SUBURBAN_OFFICE,
    &TECH_INDUSTRIAL,
    &RETAIL_SELECT,
];

// ─── HTML renderer ─────────────────────────────────────────────────────────

fn fmt_money(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    if v.abs() >= 1_000_000.0 {
        format!("${:.2}M", v / 1_000_000.0)
    } else if v.abs() >= 1_000.0 {
        format!("${:.1}K", v / 1_000.0)
    } else {
        format!("${:.2}", v)
    }
}

fn fmt_sqft(v: f64) -> String {
    format!("{:.0} sf", v)
}

fn fmt_pct(v: f64) -> String {
    format!("{:.2}%", v * 100.0)
}

fn fmt_signed_pct(v: f64) -> String {
    if v >= 0.0 {
        format!("+{:.1}%", v)
    } else {
        format!("{:.1}%", v)
    }
}

pub fn render_html() -> String {
    let mut s = String::new();
    s.push_str(HEAD);
    s.push_str("<body>\n");
    s.push_str("<h1>D1 Development Classes — Calibrated to 10.5% Dev Yield + 6.25% Cap Rate</h1>\n");
    s.push_str("<p>Companion to <code>outputs/audit_titleco3_dev_classes.json</code> (Phase A audit). \
                Engine recalibration per plan 2026-06-03. v2 supersedes the earlier <code>d1-dev-classes-2026-06-03.html</code>.</p>\n");
    s.push_str("<p>All amounts CAD. Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture.</p>\n");

    s.push_str("<h2>Portfolio Rollup (per WMC Tear Sheet)</h2>\n");
    let portfolio_nla: f64 = ALL_CLASSES.iter().map(|c| c.class_total_nla()).sum();
    let portfolio_cost: f64 = ALL_CLASSES.iter().map(|c| c.class_total_cost()).sum();
    let portfolio_rent: f64 = ALL_CLASSES.iter().map(|c| c.calibrate().required_rent_at_10_5_yield).sum();
    let portfolio_gdv: f64 = ALL_CLASSES.iter().map(|c| c.calibrate().gdv_at_6_25_cap).sum();
    s.push_str("<table class=\"summary\">\n");
    s.push_str("<tr><th>Metric</th><th>v2 Engine</th><th>WMC Tear Sheet</th></tr>\n");
    s.push_str(&format!(
        "<tr><td>Total NLA</td><td class=\"r\">{}</td><td class=\"r\">2,298,150 sf</td></tr>\n",
        fmt_sqft(portfolio_nla)
    ));
    s.push_str(&format!(
        "<tr><td>Total Project Cost</td><td class=\"r\">{}</td><td class=\"r\">$750,000,000</td></tr>\n",
        fmt_money(portfolio_cost)
    ));
    s.push_str(&format!(
        "<tr><td>Required Rent (10.5% yield)</td><td class=\"r\">{}</td><td class=\"r\">$78,750,000</td></tr>\n",
        fmt_money(portfolio_rent)
    ));
    s.push_str(&format!(
        "<tr><td>GDV at 6.25% cap (net rent)</td><td class=\"r\">{}</td><td class=\"r\">$1,260,000,000</td></tr>\n",
        fmt_money(portfolio_gdv)
    ));
    s.push_str("</table>\n");

    s.push_str("<h2>Per-Class Calibration Summary</h2>\n");
    s.push_str("<table class=\"summary\">\n");
    s.push_str("<tr><th>Class</th><th>Buildings</th><th>NLA</th><th>BRIEF NLA</th><th>Gap</th>\
                <th>$/sf gross</th><th>Total Cost</th><th>Req Rent (10.5%)</th>\
                <th>Implied $/sf</th><th>GDV (6.25% cap)</th></tr>\n");
    for class in ALL_CLASSES {
        let cal = class.calibrate();
        let bldg_count: u32 = class.variants.iter().map(|v| v.count).sum();
        s.push_str(&format!(
            "<tr><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td>\
             <td class=\"r\">{}</td><td class=\"r\">${:.2}</td><td class=\"r\">{}</td>\
             <td class=\"r\">{}</td><td class=\"r\">${:.2}</td><td class=\"r\">{}</td></tr>\n",
            class.label,
            bldg_count,
            fmt_sqft(cal.class_total_nla),
            fmt_sqft(class.brief_class_nla),
            fmt_signed_pct(cal.nla_gap_pct),
            class.cost_per_sf_gross,
            fmt_money(cal.class_total_cost),
            fmt_money(cal.required_rent_at_10_5_yield),
            cal.implied_rent_per_sf_nla,
            fmt_money(cal.gdv_at_6_25_cap),
        ));
    }
    s.push_str("</table>\n");

    for class in ALL_CLASSES {
        s.push_str(&render_class_detail(class));
    }

    s.push_str("<p class=\"footer\">Cap rate (6.25%) applied to rent net of 5.5% non-recovery cost \
                per Excel Investment Valuation methodology. Dev yield (10.5%) per WMC Tear Sheet \
                10.5% × $750M = $78.75M NOI target. Rent worked backwards from yield × cost; \
                implied $/sf NLA shown as derived output (not market-rent input). \
                Cost rates: PC/SO exact from Test Site Excel; TI/RS extrapolated per plan §B.</p>\n");

    s.push_str("</body></html>\n");
    s
}

fn render_class_detail(class: &DevClassV2) -> String {
    let cal = class.calibrate();
    let mut s = String::new();
    s.push_str(&format!("<h2>{}</h2>\n", class.label));
    s.push_str(&format!("<p><strong>Cost source:</strong> {}</p>\n", class.cost_source));

    s.push_str("<table class=\"variants\">\n");
    s.push_str("<tr><th>Variant</th><th>Floors</th><th>$/sf office</th><th>Retail</th>\
                <th>NLA per bldg</th><th>Count</th><th>Class NLA</th></tr>\n");
    for v in class.variants {
        s.push_str(&format!(
            "<tr><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td>\
             <td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td>\
             <td class=\"r\">{}</td></tr>\n",
            v.label,
            v.floors,
            fmt_sqft(v.office_sqft_per_floor),
            fmt_sqft(v.retail_sqft),
            fmt_sqft(v.nla_per_building()),
            v.count,
            fmt_sqft(v.class_nla_contribution()),
        ));
    }
    s.push_str(&format!(
        "<tr><td><strong>Class total</strong></td><td colspan=\"5\"></td>\
         <td class=\"r\"><strong>{}</strong></td></tr>\n",
        fmt_sqft(cal.class_total_nla)
    ));
    s.push_str("</table>\n");

    s.push_str("<table class=\"calibration\">\n");
    s.push_str("<tr><th>Metric</th><th>Value</th></tr>\n");
    s.push_str(&format!(
        "<tr><td>Class total NLA</td><td class=\"r\">{}</td></tr>\n",
        fmt_sqft(cal.class_total_nla)
    ));
    s.push_str(&format!(
        "<tr><td>Class total Gross SF (NLA / {:.2}%)</td><td class=\"r\">{}</td></tr>\n",
        class.nla_gross_ratio * 100.0,
        fmt_sqft(cal.class_total_gross_sf)
    ));
    s.push_str(&format!(
        "<tr><td>Cost per sf gross</td><td class=\"r\">${:.2}</td></tr>\n",
        class.cost_per_sf_gross
    ));
    s.push_str(&format!(
        "<tr><td>Class total project cost</td><td class=\"r\">{}</td></tr>\n",
        fmt_money(cal.class_total_cost)
    ));
    s.push_str(&format!(
        "<tr><td>Target Dev Yield</td><td class=\"r\">{}</td></tr>\n",
        fmt_pct(TARGET_DEV_YIELD)
    ));
    s.push_str(&format!(
        "<tr><td><strong>Required rent (worked backwards)</strong></td><td class=\"r\"><strong>{}</strong></td></tr>\n",
        fmt_money(cal.required_rent_at_10_5_yield)
    ));
    s.push_str(&format!(
        "<tr><td>Implied rent per sf NLA</td><td class=\"r\">${:.2}/sf NLA</td></tr>\n",
        cal.implied_rent_per_sf_nla
    ));
    s.push_str(&format!(
        "<tr><td>Target Cap Rate (on net rent)</td><td class=\"r\">{}</td></tr>\n",
        fmt_pct(TARGET_CAP_RATE)
    ));
    s.push_str(&format!(
        "<tr><td><strong>GDV at 6.25% cap</strong></td><td class=\"r\"><strong>{}</strong></td></tr>\n",
        fmt_money(cal.gdv_at_6_25_cap)
    ));
    s.push_str(&format!(
        "<tr><td>BRIEF §5h class NLA target</td><td class=\"r\">{}</td></tr>\n",
        fmt_sqft(class.brief_class_nla)
    ));
    s.push_str(&format!(
        "<tr><td>NLA gap vs BRIEF</td><td class=\"r\">{} ({})</td></tr>\n",
        fmt_sqft(cal.nla_gap_vs_brief),
        fmt_signed_pct(cal.nla_gap_pct)
    ));
    s.push_str("</table>\n");

    s
}

const HEAD: &str = r#"<!DOCTYPE html>
<html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>D1 Development Classes — v2 Calibrated</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1200px}
h1{font-size:1.3rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.3rem;border-bottom:1px solid #ccc;padding-bottom:2px}
p{margin:0.3rem 0;font-size:0.82rem;color:#444}
table{border-collapse:collapse;margin:0.5rem 0 1rem;font-size:0.78rem;width:100%}
th,td{border:1px solid #ccc;padding:4px 8px;text-align:left}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.r{text-align:right;white-space:nowrap}
tr:last-child td{background:#f9f9f9}
table.summary tr:last-child td{font-weight:600}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@media print{body{margin:0;font-size:11px;max-width:none}@page{size:letter landscape;margin:1.5cm}}
</style>
</head>
"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_nla_per_building_matches_jennifer_spec() {
        // PC 3-floor: 3 × 21,000 + 10,600 = 73,600 sf
        assert_eq!(PROFESSIONAL_CENTRES.variants[0].nla_per_building(), 73_600.0);
        // PC 4-floor: 4 × 21,000 + 10,600 = 94,600 sf
        assert_eq!(PROFESSIONAL_CENTRES.variants[1].nla_per_building(), 94_600.0);
        // PC 5-floor: 5 × 21,000 + 10,600 = 115,600 sf
        assert_eq!(PROFESSIONAL_CENTRES.variants[2].nla_per_building(), 115_600.0);
    }

    #[test]
    fn so_nla_per_building_matches_brief() {
        // SO 3-floor: 3 × 19,000 = 57,000 sf
        assert_eq!(SUBURBAN_OFFICE.variants[0].nla_per_building(), 57_000.0);
        // SO 4-floor: 4 × 19,000 = 76,000 sf
        assert_eq!(SUBURBAN_OFFICE.variants[1].nla_per_building(), 76_000.0);
        // SO 5-floor: 5 × 19,000 = 95,000 sf
        assert_eq!(SUBURBAN_OFFICE.variants[2].nla_per_building(), 95_000.0);
        // Class total at 3/3/3 distribution = 684,000 (≈ BRIEF 689,445; +0.8% gap acceptable)
        assert!((SUBURBAN_OFFICE.class_total_nla() - 684_000.0).abs() < 1.0);
    }

    #[test]
    fn ti_pair_structure_brief_aligned() {
        // 7,200 + 8,400 = 15,600 sf per pair × 30 pairs = 468,000 sf
        assert_eq!(TECH_INDUSTRIAL.variants[0].nla_per_building(), 7_200.0);
        assert_eq!(TECH_INDUSTRIAL.variants[1].nla_per_building(), 8_400.0);
        assert!((TECH_INDUSTRIAL.class_total_nla() - 468_000.0).abs() < 1.0);
    }

    #[test]
    fn rs_pair_structure_brief_aligned() {
        // 12 × (4,500 + 6,700 + 7,700) = 12 × 18,900 = 226,800 sf
        assert!((RETAIL_SELECT.class_total_nla() - 226_800.0).abs() < 1.0);
    }

    #[test]
    fn pc_calibration_matches_plan_at_10_5_yield() {
        let cal = PROFESSIONAL_CENTRES.calibrate();
        // PC: NLA 1,419,000 (5+5+5 split); gross = 1,419,000 / 0.7955 = 1,783,784 sf
        // cost = 1,783,784 × $311.05 ≈ $554.9M
        // required rent = $554.9M × 0.105 ≈ $58.3M
        // implied $/sf NLA = $58.3M / 1,419,000 ≈ $41.06/sf — matches Excel-PC calibration
        assert!((cal.implied_rent_per_sf_nla - 41.06).abs() < 0.1);
    }

    #[test]
    fn so_calibration_implied_rent_matches_plan() {
        let cal = SUBURBAN_OFFICE.calibrate();
        // SO: NLA 684,000; gross = 684,000 / 0.8167 = 837,517 sf
        // cost = 837,517 × $273.97 ≈ $229.4M
        // required rent = $229.4M × 0.105 ≈ $24.1M
        // implied $/sf NLA = $24.1M / 684,000 ≈ $35.22/sf — matches Excel-SO calibration
        assert!((cal.implied_rent_per_sf_nla - 35.22).abs() < 0.1);
    }

    #[test]
    fn portfolio_rollup_within_reasonable_range() {
        let portfolio_nla: f64 = ALL_CLASSES.iter().map(|c| c.class_total_nla()).sum();
        let portfolio_cost: f64 = ALL_CLASSES.iter().map(|c| c.class_total_cost()).sum();
        // BRIEF portfolio: 2,298,150 sf / $750M
        // Engine portfolio (with PC 5/5/5 = 1,419,000): ~2,797,800 sf
        // Cost rollup uses each class's $/sf rate, not the blended $326.35 (avoids double-count)
        assert!(portfolio_nla > 2_000_000.0 && portfolio_nla < 3_500_000.0);
        assert!(portfolio_cost > 500_000_000.0 && portfolio_cost < 1_200_000_000.0);
    }
}
