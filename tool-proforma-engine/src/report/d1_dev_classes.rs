use crate::excel::titleco::TitlecoProforma;

/// A development class definition parameterized from TitleCo base assumptions.
pub struct DevClass {
    pub label: &'static str,
    pub floors: u32,
    pub office_sqft_per_floor: f64,
    pub retail_sqft: f64,
    pub storeys: u32, // for single-storey types
}

/// Standard development classes from the brief.
pub const DEV_CLASSES: &[DevClass] = &[
    DevClass {
        label: "Professional Centres",
        floors: 3,
        office_sqft_per_floor: 21_000.0,
        retail_sqft: 10_600.0,
        storeys: 3,
    },
    DevClass {
        label: "Suburban Office",
        floors: 3,
        office_sqft_per_floor: 19_000.0,
        retail_sqft: 0.0,
        storeys: 3,
    },
    DevClass {
        label: "Tech Industrial",
        floors: 1,
        office_sqft_per_floor: 7_800.0, // average of 7,200 and 8,400 (pair)
        retail_sqft: 0.0,
        storeys: 1,
    },
    DevClass {
        label: "Retail Select",
        floors: 1,
        office_sqft_per_floor: 6_300.0, // average of 4,500/6,700/7,700
        retail_sqft: 0.0,
        storeys: 1,
    },
];

/// Per-year output for a single development class.
#[derive(Debug)]
pub struct DevYear {
    pub year: u32,
    // Income Statement
    pub revenue: f64,
    pub operating_costs: f64,
    pub noi: f64,
    pub interest: f64,
    pub ebt: f64,
    // Balance Sheet
    pub total_assets: f64,
    pub debt: f64,
    pub equity: f64,
    // Cash Flow
    pub cf_from_operations: f64,
    pub capex: f64,
    pub debt_drawdown: f64,
    pub distributions: f64,
    pub ending_cash: f64,
}

/// 10-year proforma for one development class.
#[derive(Debug)]
pub struct DevClassOutput {
    pub label: String,
    pub total_leasable_sqft: f64,
    pub gross_floor_area_sqft: f64,
    pub total_dev_cost: f64,
    pub stabilised_noi: f64,
    pub development_yield: f64,
    pub cap_rate: f64,
    pub stabilised_asset_value: f64,
    pub years: Vec<DevYear>,
}

/// Lease-up schedule: occupancy fraction by year (Y1=construction, Y2-Y3 ramp).
const OCCUPANCY: [f64; 10] = [0.0, 0.25, 0.75, 0.95, 0.95, 0.95, 0.95, 0.95, 0.95, 0.95];
/// Debt as fraction of total dev cost (assumed LTV on cost).
const LTV_ON_COST: f64 = 0.60;
/// Assumed debt rate (matches PCLP 1 base debenture rate).
const DEBT_RATE: f64 = 0.05;
/// Operating cost ratio (fraction of effective gross revenue).
const OPEX_RATIO: f64 = 0.30;
/// Equity distribution fraction of net income after interest.
const DIST_RATIO: f64 = 0.90;

