# app-mediakit-shell

Shared chrome chassis for the `os-mediakit` application family.

This crate is to the mediakit family what `app-console-keys` is to the console
family: it owns the shared chrome and the component vocabulary, while the OS
binary (`os-mediakit`) launches app instances and each app crate
(`app-mediakit-marketing`, and — planned — `-knowledge` / `-distributions`)
depends on this one. It is **not** a binary and **not** the OS.

## What it provides

1. **Chrome** (`shell`) — the persistent header / footer / page frame, ported
   to [maud](https://maud.lambda.xyz) from the Woodfine marketing shell
   templates. Tenant-parameterized via `Brand` (Woodfine, PointSav) so one
   binary serves multiple instances with identical chrome shape and different
   marks/links. The render entry point is `shell::render_page`.

2. **Component vocabulary** (`section`, `page`) — the typed `Section` set an AI
   author composes a page from. A page manifest is a YAML file listing typed
   sections; it either deserializes into these types (the contract) or it is
   rejected. There is no path to arbitrary HTML or CSS. Scaffold subset:
   `hero`, `prose`, `cta`.

3. **Tokens** (`tokens`) — DTCG design-token loading. Components reference only
   token custom properties; the canonical bundle from `pointsav-design-system`
   overrides the built-in fallback without any component change.

## Why CSS lives here

Section components own their responsive CSS in `static/sections.css`. Content
manifests carry **zero CSS** — an AI selects a section type and binds data; it
never writes a style rule. Mobile correctness is a tested property of the
components, not regenerated per page. This is how the platform "absorbs CSS".

## Build

```
cd app-mediakit-shell
cargo test
```

## Status

Scaffold (P1). The full section catalogue, an external-tokens sync, and
adoption by `app-mediakit-knowledge` / `-distributions` are later phases — see
`.agent/briefs/BRIEF-marketing-platform-master.md`.

Framework-agnostic by design: produces HTML + parses/validates manifests; does
not depend on axum. The mounting binary owns the web framework.
