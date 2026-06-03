# tools/ — mobile screenshot harness

`shoot.mjs` captures the mobile BentoBox bottom sheet at each detent (375×667) using Playwright,
so the sheet layout / detent heights / peek-summary can be verified without a physical phone.

## Setup (one-time)
Chromium is already in `~/.cache/ms-playwright`. Install only the JS package:
```bash
cd tools
PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1 npm i playwright
```
(`node_modules/`, `shots/`, `package-lock.json` are gitignored.)

## Run
```bash
# serve a www dir that has data/ + tiles/ (the gateway deployment is complete):
cd /srv/foundry/deployments/gateway-orchestration-gis-1/www
python3 -m http.server 8765 &
cd -                          # back to tools/
URL=http://127.0.0.1:8765 OUT=./shots node shoot.mjs
kill %1                        # stop the server
```
Outputs `shots/01-overview.png`, `02-detail-half.png`, `03-detail-peek.png`, `04-detail-full.png`.

## Notes
- The harness drives the sheet via `window.map` and `window.SHEET` (read-only debug handles
  exposed in `index.html`), plus the global `toggleUrbanFringeLayer` / `showArchetypeDetail`.
- Python's `http.server` does **not** support HTTP byte-range requests, so PMTiles (retail
  location tiles) won't render — the basemap + archetype GeoJSON dots + the BentoBox sheet do,
  which is all that's needed to verify sheet layout. For full tile rendering use a byte-serving
  static server (e.g. `npx http-server -p 8765` or nginx).
- Touch *feel* (pan sensitivity, drag inertia) cannot be screenshotted — verify on a real phone.
