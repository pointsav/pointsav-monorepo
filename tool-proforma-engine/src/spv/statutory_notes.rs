// Notes to the 10-year financial forecast + Independent Practitioner's Assurance Report.
//
// The note set MERGES the legacy Big-4 ten-note skeleton with the disclosures a contemporary
// firm expects (IAS 1 → IFRS 18, IAS 40 fair value + IFRS 13 Level 3, IFRS 7 risk, IAS 24
// related parties, ISSB climate cross-reference, MPM reconciliation). Legacy note topics and
// order are preserved so a prior reader recognises the document; modern notes are inserted at
// IAS-1-logical positions.
//
// Numbers quoted in the notes are the engine constants (single source of truth). Prose is
// original drafting that describes the applicable accounting treatment; it does not reproduce
// the text of any accounting or assurance standard.

use crate::spv::pclp1_proforma::{
    Pclp1Year, PCLP1_BENETTI_UNITS, PCLP1_CAP_RATE, PCLP1_CASH_INTEREST, PCLP1_DEBT_FINANCING_COST,
    PCLP1_DEBT_RATE_DEBENTURE, PCLP1_DEV_YIELD, PCLP1_DILUTED_UNITS, PCLP1_GROSS_EQUITY,
    PCLP1_INVESTOR_UNITS, PCLP1_TOTAL_DEV_COST, PCLP1_TOTAL_PORTFOLIO_SQFT, PCLP1_UNIT_PRICE,
};
use crate::spv::statutory_forecast::{AssuranceBlock, Jurisdiction, Note};

fn dollars(v: f64) -> String {
    let n = v.round() as i64;
    let s = n.abs().to_string();
    let b = s.as_bytes();
    let mut out = String::new();
    for (i, c) in b.iter().enumerate() {
        if i > 0 && (b.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*c as char);
    }
    format!("${out}")
}

