// D1 Development Classes — v1-aligned model (refresh 2026-06-03)
//
// Replaces the prior calibrated-from-Excel approach. The canonical model is the
// PCLP 1 prior-work deliverables printed 2026-05-14:
//   inputs/pclp1-portfolio-summary-v1.html       (Deliverable ###12)
//   inputs/pclp1-building-class-proformas-v1.html (Deliverable ###11)
//
// Portfolio: 70 buildings · 3,894,100 sf GLA
//   Suburban Office     — 19 bldgs · 9F×2 / 8F×3 / 7F×8 / 6F×6 at 19,000 sf/floor · $310/sf GLA
//   Professional Centres — 12 bldgs · 5F×5 / 4F×5 / 3F×2     at 21,000 sf/floor · $310/sf GLA
//   Tech Industrial     — 24 bldgs (12 pairs) · Med×8 (2×7,200) + Lg×4 (2×8,400) · $260/sf GLA
//   Retail Select       — 15 bldgs · 6,700×8 / 4,500×4 / 7,700×3 · $260/sf GLA
//
// Mechanics (per building):
//   gla        = floors × floor_plate_sf
//   cost       = gla × cost_per_sf_gla
//   rent       = cost × 10.5%                     (target dev yield)
//   noi        = rent × 57%                       (43% OPEX assumption)
//   asset_val  = noi / 6.25%                      (cap rate on NOI, NOT on net rent)
//   depr       = cost / 40                        (40-yr SL on building only)
//
// CLI subcommand `dev-classes-v2` is preserved; the renderer now produces the v1-aligned
// HTML at outputs/d1-dev-classes-2026-06-03-v3.html.

const TARGET_DEV_YIELD: f64 = 0.105;
const TARGET_CAP_RATE: f64 = 0.0625;
const OPEX_RATE: f64 = 0.43;
const DEPRECIATION_YRS: f64 = 40.0;

// ─── Variant + Class definitions ────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct BuildingVariant {
    pub label: &'static str,
    pub distribution: &'static str,
    pub floors: u32,
    pub floor_plate_sf: f64,
    pub count: u32,
}

impl BuildingVariant {
    pub fn gla_per_building(&self) -> f64 {
        self.floors as f64 * self.floor_plate_sf
    }

    pub fn class_gla_contribution(&self) -> f64 {
        self.gla_per_building() * self.count as f64
    }

    pub fn cost_per_building(&self, cost_per_sf_gla: f64) -> f64 {
        self.gla_per_building() * cost_per_sf_gla
    }

    pub fn rent_per_building(&self, cost_per_sf_gla: f64) -> f64 {
        self.cost_per_building(cost_per_sf_gla) * TARGET_DEV_YIELD
    }

    pub fn noi_per_building(&self, cost_per_sf_gla: f64) -> f64 {
        self.rent_per_building(cost_per_sf_gla) * (1.0 - OPEX_RATE)
    }

    pub fn asset_value_per_building(&self, cost_per_sf_gla: f64) -> f64 {
        self.noi_per_building(cost_per_sf_gla) / TARGET_CAP_RATE
    }

    pub fn depreciation_per_building(&self, cost_per_sf_gla: f64) -> f64 {
        self.cost_per_building(cost_per_sf_gla) / DEPRECIATION_YRS
    }
}

#[derive(Debug, Clone)]
pub struct DevClassV2 {
    pub label: &'static str,
    pub variants: &'static [BuildingVariant],
    pub cost_per_sf_gla: f64,
    pub pairs: bool,
    pub cost_source: &'static str,
}

#[derive(Debug, Clone)]
pub struct ClassRollup {
    pub building_count: u32,
    pub class_gla: f64,
    pub class_cost: f64,
    pub class_rent: f64,
    pub class_noi: f64,
    pub class_asset_value: f64,
    pub class_depreciation: f64,
}

impl DevClassV2 {
    pub fn building_count(&self) -> u32 {
        self.variants.iter().map(|v| v.count).sum()
    }

    pub fn class_gla(&self) -> f64 {
        self.variants
            .iter()
            .map(|v| v.class_gla_contribution())
            .sum()
    }

    pub fn rollup(&self) -> ClassRollup {
        let mut r = ClassRollup {
            building_count: self.building_count(),
            class_gla: self.class_gla(),
            class_cost: 0.0,
            class_rent: 0.0,
            class_noi: 0.0,
            class_asset_value: 0.0,
            class_depreciation: 0.0,
        };
        for v in self.variants {
            let n = v.count as f64;
            r.class_cost += v.cost_per_building(self.cost_per_sf_gla) * n;
            r.class_rent += v.rent_per_building(self.cost_per_sf_gla) * n;
            r.class_noi += v.noi_per_building(self.cost_per_sf_gla) * n;
            r.class_asset_value += v.asset_value_per_building(self.cost_per_sf_gla) * n;
            r.class_depreciation += v.depreciation_per_building(self.cost_per_sf_gla) * n;
        }
        r
    }
}

// ─── Locked v1 PCLP 1 dev-class definitions (2026-05-14 prior work) ─────────

