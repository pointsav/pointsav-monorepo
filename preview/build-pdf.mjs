#!/usr/bin/env node
// Generate a print-quality PDF from any slide-deck HTML in this directory.
//
// Usage:
//   node build-pdf.mjs <file.html>
//   node build-pdf.mjs all
//
// Output: <file>.pdf alongside the source HTML.
//
// Requires Playwright (already installed in the sandbox):
//   NODE_PATH=/home/jennifer/sandbox/working/ps-talking-points/node_modules \
//     node build-pdf.mjs building-width-calculator.html

import { createRequire } from 'node:module';
import { pathToFileURL } from 'node:url';
import { resolve, basename, dirname, join } from 'node:path';
import { existsSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

// Resolve Playwright from the sandbox node_modules if not local.
const PLAYWRIGHT_SEARCH_PATHS = [
  join(__dirname, 'node_modules', 'playwright-core'),
  '/home/jennifer/sandbox/working/ps-talking-points/node_modules/playwright-core',
  '/home/jennifer/sandbox/node_modules/playwright-core',
];
const pw = PLAYWRIGHT_SEARCH_PATHS.find(p => existsSync(p));
if (!pw) {
  console.error('playwright-core not found. Run: npm i playwright-core in preview/ or adjust PLAYWRIGHT_SEARCH_PATHS.');
  process.exit(1);
}
const { chromium } = createRequire(import.meta.url)(pw);

const arg = process.argv[2];
if (!arg) {
  console.error('usage: node build-pdf.mjs <file.html> | all');
  process.exit(2);
}

const targets = arg === 'all'
  ? readdirSync(__dirname).filter(f => f.endsWith('.html')).map(f => join(__dirname, f))
  : [resolve(arg)];

for (const absHtml of targets) {
  if (!existsSync(absHtml)) {
    console.error(`not found: ${absHtml}`);
    process.exit(2);
  }
}

const browser = await chromium.launch();
// Wide viewport so no responsive scale(0.92) rule fires.
const ctx = await browser.newContext({ viewport: { width: 1600, height: 1240 } });

for (const absHtml of targets) {
  const absPdf = join(dirname(absHtml), basename(absHtml, '.html') + '.pdf');
  console.log(`  rendering ${basename(absHtml)} …`);

  const page = await ctx.newPage();
  await page.emulateMedia({ media: 'print' });
  await page.goto(pathToFileURL(absHtml).href, { waitUntil: 'networkidle' });

  await page.pdf({
    path: absPdf,
    printBackground: true,
    preferCSSPageSize: true,         // honour @page { size: 11in 8.5in; margin: 0; }
    margin: { top: 0, right: 0, bottom: 0, left: 0 },
    displayHeaderFooter: false,      // no URL / page-number footer
    tagged: true,                    // accessible PDF (Chrome 89+)
  });

  await page.close();
  console.log(`  → wrote ${absPdf}`);
}

await browser.close();