/// Build the merged note set for the jurisdiction. Canada renders the spine (1–16) plus the
/// climate cross-reference; conditional notes (regime / GAAP-reconciliation / consolidation)
/// are added in Phase B for the other entities.
pub fn notes_for(j: &Jurisdiction, years: &[Pclp1Year]) -> Vec<Note> {
    let y10 = &years[10];
    let mut n: Vec<Note> = Vec::new();
    let mut k: u8 = 0;
    let mut next = || {
        k += 1;
        k
    };

    // 1 — Reporting entity & purpose (merges legacy Notes 1 + 2 opening).
    n.push(Note {
        number: next(),
        title: "Reporting entity and purpose of the financial forecast".into(),
        body_html: format!(
            "<p>{legal} (the &ldquo;Partnership&rdquo;; &ldquo;{term}&rdquo;) is a limited \
             partnership formed under the laws of British Columbia. Its general partner is {gp}, \
             and it is managed by {mgr} The Partnership is a real-estate investment entity whose \
             objective is income, capital appreciation and long-term liquidity for its {holders}.</p>\
             <p>This 10-year financial forecast presents future-oriented financial information \
             prepared by management for the purpose of assisting prospective investors in \
             evaluating an investment in the Partnership. <strong>It may not be appropriate for \
             other purposes.</strong> The forecast covers the ten years following commencement of \
             operations and reflects management&rsquo;s assumptions as to expected conditions and \
             its intended course of action. Because it is based on assumptions about future \
             events, actual results will vary from the forecast and the variations may be \
             material.</p>",
            legal = j.legal_name, term = j.term_name, gp = j.gp_name, mgr = j.manager_name,
            holders = j.holder_term,
        ),
    });

    // 2 — Basis of preparation, IFRS compliance & FOFI basis (incl. IFRS 18 adoption). [NEW]
    n.push(Note {
        number: next(),
        title: "Basis of preparation".into(),
        body_html: format!(
            "<p>This forecast has been prepared in accordance with International Financial \
             Reporting Standards (IFRS) as the accounting policies the Partnership expects to \
             adopt in its historical financial statements, applied on a basis consistent \
             throughout the forecast period. Amounts are presented in {ccy}.</p>\
             <p><strong>Adoption of IFRS 18.</strong> IFRS 18, <em>Presentation and Disclosure in \
             Financial Statements</em>, is effective for annual periods beginning on or after \
             1 January 2027 and will govern the majority of this forecast horizon. The statement \
             of comprehensive income is accordingly presented using the IFRS 18 operating, \
             investing and financing categories, with the mandatory <em>operating profit</em> and \
             <em>profit before financing and income taxes</em> subtotals. The recognisable \
             legacy subtotal &mdash; net operating income (NOI) &mdash; is retained as a voluntary \
             subtotal. As the Partnership&rsquo;s main business activity is investing in property, \
             rental income and the operating results of investment property are presented \
             within the operating category. Investment property is carried at cost in the primary \
             statements; its fair value is disclosed separately (Note 6).</p>\
             <p>The forecast is future-oriented financial information within the meaning of \
             {reg}. The significant assumptions are set out in Note 3; the related forward-looking \
             cautionary statement appears in the notice that accompanies these statements.</p>",
            ccy = j.currency,
            reg = j.regulator,
        ),
    });

    // 3 — Significant assumptions (elevated legacy Note 10). [EVOLVED]
    n.push(Note {
        number: next(),
        title: "Significant assumptions".into(),
        body_html: format!(
            "<p>The forecast rests on the following principal assumptions, each within \
             management&rsquo;s reasonable expectation as at the date of preparation:</p>\
             <ul>\
             <li>the Partnership is able to raise capital through private placement and to acquire \
             development sites at the forecast cost of approximately $310 per square foot;</li>\
             <li>a stabilised development yield of {dy:.1}% (net of a 5% vacancy allowance) is \
             achieved across the portfolio;</li>\
             <li>investment property is valued at a capitalisation rate of {cr:.2}%;</li>\
             <li>debenture financing carries a {dr:.1}% coupon, with interest computed on the \
             average of opening and closing balances; surplus cash earns {ci:.2}%;</li>\
             <li>the interest-coverage covenant of 1.20&times; is maintained throughout the build \
             phase.</li>\
             </ul>\
             <p>Critical judgments and the key sources of estimation uncertainty arising from \
             these assumptions are described in Note 5.</p>",
            dy = PCLP1_DEV_YIELD * 100.0,
            cr = PCLP1_CAP_RATE * 100.0,
            dr = PCLP1_DEBT_RATE_DEBENTURE * 100.0,
            ci = PCLP1_CASH_INTEREST * 100.0,
        ),
    });

    // 4 — Material accounting policy information (legacy Note 3, modernized). [EVOLVED]
    n.push(Note {
        number: next(),
        title: "Material accounting policy information".into(),
        body_html: ifrs_policy_body(j),
    });

    // 5 — Critical judgments & estimation uncertainty. [NEW]
    n.push(Note {
        number: next(),
        title: "Critical accounting judgments and key sources of estimation uncertainty".into(),
        body_html: "<p>The preparation of the forecast requires management to make judgments and \
             estimates that affect the reported amounts. The judgment with the most significant \
             effect is the election of the fair-value model for investment property under IAS 40. \
             The key source of estimation uncertainty is the fair value of investment property, \
             which is a Level 3 measurement sensitive to the capitalisation rate, market rent and \
             occupancy assumptions; the quantified sensitivity is set out in Note 6. Further \
             judgment is applied in classifying partnership units as equity under IAS 32 and in \
             determining the point at which investment property under development can be measured \
             reliably at fair value.</p>"
            .to_string(),
    });

    // 6 — Fair-value measurement: IAS 40 + IFRS 13 Level 3 + sensitivity. [NEW]
    n.push(Note {
        number: next(),
        title: "Fair-value measurement of investment property (IFRS 13)".into(),
        body_html: format!(
            "<p>Investment property is carried at cost in the primary statements; this note \
             discloses its fair value, as required under the IAS 40 cost model. Fair value is \
             determined using an income capitalisation technique: stabilised net operating income \
             is capitalised at a market capitalisation rate of {cr:.2}%. The measurement uses \
             significant unobservable inputs and is categorised within Level 3 of the IFRS 13 \
             fair-value hierarchy. The significant unobservable inputs are the capitalisation rate, \
             market rent (expressed through the {dy:.1}% development yield), and stabilised \
             occupancy (95%). The resulting fair values and net asset value are set out in the \
             supplementary fair-value exhibit.</p>\
             <p><strong>Sensitivity (reasonably possible changes, IFRS 13).</strong> A 25 basis \
             point increase in the capitalisation rate reduces the fair value of investment \
             property and net asset value; a 25 basis point decrease increases them by a similar \
             magnitude. Occupancy and development-yield movements of comparable scale have a \
             smaller effect than the capitalisation rate, which is the most sensitive input. The \
             accompanying sensitivity analysis sets out the quantified effect on net asset value \
             per unit and on the minimum interest-coverage ratio across the forecast range.</p>",
            cr = PCLP1_CAP_RATE * 100.0,
            dy = PCLP1_DEV_YIELD * 100.0,
        ),
    });

    // 7 — Revenue and rental income (legacy Note 6). [EVOLVED]
    n.push(Note {
        number: next(),
        title: "Revenue and rental income".into(),
        body_html: format!(
            "<p>Revenue comprises net lease income from the Partnership&rsquo;s building \
             operations, recognised on a straight-line basis over the lease term as operating-\
             lease income of the lessor. The portfolio is developed across four building classes \
             (Professional Centre, Suburban Office, Retail Select and Tech Industrial) to a \
             stabilised area of approximately {sqft} square feet. Net operating income is \
             modelled at the stabilised development yield of {dy:.1}% applied to delivered \
             generating cost; no separate allowance for bad debts or tenant improvements is made, \
             these being reflected within the development yield.</p>",
            sqft = dollars(PCLP1_TOTAL_PORTFOLIO_SQFT).trim_start_matches('$'),
            dy = PCLP1_DEV_YIELD * 100.0,
        ),
    });

    // 8 — Investment property under development / construction (legacy Note 8). [EVOLVED]
    n.push(Note {
        number: next(),
        title: "Investment property under development".into(),
        body_html: format!(
            "<p>Development is undertaken in phases at an average construction cost of \
             approximately $310 per square foot, for a total forecast development cost of {tdc}. \
             Property under construction is carried within investment property; borrowing costs \
             directly attributable to construction are capitalised in accordance with IAS 23 \
             until the asset is ready for its intended use, at which point it is measured at fair \
             value under the policy in Note 4.</p>",
            tdc = dollars(PCLP1_TOTAL_DEV_COST),
        ),
    });

    // 9 — Operating costs (legacy Note 7). [EVOLVED — split related-party]
    n.push(Note {
        number: next(),
        title: "Operating costs".into(),
        body_html: format!(
            "<p>Operating costs comprise an advisory and management fee of 1% per annum of gross \
             funded value, a one-time placement/referral fee of 6% of gross funded value, issue \
             costs of 1% of gross funded value, and fixed administration, compliance and board \
             costs that benefit from economies of scale as the portfolio grows. The advisory and \
             referral fees are paid to {mgr} and are related-party transactions; see Note 15. No \
             amortisation or income-tax expense is recognised at the Partnership level (Note 16).</p>",
            mgr = j.manager_name,
        ),
    });

    // 10 — Long-term debt (legacy Note 4). [EVOLVED — effective interest]
    n.push(Note {
        number: next(),
        title: "Long-term debt (debentures)".into(),
        body_html: format!(
            "<p>Subsequent buildings are financed through the issuance of first secured mortgage \
             debentures bearing interest at {dr:.1}%. Debentures are measured at amortised cost \
             using the effective-interest method; the {fc:.0}% facility cost is netted against the \
             carrying amount and amortised to finance cost, so the effective rate exceeds the \
             {dr:.1}% coupon. The Partnership monitors the interest-coverage covenant of \
             1.20&times; throughout the build phase; the forecast minimum coverage and the \
             contractual repayment profile are reflected in the statements. Liquidity and \
             refinancing risk on the debentures is addressed in Note 11.</p>\
             <p>Closing debenture balance at Year 10 is forecast at {bal}.</p>",
            dr = PCLP1_DEBT_RATE_DEBENTURE * 100.0,
            fc = PCLP1_DEBT_FINANCING_COST * 100.0,
            bal = dollars(y10.closing_debt),
        ),
    });

    // 11 — Financial instruments & financial-risk management. [NEW]
    n.push(Note {
        number: next(),
        title: "Financial instruments and financial-risk management".into(),
        body_html:
            "<p>The Partnership&rsquo;s financial instruments comprise cash, lease receivables \
             and debentures. Credit risk on lease receivables is mitigated by tenant quality and \
             diversification across building classes. Liquidity risk is managed through the \
             working-capital reserve and the phased debenture programme; the maturity profile of \
             the debentures and the sensitivity of finance cost to interest rates are the \
             principal market risks. A 25 basis point change in the debenture coupon has only a \
             modest effect on net asset value but reduces the minimum interest-coverage ratio, as \
             shown in the sensitivity analysis.</p>"
                .to_string(),
    });

    // 12 — Capital management. [NEW]
    n.push(Note {
        number: next(),
        title: "Capital management".into(),
        body_html:
            "<p>The Partnership manages partners&rsquo; capital and debenture financing to fund the \
             development programme while preserving the 1.20&times; interest-coverage covenant and \
             a prudent loan-to-value ratio. Capital is deployed in phases; surplus cash is held in \
             a working-capital reserve. The forecast loan-to-value and coverage ratios are \
             presented in the per-unit and ratio schedule accompanying the statements.</p>"
                .to_string(),
    });

    // 13 — Partners' capital / units (legacy Note 5). [EVOLVED — IAS 32]
    let price = format!("${:.0}", PCLP1_UNIT_PRICE);
    n.push(Note {
        number: next(),
        title: "Partners' capital and units".into(),
        body_html: format!(
            "<p>Authorised capital is up to {diluted} {unit}. The Partnership has raised {gross} \
             through the issue of {investor} investor units at {price} per unit. A further \
             {manager} units are issued to {spv} for services as a share-based payment; these \
             units carry the same per-unit distribution entitlement as investor units but are \
             held in escrow until the Partnership has returned 100% of contributed capital or on \
             the occurrence of a defined liquidity event. Units are classified as equity under \
             IAS 32. The share-based-payment charge and the corresponding reserve are recognised \
             in equity and have no net effect on total equity.</p>",
            diluted = dollars(PCLP1_DILUTED_UNITS).trim_start_matches('$'),
            unit = j.unit_term,
            gross = dollars(PCLP1_GROSS_EQUITY),
            investor = dollars(PCLP1_INVESTOR_UNITS).trim_start_matches('$'),
            manager = dollars(PCLP1_BENETTI_UNITS).trim_start_matches('$'),
            spv = j.spv_name,
        ),
    });

    // 14 — Distributions (legacy Note 9). [EVOLVED]
    n.push(Note {
        number: next(),
        title: "Distributions".into(),
        body_html: format!(
            "<p>The Partnership intends to distribute {policy}. Thereafter, a portion of \
             distributable income is applied to the redemption of outstanding debentures, with \
             remaining amounts distributed in accordance with the partnership&rsquo;s purposes. \
             Distributable income is a cash measure and is reconciled to IFRS net income, which \
             includes non-cash fair-value remeasurement gains that are not distributable.</p>",
            policy = j.distribution_policy,
        ),
    });

    // 15 — Related-party transactions. [NEW]
    n.push(Note {
        number: next(),
        title: "Related-party transactions".into(),
        body_html: format!(
            "<p>{mgr}, the manager, and {gp}, the general partner, are related parties of the \
             Partnership. Transactions with them comprise the advisory and management fee, the \
             placement/referral fee and the general-partner arrangements described in Note 9, \
             together with the units issued to {spv} described in Note 13. All such transactions \
             are conducted on the terms set out in the limited partnership agreement.</p>",
            mgr = j.manager_name,
            gp = j.gp_name,
            spv = j.spv_name,
        ),
    });

    // 16 — Income taxes. [NEW]
    n.push(Note {
        number: next(),
        title: "Income taxes".into(),
        body_html:
            "<p>The Partnership is a flow-through entity for income-tax purposes. Income and loss \
             are allocated to, and taxable in the hands of, the partners; accordingly no current \
             or deferred income-tax expense is recognised at the Partnership level.</p>"
                .to_string(),
    });

    // 17 — Climate & sustainability cross-reference (ISSB IFRS S1/S2). [NEW]
    n.push(Note {
        number: next(),
        title: "Climate and sustainability".into(),
        body_html:
            "<p>The Partnership develops to current energy-performance standards and considers \
             climate-related physical and transition risk in its site selection, design and \
             capitalisation-rate assumptions, consistent with the direction of the ISSB Standards \
             (IFRS S1 and IFRS S2). Decarbonisation capital expenditure and physical-risk exposure \
             are reflected, where material, in the development cost and fair-value assumptions \
             underlying this forecast.</p>"
                .to_string(),
    });

    n
}