pub const SUBURBAN_OFFICE: DevClassV2 = DevClassV2 {
    label: "Suburban Office",
    variants: &[
        BuildingVariant {
            label: "9-floor",
            distribution: "some",
            floors: 9,
            floor_plate_sf: 19_000.0,
            count: 2,
        },
        BuildingVariant {
            label: "8-floor",
            distribution: "fewer",
            floors: 8,
            floor_plate_sf: 19_000.0,
            count: 3,
        },
        BuildingVariant {
            label: "7-floor (reference)",
            distribution: "most",
            floors: 7,
            floor_plate_sf: 19_000.0,
            count: 8,
        },
        BuildingVariant {
            label: "6-floor",
            distribution: "many",
            floors: 6,
            floor_plate_sf: 19_000.0,
            count: 6,
        },
    ],
    cost_per_sf_gla: 310.0,
    pairs: false,
    cost_source: "v1 Deliverable ###11 (2026-05-14)",
};

pub const PROFESSIONAL_CENTRES: DevClassV2 = DevClassV2 {
    label: "Professional Centres",
    variants: &[
        BuildingVariant {
            label: "5-floor",
            distribution: "quite a few",
            floors: 5,
            floor_plate_sf: 21_000.0,
            count: 5,
        },
        BuildingVariant {
            label: "4-floor (reference)",
            distribution: "some",
            floors: 4,
            floor_plate_sf: 21_000.0,
            count: 5,
        },
        BuildingVariant {
            label: "3-floor",
            distribution: "barely",
            floors: 3,
            floor_plate_sf: 21_000.0,
            count: 2,
        },
    ],
    cost_per_sf_gla: 310.0,
    pairs: false,
    cost_source: "v1 Deliverable ###11 (2026-05-14)",
};

pub const TECH_INDUSTRIAL: DevClassV2 = DevClassV2 {
    label: "Tech Industrial",
    variants: &[
        BuildingVariant {
            label: "Medium pair-half (7,200 sf, reference)",
            distribution: "most",
            floors: 1,
            floor_plate_sf: 7_200.0,
            count: 16, // 8 pairs × 2 buildings each
        },
        BuildingVariant {
            label: "Large pair-half (8,400 sf)",
            distribution: "some",
            floors: 1,
            floor_plate_sf: 8_400.0,
            count: 8, // 4 pairs × 2 buildings each
        },
    ],
    cost_per_sf_gla: 260.0,
    pairs: true,
    cost_source: "v1 Deliverable ###11 (2026-05-14)",
};

pub const RETAIL_SELECT: DevClassV2 = DevClassV2 {
    label: "Retail Select",
    variants: &[
        BuildingVariant {
            label: "Medium 6,700 sf (reference)",
            distribution: "most",
            floors: 1,
            floor_plate_sf: 6_700.0,
            count: 8,
        },
        BuildingVariant {
            label: "Small 4,500 sf",
            distribution: "some",
            floors: 1,
            floor_plate_sf: 4_500.0,
            count: 4,
        },
        BuildingVariant {
            label: "Large 7,700 sf",
            distribution: "few",
            floors: 1,
            floor_plate_sf: 7_700.0,
            count: 3,
        },
    ],
    cost_per_sf_gla: 260.0,
    pairs: false,
    cost_source: "v1 Deliverable ###11 (2026-05-14)",
};

pub const ALL_CLASSES: &[&DevClassV2] = &[
    &SUBURBAN_OFFICE,
    &PROFESSIONAL_CENTRES,
    &TECH_INDUSTRIAL,
    &RETAIL_SELECT,
];

// ─── ID prefix per class (used in per-building breakout) ────────────────────

fn class_id_prefix(label: &str) -> &'static str {
    match label {
        "Suburban Office" => "SO",
        "Professional Centres" => "PC",
        "Tech Industrial" => "TI",
        "Retail Select" => "RS",
        _ => "XX",
    }
}

// ─── Formatting helpers ─────────────────────────────────────────────────────

fn fmt_money_m(v: f64) -> String {
    format!("${:.2}M", v / 1_000_000.0)
}

fn fmt_money_m1(v: f64) -> String {
    format!("${:.1}M", v / 1_000_000.0)
}

fn fmt_money_m_yr(v: f64) -> String {
    format!("${:.2}M/yr", v / 1_000_000.0)
}

fn fmt_sqft(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(*b as char);
    }
    format!("{out} sqft")
}

