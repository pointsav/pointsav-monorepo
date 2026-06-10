/* Woodfine Direct-Hold Solutions — Sensitivity Analysis (client).
   Slider-free: a fixed set of pre-set charts + tables drawn once from engine pre-computed data.
   No financial recomputation here. */
const FLOOR = 1.20, PAR = 100.0, U = DATA.units, OCC = DATA.baseOcc;
const charts = {};
// Woodfine brand palette (MEMO-Woodfine-Color-Matrix): ink/grey neutrals,
// green=finance accent, orange=stress/highlight, red=covenant/alert, gold=Direct-Hold marker.
const INK = '#111827', GREY = '#9CA3AF', RED = '#ED1B2F', AMBER = '#F15F22', POS = '#54924E', GOLD = '#F57F17', BLUE = '#164679';
const WASH = 'rgba(237,27,47,0.07)';
const dash = '—';

/* ── formatters ─────────────────────────────────────────────────────── */
function usd2(v) { if (v == null) return dash; if (Math.abs(v) < 0.005) return dash; return v < 0 ? '($' + Math.abs(v).toFixed(2) + ')' : '$' + v.toFixed(2); }
function usd0(v) { return '$' + Math.round(v); }
function pct2(v) { if (v == null) return dash; if (Math.abs(v) < 1e-9) return dash; return (v * 100).toFixed(2) + '%'; }
function ratio(v) { return (v != null && v > 0) ? v.toFixed(2) + '×' : dash; }
function kfmt(v) { return (v != null && v > 0) ? (v / 1000).toFixed(1) + 'K' : dash; }
function fM(v) { return '$' + (v / 1e6).toFixed(1) + 'M'; }
const labels10 = Array.from({ length: 10 }, (_, i) => 'Y' + (i + 1));

/* ── per-Investment-Unit forecast table ─────────────────────────────── */
function rowsFor(years) {
  const cr = Math.pow(years[7].mvpu / PAR, 1 / 8) - 1;
  const R = [];
  const head = (t) => R.push({ head: t });
  const row = (label, fn, cls) => R.push({ label, cells: years.map((y, i) => fn(y, i)), cls });
  head('Revenue');
  row('Revenue (per Unit)', y => usd2(y.noi / U));
  row('Distributions (per Unit)', y => usd2(y.dpu));
  row('Distribution Yield on Initial Capital', y => pct2(y.dpu / PAR));
  head('Asset Valuation (6.25% Cap Rate)');
  row('Asset Value (per Unit)', y => usd2(y.avpu));
  row('Total Debt (per Unit)', y => usd2(y.debt / U));
  row('Net Asset Value (NAV) per Unit', y => usd2(y.navpu));
  head('Selling in the Secondary Market (20% Discount to Net Asset Value)');
  row('Market Value per Unit', y => usd2(y.mvpu));
  // Discount/Premium vs NAV is "—" for Y1–Y3 (no secondary market during construction).
  row('Discount / Premium vs NAV', (y, i) => i < 3 ? dash : (y.navpu > 0 ? pct2((y.mvpu - y.navpu) / y.navpu) : dash));
  row('Compounded Annual Return (Y8)', (y, i) => i === 7 ? pct2(cr) : dash);
  row('Distribution Yield to Buyers at Market', y => pct2(y.dyb));
  head('Operating Metrics');
  row('Interest Coverage Ratio', y => ratio(y.icr), y => (y.icr > 0 && y.icr < FLOOR) ? 'binding' : '');
  row('Debt vs Development Cost', y => pct2(y.ddc));
  row('Debt to Asset Value', y => pct2(y.dav));
  row('Total Expense Ratio', y => y.navpu > 0 ? pct2(y.fopex / (y.navpu * U)) : dash);
  row('Sqft (stabilised)', y => kfmt(y.sqft));
  return R;
}
/* Continuous left-gutter line numbers across all tables (conference-call reference). */
let lineNo = 1;
function gnum() { return '<td class="lnum">' + (lineNo++) + '</td>'; }
function ghead() { return '<th class="lnum"></th>'; }
function gblank() { return '<td class="lnum"></td>'; }
function renderTable() {
  const years = DATA.base.run.years, t = document.getElementById('tbl-base');
  t.querySelector('thead').innerHTML = '<tr>' + ghead() + '<th>Per Investment Unit</th>' + labels10.map(l => '<th>' + l + '</th>').join('') + '</tr>';
  t.querySelector('tbody').innerHTML = rowsFor(years).map(r => {
    if (r.head) return '<tr class="section-head">' + gblank() + '<td class="section-head-cell" colspan="11">' + r.head + '</td></tr>';
    return '<tr>' + gnum() + '<td>' + r.label + '</td>' + r.cells.map((c, i) => {
      const k = r.cls ? r.cls(years[i]) : '';
      return '<td' + (k ? ' class="' + k + '"' : '') + '>' + c + '</td>';
    }).join('') + '</tr>';
  }).join('');
}

