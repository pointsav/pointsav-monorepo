---
schema: foundry-draft-v1
state: draft-pending-language-pass
language_protocol: PROSE-GUIDE
originating_cluster: project-design
target_repo: woodfine/woodfine-fleet-deployment
target_path: <tbd-by-project-editorial>
target_filename: guide-design-system-help-overview.md
audience: customer-public
bcsc_class: current-fact
authored: 2026-05-08T00:00:00Z
authored_by: task@project-design
authored_with: claude-sonnet-4-6
research_done_count: 2
research_suggested_count: 1
open_questions_count: 0
research_provenance: tacit
research_inline: true
notes_for_editor: |
  Vault stub is live at https://design.pointsav.com/help/overview/.
  Language pass: Bloomberg standard. The label taxonomy (design, token, component,
  bug, question) is proposed here but not yet set up in the GitHub repository —
  flag to operator to create these labels before the guide is published.
  "GitHub Discussions" referenced as conditional — verify whether Discussions is
  enabled on the vendor repository before publishing.
---

## Research trail

### Done — what informed this draft
- [tacit: vault stub help/overview.md] — skeleton content
- [tacit: app-privategit-design CLAUDE.md] — /healthz confirmed as version endpoint

### Suggested — what project-editorial should consult
- [pointsav/pointsav-design-system GitHub] — verify issue label taxonomy is
  actually configured; correct the guide if labels differ (medium priority)

---

# Help

## Getting support

File an issue on the design system GitHub repository. Include:

- The component or token name (use the exact slug from the sidebar)
- A screenshot showing the problem
- Your substrate version — visible at `/healthz` on your instance under `version`
- Browser name and version
- Operating system

If you are working from a forked vault, file the issue on your fork first. If you
believe the issue exists in the vendor vault, open a corresponding issue on the
vendor repository.

**Issue label guidance:**

| Label | When to use |
|---|---|
| `design` | Visual decisions: colour, spacing, typography, component anatomy |
| `token` | A token value, name, or alias is incorrect or missing |
| `component` | A component recipe, HTML structure, or ARIA pattern is incorrect |
| `bug` | The substrate binary returns an error or behaves unexpectedly |
| `question` | You need clarification before filing a specific issue |

---

## Common questions

**Where are my tokens?**
Token files live in `vault/tokens/` in your forked repository. The primitive layer
is in `primitive.json`. Semantic overrides for your brand theme are in
`themes/<your-tenant>.json`. Both files load at startup; restart the substrate to
pick up edits.

**How do I add a component?**
Add a directory under `vault/components/<slug>/` containing a `recipe.json` file
and tab markdown files (`usage.md`, `style.md`, `code.md`, `accessibility.md`).
The substrate reads new component directories automatically on restart — no rebuild
required.

**How do I update vault content without rebuilding?**
The vault is read once when the substrate starts. Edit the vault files in your Git
repository, commit, pull to the server, and restart the systemd unit:

```bash
sudo systemctl restart local-design
```

Changes to the binary (CSS, JavaScript, Rust source) require rebuilding the binary
and redeploying it. Vault content changes (markdown, token JSON, recipes) require
only a restart.

**How do I check my substrate version?**
```bash
curl -sS http://127.0.0.1:9094/healthz | jq .version
```

Or visit `/healthz` in your browser on the live domain.

**Is there a GitHub Discussions forum?**
If GitHub Discussions is enabled on the repository, it is the preferred venue for
usage questions, design feedback, and requests for new components. Check the
repository's Discussions tab. If Discussions is not enabled, use GitHub Issues
with the `question` label.

---

## Out of scope

This design system substrate does not include:

- SaaS hosting — you run the binary on your own server
- Custom domain management — your DNS and nginx configuration handle this
- Figma plugin support — use Tokens Studio for token sync; the substrate is
  editor-agnostic at the protocol layer (DTCG)
- Automatic upgrades — you control when to pull vendor changes into your fork