fn fmt_int(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

fn fmt_pct(v: f64) -> String {
    format!("{:.1}%", v * 100.0)
}

// ─── HTML renderer ──────────────────────────────────────────────────────────

pub fn render_html() -> String {
    let mut s = String::new();
    s.push_str(HEAD);
    s.push_str("<body>\n");
    s.push_str(
        "<h1>D1 Development Classes — Calibrated to 10.5% Dev Yield + 6.25% Cap Rate</h1>\n",
    );
    s.push_str("<p>PCLP 1 building portfolio per prior-work deliverables 2026-05-14 (<code>inputs/pclp1-portfolio-summary-v1.html</code> + <code>inputs/pclp1-building-class-proformas-v1.html</code>). Engine v3 supersedes the earlier <code>d1-dev-classes-2026-06-03-v2.html</code>.</p>\n");
    s.push_str("<p>All amounts CAD. Forward-looking projections; planned / intended values per BCSC continuous-disclosure posture.</p>\n");

    s.push_str(&render_portfolio_allocation());
    s.push_str(&render_distribution_tables());
    s.push_str(&render_reference_proformas());
    s.push_str(&render_per_building_breakdown());
    s.push_str(&render_assumptions_and_disclaimer());

    s.push_str("</body></html>\n");
    s
}

// ─── Section 2 — Portfolio Allocation ───────────────────────────────────────

fn render_portfolio_allocation() -> String {
    let mut s = String::new();
    s.push_str("<h2>Portfolio Allocation Summary</h2>\n");
    s.push_str("<p class=\"note\">Four Development Classes. Geometric distribution — fewer total buildings than a uniform floor-count assumption, variety preserved across all classes. Floor plates: SO = 19,000 sqft/floor; PC = 21,000 sqft/floor.</p>\n");

    s.push_str("<table>\n");
    s.push_str("<tr><th>Development Class</th><th>Buildings</th><th>GLA (sqft)</th><th>% of Portfolio</th><th>Cost/sqft</th><th>Total Cost</th><th>Rent (10.5%)</th><th>NOI (57%)</th><th>Asset Value (6.25%)</th><th>Depreciation (40yr)</th></tr>\n");

    let total_gla: f64 = ALL_CLASSES.iter().map(|c| c.class_gla()).sum();
    let total_cost: f64 = ALL_CLASSES.iter().map(|c| c.rollup().class_cost).sum();
    let total_rent: f64 = ALL_CLASSES.iter().map(|c| c.rollup().class_rent).sum();
    let total_noi: f64 = ALL_CLASSES.iter().map(|c| c.rollup().class_noi).sum();
    let total_av: f64 = ALL_CLASSES
        .iter()
        .map(|c| c.rollup().class_asset_value)
        .sum();
    let total_depr: f64 = ALL_CLASSES
        .iter()
        .map(|c| c.rollup().class_depreciation)
        .sum();
    let total_count: u32 = ALL_CLASSES.iter().map(|c| c.building_count()).sum();

    for class in ALL_CLASSES {
        let r = class.rollup();
        let bldg_label = if class.pairs {
            format!("{} pairs ({} bldg)", r.building_count / 2, r.building_count)
        } else {
            format!("{}", r.building_count)
        };
        s.push_str(&format!(
            "<tr><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">${:.0}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
            class.label,
            bldg_label,
            fmt_int(r.class_gla),
            fmt_pct(r.class_gla / total_gla),
            class.cost_per_sf_gla,
            fmt_money_m1(r.class_cost),
            fmt_money_m_yr(r.class_rent),
            fmt_money_m_yr(r.class_noi),
            fmt_money_m1(r.class_asset_value),
            fmt_money_m_yr(r.class_depreciation),
        ));
    }
    s.push_str(&format!(
        "<tr class=\"total\"><td>Total Portfolio</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">100%</td><td class=\"r\">—</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
        total_count,
        fmt_int(total_gla),
        fmt_money_m1(total_cost),
        fmt_money_m_yr(total_rent),
        fmt_money_m_yr(total_noi),
        fmt_money_m1(total_av),
        fmt_money_m_yr(total_depr),
    ));
    s.push_str("</table>\n");

    s.push_str(&format!(
        "<p class=\"note\">Portfolio GLA: {} sqft (99.7% of 3,906,855 sqft base-case target). Construction cost does not scale linearly with GLA — revenue and balance sheet computations use total construction cost by class.</p>\n",
        fmt_int(total_gla)
    ));
    s
}

// ─── Section 3 — Per-class floor/size distribution ──────────────────────────

fn render_distribution_tables() -> String {
    let mut s = String::new();
    for class in ALL_CLASSES {
        s.push_str(&render_distribution_for_class(class));
    }
    s
}

fn render_distribution_for_class(class: &DevClassV2) -> String {
    let mut s = String::new();

    let heading = if class.pairs {
        format!(
            "{} — Pair Distribution ({} Pairs · {} Buildings)",
            class.label,
            class.building_count() / 2,
            class.building_count()
        )
    } else {
        format!(
            "{} — Distribution ({} Buildings)",
            class.label,
            class.building_count()
        )
    };
    s.push_str(&format!("<h2>{}</h2>\n", heading));

    let note = match class.label {
        "Suburban Office" => "Floor plate: 19,000 sqft per floor. Geometric distribution: most 7-floor, many 6-floor, fewer 8-floor, some 9-floor.",
        "Professional Centres" => "Floor plate: 21,000 sqft per floor. Geometric distribution: quite a few 5-floor, some 4-floor, barely 3-floor.",
        "Tech Industrial" => "Single-storey paired buildings. Always constructed and leased in pairs. Medium = 2 × 7,200 sqft; Large = 2 × 8,400 sqft. Construction cost: $260/sqft.",
        "Retail Select" => "Single-storey retail pads. Three size categories. Construction cost: $260/sqft.",
        _ => "",
    };
    s.push_str(&format!("<p class=\"note\">{}</p>\n", note));

    if class.pairs {
        s.push_str("<table>\n");
        s.push_str("<tr><th>Pair Type</th><th>Distribution</th><th>Pairs</th><th>Buildings</th><th>Sqft/Pair</th><th>GLA (sqft)</th></tr>\n");
        for v in class.variants {
            let pairs = v.count / 2;
            let pair_label = if v.label.contains("Medium") {
                "Medium (2 × 7,200 sqft)"
            } else {
                "Large (2 × 8,400 sqft)"
            };
            let pair_sqft = v.floor_plate_sf * 2.0;
            s.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
                pair_label,
                v.distribution,
                pairs,
                v.count,
                fmt_int(pair_sqft),
                fmt_int(v.class_gla_contribution()),
            ));
        }
        let avg_pair = class.class_gla() / (class.building_count() / 2) as f64;
        s.push_str(&format!(
            "<tr class=\"total\"><td>Total</td><td>—</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">avg {}</td><td class=\"r\">{}</td></tr>\n",
            class.building_count() / 2,
            class.building_count(),
            fmt_int(avg_pair),
            fmt_int(class.class_gla()),
        ));
        s.push_str("</table>\n");
    } else {
        let col1 = if class.label == "Retail Select" {
            "Building Size"
        } else {
            "Floor Count"
        };
        s.push_str("<table>\n");
        s.push_str(&format!(
            "<tr><th>{}</th><th>Distribution</th><th>Buildings</th><th>Sqft/Building</th><th>GLA (sqft)</th></tr>\n",
            col1
        ));
        for v in class.variants {
            let row_label = if class.label == "Retail Select" {
                format!("{} sqft", fmt_int(v.floor_plate_sf))
            } else {
                format!("{} floors", v.floors)
            };
            s.push_str(&format!(
                "<tr><td>{}</td><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
                row_label,
                v.distribution,
                v.count,
                fmt_int(v.gla_per_building()),
                fmt_int(v.class_gla_contribution()),
            ));
        }
        let avg_bldg = class.class_gla() / class.building_count() as f64;
        s.push_str(&format!(
            "<tr class=\"total\"><td>Total</td><td>—</td><td class=\"r\">{}</td><td class=\"r\">avg {}</td><td class=\"r\">{}</td></tr>\n",
            class.building_count(),
            fmt_int(avg_bldg),
            fmt_int(class.class_gla()),
        ));
        s.push_str("</table>\n");
    }
    s
}

