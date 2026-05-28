---
schema: foundry-journal-v1
artifact_type: JOURNAL
state: draft
version: "0.2"
title: "Muscle-Memory-Preserving Desktop Environments for Professional AEC Software Migration: A Keystroke-Economy Framework"
target_journal: "ACM Transactions on Computer-Human Interaction"
target_publisher: "ACM"
impact_factor: ""
alternate_venue: "International Journal of Human-Computer Studies (Elsevier, IF 6.96); Human-Computer Interaction (Taylor & Francis, IF ~4.5)"
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: jmwoodfine@gmail.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., Vancouver, British Columbia, Canada"
    email: ""
    orcid: ""
    credit_roles:
      - Software
      - Writing – Review & Editing
subject_codes:
  - "H.5.2 User Interfaces"
  - "H.1.2 User/Machine Systems"
  - "J.6 Computer-Aided Engineering"
keywords:
  - muscle memory
  - command-line interface
  - AutoCAD migration
  - BIM software
  - keystroke economy
  - professional software adoption
  - desktop environment design
  - IFC
  - motor learning
bcsc_class: public-disclosure-safe
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: jmwoodfine@gmail.com
word_count_body: 5200
word_count_target: 8000
submission_status: not-submitted
writing_pass_date: 2026-05-28
language_pass_date: 2026-05-28
cites:
  - card-1980-ksm
  - fitts-1954-motor
  - fitts-posner-1967
  - reason-1990-human-error
  - squire-1992-memory
  - schmidt-lee-2014
  - proctor-dutta-1995
  - eastman-2011-bim
  - iso-16739-2018
  - buildingsmart-bcf3
forbidden_terms_cleared: true
notes_for_editor: |
  Writing pass 2026-05-28: §1–§5 body written (~5,200 words). §6 Results pending user
  study execution. §7 Discussion written with formal hypotheses and falsification
  programme. §8 Conclusion written.

  Pre-submission blockers:
    - §6 Results: user study data collection not yet executed
    - ORCID IDs for all three authors
    - word count to reach 8,000 will come from user study results (§6) + fuller discussion

  Venue decision: TOCHI (ACM Transactions on Computer-Human Interaction) is confirmed
  as primary target. Word limit: 30 ACM body pages. The planned study with ≥20
  participants will satisfy TOCHI's empirical contribution requirement.
---

# Muscle-Memory-Preserving Desktop Environments for Professional AEC Software Migration: A Keystroke-Economy Framework

**Woodfine Management Corp.**
Vancouver, British Columbia, Canada

*Corresponding author:* jmwoodfine@gmail.com

---

## Abstract

Professional migration from established AEC design software to new alternatives incurs productivity costs attributable not only to feature-coverage gaps but to the disruption of procedural motor routines acquired through years of sustained practitioner use. Existing migration studies quantify workflow coverage and learning curves but do not treat motor-pattern disruption as a distinct measurable variable. This paper proposes a muscle-memory preservation (MMP) framework for BIM desktop editor design comprising three principles: verbatim command-alias mapping from the practitioner's habituated alias set; spatial replication of palette and toolbar layouts to preserve Fitts's-Law-governed pointing habits; and function-key binding preservation for high-frequency toggle operations. The MMP framework is implemented in a prototype BIM authoring environment and documented through a command-alias table of 18 core mappings, a layer-to-IFC-category assignment that preserves AutoCAD layer-panel appearance while enforcing IFC semantic precision, and an F-key binding matrix consistent with the AutoCAD vocabulary. Three formal hypotheses are stated: H₁ (the MMP environment reduces command-error rate by ≥25% versus a feature-equivalent non-MMP alternative in practitioners with ≥3 years AutoCAD experience); H₂ (task-completion time is ≥30% lower under MMP in the first ten hours of use); and H₃ (NASA-TLX workload scores are ≥20% lower under MMP). A user study protocol designed to test H₁–H₃ is described; data collection is planned. The MMP framework and its implementation provide a replicable template for future BIM editor design.

---

## 1. Introduction

Professional AEC software — principally Autodesk AutoCAD, Revit, and Navisworks — is learned through extended practice that encodes not only declarative knowledge of features but procedural motor routines: the specific keystroke sequences, command abbreviations, and spatial pointing gestures that allow experienced practitioners to operate without conscious attention to the tool interface [Card et al. 1980; Fitts and Posner 1967]. These routines, once acquired, function as skilled motor memory that is robust to interruption within a familiar environment but fragile when the environment changes [Reason 1990].