/* ── cards + summary tables ─────────────────────────────────────────── */
function card(lab, val, sub, down) {
  return '<div class="card"><div class="lab">' + lab + '</div><div class="val' + (down ? ' down' : '') + '">' + val + '</div><div class="sub">' + (sub || '') + '</div></div>';
}
function renderCards() {
  const b = DATA.base, ys = b.run.years;
  document.getElementById('cards-base').innerHTML =
    card('Y8 NAV / Unit', usd2(b.y8nav), 'the value · on $100 invested') +
    card('Y8 Income Yield on Capital', pct2(ys[7].dpu / PAR), 'the income · on $100 invested') +
    card('Y10 NAV / Unit', usd2(b.y10nav), 'capital growth') +
    card('Y8 Market Value / Unit', usd2(b.y8mv), 'distribution ÷ 8% buyer yield') +
    card('Min Interest Coverage', ratio(b.minICR), 'Year ' + b.minICRyear + ' (covenant 1.20×)') +
    card('Stabilised Occupancy', OCC.toFixed(0) + '%', 'net of 5% vacancy');
}
function renderFourStress() {
  const t = document.getElementById('tbl-fourstress');
  t.querySelector('tbody').innerHTML = DATA.fourStress.map(s =>
    '<tr>' + gnum() + '<td>' + s.driver + ' · ' + s.move + '</td><td>' + s.builtPct.toFixed(0) + '%</td><td' +
    (s.minICR < FLOOR - 0.005 ? ' class="binding"' : '') + '>' + ratio(s.minICR) + '</td><td>' + usd2(s.y8nav) + '</td><td>' + pct2(s.y8dist) + '</td></tr>'
  ).join('');
}
function renderOneway() {
  const t = document.getElementById('tbl-oneway');
  t.querySelector('tbody').innerHTML = DATA.oneway.map(o => {
    const dn = o.navAdverse - o.navBase, up = o.navFavorable - o.navBase;
    const di = o.distAdverse - o.distBase;
    return '<tr>' + gnum() + '<td>' + o.driver + '</td><td>' + o.adverse + '</td><td>' + o.favorable + '</td><td>' +
      usd2(o.navAdverse) + ' <span class="d">(' + (dn < 0 ? '−$' + Math.abs(dn).toFixed(2) : '+$' + dn.toFixed(2)) + ')</span></td><td>' +
      usd2(o.navFavorable) + ' <span class="d">(' + (up < 0 ? '−$' + Math.abs(up).toFixed(2) : '+$' + up.toFixed(2)) + ')</span></td><td>' +
      pct2(o.distAdverse) + ' <span class="d">(' + (di < 0 ? '−' : '+') + (Math.abs(di) * 100).toFixed(2) + ' pp)</span></td><td' +
      (o.minICRAdverse < FLOOR ? ' class="binding"' : '') + '>' + ratio(o.minICRAdverse) + '</td></tr>';
  }).join('');
}
function renderCureCard() {
  const c = DATA.cure;
  document.getElementById('cure-card').innerHTML =
    '<div class="cure-grid">' +
    kvb(c.shockRatePct.toFixed(2) + '%', 'Refinancing rate (+' + c.shockRateBps.toFixed(0) + ' bps)') +
    kvb(c.shockCapPct.toFixed(2) + '%', 'Cap rate (+' + c.shockCapBps.toFixed(0) + ' bps)') +
    kvb(c.shockOccPct.toFixed(0) + '%', 'Occupancy (−' + c.shockOccPp.toFixed(0) + ' pp)') +
    kvb(usd2(c.navpu), 'NAV / unit preserved') +
    kvb(usd2(c.distPerUnitPostCure), 'Distribution / unit post-cure') +
    kvb((c.frac * 100).toFixed(1) + '%', 'Minimum disposition') +
    '</div>' +
    '<p class="cap">Under a maximal, internally-consistent combined shock — refinancing rate <b>+' + c.shockRateBps.toFixed(0) + ' bps</b> (to ' +
    c.shockRatePct.toFixed(2) + '%), cap rate <b>+' + c.shockCapBps.toFixed(0) + ' bps</b> (to ' + c.shockCapPct.toFixed(2) + '%) and occupancy <b>−' +
    c.shockOccPp.toFixed(0) + ' pp</b> (to ' + c.shockOccPct.toFixed(0) + '%) at once — unit NAV is <b>preserved at ' + usd2(c.navpu) + '</b>, above the $' +
    c.floor.toFixed(0) + ' of initial capital. Selling the minimum ≈' + (c.frac * 100).toFixed(0) +
    '% of the portfolio at its stressed value lifts coverage from ' + ratio(c.before) + ' back to the 1.20× covenant (a market-value sale, so NAV is unchanged). ' +
    'Income compresses with coverage held at the covenant — the distribution per unit falls from <b>' + usd2(c.distPerUnitBase) +
    '</b> to <b>' + usd2(c.distPerUnitPostCure) + '</b>, but stays positive.</p>';
}
function kvb(b, s) { return '<div class="kv"><b>' + b + '</b><span>' + s + '</span></div>'; }