// ─── Section 4 — Reference proformas by class ───────────────────────────────

fn render_reference_proformas() -> String {
    let mut s = String::new();
    s.push_str("<h2>Reference Proformas by Development Class</h2>\n");
    s.push_str("<p class=\"note\">Each proforma uses a reference building — the most common floor count or size within that class. Amounts per building unless noted. All figures are forward-looking management estimates prepared for internal planning purposes.</p>\n");
    for class in ALL_CLASSES {
        s.push_str(&render_reference_proforma_for_class(class));
    }
    s
}

fn render_reference_proforma_for_class(class: &DevClassV2) -> String {
    let mut s = String::new();
    s.push_str(&format!("<h3>{}</h3>\n", class.label));

    let stmt_note = build_class_note(class);
    s.push_str(&format!("<p class=\"note\">{}</p>\n", stmt_note));

    let n_variants = class.variants.len();
    s.push_str("<table>\n");
    let _ = n_variants;

    // Header row
    s.push_str("<tr><th>Item</th>");
    for v in class.variants {
        let header = if class.pairs {
            if v.label.contains("Medium") {
                "Medium Pair — Reference (2 × 7,200 sqft)".to_string()
            } else {
                "Large Pair (2 × 8,400 sqft)".to_string()
            }
        } else if class.label == "Retail Select" {
            let prefix = if v.label.contains("(reference)") {
                "Medium — Reference"
            } else if v.label.contains("Small") {
                "Small"
            } else {
                "Large"
            };
            format!("{} ({} sqft)", prefix, fmt_int(v.floor_plate_sf))
        } else {
            let total_sqft = v.gla_per_building();
            let label = if v.label.contains("(reference)") {
                format!("{}F — Reference", v.floors)
            } else {
                format!("{}F", v.floors)
            };
            format!("{} ({} sqft)", label, fmt_int(total_sqft))
        };
        s.push_str(&format!("<th>{}</th>", header));
    }
    s.push_str("</tr>\n");

    let span = class.variants.len() + 1;

    // Construction group
    s.push_str(&format!(
        "<tr><td class=\"grp\" colspan=\"{}\">Construction</td></tr>\n",
        span
    ));
    s.push_str(&format!(
        "<tr><td>Gross leasable area{}</td>",
        if class.pairs { " (pair)" } else { "" }
    ));
    for v in class.variants {
        let gla = if class.pairs {
            v.floor_plate_sf * 2.0
        } else {
            v.gla_per_building()
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_sqft(gla)));
    }
    s.push_str("</tr>\n");

    s.push_str(&format!(
        "<tr><td>Construction cost (${:.0}/sqft)</td>",
        class.cost_per_sf_gla
    ));
    for v in class.variants {
        let cost = if class.pairs {
            v.cost_per_building(class.cost_per_sf_gla) * 2.0
        } else {
            v.cost_per_building(class.cost_per_sf_gla)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_money_m1(cost)));
    }
    s.push_str("</tr>\n");

    // Revenue & Yield group
    s.push_str(&format!(
        "<tr><td class=\"grp\" colspan=\"{}\">Revenue &amp; Yield</td></tr>\n",
        span
    ));
    s.push_str("<tr><td>Calibrated base rent (10.5% devYield)</td>");
    for v in class.variants {
        let rent = if class.pairs {
            v.rent_per_building(class.cost_per_sf_gla) * 2.0
        } else {
            v.rent_per_building(class.cost_per_sf_gla)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_money_m_yr(rent)));
    }
    s.push_str("</tr>\n");

    s.push_str("<tr><td>Net operating income (calibrated)</td>");
    for v in class.variants {
        let noi = if class.pairs {
            v.noi_per_building(class.cost_per_sf_gla) * 2.0
        } else {
            v.noi_per_building(class.cost_per_sf_gla)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_money_m_yr(noi)));
    }
    s.push_str("</tr>\n");

    s.push_str("<tr><td>Development yield target</td>");
    for _ in class.variants {
        s.push_str("<td class=\"r\">10.5%</td>");
    }
    s.push_str("</tr>\n");

    // Valuation group
    s.push_str(&format!(
        "<tr><td class=\"grp\" colspan=\"{}\">Valuation</td></tr>\n",
        span
    ));
    s.push_str("<tr><td>Asset value at 6.25% cap rate</td>");
    for v in class.variants {
        let av = if class.pairs {
            v.asset_value_per_building(class.cost_per_sf_gla) * 2.0
        } else {
            v.asset_value_per_building(class.cost_per_sf_gla)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_money_m1(av)));
    }
    s.push_str("</tr>\n");

    s.push_str("<tr><td>Annual depreciation (40-yr SL)</td>");
    for v in class.variants {
        let depr = if class.pairs {
            v.depreciation_per_building(class.cost_per_sf_gla) * 2.0
        } else {
            v.depreciation_per_building(class.cost_per_sf_gla)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", fmt_money_m_yr(depr)));
    }
    s.push_str("</tr>\n");

    // Portfolio count
    s.push_str("<tr class=\"total\"><td>Portfolio count</td>");
    for v in class.variants {
        let cell = if class.pairs {
            format!("{} pairs ({} buildings)", v.count / 2, v.count)
        } else {
            format!("{} buildings", v.count)
        };
        s.push_str(&format!("<td class=\"r\">{}</td>", cell));
    }
    s.push_str("</tr>\n");

    s.push_str("</table>\n");
    s
}