The AEC industry is undergoing a sustained migration from proprietary file formats toward open IFC-based workflows and platform-independent editors. This migration creates a well-documented productivity valley: practitioners accustomed to AutoCAD's left-hand-on-keyboard command-alias paradigm encounter new environments whose interfaces are visually different, spatially reorganised, and — in most cases — designed from first principles rather than designed to preserve existing motor patterns.

Prior literature has studied this transition primarily through two lenses: feature-coverage analysis (which capabilities the new tool provides relative to the incumbent) and learning-curve measurement (how quickly new users reach proficiency). Neither lens isolates the motor-pattern disruption variable. A practitioner with ten years of AutoCAD experience and a practitioner with no prior CAD experience face categorically different adoption challenges when migrating to a new BIM editor, yet existing migration studies typically aggregate them under the same learning-curve framework. This aggregation obscures a structurally distinct problem: the experienced practitioner does not need to learn *what* the tool can do — they need to *unlearn* existing motor routines before learning the new ones, a process the motor-learning literature calls negative transfer [Schmidt and Lee 2014].

The Keystroke-Level Model (KLM) [Card, Moran, and Newell 1980] provides a theoretical foundation for quantifying the cost of command-interface changes. Each deviation from a practitioner's habituated keystroke sequence — a renamed command alias, a relocated toolbar button, a remapped function key — introduces what we term a *motor interference event*: a moment at which the practitioner's habituated motor program produces an incorrect action, requiring error detection and correction. At low frequencies, motor interference events are minor inconveniences. At the frequencies typical of professional AEC drafting — several hundred command invocations per hour — these events aggregate into measurable productivity loss, elevated error rates, and practitioner frustration that is often attributed incorrectly to the new tool's general quality rather than to its specific command-interface design choices.

This paper makes three contributions:

1. **A muscle-memory preservation (MMP) framework** — a principled taxonomy of motor-pattern preservation requirements for AEC desktop software migration, derived from the KLM, Fitts's Law [Fitts 1954], and an analysis of command-alias usage in professional AutoCAD and Navisworks practice.

2. **A reference implementation** — a prototype BIM authoring environment that instantiates the three MMP principles, with a documented command-alias table (18 core mappings), a layer-to-IFC-category assignment that preserves AutoCAD layer-panel appearance while enforcing IFC semantic precision, and a function-key binding matrix consistent with the AutoCAD F-key vocabulary across twelve keys.

3. **A user study protocol** — a within-subjects comparative study design for testing H₁–H₃ with experienced AEC professionals; the study is in participant recruitment phase.

The remainder of this paper is organised as follows. Section 2 reviews background on motor learning, the KLM, Fitts's Law, and existing BIM migration research. Section 3 states the three MMP design principles. Section 4 describes the prototype implementation. Section 5 presents the planned user study design. Section 6 reports preliminary results where available. Section 7 discusses implications, formal hypotheses, falsification conditions, and limitations. Section 8 concludes.

---

## 2. Background and Related Work

### 2.1 Motor Learning and Professional Tool Use

Skilled tool use is mediated by procedural memory — a form of long-term memory that encodes sequences of motor operations as unified routines executable rapidly and with minimal conscious attention [Squire 1992]. Professional software interfaces build procedural memory through extended practitioner use: an experienced AutoCAD drafter does not consciously recall that `L` initiates a LINE command; the keystroke is part of a motor program that activates automatically when the operator intends to draw a line segment.

The motor-learning literature distinguishes two stages of skill acquisition [Fitts and Posner 1967]: a *cognitive stage*, in which the practitioner consciously attends to each step in the sequence; and an *autonomous stage*, in which the sequence executes as a single habituated routine. Professional AEC software users operate in the autonomous stage for their core command vocabulary after approximately 200–500 hours of practice [Schmidt and Lee 2014]. Migration to a new environment that disrupts this vocabulary forces a partial regression to the cognitive stage — a regression that manifests as slowed command execution and elevated error rates even when the practitioner is fully capable of learning the new interface at the declarative level.

This phenomenon is studied under the heading of *negative transfer* [Proctor and Dutta 1995]: the prior habituated routine actively interferes with execution of the new pattern. The severity of negative transfer is proportional to the similarity of the two environments — tools that share some features but differ in others tend to produce more negative transfer than entirely novel tools, because the practitioner applies the old motor program in a partially familiar context before the mismatch is detected.

### 2.2 The Keystroke-Level Model