/// Material accounting policy information body, branched by framework.
fn ifrs_policy_body(j: &Jurisdiction) -> String {
    format!(
        "<p><strong>Basis of presentation.</strong> The forecast comprises the statement of \
         forecasted financial position, the statement of forecasted comprehensive income, the \
         statement of forecasted changes in equity and the statement of forecasted cash flows, \
         presented in {ccy} on the basis described in Note 2.</p>\
         <p><strong>Investment property.</strong> Investment property is measured under the IAS 40 \
         cost model: it is carried at cost and is not depreciated in this forecast. Its fair value \
         is disclosed in the supplementary fair-value and net-asset-value exhibit (Note 6). Property \
         under development is included within investment property at cost (Note 8).</p>\
         <p><strong>Equity-based compensation.</strong> Units issued to the manager-unit holder for \
         services are accounted for as a share-based payment under IFRS 2, measured at fair value \
         ($100 per unit) and expensed within operating profit, with a corresponding reserve in \
         equity (Note 13).</p>\
         <p><strong>Revenue.</strong> Operating-lease rental income is recognised on a straight-\
         line basis over the lease term (Note 7).</p>\
         <p><strong>Financial instruments.</strong> Cash and lease receivables are measured at \
         amortised cost; debentures are measured at amortised cost using the effective-interest \
         method, with directly attributable issuance costs included in the effective rate \
         (Note 10).</p>\
         <p><strong>Partnership units.</strong> Units are classified as equity under IAS 32; \
         units issued for services are accounted for as share-based payments (Note 13).</p>",
        ccy = j.currency,
    )
}