fn build_class_note(class: &DevClassV2) -> String {
    let total_bldgs = class.building_count();
    let breakdown: Vec<String> = class
        .variants
        .iter()
        .map(|v| {
            if class.pairs {
                format!(
                    "{}-pair × {}",
                    if v.label.contains("Medium") {
                        "Med"
                    } else {
                        "Lg"
                    },
                    v.count / 2
                )
            } else if class.label == "Retail Select" {
                format!("{} sqft × {}", fmt_int(v.floor_plate_sf), v.count)
            } else {
                format!("{}F × {}", v.floors, v.count)
            }
        })
        .collect();
    let breakdown_s = breakdown.join(", ");

    let reference = class
        .variants
        .iter()
        .find(|v| v.label.contains("(reference)"))
        .copied();

    match (class.label, reference) {
        ("Tech Industrial", _) => format!(
            "{} pairs · {} buildings total ({}). Single-storey. Always constructed and leased in pairs. Construction cost: $260/sqft.",
            total_bldgs / 2,
            total_bldgs,
            breakdown_s,
        ),
        ("Retail Select", _) => format!(
            "{} buildings total ({}). Single-storey retail pads. Construction cost: $260/sqft.",
            total_bldgs, breakdown_s,
        ),
        (_, Some(r)) => format!(
            "{} buildings total ({}). Reference building: {}-floor, {} sqft. Floor plate: {} sqft/floor.",
            total_bldgs,
            breakdown_s,
            r.floors,
            fmt_int(r.gla_per_building()),
            fmt_int(r.floor_plate_sf),
        ),
        (_, None) => format!("{} buildings total ({}).", total_bldgs, breakdown_s),
    }
}

// ─── Section 5 — Per-Building Financial Breakdown (70 rows) ─────────────────

