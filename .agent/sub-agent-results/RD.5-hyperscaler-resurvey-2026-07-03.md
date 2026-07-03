# RD.5 — Hyperscaler Design-System Re-survey (validation of the April Spectrum pick)

**Date:** 2026-07-03
**Task:** Validate or contradict BB.14's Adobe Spectrum pick (14/15 bankers' distinguishability) against the live state of the surveyed design systems today. Input: RD.4 extraction of BB.13/BB.14 (2026-04-28).
**Method:** Live checks — spectrum.adobe.com, s2.spectrum.adobe.com, react-spectrum.adobe.com/releases, carbondesignsystem.com (via release channels; both Adobe and IBM doc sites are client-rendered SPAs that return empty shells to non-JS fetchers, so chrome claims there rest on release-note evidence, flagged below), vercel.com/geist/introduction, plus direct CSS inspection of the sibling product at `http://127.0.0.1:9094/` (design.pointsav.com build), plus web searches for post-April-2026 design-system launches.

---

## Verdict up front

**The April Spectrum pick still stands.** Nothing at Adobe, and nothing new in the field, undermines it. Two findings do warrant small adjustments — both about the *sibling product's baseline*, which has drifted since RD.4's extraction, not about Spectrum:

1. The distinguishability case now leans **more on typography and chrome, slightly less on color** — the sibling introduced a dark-navy link/selected color (`#0e3a66`) in the same family as BIM's planned `#1A4480` drafting blue.
2. The sibling swapped Inter → **IBM Plex Sans/Mono**, making it *more* canonically Carbon — which makes BIM's Source Serif 4 + Geist spine an even stronger differentiator than BB.14 scored it.

Re-scored on BB.14's own axes: Color 4 (was 5), Typography 5 (was 4), Chrome 5 (unchanged) = **still 14/15**. Runner-up Geist is unchanged (13/15). No re-pick needed.

---

## Finding 1 — Adobe Spectrum: no meaningful change since April

- **Spectrum 2 v1.0 shipped stable 2025-12-16** (React Spectrum S2) — i.e., it was *already* the stable current version when BB.14 was written in April 2026. Nothing newer has replaced it.
- **s2.spectrum.adobe.com is still a vision/marketing page**, not a docs site — verified live today. It has minimal chrome (top bar + vertical scroll), explicitly future-facing language, and directs users to spectrum.adobe.com for actual components/guidelines. **spectrum.adobe.com remains the working documentation site.**
- Documented 2026 site changes are incremental only: a February 2026 docs update added a dark/light mode switch, a Color section in search, and "Agent Skills" for AI coding tools. No palette, typography, or chrome overhaul found in any release channel.
- Typeface: S2 introduced **"Adobe Clean Spectrum VF"** — still the proprietary Adobe Clean family. BB.14's constraint (Adobe Clean must be substituted; done via Source Serif 4 + Geist) is unchanged and still correct.
- One nuance worth recording: S2's *component* styling direction is Adobe-Express-inspired — publicly characterized as "functional and joyful," rounder and friendlier than the S1 instrument-panel look. This does not affect the pick, because **BB.14's frozen spec (§6/§B.7) mirrors the docs-site chrome grammar** (272px sidebar, panel-band tab bar, 4px radius, dark code blocks) — which is the S1-lineage spectrum.adobe.com grammar, not S2's rounder component skin. Recommendation: treat BB.14's spec values as frozen and do **not** track Adobe's S2 component-roundness evolution — the instrument-panel qualities the pick was made for live in the docs chrome BB.14 already extracted, and chasing S2's friendlier direction would erode exactly the qualities that scored "Very High" AEC resonance.
- Caveat: spectrum.adobe.com's rendered chrome could not be re-verified pixel-level (SPA returns only a title to fetchers). The claim "no redesign since April" rests on release notes and the absence of any redesign announcement, which is strong but indirect evidence.

## Finding 2 — Sibling baseline (design.pointsav.com) HAS drifted, in a helpful direction

Direct CSS inspection at `127.0.0.1:9094` (reachable; title "PointSav Design System"):

| Axis | RD.4/BB.14 recorded (April) | Live today |
|---|---|---|
| Fonts | Inter | **IBM Plex Sans + IBM Plex Mono** |
| Token namespace | `--ps-*` | **`--cds-*`** (literal Carbon token names, with comment "Carbon Design System tokens") |
| Primary/interactive | `#234ed8` indigo | `--cds-interactive: #0050e6`, `--cds-focus: #0f62fe` (Carbon blue-60 exactly), **`--cds-link-primary: #0e3a66` dark navy** |
| Radius | near-zero (0.125–0.5rem) | `--cds-radius: 4px` |
| Dark mode | not recorded | full `data-theme` dark mode with Carbon dark palette (`#161616` bg, `#78a9ff` links) |

