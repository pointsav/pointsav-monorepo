// Mobile BentoBox screenshot harness — captures the bottom sheet at each detent at 375×667.
// Usage:  URL=http://127.0.0.1:8765 OUT=./shots node shoot.mjs
// Requires window.map and window.SHEET to be exposed by index.html (debug handles).
import { chromium } from 'playwright';
import { mkdirSync } from 'fs';

const URL = process.env.URL || 'http://127.0.0.1:8765';
const OUT = process.env.OUT || './shots';
mkdirSync(OUT, { recursive: true });

const browser = await chromium.launch();
const ctx = await browser.newContext({
  viewport: { width: 375, height: 667 },
  deviceScaleFactor: 2, isMobile: true, hasTouch: true,
});
const page = await ctx.newPage();
page.on('console', m => { if (m.type() === 'error') console.log('PAGE ERR:', m.text()); });
page.on('pageerror', e => console.log('PAGE EXCEPTION:', e.message));

console.log('goto', URL);
await page.goto(URL, { waitUntil: 'load', timeout: 30000 });
await page.waitForFunction(() => window.map && window.map.isStyleLoaded(), { timeout: 30000 });
await page.waitForTimeout(1500);
await page.screenshot({ path: `${OUT}/01-overview.png` });
console.log('shot: overview');

// Turn on Urban Fringe, then select the strongest VWH feature and render its detail.
await page.evaluate(() => {
  const btn = document.getElementById('lc-urban-fringe');
  if (btn) window.toggleUrbanFringeLayer(btn);
});
await page.waitForTimeout(1000);
await page.evaluate(async () => {
  const r = await fetch('data/archetype-vwh.geojson');
  const d = await r.json();
  const f = d.features.find(x => x.properties.vwh_tier === 1) || d.features[0];
  window.showArchetypeDetail(f.properties);
});
await page.waitForTimeout(900);
await page.screenshot({ path: `${OUT}/02-detail-half.png` });
console.log('shot: detail-half', await page.evaluate(() => window.SHEET.current));

await page.evaluate(() => window.SHEET.snapTo('peek'));
await page.waitForTimeout(700);
await page.screenshot({ path: `${OUT}/03-detail-peek.png` });
console.log('shot: detail-peek');

await page.evaluate(() => window.SHEET.snapTo('full'));
await page.waitForTimeout(700);
await page.screenshot({ path: `${OUT}/04-detail-full.png` });
console.log('shot: detail-full');

await browser.close();
console.log('done →', OUT);