fn render_per_building_breakdown() -> String {
    let mut s = String::new();
    s.push_str("<h2>Per-Building Financial Breakdown</h2>\n");
    s.push_str("<p class=\"note\">Every building in the portfolio (70 total) shown as its own row. Class subtotals reconcile to the Portfolio Allocation Summary above; portfolio grand total at the bottom. Building IDs follow class table order: tallest first for office classes; reference (most common) first for TI/RS.</p>\n");

    let mut portfolio_gla = 0.0;
    let mut portfolio_cost = 0.0;
    let mut portfolio_rent = 0.0;
    let mut portfolio_noi = 0.0;
    let mut portfolio_av = 0.0;
    let mut portfolio_depr = 0.0;

    for class in ALL_CLASSES {
        s.push_str(&render_per_building_table_for_class(class));
        let r = class.rollup();
        portfolio_gla += r.class_gla;
        portfolio_cost += r.class_cost;
        portfolio_rent += r.class_rent;
        portfolio_noi += r.class_noi;
        portfolio_av += r.class_asset_value;
        portfolio_depr += r.class_depreciation;
    }

    s.push_str("<h3>Portfolio Grand Total</h3>\n");
    s.push_str("<table>\n");
    s.push_str("<tr><th>Metric</th><th>Value</th></tr>\n");
    s.push_str(&format!(
        "<tr><td>Total buildings</td><td class=\"r\">{}</td></tr>\n",
        70
    ));
    s.push_str(&format!(
        "<tr><td>Total GLA</td><td class=\"r\">{} sqft</td></tr>\n",
        fmt_int(portfolio_gla)
    ));
    s.push_str(&format!(
        "<tr><td>Total construction cost</td><td class=\"r\">{}</td></tr>\n",
        fmt_money_m(portfolio_cost)
    ));
    s.push_str(&format!(
        "<tr><td>Total calibrated base rent</td><td class=\"r\">{}</td></tr>\n",
        fmt_money_m_yr(portfolio_rent)
    ));
    s.push_str(&format!(
        "<tr><td>Total NOI (57%)</td><td class=\"r\">{}</td></tr>\n",
        fmt_money_m_yr(portfolio_noi)
    ));
    s.push_str(&format!(
        "<tr class=\"total\"><td>Total asset value at 6.25% cap</td><td class=\"r\">{}</td></tr>\n",
        fmt_money_m(portfolio_av)
    ));
    s.push_str(&format!(
        "<tr><td>Total annual depreciation</td><td class=\"r\">{}</td></tr>\n",
        fmt_money_m_yr(portfolio_depr)
    ));
    s.push_str("</table>\n");

    s
}

fn render_per_building_table_for_class(class: &DevClassV2) -> String {
    let mut s = String::new();
    let prefix = class_id_prefix(class.label);
    s.push_str(&format!(
        "<h3>{} — Per-Building ({} buildings)</h3>\n",
        class.label,
        class.building_count(),
    ));

    s.push_str("<table>\n");
    s.push_str("<tr><th>Row</th><th>Building ID</th><th>Variant</th><th>GLA (sqft)</th><th>Construction Cost</th><th>Rent (10.5%)</th><th>NOI (57%)</th><th>Asset Value (6.25%)</th><th>Depreciation (40yr)</th></tr>\n");

    let mut row = 0u32;
    let mut class_gla = 0.0;
    let mut class_cost = 0.0;
    let mut class_rent = 0.0;
    let mut class_noi = 0.0;
    let mut class_av = 0.0;
    let mut class_depr = 0.0;

    for v in class.variants {
        // For paired classes, IDs use pair-letter suffix (1A, 1B, 2A, 2B…). For non-pair, plain 01.
        if class.pairs {
            // Determine starting pair index for this variant: number of pairs in earlier variants
            let earlier_pairs: u32 = class
                .variants
                .iter()
                .take_while(|x| x.label != v.label)
                .map(|x| x.count / 2)
                .sum();
            let pairs_in_variant = v.count / 2;
            for pi in 0..pairs_in_variant {
                let pair_num = earlier_pairs + pi + 1;
                for letter in ['A', 'B'] {
                    row += 1;
                    let id = format!("{}-{:02}{}", prefix, pair_num, letter);
                    let gla = v.gla_per_building();
                    let cost = v.cost_per_building(class.cost_per_sf_gla);
                    let rent = v.rent_per_building(class.cost_per_sf_gla);
                    let noi = v.noi_per_building(class.cost_per_sf_gla);
                    let av = v.asset_value_per_building(class.cost_per_sf_gla);
                    let depr = v.depreciation_per_building(class.cost_per_sf_gla);
                    s.push_str(&format!(
                        "<tr><td class=\"r\">{}</td><td>{}</td><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
                        row, id, v.label,
                        fmt_int(gla),
                        fmt_money_m1(cost),
                        fmt_money_m_yr(rent),
                        fmt_money_m_yr(noi),
                        fmt_money_m1(av),
                        fmt_money_m_yr(depr),
                    ));
                    class_gla += gla;
                    class_cost += cost;
                    class_rent += rent;
                    class_noi += noi;
                    class_av += av;
                    class_depr += depr;
                }
            }
        } else {
            for _ in 0..v.count {
                row += 1;
                let id = format!("{}-{:02}", prefix, row);
                let gla = v.gla_per_building();
                let cost = v.cost_per_building(class.cost_per_sf_gla);
                let rent = v.rent_per_building(class.cost_per_sf_gla);
                let noi = v.noi_per_building(class.cost_per_sf_gla);
                let av = v.asset_value_per_building(class.cost_per_sf_gla);
                let depr = v.depreciation_per_building(class.cost_per_sf_gla);
                s.push_str(&format!(
                    "<tr><td class=\"r\">{}</td><td>{}</td><td>{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
                    row, id, v.label,
                    fmt_int(gla),
                    fmt_money_m1(cost),
                    fmt_money_m_yr(rent),
                    fmt_money_m_yr(noi),
                    fmt_money_m1(av),
                    fmt_money_m_yr(depr),
                ));
                class_gla += gla;
                class_cost += cost;
                class_rent += rent;
                class_noi += noi;
                class_av += av;
                class_depr += depr;
            }
        }
    }

    s.push_str(&format!(
        "<tr class=\"subtotal\"><td colspan=\"3\">{} subtotal — {} buildings</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td><td class=\"r\">{}</td></tr>\n",
        class.label,
        class.building_count(),
        fmt_int(class_gla),
        fmt_money_m1(class_cost),
        fmt_money_m_yr(class_rent),
        fmt_money_m_yr(class_noi),
        fmt_money_m1(class_av),
        fmt_money_m_yr(class_depr),
    ));
    s.push_str("</table>\n");
    s
}

