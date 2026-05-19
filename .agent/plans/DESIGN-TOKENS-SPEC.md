# DESIGN-TOKENS-SPEC.md

> **Purpose:** To define a comprehensive set of design tokens for the PointSav platform, enabling customers and community members to create Wikipedia-style articles with full functional and aesthetic parity.
> **Scope:** Encompasses all identified UI elements (Platform Tokens) and content structures (Article Tokens) derived from the MediaWiki Vector 2022 analysis.
> **Goal:** To provide `project-design` with clear specifications for a new section on `design.pointsav.com` where these tokens can be accessed, documented, and utilized.

---

## 1. Token Philosophy

PointSav design tokens are semantic, reusable units that abstract away the complexity of UI and content structure. They are the building blocks for creating articles that mirror Wikipedia's fidelity while leveraging our unique "flat software" and "Leapfrog 2030" architecture. These tokens should be:
*   **Declarative:** Define *what* the element is, not *how* it's implemented.
*   **Composable:** Allow for flexible assembly into complex page layouts.
*   **Versioned:** Trackable and manageable within the design system.

---

## 2. Token Categories

### 2.1 Platform UI Tokens (The Application Shell)

These tokens define the core interface elements of the `app-mediakit-knowledge` application.

*   **Global Header (`.mw-header`):**
    *   Logo (`a.mw-logo`): Site wordmark/icon.
    *   Search (`form#searchform`): Input, Autocomplete component, Shortcut handler.
    *   User Tools (`nav#p-personal`): Logged Out (Donate, Create Account, Log in), Logged In (Profile, Alerts, Notices, Preferences, Watchlist, Logout).
    *   Language Switcher (`#p-lang-btn`): Globe icon, Language list modal.
    *   Appearance Menu: Theme toggles (Day/Night/OS), Font Size, Width controls.
*   **Main Menu (Left Sidebar - `.vector-main-menu`):**
    *   Pinnable State: Fixed sidebar vs. Hamburger menu.
    *   Navigation Section: Main page, Contents, Random article, etc.
    *   Contribute Section: Help, Learn to edit, Recent changes, etc.
    *   In Other Languages List.
*   **Table of Contents (TOC - `.vector-toc`):**
    *   Pinnable State: Left sidebar vs. Floating toggle.
    *   Hierarchical Levels (Numbered): 1, 1.1, 1.2...
    *   Interactive Toggles: Collapsible subsections.
    *   Active Section Tracking.
*   **Page Header & Toolbar (`.vector-page-toolbar`):**
    *   Namespace Tabs: Article, Talk.
    *   View Tabs: Read, Edit, View history.
    *   "More" Dropdown: Move, Watch (Star icon), Protect.
*   **Page Tools (Right Sidebar - `#vector-page-tools`):**
    *   Pinnable State: Sidebar vs. Header icon.
    *   Actions: What links here, Related changes, Upload file, Special pages, Permanent link, Page information, Cite this page.
    *   Print/Export: Download as PDF, Printable version.
*   **Interactive Elements:**
    *   Keyboard Shortcuts: `/`, `Alt+Shift+F/E/H/W/M/X`.
    *   Sticky Header behavior.
    *   Hover Previews (AJAX Tooltips).

### 2.2 Article Content Tokens (The Page Structure)

These tokens define the structured components within an article's body and metadata.

*   **Archetype-Specific Infoboxes:**
    *   `{{Infobox-Person}}`: For biographies.
    *   `{{Infobox-Settlement}}`: For cities, regions, geographical locations.
    *   `{{Infobox-Company}}`: For corporate entities.
    *   `{{Infobox-ProgrammingLanguage}}`: For programming languages.
    *   *(Future: Expandable based on further audits)*
*   **Structural & Data Tokens:**
    *   `{{Navbox-*}}`: Collapsible footer navigation.
    *   `{{Timeline}}` / `{{Timeline-Row}}`: For chronological data representation.
    *   `{{Citation-Cluster}}` / `{{reflist}}` / `{{notelist}}`: Standardized citation management.
    *   `{{Math-Formulae}}` / `{{Chemical-Formulae}}`: For rendering complex scientific notation (LaTeX-like).
    *   `{{Diagram-Row}}`: For visual explanations.
    *   `{{Physical-Property-Table}}`: Standardized layout for scientific constants.
    *   `{{Demographics-Table}}`: For population and census data.
    *   `{{Financial-Table}}`: For economic and corporate reporting data.
    *   `{{Map-Embed}}`: For geospatial articles.
    *   `{{Logo-Embed}}`: For corporate identity.
    *   `{{Code-Block}}` / `{{Syntax-Highlight}}`: For programming language and technical articles.
*   **Metadata & Linking Tokens:**
    *   `{{Authority control}}`: For library and index metadata.
    *   `{{See Also}}` Section: Standardized linking to related articles.
    *   `{{External Links}}` Section: Canonical source linking.
    *   Redlink Handling: Explicitly flagged links to non-existent pages.
*   **Interactive Content Elements:**
    *   Collapsible Tables: Mechanism for hiding/showing large data sets.
    *   Bidirectional References: "Jump-back" functionality from citation footer to in-text marker.

---

## 3. Presentation Guidelines for design.pointsav.com

The `project-design` team should create a dedicated section on `design.pointsav.com` for these tokens. This section should include:

*   **Token Library:** A clear listing of all available tokens, categorized by Platform UI and Article Content Archetypes.
*   **Visual Examples:** For each token, provide a visual representation of its default state and any interactive states (e.g., hover, collapsed).
*   **Usage Documentation:** Clear instructions on when and how to use each token, including required parameters and examples.
*   **API/Integration Notes:** Basic information on how these tokens are consumed by the `app-mediakit-knowledge` engine (e.g., Markdown syntax, frontmatter requirements).
*   **Customer/Community Contribution Guidelines:** How external users can suggest new tokens or variations.

---

## 4. Instructions for `project-design`

1.  **Create New Section:** Establish a dedicated area on `design.pointsav.com` for "PointSav Article Tokens" or similar.
2.  **Implement Token Library:** Populate this section with the Platform UI and Article Content Tokens listed above.
3.  **Define Visuals & Interactions:** For each token, create visual mockups and document interactive behaviors (collapsibility, hover effects, etc.).
4.  **Develop Usage Guides:** Write clear, concise documentation for each token, suitable for non-technical users.
5.  **Establish Contribution Workflow:** Outline how community members can propose new tokens or improvements.

---
