// Comprehensive mobile capture — map panels (VWH/PKS/Retail) at detents + research pages.
// URL=http://127.0.0.1:8765 OUT=./shots node shoot-all.mjs
import { chromium } from 'playwright';
import { mkdirSync } from 'fs';

const URL = process.env.URL || 'http://127.0.0.1:8765';
const OUT = process.env.OUT || './shots';
mkdirSync(OUT, { recursive: true });

const browser = await chromium.launch();
const ctx = await browser.newContext({
  viewport: { width: 375, height: 667 }, deviceScaleFactor: 2, isMobile: true, hasTouch: true,
});
const page = await ctx.newPage();
page.on('pageerror', e => console.log('PAGE EXCEPTION:', e.message));
const shot = (n) => page.screenshot({ path: `${OUT}/${n}.png` }).then(() => console.log('shot', n));
const wait = (ms) => page.waitForTimeout(ms);

// ── MAP PANELS ───────────────────────────────────────────────────────────────
await page.goto(URL, { waitUntil: 'load', timeout: 30000 });
await page.waitForFunction(() => window.map && window.map.isStyleLoaded(), { timeout: 30000 });
await wait(1500);

async function archetype(toggleId, geojson, prefix) {
  await page.evaluate((id) => { window.flyToOverview && window.flyToOverview(); const b = document.getElementById(id); if (b) window['toggle' + (id === 'lc-urban-fringe' ? 'UrbanFringeLayer' : 'CommuterLayer')](b); }, toggleId);
  await wait(900);
  await page.evaluate(async (g) => {
    const d = await (await fetch(g)).json();
    const tierKey = g.includes('vwh') ? 'vwh_tier' : 'commuter_tier';
    const f = d.features.find(x => x.properties[tierKey] === 1) || d.features[0];
    window.showArchetypeDetail(f.properties);
  }, geojson);
  await wait(900);
  await shot(`${prefix}-half`);
  await page.evaluate(() => window.SHEET.snapTo('peek')); await wait(600); await shot(`${prefix}-peek`);
  await page.evaluate(() => window.SHEET.snapTo('full')); await wait(600); await shot(`${prefix}-full`);
}

await archetype('lc-urban-fringe', 'data/archetype-vwh.geojson', 'map-vwh');
await archetype('lc-commuter', 'data/archetype-pks.geojson', 'map-pks');

// Retail cluster panel
await page.evaluate(() => window.flyToOverview && window.flyToOverview());
await wait(700);
await page.evaluate(async () => {
  const d = await (await fetch('data/clusters-meta.json')).json();
  const arr = Array.isArray(d) ? d : (d.clusters || d.features || []);
  const c = arr.find(x => (x.t || x.tier) === 1) || arr[0];
  if (c && window.metaToClusterProps) window.showClusterDetail(window.metaToClusterProps(c), [c.lon, c.lat]);
}).catch(e => console.log('retail err', e.message));
await wait(900);
await shot('map-retail-half');
await page.evaluate(() => window.SHEET && window.SHEET.snapTo('peek')); await wait(600); await shot('map-retail-peek');

// ── RESEARCH PAGES ───────────────────────────────────────────────────────────
const pages = [
  ['research.html', 'res-overview'],
  ['research-colocation.html', 'res-colocation'],
  ['research-regional-markets.html', 'res-regional'],
  ['research-summary.html', 'res-summary'],
  ['research-aec.html', 'res-aec'],
];
for (const [path, name] of pages) {
  await page.goto(`${URL}/${path}`, { waitUntil: 'load', timeout: 20000 });
  await wait(1200);
  await shot(name);                              // top of page (nav + header + first sections)
  // tall full-page capture for the print-doc / article pages
  await page.screenshot({ path: `${OUT}/${name}-full.png`, fullPage: true }).catch(() => {});
}

await browser.close();
console.log('done →', OUT);
