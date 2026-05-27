# Workplace Workbench

Native desktop shell for the PointSav privategit development workbench.
Opens the locally-running HTTP server in a native macOS window.

**Platform:** macOS 10.13 High Sierra and later  
**Stack:** Tauri v1.7 (WebView shell — no bundled frontend logic)  
**Licence:** Apache-2.0

## What this is

`app-workplace-workbench` is a thin Tauri WebView shell. It loads the
`app-privategit-workbench` HTTP server running at a configurable localhost port
(default: 3000) inside a native macOS application window.

The workbench HTTP server runs as a separate, independent process.
This app does not start, stop, or manage that process — it only provides
a native window to access it.

## Setup

1. Ensure the privategit workbench HTTP server is running
2. Install Tauri CLI: `npm install`
3. Copy icons: `cp -r ../app-workplace-memo/src-tauri/icons src-tauri/`
4. Build: `npm run build`

## Configuration

Port is stored in `~/Library/Application Support/systems.pointsav.workplace-workbench/workbench-config.json`:

```json
{ "port": 3000 }
```

If the file is absent, port defaults to 3000. The port can be changed via the
`set_workbench_port` IPC command from the frontend.

## Transition plan

When `app-workplace-workbench` reaches stable, the privategit HTTP server
continues running independently — it is not removed or made optional.
Both access paths (browser and native window) remain available.