// ─── Section 6 — Proforma assumptions + disclaimer ──────────────────────────

fn render_assumptions_and_disclaimer() -> String {
    let mut s = String::new();
    s.push_str("<h2>Proforma Assumptions</h2>\n");
    s.push_str("<p>Construction cost per Development Class: Professional Centres and Suburban Office at $310/sqft; Tech Industrial and Retail Select at $260/sqft. All construction costs are capitalized as WIP during the construction phase and transferred to investment property on completion. Development yield = net operating income ÷ total construction cost. Net operating income = calibrated base rent × 57% (43% operating cost ratio applied uniformly across classes). Asset value = NOI ÷ 6.25% capitalization rate. Depreciation = straight-line over 40 years on building component (land excluded). All figures are forward-looking management estimates; actual results will depend on market conditions, leasing outcomes, and construction costs at specific Test Sites.</p>\n");
    s.push_str("<p><strong>Floor plate assumptions:</strong> Suburban Office = 19,000 sqft per floor · Professional Centres = 21,000 sqft per floor · Tech Industrial = single-storey (7,200 or 8,400 sqft per building) · Retail Select = single-storey (4,500 / 6,700 / 7,700 sqft per building).</p>\n");
    s.push_str("<p class=\"footer\"><strong>Forward-Looking Information — Notice under applicable securities legislation including the British Columbia Securities Commission (BCSC) and NI 51-102.</strong> This document contains forward-looking information within the meaning of applicable securities legislation. All per-building cost estimates, revenue projections, NOI figures, asset valuations, depreciation schedules, and development yield targets are management estimates based on planning assumptions as of the date of this document and are subject to material change. Actual results may differ materially from those projected. This document is prepared for internal planning purposes and does not constitute an offering memorandum, financial advice, or an offer to sell or solicitation to buy any security. Readers should not place undue reliance on forward-looking information.</p>\n");
    s
}

// ─── HTML head (CSS) ────────────────────────────────────────────────────────