pub fn compute(class: &DevClass, base: &TitlecoProforma) -> DevClassOutput {
    // Leasable area
    let office_sqft = class.office_sqft_per_floor * class.floors as f64;
    let total_leasable = office_sqft + class.retail_sqft;

    // Gross floor area adds ~27% for structure/common areas (ratio from TitleCo base)
    let base_gfa_ratio = if base.total_sqft > 0.0 {
        80_080.0 / base.total_sqft
    } else {
        1.257
    };
    let gross_floor_area = total_leasable * base_gfa_ratio;

    // Development cost — use base construction rate per GFA sqft
    let construction = gross_floor_area * base.construction_rate;
    let contingency = construction * base.contingency_rate;
    let ti = total_leasable * base.ti_rate;
    // Scale professional fees and marketing proportionally to construction cost
    let base_hard_cost = base.construction_cost;
    let fee_ratio = if base_hard_cost > 0.0 {
        (base.professional_fees + base.marketing_leasing) / base_hard_cost
    } else {
        0.15
    };
    let fees = construction * fee_ratio;
    let land = base.land_cost * (gross_floor_area / 80_080.0); // scale from base
    let total_dev_cost = land + construction + contingency + ti + fees;

    // Stabilised revenue and NOI
    // Use office rate from base (first area with rate > 0)
    let office_rate = base
        .areas
        .iter()
        .find(|a| a.label.contains("Office") && a.rate_per_sqft > 0.0)
        .map(|a| a.rate_per_sqft)
        .unwrap_or(38.0);
    let retail_rate = base
        .areas
        .iter()
        .find(|a| a.label.contains("Retail") && a.rate_per_sqft > 0.0)
        .map(|a| a.rate_per_sqft)
        .unwrap_or(45.0);

    let stabilised_gross_rev = office_sqft * office_rate + class.retail_sqft * retail_rate;
    let stabilised_noi = stabilised_gross_rev * (1.0 - OPEX_RATIO);
    let cap_rate = base.net_initial_yield;
    let stabilised_av = if cap_rate > 0.0 {
        stabilised_noi / cap_rate
    } else {
        0.0
    };
    let development_yield = if total_dev_cost > 0.0 {
        stabilised_noi / total_dev_cost
    } else {
        0.0
    };

    // Debt and equity
    let debt = total_dev_cost * LTV_ON_COST;
    let _equity_invested = total_dev_cost - debt;

    let mut years = Vec::with_capacity(10);
    let mut ending_cash: f64 = 0.0;

    for (i, &occ) in OCCUPANCY.iter().enumerate() {
        let year = i as u32 + 1;

        let gross_rev = stabilised_gross_rev * occ;
        let opex = gross_rev * OPEX_RATIO;
        let noi = gross_rev - opex;
        let interest = if i >= 1 { debt * DEBT_RATE } else { 0.0 };
        let ebt = noi - interest;

        // Balance sheet
        let wip = if i == 0 { total_dev_cost } else { 0.0 };
        let asset_value = if occ > 0.0 && cap_rate > 0.0 {
            noi / cap_rate
        } else {
            0.0
        };
        let total_assets = asset_value + wip + ending_cash;
        let equity = (total_assets - debt).max(0.0);

        // Cash flow
        let capex = if i == 0 { -total_dev_cost } else { 0.0 };
        let debt_drawdown = if i == 0 { debt } else { 0.0 };
        let dist = if ebt > 0.0 { ebt * DIST_RATIO } else { 0.0 };
        let cf_ops = noi - interest;
        ending_cash = (ending_cash + cf_ops - dist + debt_drawdown + capex).max(0.0);

        years.push(DevYear {
            year,
            revenue: gross_rev,
            operating_costs: opex,
            noi,
            interest,
            ebt,
            total_assets,
            debt,
            equity,
            cf_from_operations: cf_ops,
            capex,
            debt_drawdown,
            distributions: dist,
            ending_cash,
        });
    }

    DevClassOutput {
        label: class.label.to_string(),
        total_leasable_sqft: total_leasable,
        gross_floor_area_sqft: gross_floor_area,
        total_dev_cost,
        stabilised_noi,
        development_yield,
        cap_rate,
        stabilised_asset_value: stabilised_av,
        years,
    }
}

fn fmt_m(v: f64) -> String {
    if v == 0.0 {
        return "—".to_string();
    }
    let m = v / 1_000_000.0;
    if m < 0.0 {
        format!("({:.2}M)", m.abs())
    } else {
        format!("{:.2}M", m)
    }
}

fn fmt_pct(v: f64) -> String {
    format!("{:.2}%", v * 100.0)
}

fn yr_header() -> String {
    "| | Y1 | Y2 | Y3 | Y4 | Y5 | Y6 | Y7 | Y8 | Y9 | Y10 |\n".to_string()
}

fn sep() -> String {
    "|:---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n".to_string()
}

