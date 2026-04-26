// build.mjs — esbuild config for the SAA editor vendor bundle.
//
// Output: ../static/vendor/cm-saa.bundle.js
// Format: IIFE, exposing window.CMSAA
// Target: ES2022 (broad evergreen support)
// Minify: yes (production bundle; sourcemap optional, omitted for v1)
//
// Run: `npm ci && node build.mjs` from inside vendor-js/.
//
// The output bundle is committed to Git (see ../.gitignore — vendor-js/
// excludes node_modules but ../static/vendor/ is tracked). This keeps the
// Rust build self-contained — no NPM in the Rust build path.

import * as esbuild from 'esbuild';

const result = await esbuild.build({
  entryPoints: ['entry.js'],
  bundle: true,
  format: 'iife',
  globalName: 'CMSAA',
  target: 'es2022',
  minify: true,
  outfile: '../static/vendor/cm-saa.bundle.js',
  metafile: true,
  legalComments: 'none',
});

const out = '../static/vendor/cm-saa.bundle.js';
const bytes = result.metafile.outputs[out]?.bytes ?? 0;
console.log(`bundle: ${out}`);
console.log(`size:   ${(bytes / 1024).toFixed(1)} KB`);