const HEAD: &str = r#"<!DOCTYPE html><html lang="en"><head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>D1 Development Classes — v3 (v1-aligned PCLP 1 portfolio)</title>
<style>
body{font-family:system-ui,sans-serif;font-size:13px;margin:2rem;color:#111;max-width:1280px}
h1{font-size:1.3rem;margin-bottom:0.25rem}
h2{font-size:1rem;margin-top:1.5rem;margin-bottom:0.3rem;border-bottom:1px solid #ccc;padding-bottom:2px}
h3{font-size:0.9rem;margin-top:1rem;margin-bottom:0.2rem;color:#333}
p{margin:0.3rem 0;font-size:0.82rem;color:#444}
p.note{font-size:0.78rem;color:#555;font-style:italic}
table{border-collapse:collapse;margin:0.5rem 0 1rem;font-size:0.78rem;width:100%}
th,td{border:1px solid #ccc;padding:4px 8px;text-align:left}
th{background:#f5f5f5;text-align:center;font-weight:600}
td.r{text-align:right;white-space:nowrap}
td.grp{background:#f0f0f0;font-weight:600;font-size:0.75rem;text-transform:uppercase;letter-spacing:.3px;color:#444}
tr.total td{background:#eef2f7;font-weight:700;border-top:2px solid #888}
tr.subtotal td{background:#f5f7fa;font-weight:600;border-top:1px solid #aaa}
.footer{font-size:0.72rem;color:#666;margin-top:1.5rem;border-top:1px solid #ddd;padding-top:0.5rem}
@media print{body{margin:0;font-size:11px;max-width:none}@page{size:letter landscape;margin:1.5cm}}
</style></head>
"#;

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn so_portfolio_matches_v1_19_buildings_2_546_000_sf() {
        let r = SUBURBAN_OFFICE.rollup();
        assert_eq!(r.building_count, 19);
        assert_eq!(r.class_gla as i64, 2_546_000);
    }

    #[test]
    fn pc_portfolio_matches_v1_12_buildings_1_071_000_sf() {
        let r = PROFESSIONAL_CENTRES.rollup();
        assert_eq!(r.building_count, 12);
        assert_eq!(r.class_gla as i64, 1_071_000);
    }

    #[test]
    fn ti_portfolio_matches_v1_24_buildings_12_pairs_182_400_sf() {
        let r = TECH_INDUSTRIAL.rollup();
        assert_eq!(r.building_count, 24);
        assert_eq!(r.class_gla as i64, 182_400);
    }

    #[test]
    fn rs_portfolio_matches_v1_15_buildings_94_700_sf() {
        let r = RETAIL_SELECT.rollup();
        assert_eq!(r.building_count, 15);
        assert_eq!(r.class_gla as i64, 94_700);
    }

    #[test]
    fn total_portfolio_matches_v1_70_buildings_3_894_100_sf() {
        let total_count: u32 = ALL_CLASSES.iter().map(|c| c.building_count()).sum();
        let total_gla: f64 = ALL_CLASSES.iter().map(|c| c.class_gla()).sum();
        assert_eq!(total_count, 70);
        assert_eq!(total_gla as i64, 3_894_100);
    }

    #[test]
    fn pc_4f_reference_matches_v1_cost_26_0m_rent_2_73m_noi_1_56m_av_24_9m() {
        let pc = &PROFESSIONAL_CENTRES;
        let v_4f = pc.variants.iter().find(|v| v.floors == 4).unwrap();
        let cost = v_4f.cost_per_building(pc.cost_per_sf_gla);
        let rent = v_4f.rent_per_building(pc.cost_per_sf_gla);
        let noi = v_4f.noi_per_building(pc.cost_per_sf_gla);
        let av = v_4f.asset_value_per_building(pc.cost_per_sf_gla);

        // v1 deliverable: GLA 84,000 sf · cost $26.04M · rent $2.73M · NOI $1.56M · AV $24.9M
        assert_eq!(v_4f.gla_per_building() as i64, 84_000);
        assert!((cost - 26_040_000.0).abs() < 1.0);
        assert!((rent - 2_734_200.0).abs() < 1.0);
        assert!((noi - 1_558_494.0).abs() < 1.0);
        // Asset value rounded to $24.9M in v1; engine emits $24,935,904
        assert!((av - 24_935_904.0).abs() < 1.0);
    }

    #[test]
    fn so_7f_reference_matches_v1_cost_41_2m_rent_4_33m_noi_2_47m_av_39_5m() {
        let so = &SUBURBAN_OFFICE;
        let v_7f = so.variants.iter().find(|v| v.floors == 7).unwrap();
        let cost = v_7f.cost_per_building(so.cost_per_sf_gla);
        let rent = v_7f.rent_per_building(so.cost_per_sf_gla);
        let noi = v_7f.noi_per_building(so.cost_per_sf_gla);
        let av = v_7f.asset_value_per_building(so.cost_per_sf_gla);

        // v1 deliverable: GLA 133,000 sf · cost $41.2M · rent $4.33M · NOI $2.47M · AV $39.5M
        assert_eq!(v_7f.gla_per_building() as i64, 133_000);
        assert!((cost - 41_230_000.0).abs() < 1.0);
        assert!((rent - 4_329_150.0).abs() < 1.0);
        assert!((noi - 2_467_615.5).abs() < 1.0);
        assert!((av - 39_481_848.0).abs() < 1.0);
    }

    #[test]
    fn per_building_sums_equal_class_subtotals_within_1_dollar() {
        for class in ALL_CLASSES {
            let r = class.rollup();
            let mut sum_cost = 0.0;
            let mut sum_rent = 0.0;
            let mut sum_noi = 0.0;
            let mut sum_av = 0.0;
            let mut sum_depr = 0.0;
            let mut sum_gla = 0.0;
            for v in class.variants {
                let n = v.count as f64;
                sum_gla += v.gla_per_building() * n;
                sum_cost += v.cost_per_building(class.cost_per_sf_gla) * n;
                sum_rent += v.rent_per_building(class.cost_per_sf_gla) * n;
                sum_noi += v.noi_per_building(class.cost_per_sf_gla) * n;
                sum_av += v.asset_value_per_building(class.cost_per_sf_gla) * n;
                sum_depr += v.depreciation_per_building(class.cost_per_sf_gla) * n;
            }
            assert!(
                (sum_gla - r.class_gla).abs() < 1.0,
                "{}: gla mismatch",
                class.label
            );
            assert!(
                (sum_cost - r.class_cost).abs() < 1.0,
                "{}: cost mismatch",
                class.label
            );
            assert!(
                (sum_rent - r.class_rent).abs() < 1.0,
                "{}: rent mismatch",
                class.label
            );
            assert!(
                (sum_noi - r.class_noi).abs() < 1.0,
                "{}: noi mismatch",
                class.label
            );
            assert!(
                (sum_av - r.class_asset_value).abs() < 1.0,
                "{}: av mismatch",
                class.label
            );
            assert!(
                (sum_depr - r.class_depreciation).abs() < 1.0,
                "{}: depr mismatch",
                class.label
            );
        }
    }
}
