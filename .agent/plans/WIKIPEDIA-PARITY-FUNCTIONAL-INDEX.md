# WIKIPEDIA-PARITY-FUNCTIONAL-INDEX.md

> **Purpose:** A 100% comprehensive inventory of the MediaWiki Vector 2022 functional surface area.
> **Scope:** Every menu, toolbar, system page, and interactive behavior required to achieve "muscle memory" parity.
> **Constraint:** This is the master checklist. Implementation may prioritize or defer, but must first acknowledge the existence of these elements.

---

## 1. Structural Navigation (The Containers)

### 1.1 Global Header (`.mw-header`)
- **Logo Area:** `a.mw-logo` with site name/icon.
- **Search Container:** Centralised search bar with real-time API feedback.
- **User Tools:** Personal menu (Login/Account or Profile/Settings).
- **Secondary Actions:** "Donate" (Wikipedia specific), "Create account", "Log in".

### 1.2 Main Menu (Sidebar Left - `.vector-main-menu`)
- **Pinnable State:** Can be "Pinned" (fixed sidebar) or "Unpinned" (hamburger menu).
- **Navigation Section:**
  - Main page
  - Contents
  - Current events
  - Random article
  - About Wikipedia (PointSav: About Wiki)
  - Contact us
- **Contribute Section:**
  - Help
  - Learn to edit (PointSav: Style Guide)
  - Community portal
  - Recent changes
  - Upload file
- **In Other Languages:** Dynamic list of translated siblings.

### 1.3 Table of Contents (TOC - `.vector-toc`)
- **Pinning:** Fixed in left sidebar vs. Floating button.
- **Hierarchical Levels:** Numbers (1, 1.1, 1.2) + Text.
- **Interactive Toggles:** Collapsible sub-sections with chevron icons.
- **Active Tracking:** Highlights the section currently in viewport.

### 1.4 Page Header & Toolbar (`.vector-page-toolbar`)
- **Namespaces (Left):** "Article" (Page) and "Talk" (Discussion).
- **Views (Right):** "Read", "Edit", "View history".
- **More Menu:** Dropdown containing "Move", "Watch" (Star icon), "Protect".

### 1.5 Page Tools (Sidebar Right - `#vector-page-tools`)
- **Pinnable State:** Pinned (sidebar) vs. Unpinned (header icon).
- **Actions Section:** (Duplicate of Views for accessibility).
- **General Section:**
  - What links here
  - Related changes
  - Upload file
  - Special pages
  - Permanent link
  - Page information
  - Cite this page
  - Get shortened URL
- **Print/Export:**
  - Download as PDF
  - Printable version

---

## 2. Interactive Behaviors (The Experience)

### 2.1 Search Intelligence
- **Typeahead:** AJAX dropdown with titles + snippets + thumbnails.
- **Search-for vs. Go-to:** Direct match jumps to page; no match opens search results.
- **Keyboard Shortcut:** `/` key focuses the search input.

### 2.2 Sticky Header
- **Trigger:** Activation when the main header scrolls off-screen.
- **Persistent Elements:** Site wordmark, Article title, Search bar, Read/Edit/History actions.

### 2.3 Appearance & Themes
- **Day/Night/OS:** Preference stored in `localStorage` or user profile.
- **Font Size:** Standardized Vector 2022 sizing (`0.875rem` body).
- **Limited Width:** Max-width discipline (960px) vs. Full-width toggle.

### 2.4 Hover Previews
- **AJAX Tooltips:** Hovering a wikilink shows a card with title, snippet, and image.
- **Timing:** ~200ms delay before appearance to avoid flicker.

---

## 3. System & Special Pages

### 3.1 Special:Login & Special:CreateAccount
- **Fields:** Username, Password, CAPTCHA (if applicable), Remember me checkbox.
- **Flow:** Validation -> Redirect back to original page (returnto).

### 3.2 Special:RecentChanges
- **Filters:** Group by page, Hide minor, Hide bots.
- **Legend:** (N) New, (m) minor, (b) bot.

### 3.3 Special:Search
- **Results:** Title, Snippet, Last modified date.
- **Advanced:** Filter by category/namespace.

### 3.4 Special:AllPages & Special:Categories
- Alphabetical directory and category cloud/list.

---

## 4. Keyboard Shortcuts (Standard MW)
- `/`: Search focus.
- `Alt+Shift+F`: Search focus.
- `Alt+Shift+E`: Edit.
- `Alt+Shift+H`: History.
- `Alt+Shift+W`: Watch.
- `Alt+Shift+M`: Move.
- `Alt+Shift+X`: Random page.