/* ── charts ─────────────────────────────────────────────────────────── */
function mk(id, cfg) {
  const c = document.getElementById(id); if (!c) return;
  if (charts[id]) charts[id].destroy();
  cfg.options = cfg.options || {};
  cfg.options.animation = false;
  cfg.options.responsive = true;
  if (cfg.options.aspectRatio == null) cfg.options.aspectRatio = 2.4; // deterministic shape for print
  charts[id] = new Chart(c, cfg);
}
function legend(on) { return { display: on, labels: { boxWidth: 12, font: { size: 10 } } }; }
function covenantSets() {
  return [
    { label: 'Breach zone', data: labels10.map(() => FLOOR), borderColor: 'transparent', backgroundColor: WASH, pointRadius: 0, fill: 'start' },
    { label: 'Covenant 1.20×', data: labels10.map(() => FLOOR), borderColor: RED, borderDash: [3, 3], pointRadius: 0, fill: false },
  ];
}

function chartBaseNav() {
  mk('cv-base-nav', { type: 'line', data: { labels: labels10, datasets: [
    { label: 'NAV / unit', data: DATA.baseNav, borderColor: BLUE, borderWidth: 2, pointRadius: 2, tension: .25, fill: false },
    { label: 'Capital reference $100', data: labels10.map(() => PAR), borderColor: RED, borderDash: [3, 3], pointRadius: 0, fill: false },
  ] }, options: { plugins: { legend: legend(true) }, scales: { y: { min: 0, suggestedMax: 450, ticks: { font: { size: 10 }, callback: v => '$' + v } }, x: { ticks: { font: { size: 10 } } } } } });
}
function chartBaseCov() {
  mk('cv-base-cov', { type: 'line', data: { labels: labels10, datasets: [
    ...covenantSets(),
    { label: 'Interest coverage', data: DATA.baseCov, borderColor: BLUE, borderWidth: 2, pointRadius: 2, tension: .25, spanGaps: true, fill: false },
  ] }, options: { plugins: { legend: legend(true) }, scales: { y: { min: 0, max: 3.0, ticks: { font: { size: 10 }, callback: v => v.toFixed(1) + '×' } }, x: { ticks: { font: { size: 10 } } } } } });
}
function chartCoverageHeld() {
  const ch = DATA.coverageHeld;
  mk('cv-coverage-held', { type: 'line', data: { labels: labels10, datasets: [
    ...covenantSets(),
    { label: 'Base (5.00%, 95% occ)', data: DATA.baseCov, borderColor: BLUE, borderWidth: 2, pointRadius: 0, tension: .25, spanGaps: true, fill: false },
    { label: 'Interest +200 bps', data: ch.rate200, borderColor: AMBER, borderDash: [6, 3], borderWidth: 1.6, pointRadius: 0, tension: .25, spanGaps: true, fill: false },
    { label: 'Occupancy 75%', data: ch.occ75, borderColor: POS, borderDash: [3, 3], borderWidth: 1.6, pointRadius: 0, tension: .25, spanGaps: true, fill: false },
    { label: 'Dev yield 8.50%', data: ch.dev85, borderColor: GOLD, borderDash: [2, 4], borderWidth: 1.6, pointRadius: 0, tension: .25, spanGaps: true, fill: false },
  ] }, options: { plugins: { legend: legend(true) }, scales: { y: { min: 0, max: 3.0, ticks: { font: { size: 10 }, callback: v => v.toFixed(1) + '×' } }, x: { ticks: { font: { size: 10 } } } } } });
}
function chartLever() {
  const L = DATA.lever;
  mk('cv-lever', { type: 'line', data: { labels: L.map(l => l.ratePct.toFixed(2) + '%'), datasets: [
    { label: '% of programme built', data: L.map(l => l.builtPct), borderColor: BLUE, backgroundColor: 'rgba(22,70,121,0.08)', borderWidth: 2, pointRadius: 0, tension: .2, fill: true, yAxisID: 'y' },
    { label: 'min coverage', data: L.map(l => l.minICR), borderColor: RED, borderWidth: 2, pointRadius: 0, tension: .2, fill: false, yAxisID: 'y1' },
  ] }, options: { plugins: { legend: legend(true) }, scales: {
    y: { min: 0, max: 100, position: 'left', title: { display: true, text: '% built', font: { size: 10 } }, ticks: { font: { size: 10 }, callback: v => v + '%' } },
    y1: { min: 0, max: 2.0, position: 'right', grid: { drawOnChartArea: false }, title: { display: true, text: 'min coverage ×', font: { size: 10 } }, ticks: { font: { size: 10 }, callback: v => v.toFixed(1) + '×' } },
    x: { title: { display: true, text: 'debenture coupon', font: { size: 10 } }, ticks: { font: { size: 9 }, maxRotation: 0, autoSkip: true } } } } });
}
function chartHeadroom() {
  const H = DATA.headroom;
  mk('cv-headroom', { type: 'line', data: { labels: H.map(h => h.ratePct.toFixed(2) + '%'), datasets: [
    { label: 'Breach zone', data: H.map(() => FLOOR), borderColor: 'transparent', backgroundColor: WASH, pointRadius: 0, fill: 'start' },
    { label: 'Covenant 1.20×', data: H.map(() => FLOOR), borderColor: RED, borderDash: [3, 3], pointRadius: 0, fill: false },
    { label: 'Year-8 coverage', data: H.map(h => h.y8cov), borderColor: BLUE, borderWidth: 2, pointRadius: 0, tension: .15, fill: false },
  ] }, options: { plugins: { legend: legend(true) }, scales: { y: { min: 0, suggestedMax: 3.0, ticks: { font: { size: 10 }, callback: v => v.toFixed(1) + '×' } }, x: { title: { display: true, text: 'debenture coupon', font: { size: 10 } }, ticks: { font: { size: 9 }, maxRotation: 0, autoSkip: true } } } } });
}
function chartNavResilience() {
  mk('cv-nav-resilience', { type: 'line', data: { labels: labels10, datasets: [
    { label: 'Base', data: DATA.baseNav, borderColor: BLUE, borderWidth: 2, pointRadius: 0, tension: .25, fill: false },
    { label: 'Bear (cap +150 bps, occ 88%)', data: DATA.navBear, borderColor: AMBER, borderDash: [3, 3], borderWidth: 1.6, pointRadius: 0, tension: .25, fill: false },
    { label: 'Bull (cap −50 bps)', data: DATA.navBull, borderColor: POS, borderDash: [6, 3], borderWidth: 1.6, pointRadius: 0, tension: .25, fill: false },
    { label: 'Capital reference $100', data: labels10.map(() => PAR), borderColor: RED, borderDash: [3, 3], pointRadius: 0, fill: false },
  ] }, options: { plugins: { legend: legend(true) }, scales: { y: { min: 0, suggestedMax: 450, ticks: { font: { size: 10 }, callback: v => '$' + v } }, x: { ticks: { font: { size: 10 } } } } } });
}
function chartCure() {
  const c = DATA.cure;
  mk('cv-cure', { type: 'bar', data: { labels: ['At shock', 'After disposition'], datasets: [
    { label: 'Interest coverage', data: [c.before, c.after], backgroundColor: [RED, POS] },
  ] }, options: { aspectRatio: 4.5, indexAxis: 'y', plugins: { legend: legend(false) }, scales: { x: { min: 0, suggestedMax: 1.6, ticks: { font: { size: 10 }, callback: v => v.toFixed(2) + '×' }, grid: { color: ctx => ctx.tick.value === 1.2 ? RED : 'rgba(0,0,0,0.06)' } }, y: { ticks: { font: { size: 10 } } } } } });
}
function chartTornado() {
  const ow = DATA.oneway.map(o => ({ driver: o.driver, delta: o.navAdverse - o.navBase })).sort((a, b) => Math.abs(b.delta) - Math.abs(a.delta));
  mk('cv-tornado', { type: 'bar', data: { labels: ow.map(o => o.driver), datasets: [
    { label: 'Δ Y8 NAV/unit (adverse ±25 bps)', data: ow.map(o => o.delta), backgroundColor: ow.map(o => o.delta < 0 ? 'rgba(241,95,34,0.85)' : 'rgba(84,146,78,0.85)') },
  ] }, options: { aspectRatio: 3.4, indexAxis: 'y', plugins: { legend: legend(false) }, scales: { x: { ticks: { font: { size: 10 }, callback: v => '$' + v.toFixed(0) }, title: { display: true, text: 'effect on Year-8 NAV per Investment Unit (adverse ±25 bps move)', font: { size: 10 } } }, y: { ticks: { font: { size: 10 } } } } } });
}

/* ── draw everything once ───────────────────────────────────────────── */
function renderAll() {
  lineNo = 1; // restart the continuous row numbering
  renderCards(); renderTable(); renderFourStress(); renderOneway(); renderCureCard();
  chartBaseNav(); chartBaseCov(); chartCoverageHeld(); chartLever();
  chartHeadroom(); chartNavResilience(); chartCure(); chartTornado();
}
renderAll();
window.addEventListener('beforeprint', () => { Object.values(charts).forEach(c => c.resize()); });
