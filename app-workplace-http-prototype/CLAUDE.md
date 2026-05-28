@~/Foundry/AGENT.md

# app-workplace-http-prototype — Project Guide

> **State:** Active | **Last updated:** 2026-05-28
> **Type:** app-workplace | **Port:** 9110

HTTP prototype surface for `os-workplace`. Serves all 8 Workplace tool
surfaces from a single axum binary over WireGuard PPN while native Tauri
builds await a macOS host.

## Brief

`.agent/briefs/BRIEF-workplace-http-prototype.md`

## Scope

Stage 1 (Memo) is the active build target. Remaining 7 stages are
scaffolded; pick up from `NEXT.md`.

Retired when Wave 1 native apps ship for macOS. Do not invest in
production hardening — this is a prototype.

## Run

```
cd app-workplace-http-prototype
cargo build --release
WORKPLACE_PROTO_WORKSPACE=/home/jennifer/workbench cargo run --release
```

Reachable at `http://10.8.0.1:9110` over WireGuard PPN.
`WORKPLACE_PROTO_PORT` overrides the default port.

## File format — Memo

`.html` fragments (innerHTML of contenteditable). Read/write-compatible
with the native `app-workplace-memo` format.

## SYS-ADR-07

Proforma formula evaluation, Schedule CPM, GIS projections, BIM IFC
operations — native Rust only; never through service-slm inference.