Implications:

- The site is now *more* Carbon-family than in April (literal `--cds-*` tokens, IBM Plex, `#0f62fe` focus). The premise "the sibling is Carbon-shaped, BIM must be distinguishable from it" is **stronger than ever**, and Carbon's disqualification (3/15) is even more final.
- **Color convergence flag:** the sibling's link/selected-text navy `#0e3a66` sits in the same dark-navy family as BIM's `#1A4480` drafting blue. At thumbnail scale, both sites will show dark-navy text accents. Mitigation, not re-pick: keep `#1A4480` (its drafting-document semantics are the point), but recognize that thumbnail differentiation now rides primarily on (a) the serif display headings, (b) the `#EFEFEF` sidebar panel + `#E8E8E8` tab-band chrome vs the sibling's flat `#f4f4f4` sidebar, and (c) the isometric hero + classification chips. If the operator wants belt-and-suspenders on color, the lever is making the BIM *interactive* states (buttons, active nav border) unmistakably `#1A4480` while the sibling's interactive is bright `#0050e6`/`#0f62fe` — that pairing still reads differently.
- **Typography divergence improved:** IBM Plex Sans vs Source Serif 4 + Geist Sans is a bigger visible gap than the Inter-vs-Geist gap BB.14 scored 4/5. Typography axis now merits 5/5.
- **Radius convergence is trivial:** 4px vs BIM's 4/6/8px — radius was never a scoring axis on its own and is invisible at thumbnail scale.

## Finding 3 — Distinguishability case: holds, re-scored 14/15

Using BB.14's own three-axis method against the *live* sibling: **Color 4** (navy-family overlap on links/selected states; interactive blues still clearly apart), **Typography 5** (serif display + Geist vs IBM Plex — the strongest axis now), **Chrome 5** (sibling went denser/darker-header classic Carbon; Spectrum panel-band chrome unaffected) = **14/15**. Same total, different composition. Geist would re-score ~12–13 (its monochrome vs the sibling's navy-and-blue is still distinct, but Geist Sans is now also the sibling of `geistcn`/shadcn ubiquity — see Finding 4). Spectrum remains the clear leader.

## Finding 4 — No new AEC-resonant candidate has emerged since April

- Searched for post-April-2026 design-system launches. The only "important new 2026 design system" narrative in circulation is a May 2026 piece about **shadcn/ui** — which launched in 2023, was already surveyed (9/15, Medium AEC resonance), and whose actual 2026 story is that AI coding tools have converged on it as their default output. That is an argument *for* the BB.13/BB.14 AEC-marker strategy: generic shadcn-flavored looks are now the ambient default of AI-generated sites, so the IFC chips / GUID monospace / isometric hero / serif display markers are worth more, not less, as identity signals.
- **Vercel Geist** (runner-up): verified live — still Geist Sans/Mono, high-contrast near-monochrome, grid-centric materials. Notable additions: `@vercel/geistcn` component distribution and markdown/`/design.md` machine-readable docs. No visual identity change; remains a valid runner-up, nothing more.
- **IBM Carbon:** v11 still current; v12 in early flag-development, unreleased, no announced visual overhaul. Irrelevant to the pick beyond the baseline analysis above.
- Material 3 (incl. the "Expressive" direction), Polaris (docs still in deprecation/rebuild limbo), Fluent 2, Primer: nothing found that moves any of them toward AEC resonance. No forced candidate.

---

## Recommendation

**Proceed with the Spectrum pick exactly as specified in BB.14 §6/§B.7 — the spec values are frozen and correct.** Three riders:

1. **Do not track Spectrum 2's component-styling evolution.** The pick is the docs-site chrome grammar as extracted in April; Adobe's own direction is drifting friendlier/rounder, and following it would dilute the instrument-panel rationale.
2. **Record the navy-convergence flag** (`#0e3a66` sibling links vs `#1A4480` BIM accent) in the redesign brief. Keep `#1A4480`; rely on serif+chrome+markers for thumbnail separation; optionally weight `#1A4480` heavily into interactive states where the sibling shows bright `#0050e6`.
3. **RD.4's sibling-baseline table (§B.1 ground-truth) is stale** — update any downstream document that quotes "Inter / `#234ed8` / near-zero radius" for design.pointsav.com to the live values above before the redesign round scores anything against it.

Sources: s2.spectrum.adobe.com (live fetch), react-spectrum.adobe.com/releases + search results (S2 v1.0 2025-12-16; Adobe Clean Spectrum VF; Feb 2026 docs updates), vercel.com/geist/introduction (live fetch), carbon-design-system release channels (v11/v12 status), medium.com/design-bootcamp shadcn piece (May 2026), 127.0.0.1:9094 `/static/tokens.css` + `/static/portal.css` (direct inspection).
