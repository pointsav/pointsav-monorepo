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

// Phase 2 Step 7 — second bundle for collab. Default-off; loaded lazily
// by saa-init.js only when window.WIKI_COLLAB_ENABLED is set by the
// server (--enable-collab CLI flag). Keeps the SAA bundle lean for
// production deploys that don't enable collab.

const builds = [
  {
    entryPoints: ['entry.js'],
    outfile: '../static/vendor/cm-saa.bundle.js',
    globalName: 'CMSAA',
  },
  {
    entryPoints: ['entry-collab.js'],
    outfile: '../static/vendor/cm-collab.bundle.js',
    globalName: 'CMCOLLAB',
  },
];

for (const b of builds) {
  const result = await esbuild.build({
    entryPoints: b.entryPoints,
    bundle: true,
    format: 'iife',
    globalName: b.globalName,
    target: 'es2022',
    minify: true,
    outfile: b.outfile,
    metafile: true,
    legalComments: 'none',
  });
  const bytes = result.metafile.outputs[b.outfile]?.bytes ?? 0;
  console.log(`bundle: ${b.outfile}`);
  console.log(`size:   ${(bytes / 1024).toFixed(1)} KB`);
}