The KLM [Card, Moran, and Newell 1980] models human-computer task time as the sum of six operator types: K (keystroke, approximately 0.23 s per key for expert users), P (pointing, modelled by Fitts's Law), H (homing, approximately 0.4 s for hand movement between keyboard and pointing device), D (drawing), M (mental preparation, approximately 1.35 s), and R (system response time). For expert users performing well-practised tasks, the model predicts execution time to within approximately 20%.

Applied to command-alias disruption: if a practitioner's habituated motor program for initiating a LINE is K(L) + K(Enter) ≈ 0.46 s with no mental preparation (the command is fully automated), then a migration that renames the alias requires K(D) + K(R) + K(Enter) ≈ 0.69 s plus M(1.35 s) for the non-habituated sequence — a 4× slowdown on that specific sub-operation. For a practitioner who invokes LINE approximately 120 times per hour, the disruption cost for that single alias is approximately 127 seconds per hour of drafting. Multiplied across 18 core aliases, with the full 18-alias set disrupted simultaneously, the aggregate cost exceeds 30 minutes of productive drafting time per 8-hour session during the negative-transfer period.

### 2.3 Fitts's Law and Spatial Palette Habits

Fitts's Law [Fitts 1954] models pointing time as a function of target distance and target width: ID = log₂(2A/W), where A is the movement amplitude and W is the target width, and the pointing movement time is MT = a + b · ID for empirically fitted constants a and b. Toolbar buttons and palette entries at specific screen positions acquire Fitts-law motor programs tuned to those positions: the practitioner's pointing gesture is calibrated to the button's spatial location.

Repositioning a button — or replacing a spatially fixed toolbar with a context-sensitive ribbon that changes layout based on selection state — increases A for many targets, increasing pointing time for the same operation. More critically, it introduces pointing errors: the practitioner initiates a pointing gesture toward the old position, discovers the button has moved, and must visually search for the new location. This visual-search cost is not captured in the habituated motor program and represents a qualitatively different form of disruption than a renamed command alias.

### 2.4 AutoCAD Command Alias Usage Patterns

AutoCAD defines its default alias set in a plain-text configuration file (`acad.pgp`) that maps abbreviated inputs to full command names. Autodesk ships a default set of approximately 200 aliases [Autodesk 2024]; practitioner surveys and usage log analyses suggest that 15–25 aliases account for 70–80% of command invocations in typical drafting sessions [CITATION NEEDED: empirical alias usage study]. The productivity-critical core identified across practitioner accounts includes: LINE (`L`), PLINE (`PL`), CIRCLE (`C`), RECTANG (`REC`), ARC (`A`), HATCH (`H`), MOVE (`M`), COPY (`CO`), TRIM (`TR`), EXTEND (`EX`), OFFSET (`O`), MIRROR (`MI`), ROTATE (`RO`), SCALE (`SC`), FILLET (`F`), LAYER (`LA`), ZOOM (`Z`), and ERASE (`E`). These 18 aliases are issued under near-zero conscious attention by experienced practitioners; each one represents a candidate motor interference event in a migration to a non-preserving environment.

AutoCAD's function-key vocabulary is similarly well-established: F3 toggles Object Snap (OSNAP), F8 toggles Orthogonal mode, F10 toggles Polar Tracking, and F12 toggles Dynamic Input. These bindings are consistent across AutoCAD releases from version 2000 to the current release [Autodesk 2024] and are invoked reflexively — often mid-drawing-operation — by experienced practitioners.

### 2.5 BIM Adoption and Migration Research

The BIM adoption literature documents recurring patterns of productivity decline during migration from AutoCAD to Revit [Eastman et al. 2011]. These studies attribute the decline primarily to Revit's parametric constraint model, which differs conceptually from AutoCAD's direct-manipulation geometry paradigm. The command-interface dimension of the transition — the disruption of typed-alias habits and pointing-gesture habits — receives less systematic attention. Where it is mentioned, it is typically grouped with the general "different interface" factor rather than treated as a distinct variable with a quantifiable cost.

Reported productivity losses during BIM migration range from 20% to 40% of pre-migration throughput in the first three to six months [Eastman et al. 2011], with recovery trajectories that depend heavily on organisational training investment. These figures represent aggregate productivity loss across all disruption sources; disentangling the motor-pattern disruption component from the conceptual-model-change component and the feature-coverage-gap component requires a study design that holds two of the three variables constant — precisely the design we propose in Section 5.

### 2.6 Related Interface Preservation Approaches

Compatibility layers — software shims that intercept commands from one tool's vocabulary and translate them to another system's — have been used for operating-system migration (Wine for Windows application compatibility on Linux) and for programming language migration (compatibility modules). In CAD tooling, several alternative drafting applications expose an "AutoCAD compatibility mode" that accepts AutoCAD command syntax. These modes are documented as onboarding aids rather than as principled MMP frameworks; their coverage of the core alias set and function-key vocabulary is inconsistent, and they are typically implemented as surface syntax translators without the spatial layout and F-key preservation components.

Our approach differs from compatibility-layer shims in that the MMP principles are first-class design requirements instantiated natively from the ground up, not post-hoc additions. The distinction matters for spatial layout preservation (Principle 2), which cannot be achieved by a command-syntax translator without also controlling the visual layout of the host environment.

---

## 3. Muscle-Memory Preservation Design Principles

We define three principles for muscle-memory-preserving BIM desktop environments, derived from the theoretical analysis in Section 2.

### 3.1 Principle 1: Verbatim Command-Alias Preservation

Every command alias in the practitioner's habituated core vocabulary must map to the semantically equivalent operation in the new environment, using the identical abbreviation. Renaming or remapping any alias in the core set — even to a more intuitive name from a fresh-design perspective — introduces a motor interference event for each practitioner who has habituated the original alias.

The implementation corollary is that the alias set is defined declaratively in a configuration layer and applied consistently across all contexts in which commands can be entered. Aliases are not context-sensitive: an alias that functions in one mode but not another violates the practitioner's expectation. The alias dispatcher accepts both the abbreviated form and the full command name, consistent with AutoCAD's acad.pgp behaviour. Invalid inputs produce a notification in the command-line area without dismissing input focus or producing modal interruptions — again consistent with AutoCAD's error-handling convention.

This principle extends beyond abbreviation spelling to command *interaction sequence*: a practitioner who types `TR` (TRIM) expects to be prompted for cutting edges, then for the object to trim, in that order. Alias preservation without interaction-sequence consistency is insufficient; the motor program that `TR` triggers encodes the anticipation of the subsequent prompts.

### 3.2 Principle 2: Spatial Palette and Toolbar Layout Replication

Fitts's-Law-governed pointing habits are acquired at the granularity of button position, not button label. A practitioner who reaches for the HATCH command at a specific vertical position in the left toolbar will exhibit negative transfer when that button is repositioned, regardless of tooltip labels or visual similarity.

The MMP principle for spatial layout requires that the vertical order and grouping of tools in the palette replicate the practitioner's reference environment. For AutoCAD 2D drafting, this means: Draw commands grouped at the top of the left-side palette (LINE, PLINE, CIRCLE, RECTANG, ARC, HATCH in standard order); Modify commands in the second group (MOVE, COPY, TRIM, EXTEND, OFFSET, MIRROR, ROTATE, SCALE, FILLET, ERASE); the LAYER panel accessible from the top control bar with the same visual layout as AutoCAD's Layer Properties Manager.

A practical constraint applies in IFC-based environments: AutoCAD uses freeform layer names while IFC requires typed element category identifiers. The MMP solution preserves the *visual appearance* of the layer panel — name, colour swatch, visibility toggle, freeze/thaw state, lock state — while substituting IFC element type identifiers for the names in the underlying data model. What appears to the practitioner as a familiar list of named layers is, in the data representation, a list of IFC element categories (`IfcWall`, `IfcDoor`, `IfcWindow`, `IfcSlab`, etc.). This design achieves simultaneous objectives: the practitioner's spatial memory of the layer panel is preserved; IFC semantic precision is enforced by construction, with no freeform layer names permitted.

### 3.3 Principle 3: Function-Key Binding Preservation

AutoCAD's F-key vocabulary is invoked reflexively at the autonomous stage. Practitioners toggle OSNAP (F3) and Orthogonal mode (F8) without conscious attention; these keystrokes are integrated into the motor programs for drawing operations — for instance, initiating a LINE followed immediately by F8 to constrain the direction to horizontal or vertical. Remapping these bindings — or leaving them undefined, allowing the host operating system to capture them for system functions — produces frequent motor interference events in experienced practitioners.

The MMP binding matrix preserves the AutoCAD F-key vocabulary verbatim for F3, F8, F10, and F12. These four bindings account for the highest-frequency function-key invocations in typical drafting; preserving them eliminates the most common motor interference events in this category. The remaining F-key bindings (F1, F2, F6, F7, F9, F11) are assigned to analogous functions where AutoCAD equivalents exist and left available for practitioner configuration where AutoCAD uses them for infrequently invoked features.

---

## 4. Implementation

The three MMP principles are instantiated in a prototype BIM authoring environment targeting Phase 1 (2D drafting, AutoCAD motor-memory profile) and Phase 2 (3D coordination, Navisworks motor-memory profile). The prototype is structured around a web-rendered drawing surface embedded in a native desktop application container, with IFC geometry operations delegated to an IfcOpenShell [IfcOpenShell Contributors 2025] subprocess. This architecture is not specific to the MMP framework; it is a reasonable implementation choice for cross-platform BIM development that imposes no constraints on command-interface design.

### 4.1 Command Alias Dispatcher

The command alias dispatcher implements Principle 1. It exposes a persistent single-line command input at the bottom of the application window — identical in position to AutoCAD's command line — that accepts typed input at all times, regardless of drawing mode or selection state. The dispatcher resolves input against two levels:

1. The *alias table* (Table 1), which maps abbreviated inputs to canonical command identifiers.
2. The *command registry*, which maps canonical identifiers to handler functions.

**Table 1. Core command alias set.**

| Alias | Command | Equivalent AutoCAD alias |
|-------|---------|--------------------------|
| L | LINE | L |
| PL | PLINE | PL |
| C | CIRCLE | C |
| REC | RECTANG | REC |
| A | ARC | A |
| H | HATCH | H |
| M | MOVE | M |
| CO | COPY | CO |
| TR | TRIM | TR |
| EX | EXTEND | EX |
| O | OFFSET | O |
| MI | MIRROR | MI |
| RO | ROTATE | RO |
| SC | SCALE | SC |
| F | FILLET | F |
| LA | LAYER | LA |
| Z | ZOOM | Z |
| E | ERASE | E |

Input is case-insensitive. Both the abbreviated and full-command forms are accepted. Invalid inputs produce a notification in the command line area; they do not dismiss input focus or generate modal interruptions. The command line retains the last five commands in a scrollable history accessible by pressing the up arrow key, consistent with AutoCAD's command history behaviour.

### 4.2 Layer Panel as IFC Category Index

The LAYER panel (accessible via the `LA` alias or the top-bar Layer control) presents a list of element categories. Each row displays:

- A colour swatch (assigned per IFC element type; practitioner-adjustable)
- A human-readable category label (e.g. "Walls", "Doors")
- A visibility toggle (eye icon)
- A freeze toggle (sun/snowflake icon)
- A lock toggle (padlock icon)

The internal data model stores IFC element type identifiers against each row. New geometry drawn while a category is set as current receives the corresponding IFC type identifier at creation time; no freeform category names are permitted. The default category set instantiated on new-file creation is:

| Display name | IFC type |
|---|---|
| Walls | IfcWall |
| Doors | IfcDoor |
| Windows | IfcWindow |
| Floors | IfcSlab |
| Roofs | IfcRoof |
| Columns | IfcColumn |
| Beams | IfcBeam |
| Spaces | IfcSpace |
| Stairs | IfcStair |
| Furniture | IfcFurnishingElement |
| MEP — Mechanical | IfcDistributionFlowElement |
| MEP — Electrical | IfcElectricDistributionBoard |
| Annotations | IfcAnnotation |
| Grids | IfcGrid |

This mapping is extensible; practitioners may add custom categories from the IFC element type vocabulary. Freeform names are rejected by the validator to enforce semantic consistency across files.

### 4.3 Function-Key Binding Matrix

The function-key bindings are captured at the application level before host operating-system key handling. The full binding matrix:

| Key | Function | Status bar indicator |
|-----|----------|---------------------|
| F1 | Help overlay | — |
| F2 | Command history expand/collapse | — |
| F3 | Object Snap (OSNAP) toggle | OSNAP |
| F4 | Tablet emulation (reserved) | — |
| F5 | Isoplane cycle | ISOPLANE |
| F6 | Coordinate display toggle | COORDS |
| F7 | Grid display toggle | GRID |
| F8 | Orthogonal mode toggle | ORTHO |
| F9 | Snap-to-grid toggle | SNAP |
| F10 | Polar Tracking toggle | POLAR |
| F11 | Object Snap Tracking toggle | OTRACK |
| F12 | Dynamic Input toggle | DYN |

The status bar at the bottom of the screen displays persistent state indicators for OSNAP, ORTHO, POLAR, and DYN, consistent with AutoCAD's status bar layout. Each indicator is click-toggleable as well as keyboard-toggleable, matching AutoCAD's dual-input convention.

### 4.4 3D Coordination Viewport and Navisworks Navigation Grammar

The 3D coordination view adopts the Navisworks navigation grammar verbatim:

| Input gesture | Action |
|---------------|--------|
| Middle-button drag | Orbit (tumble rotation about scene centre) |
| Right-button drag | Pan (translate camera plane) |
| Scroll wheel forward | Zoom in (dolly) |
| Scroll wheel backward | Zoom out (dolly) |
| Left-click | Select element; Properties panel opens automatically |
| Shift + left-click | Additive selection |
| Escape | Clear selection |

The Properties panel opens on selection without a separate command, matching Navisworks behaviour. Properties displayed include: IFC element type, IFC GUID (the globally unique identifier assigned to each element per ISO 16739-1:2018 [ISO 2018]), name, description, and all populated IFC property sets from the source file. Practitioners accustomed to the Navisworks Properties panel encounter the same data fields at the same panel position.

### 4.5 BCF Issue Creation

BCF (BIM Collaboration Format) issue creation in the 3D view preserves the Navisworks workflow gesture sequence: right-click on a selected element or viewport area → "Create Issue" → dialog with title, description, and assignment fields → save produces a BCF 3.0-format [buildingSMART International 2020] record linked to the selected element's IFC GUID and the current camera viewpoint. A viewpoint snapshot is stored alongside the issue XML markup, consistent with the BCF 3.0 specification. The motor routine for BCF issue creation — right-click, complete dialog, confirm — requires no deviation from the practitioner's Navisworks habit.

### 4.6 IFC Authoring Data Flow and the Archive Commit Action

IFC geometry modifications in the 2D editing view flow through an IfcOpenShell subprocess rather than through direct in-process manipulation of IFC-SPF data. The authoring sequence is: (1) operator action in the drawing canvas; (2) application delegates the operation to IfcOpenShell via subprocess call; (3) IfcOpenShell applies the operation to a working draft IFC file in a temporary directory; (4) the drawing canvas re-renders from the updated draft. This design enforces that geometry modifications are always mediated by a standards-compliant IFC processor, preventing the accumulation of malformed IFC data that is common in direct-editing implementations.

When the operator is satisfied with the working draft, an explicit commit action moves the draft to a companion archive service's incoming queue for validation, SHA-256 sealing, and permanent archival. This archival step is an explicit operator action — it is not automatic and it is not triggered by the drawing environment. The F12 key, which in AutoCAD 2D mode toggles Dynamic Input (a display mode), is retained for that display-mode function; the archival action is assigned to a distinct, explicitly labelled control separate from the F-key vocabulary. This design separates the motor vocabulary of drawing operations (where F-key bindings are habituated and should not trigger irreversible actions) from the motor vocabulary of archival operations (which should require deliberate attention).

---

## 5. User Study

*TODO — study data collection not yet executed. Structured study design below.*

### 5.1 Study Design

A within-subjects counterbalanced comparative study. Each participant uses both the MMP prototype and a feature-equivalent non-MMP control environment (same IFC authoring capability; different command alias set, non-preserved spatial layout, remapped F-keys). Counterbalancing assigns half the participants to MMP-first, half to control-first.

**Participants:** AEC professionals with ≥3 years of sustained AutoCAD usage (≥10 hours per week). Recruitment target: 24 participants (power analysis for d = 0.6, α = 0.05, 1–β = 0.80 in a paired t-test). Exclusion criteria: participants who have used the specific prototype prior to the study; participants with ≥1 year of sustained use of any IFC-native BIM editor (to control for existing non-AutoCAD motor habits).

**Tasks:** Three representative drafting tasks, each approximately 20 minutes in duration:
- Task 1: Reproduce a floor plan from a paper reference drawing (2D AutoCAD-equivalent operations: LINE, PLINE, OFFSET, TRIM, FILLET, LAYER).
- Task 2: Annotate a provided IFC model with BCF issues identified from a defect checklist (3D Navisworks-equivalent operations: orbit, pan, select, Create Issue).
- Task 3: Edit IFC element properties for a set of 15 elements specified in a written brief (Revit property-editing equivalent: select, Properties panel, edit field).

**Measures:**
- Command-error rate: count of commands resulting in an error notification or an operator-visible undo within the task session (primary outcome for H₁).
- Task-completion time: total elapsed time from task start to operator submission (primary outcome for H₂).
- NASA-TLX [Hart and Staveland 1988] administered after each task block (primary outcome for H₃).
- Qualitative exit interview: semi-structured, approximately 20 minutes; topics include specific interface elements that caused confusion, comparison to prior AutoCAD experience, and impressions of the IFC layer-to-category mapping.

**Analysis:** Paired t-tests on command-error rate and task-completion time; Wilcoxon signed-rank test on NASA-TLX (non-parametric due to ordinal scale); thematic analysis on interview transcripts.

### 5.2 Ethical Clearance and Data Management

*TODO — ethical clearance application in preparation. Anticipated timeline: submission within 60 days of this writing pass; data collection to begin on clearance.*

---

## 6. Results

*TODO — pending user study execution. Section will report: mean command-error rate by condition; mean task-completion time by condition; NASA-TLX total workload by condition; qualitative themes. Preliminary implementation coverage metrics (number of aliases correctly mapped, F-key binding test results) to be added here once formal test protocol is executed against the prototype.*

---

## 7. Discussion

### 7.1 Implications of the MMP Framework

The MMP framework established in this paper rests on a straightforward empirical claim: that motor-pattern disruption is a quantifiable, structurally distinct component of BIM migration cost, and that it can be substantially reduced by deliberate design choices that are low-cost to implement. Verbatim alias mapping is a configuration decision; spatial layout replication is a UI design decision constrained by the reference environment rather than by fresh-design intuition; function-key binding preservation is primarily a priority decision about which keystrokes the application captures before the host operating system.

The IFC layer-to-category mapping described in Section 4.2 represents the most substantive MMP design challenge: it requires resolving a tension between the practitioner's expectation of freeform layer names and the IFC model's requirement for typed element category identifiers. The solution adopted here — preserving visual appearance while substituting semantic precision in the data model — is one approach; others, including a vocabulary-translation layer that maps common freeform names to IFC types on import, may be viable alternatives. Future work should evaluate whether practitioners notice the semantic substitution during use or whether the visual consistency is sufficient for full motor-pattern transfer.

### 7.2 Formal Hypotheses

**H₁ (Motor-Error Reduction):** A muscle-memory-preserving BIM desktop environment, as defined by the three MMP principles in Section 3, reduces the command-error rate of experienced AutoCAD practitioners (≥3 years, ≥10 h/week) by ≥25% compared to a feature-equivalent non-MMP alternative, in a 20-minute task session designed around the 18-alias core command set.

**H₀ (Null Hypothesis):** There is no statistically significant difference in command-error rate between the MMP and non-MMP environments in the target practitioner population.

**H₂ (Task-Completion Time):** The MMP environment reduces mean task-completion time by ≥30% compared to the non-MMP alternative in the first 10 hours of use, as measured across three representative drafting tasks.

**H₃ (Perceived Workload):** NASA-TLX total workload scores are ≥20% lower in the MMP condition than in the non-MMP condition, as reported by practitioners with ≥3 years AutoCAD experience.

### 7.3 Falsification Programme

The following tests constitute the falsification programme for H₁–H₃. An adequately powered study that produces results outside the stated bounds falsifies the corresponding hypothesis.

| Test | Condition | Falsification criterion |
|------|-----------|------------------------|
| T1 | H₁: command-error rate | Mean error rate difference MMP vs. non-MMP < 25%, or p > 0.05 in paired t-test |
| T2 | H₁ specificity | Error rate reduction concentrated in alias-vocabulary tasks, not present in non-alias tasks |
| T3 | H₂: task-completion time | Mean time difference < 30%, or p > 0.05 |
| T4 | H₂ temporal decay | Time difference does not decrease substantially between session 1 and session 5 (if H₂ is a novelty effect, it will decay) |
| T5 | H₃: NASA-TLX | Mean TLX score difference < 20%, or p > 0.05 on Wilcoxon signed-rank |
| T6 | Transfer maintenance | Error rate reduction persists at 4-week follow-up without degradation to non-MMP levels |
| T7 | F-key isolation | F-key remapping alone (without alias or spatial changes) accounts for ≤10% of total error-rate improvement in a partial-MMP condition |

### 7.4 Limitations

Five limitations bound the planned study's generalisability:

1. **Single-domain sample.** Participant recruitment targets AEC professionals. The MMP framework is stated in domain-general terms (it applies to any professional tool migration with a well-established command vocabulary) but empirical validation in this study is AEC-specific.

2. **Controlled laboratory conditions.** The study tasks are bounded in duration and scope. Real-world drafting sessions involve longer, iterative workflows, interruption, and multi-session task management. Laboratory task-completion time may underestimate the real-world benefit of alias preservation for extended sessions.

3. **Single MMP configuration.** The study tests one specific implementation of the MMP principles (the 18-alias set, one spatial layout, one F-key matrix). Practitioners with different habituated alias vocabularies — for instance, those who have customised their `acad.pgp` file — may experience different outcomes. Personalisation of the alias set is a natural extension of the framework but is not tested in this study.

4. **Counterbalancing latency.** The within-subjects design involves exposing participants to both conditions. Even with counterbalancing and an intervening washout period, learning effects from the first condition may influence performance in the second. A parallel between-subjects design would eliminate this concern at the cost of requiring approximately double the participant count.

5. **No long-term productivity data.** The study measures performance within the first use session. Long-term outcomes — whether the MMP advantage persists, increases, or decays as participants acquire new motor programs for the new environment — require longitudinal measurement beyond the scope of this study.

---

## 8. Conclusion

This paper has identified motor-pattern disruption as a structurally distinct and quantifiable component of BIM migration cost, proposed a three-principle muscle-memory preservation (MMP) framework for desktop editor design, and described a prototype implementation that instantiates all three principles. The implementation provides a 18-alias command dispatcher, a layer-to-IFC-category mapping that preserves visual conventions while enforcing semantic precision, and a function-key binding matrix consistent with the AutoCAD F-key vocabulary.

The KLM analysis in Section 2.2 establishes that the aggregate cost of alias disruption across the 18-alias core set can exceed 30 minutes of productive drafting per session during the negative-transfer period — a cost that is entirely avoidable through the design choices described in this paper. Whether the empirical reduction is as large as H₁ predicts (≥25% error-rate reduction) or smaller, the framework provides a principled basis for evaluating command-interface decisions in future BIM editor design.

The planned user study will provide the first controlled empirical measurement of the MMP framework's effect on command-error rate, task-completion time, and perceived workload in experienced AEC professionals. Results will be reported in a subsequent publication; the present paper establishes the theoretical foundation, implementation reference, and study protocol on which that measurement rests.

---

## AI Use Disclosure

The authors used claude-sonnet-4-6 (Anthropic) to assist with manuscript drafting. All substantive intellectual content — research design, hypotheses, implementation decisions, theoretical analysis — reflects the authors' original work. The manuscript was reviewed and revised under the authors' editorial direction. This disclosure is made in accordance with COPE guidelines (2024) on the use of AI tools in academic publishing.

## CRediT Contributor Roles

**Jennifer M. Woodfine:** Conceptualization, Methodology, Formal Analysis, Writing – Original Draft, Writing – Review & Editing.
**Peter M. Woodfine:** Conceptualization, Validation, Writing – Review & Editing.
**Mathew Woodfine:** Software, Writing – Review & Editing.

## Conflict of Interest

The authors declare no conflict of interest.

## Funding

No external funding received.

## Data Availability

Study protocol, data collection instruments, and anonymised response data will be made available upon journal acceptance via an open data repository.

---

## References

Autodesk. 2024. *AutoCAD 2025 Command Reference.* San Rafael, CA: Autodesk, Inc.

buildingSMART International. 2020. *BIM Collaboration Format (BCF) Version 3.0.* Oslo: buildingSMART International.

Card, Stuart K., Thomas P. Moran, and Allen Newell. 1980. "The Keystroke-Level Model for User Performance Time with Interactive Systems." *Communications of the ACM* 23 (7): 396–410.

Eastman, Charles, Paul Teicholz, Rafael Sacks, and Kathleen Liston. 2011. *BIM Handbook: A Guide to Building Information Modeling for Owners, Managers, Designers, Engineers, and Contractors.* 2nd ed. Hoboken, NJ: Wiley.

Fitts, Paul M. 1954. "The Information Capacity of the Human Motor System in Controlling the Amplitude of Movement." *Journal of Experimental Psychology* 47 (6): 381–391.

Fitts, Paul M., and Michael I. Posner. 1967. *Human Performance.* Belmont, CA: Brooks/Cole.

Hart, Sandra G., and Lowell E. Staveland. 1988. "Development of NASA-TLX (Task Load Index): Results of Empirical and Theoretical Research." In *Human Mental Workload,* edited by Peter A. Hancock and Najmedin Meshkati, 139–183. Amsterdam: Elsevier.

IfcOpenShell Contributors. 2025. *IfcOpenShell: Open Source IFC Library and Geometry Engine.* Version 0.8.5. Available: https://ifcopenshell.org

ISO (International Organization for Standardization). 2018. *ISO 16739-1:2018 — Industry Foundation Classes (IFC) for data sharing in the construction and facility management industries — Part 1: Data schema.* Geneva: ISO.

Proctor, Robert W., and Addie Dutta. 1995. *Skill Acquisition and Human Performance.* Thousand Oaks, CA: SAGE Publications.

Reason, James. 1990. *Human Error.* Cambridge: Cambridge University Press.

Schmidt, Richard A., and Timothy D. Lee. 2014. *Motor Learning and Performance: From Principles to Application.* 5th ed. Champaign, IL: Human Kinetics.

Squire, Larry R. 1992. "Memory and the Hippocampus: A Synthesis from Findings with Rats, Monkeys, and Humans." *Psychological Review* 99 (2): 195–231.

---

*Version 0.2 — writing pass 2026-05-28*
*§1–§5 body written (~5,200 words); §6 Results pending user study execution*
*Forbidden vocabulary cleared; language pass complete*
