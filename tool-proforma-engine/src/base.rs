/// Diluted LP unit count (includes 10% Benetti Holdings allocation).
pub const DILUTED: f64 = 2_777_777.0;

// ── Base-case sensitivity assumptions ──────────────────────────────────────
pub const BASE_OCC: f64 = 0.95;
pub const BASE_CAP_RATE: f64 = 0.0625;
pub const BASE_DEBT_RATE: f64 = 0.05;
pub const BASE_DEV_YIELD: f64 = 0.105;
pub const BASE_MARKET_YIELD: f64 = 0.08;
pub const BASE_LEASE_UP_MONTHS: u32 = 12;

// ── Fund-level operating data (Y1–Y10) ─────────────────────────────────────
// Source: 2026-01-06 Excel proforma (PCLP 1). Matches pclp1-sensitivity-v6.html BASE object.

pub const BASE_NOI: [f64; 10] = [
    0.0, 0.0, 0.0,
    22_771_875.0, 22_771_875.0,
    58_419_375.0, 58_419_375.0,
    127_168_125.0, 127_168_125.0, 127_168_125.0,
];

pub const BASE_EBITDA: [f64; 10] = [
    -20_950_000.0, -3_450_000.0, -3_450_000.0,
     8_821_875.0, 19_321_875.0,
    34_719_375.0, 54_969_375.0,
    123_718_125.0, 123_718_125.0, 123_718_125.0,
];

pub const BASE_INTEREST: [f64; 10] = [
    0.0, -78_379.0, -40_548.0,
    4_372_303.0, 13_119_456.0,
    25_929_020.0, 41_854_259.0,
    49_360_667.0, 48_988_880.0, 48_617_093.0,
];

// ── Balance sheet (Y1–Y10) ─────────────────────────────────────────────────

pub const BASE_CASH: [f64; 10] = [
    156_758_333.0, 81_095_046.0, 5_393_927.0,
    11_088_884.0,  16_959_126.0,
    27_963_161.0,   1_629_310.0,
     1_629_310.0,   1_629_310.0,  1_629_310.0,
];

pub const BASE_DEBT: [f64; 10] = [
    0.0, 0.0, 0.0,
    175_000_000.0, 350_000_000.0,
    687_500_000.0, 987_229_637.0,
    979_793_891.0, 972_320_967.0, 964_885_628.0,
];

pub const BASE_WIP: [f64; 10] = [
    72_291_667.0, 144_583_333.0, 216_875_000.0,
    169_750_000.0, 339_500_000.0,
    327_375_000.0, 654_750_000.0,
    0.0, 0.0, 0.0,
];

// ── Per-unit par values for Y1–Y3 (income continuity; not stressed) ────────
// Y1–Y3 NAV and asset value are held at proforma par regardless of assumptions.
pub const BASE_NAV_PU: [f64; 3] = [100.026, 100.252, 100.177];
pub const BASE_ASSET_PU: [f64; 3] = [100.026, 100.252, 100.177];

// ── Market value per unit: Y1–Y7 fixed proforma values ────────────────────
// Y8–Y10 are computed as dist_per_unit / market_yield (interest-rate sensitivity).
pub const BASE_MV_PU_FIXED: [f64; 7] = [100.0, 100.0, 100.0, 125.8, 132.1, 171.5, 177.3];

// ── Stabilisation-year indices (0-based: Y4=3, Y6=5, Y8=7) ───────────────
// Lease-up multiplier applies only in these years.
pub const STAB_IDX: [usize; 3] = [3, 5, 7];

// ── Preset scenario assumptions ───────────────────────────────────────────
pub mod scenarios {
    use crate::model::Assumptions;

    pub fn base() -> Assumptions {
        Assumptions {
            cap_rate: 0.0625,
            occupancy: 0.95,
            lease_up_months: 12,
            dev_yield: 0.105,
            debt_rate: 0.05,
            market_yield: 0.08,
        }
    }

    pub fn bear() -> Assumptions {
        Assumptions {
            cap_rate: 0.075,
            occupancy: 0.70,
            lease_up_months: 24,
            dev_yield: 0.105,
            debt_rate: 0.07,
            market_yield: 0.10,
        }
    }

    pub fn bull() -> Assumptions {
        Assumptions {
            cap_rate: 0.055,
            occupancy: 1.00,
            lease_up_months: 6,
            dev_yield: 0.105,
            debt_rate: 0.04,
            market_yield: 0.06,
        }
    }
}