fn yr_row(label: &str, vals: Vec<f64>) -> String {
    let cells: Vec<String> = vals.iter().map(|&v| fmt_m(v)).collect();
    format!("| {} | {} |\n", label, cells.join(" | "))
}

fn render_one(out: &DevClassOutput) -> String {
    let mut s = String::new();

    s.push_str(&format!("## {}\n\n", out.label));
    s.push_str(&format!(
        "| Leasable Area | GFA | Dev Cost | Stab. NOI | Dev Yield | Cap Rate | Stab. Asset Value |\n\
         |---:|---:|---:|---:|---:|---:|---:|\n\
         | {:.0} sqft | {:.0} sqft | {} | {} | {} | {} | {} |\n\n",
        out.total_leasable_sqft,
        out.gross_floor_area_sqft,
        fmt_m(out.total_dev_cost),
        fmt_m(out.stabilised_noi),
        fmt_pct(out.development_yield),
        fmt_pct(out.cap_rate),
        fmt_m(out.stabilised_asset_value),
    ));

    // Income Statement
    s.push_str("### Income Statement\n\n");
    s.push_str(&yr_header());
    s.push_str(&sep());
    s.push_str(&yr_row(
        "Revenue",
        out.years.iter().map(|y| y.revenue).collect(),
    ));
    s.push_str(&yr_row(
        "Operating Costs",
        out.years.iter().map(|y| y.operating_costs).collect(),
    ));
    s.push_str(&yr_row(
        "**NOI**",
        out.years.iter().map(|y| y.noi).collect(),
    ));
    s.push_str(&yr_row(
        "Interest",
        out.years.iter().map(|y| y.interest).collect(),
    ));
    s.push_str(&yr_row(
        "**EBT**",
        out.years.iter().map(|y| y.ebt).collect(),
    ));

    // Balance Sheet
    s.push_str("\n### Balance Sheet\n\n");
    s.push_str(&yr_header());
    s.push_str(&sep());
    s.push_str(&yr_row(
        "Total Assets",
        out.years.iter().map(|y| y.total_assets).collect(),
    ));
    s.push_str(&yr_row("Debt", out.years.iter().map(|y| y.debt).collect()));
    s.push_str(&yr_row(
        "**Equity**",
        out.years.iter().map(|y| y.equity).collect(),
    ));

    // Cash Flow
    s.push_str("\n### Cash Flow Statement\n\n");
    s.push_str(&yr_header());
    s.push_str(&sep());
    s.push_str(&yr_row(
        "CF from Operations",
        out.years.iter().map(|y| y.cf_from_operations).collect(),
    ));
    s.push_str(&yr_row(
        "Capex",
        out.years.iter().map(|y| y.capex).collect(),
    ));
    s.push_str(&yr_row(
        "Debt Drawdown",
        out.years.iter().map(|y| y.debt_drawdown).collect(),
    ));
    s.push_str(&yr_row(
        "Distributions",
        out.years.iter().map(|y| y.distributions).collect(),
    ));
    s.push_str(&yr_row(
        "Ending Cash",
        out.years.iter().map(|y| y.ending_cash).collect(),
    ));

    s
}

pub fn render(base: &TitlecoProforma) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "# Development Classes — 10-Year Proforma\n\n**{}** — {}\n\n",
        base.entity, base.date
    ));
    out.push_str(&format!(
        "> Base assumptions from TitleCo 3: construction ${:.2}/sqft GFA, \
        office ${:.0}/sqft NLA, TI ${:.0}/sqft NLA, cap rate {:.2}%.\n\n",
        base.construction_rate,
        base.areas
            .iter()
            .find(|a| a.label.contains("Office"))
            .map(|a| a.rate_per_sqft)
            .unwrap_or(38.0),
        base.ti_rate,
        base.net_initial_yield * 100.0,
    ));

    for class in DEV_CLASSES {
        let result = compute(class, base);
        out.push_str("---\n\n");
        out.push_str(&render_one(&result));
        out.push('\n');
    }

    out
}