/// Independent Practitioner's Assurance Report on the forecast (examination model).
///
/// Original drafting describing the structure of an examination engagement over future-oriented
/// financial information: negative assurance that the assumptions provide a reasonable basis,
/// an opinion that the forecast is properly prepared on that basis and presented in accordance
/// with the framework, and the mandatory caution that actual results will vary. It deliberately
/// does not state or imply that the forecast has been audited.
pub fn assurance_block(j: &Jurisdiction) -> AssuranceBlock {
    AssuranceBlock {
        title: "Independent Practitioner's Assurance Report on the Future-Oriented Financial Information".into(),
        addressee: format!("To the {} of {}", j.holder_term, j.legal_name),
        standard: j.assurance_standard.to_string(),
        body_html: format!(
            "<p>We have examined the accompanying 10-year financial forecast of {legal} \
             (the &ldquo;Partnership&rdquo;), comprising the statements of forecasted financial \
             position, comprehensive income, changes in equity and cash flows for each of the ten \
             forecast years, and the related notes. The forecast has been prepared by management \
             using the significant assumptions set out in Note 3.</p>\
             <p><strong>Management&rsquo;s responsibility.</strong> Management is responsible for \
             the preparation of the forecast, including the reasonableness of the assumptions on \
             which it is based, in accordance with International Financial Reporting Standards.</p>\
             <p><strong>Practitioner&rsquo;s responsibility.</strong> Our responsibility is to \
             express a conclusion on the forecast based on our examination, conducted in \
             accordance with {standard}.</p>\
             <p><strong>Conclusion on the assumptions.</strong> Based on our examination of the \
             evidence supporting the assumptions, nothing has come to our attention that causes us \
             to believe that these assumptions do not provide a reasonable basis for the forecast.</p>\
             <p><strong>Opinion.</strong> In our opinion, the forecast is properly prepared on the \
             basis of those assumptions and is presented in accordance with International Financial \
             Reporting Standards.</p>\
             <p><strong>Caution.</strong> Since the forecast is based on assumptions regarding \
             future events, actual results will vary from the information presented, and the \
             variations may be material. We have no responsibility to update this report for \
             events and circumstances occurring after its date. This report does not constitute an \
             audit of the forecast results, which lie in the future.</p>",
            legal = j.legal_name, standard = j.assurance_standard,
        ),
    }
}
