// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Structured parsing of "Index Topic" bodies — a curated per-category hub
//! page (`_index.md`/`_index.es.md` with `index_type` set) whose body encodes
//! a highlighted "start here" pick plus grouped, annotated member links
//! inside HTML-comment-delimited blocks a generator script regenerates on
//! commit.
//!
//! Markers are matched as substrings, not full literal comment text — the
//! real comment text varies (e.g. a `regenerate from index_group: <slug>`
//! tail) and the real `START-HERE-HIGHLIGHT` opening comment spans two
//! physical source lines. Any malformed or unexpected shape (missing END
//! marker, a block with no wikilink where one is required, nothing
//! structured found at all) bails to `None` — the caller falls back to
//! rendering the body as an ordinary article.
//!
//! **Must be called with comment-intact body text** — i.e. from
//! `frontmatter::parse_raw` / `walk::load_raw`, not `frontmatter::parse`
//! (which strips all HTML comments, the markers included, for ordinary
//! articles' search-index/summary hygiene).
//!
//! Deliberately does not track fenced code blocks (unlike
//! `render::extract_headings`/`strip_heading_attrs`, which must — ordinary
//! articles routinely contain code examples). An Index Topic is a curated
//! link-index page; a marker string coincidentally appearing inside a code
//! fence in one is realistic enough to not crash on, but not worth the extra
//! state-tracking to specifically handle — worst case, parsing bails to
//! `None` and the page renders as a plain article instead of mis-rendering.

use super::render::{render, slugify, strip_trailing_heading_attr};

const START_HERE_OPEN: &str = "START-HERE-HIGHLIGHT";
const START_HERE_CLOSE: &str = "END-START-HERE-HIGHLIGHT";
const GROUP_OPEN: &str = "AUTO-GENERATED MEMBERSHIP";
const GROUP_CLOSE: &str = "END AUTO-GENERATED";

/// The single "start here" highlight: the target article plus the
/// surrounding prose. The wikilink stays inline in `prose_html` (rendered
/// as-authored); `href`/`label` are the same link, extracted separately so
/// the renderer can make the whole card clickable / show a distinct title.
#[derive(Debug, Clone)]
pub struct StartHere {
    pub href: String,
    pub label: String,
    pub prose_html: String,
    /// Slug the link resolves to (post-slugify) — lets a caller with index
    /// access look up the real article title when `explicit_label` is false.
    pub slug: String,
    /// `false` for a bare `[[slug]]` (no `|label`) — `label` is then just the
    /// raw bracket text, not necessarily the article's real title.
    pub explicit_label: bool,
}

/// One curated member link under a group.
#[derive(Debug, Clone)]
pub struct Member {
    pub href: String,
    pub label: String,
    pub annotation_html: String,
    /// See `StartHere::slug`.
    pub slug: String,
    /// See `StartHere::explicit_label`.
    pub explicit_label: bool,
}

/// One H2 group: a heading followed (after an optional short prose intro) by
/// an `AUTO-GENERATED MEMBERSHIP` block.
#[derive(Debug, Clone)]
pub struct Group {
    pub title: String,
    pub intro_html: Option<String>,
    pub members: Vec<Member>,
}

impl Group {
    /// Live count of members — never trust the embedded `{#group-count-N}`
    /// heading attribute (an external generator script's own bookkeeping,
    /// which can drift); this is the real, self-verifying count.
    pub fn count(&self) -> usize {
        self.members.len()
    }
}

/// A parsed Index Topic body.
#[derive(Debug, Clone)]
pub struct IndexTopic {
    pub intro_html: String,
    pub start_here: Option<StartHere>,
    pub groups: Vec<Group>,
    pub tail_html: String,
}

/// Parse an Index Topic body. Returns `None` on any malformed/unexpected
/// shape, including a body with neither a start-here block nor any
/// recognizable group (nothing structured found — most likely a stub
/// `_index.md` that got `index_type` set without real content yet).
pub fn parse_index_topic(raw_body_md: &str) -> Option<IndexTopic> {
    let text = raw_body_md;
    let mut intro_end = text.len();
    let mut cursor = 0usize;
    let mut start_here = None;

    if let Some((sh_marker_pos, sh_content_start)) =
        marker_end(text, 0, text.len(), START_HERE_OPEN)
    {
        intro_end = sh_marker_pos;
        let (sh_close_marker_pos, sh_close_end) =
            marker_end(text, sh_content_start, text.len(), START_HERE_CLOSE)?;
        let block_md = text[sh_content_start..sh_close_marker_pos].trim();
        let (target, label, explicit_label) = first_wikilink(block_md)?;
        let slug = slugify(&target);
        start_here = Some(StartHere {
            href: format!("/wiki/{slug}"),
            label,
            prose_html: render(block_md).html,
            slug,
            explicit_label,
        });
        cursor = sh_close_end;
    }

    let mut groups = Vec::new();
    let mut intro_end_set = start_here.is_some();
    while let Some((h_start, h_after, raw_title)) = next_h2(text, cursor) {
        if !intro_end_set {
            intro_end = h_start;
            intro_end_set = true;
        }
        let next_bound = next_h2(text, h_after).map(|(s, _, _)| s).unwrap_or(text.len());
        let Some((group_marker_pos, group_content_start)) =
            marker_end(text, h_after, next_bound, GROUP_OPEN)
        else {
            // No AUTO-GENERATED block follows before the next heading (or
            // EOF) — this H2 isn't a group. Stop structured parsing here;
            // everything from `h_start` onward becomes tail content.
            break;
        };
        let group_intro = text[h_after..group_marker_pos].trim();
        let (group_close_pos, group_close_end) =
            marker_end(text, group_content_start, next_bound, GROUP_CLOSE)?;
        let members_md = &text[group_content_start..group_close_pos];
        let members = parse_members(members_md)?;
        groups.push(Group {
            title: strip_trailing_heading_attr(raw_title.trim()).trim().to_string(),
            intro_html: (!group_intro.is_empty()).then(|| render(group_intro).html),
            members,
        });
        cursor = group_close_end;
    }

    if start_here.is_none() && groups.is_empty() {
        return None;
    }

    let tail_md = &text[cursor.min(text.len())..];
    Some(IndexTopic {
        intro_html: render(&text[..intro_end.min(text.len())]).html,
        start_here,
        groups,
        tail_html: render(tail_md).html,
    })
}

/// Byte offset of the next H2 ATX heading (`## `) at or after `from`, plus
/// the offset right after its line and its trimmed title text.
fn next_h2(text: &str, from: usize) -> Option<(usize, usize, &str)> {
    let mut pos = from;
    for raw_line in text[from..].split_inclusive('\n') {
        let line_start = pos;
        let line_end = pos + raw_line.len();
        let trimmed = raw_line.trim_end_matches(['\n', '\r']);
        let t = trimmed.trim_start();
        if let Some(rest) = t.strip_prefix("## ") {
            let title = rest.trim();
            if !title.is_empty() {
                return Some((line_start, line_end, title));
            }
        }
        pos = line_end;
    }
    None
}

/// Search `text[from..end]` for `marker`; if found, scan forward (unbounded —
/// a well-formed comment's `-->` always follows shortly after, even for the
/// one marker whose opening comment spans two lines) for the closing `-->`.
/// Returns `(comment_open, position_right_after_close)` — `comment_open` is
/// the position of the enclosing `<!--`, not of `marker` itself, so a caller
/// slicing "content before this comment" doesn't leak a trailing `<!-- `
/// fragment. `None` if the marker isn't present in range, or has no closing
/// `-->` at all (an unterminated marker — the caller must bail, never scan to
/// EOF regardless).
fn marker_end(text: &str, from: usize, end: usize, marker: &str) -> Option<(usize, usize)> {
    let window = text.get(from..end)?;
    let marker_pos = from + window.find(marker)?;
    let comment_open = text[from..marker_pos]
        .rfind("<!--")
        .map(|p| from + p)
        .unwrap_or(marker_pos);
    let close = marker_pos + text[marker_pos..].find("-->")? + 3;
    Some((comment_open, close))
}

/// The first `[[target]]` or `[[target|label]]` in `text` — same convention
/// as `render::resolve_wikilinks`.
fn first_wikilink(text: &str) -> Option<(String, String, bool)> {
    let start = text.find("[[")?;
    let rest = &text[start + 2..];
    let close = rest.find("]]")?;
    let inner = &rest[..close];
    let (target, label, explicit_label) = match inner.split_once('|') {
        Some((t, l)) => (t.trim(), l.trim(), true),
        None => (inner.trim(), inner.trim(), false),
    };
    if target.is_empty() {
        return None;
    }
    Some((target.to_string(), label.to_string(), explicit_label))
}

/// Parse a `- [[target|label]] — annotation` bullet list into `Member`s.
/// Blank lines are skipped; a non-bullet stray line is tolerated (skipped,
/// not fatal). A bullet line with no wikilink at all fails the whole parse
/// (`?` inside the loop) — that's a real authoring defect, not noise to
/// silently drop. Returns `None` (not `Some(vec![])`) if no members were
/// found at all — an empty group is itself a malformed shape.
fn parse_members(text: &str) -> Option<Vec<Member>> {
    let mut members = Vec::new();
    for line in text.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let Some(rest) = t.strip_prefix("- ") else {
            continue;
        };
        let (target, label, explicit_label) = first_wikilink(rest)?;
        let slug = slugify(&target);
        let link_end = rest.find("]]").map(|i| i + 2).unwrap_or(0);
        let after_link = rest[link_end..].trim();
        let annotation_md = after_link.strip_prefix('\u{2014}').unwrap_or(after_link).trim();
        members.push(Member {
            href: format!("/wiki/{slug}"),
            label,
            annotation_html: render(annotation_md).html,
            slug,
            explicit_label,
        });
    }
    if members.is_empty() {
        None
    } else {
        Some(members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real reference content (`security/_index.md` in
    /// `media-knowledge-documentation`, as of 2026-08-04) — verified against
    /// the actual file, not a simplified stand-in, so this test catches real
    /// shape mismatches, not just idealized ones.
    const REAL_SECURITY_INDEX_BODY: &str = concat!(
        "**This page indexes the 13 articles in the security category.** It covers identity and\n",
        "permissions, cryptographic verification, isolation boundaries, how data is handled and kept\n",
        "private, and the supply-chain controls designed to keep code honest from contributor to\n",
        "production.\n",
        "\n",
        "This is the platform's answer to the diligence reader's question — *can this be trusted?* — and\n",
        "the front door for engineers looking up a specific security mechanism: capability-based access\n",
        "control, machine-based authentication, attestation, cryptographic ledgers, and the\n",
        "trust-establishment bootstrap a device passes through before it joins a deployment.\n",
        "\n",
        "<!-- START-HERE-HIGHLIGHT: engine reads this block to render the single \"start here\" card\n",
        "     (reuses the existing cluster-card--start-here component). Do not add more than one. -->\n",
        "**Start here:** [[capability-based-security|Capability-based security]] — the access-control\n",
        "model the whole category is named for: components hold verified cryptographic tokens instead of\n",
        "ambient privilege. One software layer implements it today; kernel-level enforcement is planned.\n",
        "<!-- END-START-HERE-HIGHLIGHT -->\n",
        "\n",
        "## Identity and permissions {#group-count-5}\n",
        "\n",
        "Who is known to the system, how a device proves it, and what it's allowed to do.\n",
        "\n",
        "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: identity-and-permissions -->\n",
        "- [[capability-based-security|Capability-based security]] — the access-control model: components hold verified cryptographic tokens instead of ambient privilege\n",
        "- [[machine-based-auth|Machine-based authorization]] — pairing hardware to hardware replaces passwords; the pairing itself is the permission\n",
        "- [[personnel-permissions|Personnel and permissions]] — how contributor identity and the four permission tiers are expressed through pairings, not database roles\n",
        "- [[identity-ledger-schema-design|Identity ledger schema design]] — the Person/Anchor/Claim record types behind Ring 1 identity resolution\n",
        "- [[verification-surveyor|Verification surveyor]] — the human-in-the-loop checkpoint that confirms an extracted identity before it's committed\n",
        "<!-- END AUTO-GENERATED -->\n",
        "\n",
        "## Cryptographic verification {#group-count-2}\n",
        "\n",
        "How a reader independently checks that a record hasn't been altered.\n",
        "\n",
        "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: cryptographic-verification -->\n",
        "- [[crypto-attestation|Cryptographic payload attestation]] — client-side SHA-256 hashing that would let any viewer verify published content wasn't changed in transit; today it's an unwired, cosmetic pattern in a few templates, not a capability any shipped surface actually offers\n",
        "- [[cryptographic-ledgers|Cryptographic ledgers]] — the immutable-state storage pattern: hash-chained entries, signed checkpoints, monthly Sigstore Rekor anchoring\n",
        "<!-- END AUTO-GENERATED -->\n",
        "\n",
        "## Isolation boundaries {#group-count-3}\n",
        "\n",
        "What contains a compromise once one occurs. Thin relative to the category's own scope — see the\n",
        "[[ppn-tenant-vm-isolation|tenant isolation]] and [[service-vm-tenant|VM tenant]] articles in\n",
        "[[infrastructure|Where It Runs]] for the commercially load-bearing case, which isn't yet\n",
        "cross-referenced from here.\n",
        "\n",
        "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: isolation-boundaries -->\n",
        "- [[sel4-capability-topology|seL4 capability topology]] — why security in an seL4 system is the shape of the capability graph, not a policy layer\n",
        "- [[diode-standard|Diode standard]] — the unidirectional authority-to-subject command flow that removes lateral movement by design\n",
        "- [[genesis-protocol|Genesis protocol]] — the fleet-bootstrapping sequence a node runs at first boot to reach a secure, claimable state\n",
        "<!-- END AUTO-GENERATED -->\n",
        "\n",
        "## Data handling and privacy {#group-count-1}\n",
        "\n",
        "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: data-handling-and-privacy -->\n",
        "- [[data-sovereignty-telemetry|Data sovereignty and zero-state telemetry]] — the platform's only article on this clause today; retention, deletion, and encryption-at-rest have no dedicated article yet\n",
        "<!-- END AUTO-GENERATED -->\n",
        "\n",
        "## Supply-chain controls {#group-count-2}\n",
        "\n",
        "Keeping code honest from a contributor's machine to production.\n",
        "\n",
        "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW — regenerate from index_group: supply-chain-controls -->\n",
        "- [[five-stage-supply-chain|Five-stage supply chain]] — the contributor-to-customer promotion path, gated by a heavily guarded promotion script rather than a pull request, and the double-blind air-gap between contributor and customer\n",
        "- [[pre-commit-defense-in-depth|Pre-commit defense in depth]] — the helper-only gate, secret-pattern scan, and size guard that run on every commit\n",
        "<!-- END AUTO-GENERATED -->\n",
        "\n",
        "## What this is not\n",
        "\n",
        "This page is not a substitute for reading the linked articles — each group's one-line annotation\n",
        "orients, it doesn't replace the underlying mechanism's own caveats and \"what this is not\"\n",
        "section. It is not exhaustive of every security-relevant fact in this wiki: isolation boundaries\n",
        "in particular are thin here because the commercially load-bearing tenant-isolation work lives in\n",
        "[[infrastructure|Where It Runs]], not this category. It is not a compliance attestation — several\n",
        "linked articles describe planned, not-yet-built mechanisms, hedged accordingly in their own text.\n",
        "\n",
        "## See also\n",
        "\n",
        "- [Architecture](/architecture/) — how the platform is put together\n",
        "- [Governance and Standards](/governance/) — what was decided and why it is compliant\n",
        "- [Where It Runs](/infrastructure/) — the deployed storage and ledger infrastructure these mechanisms protect\n",
    );

    #[test]
    fn parses_the_real_security_index_end_to_end() {
        let topic = parse_index_topic(REAL_SECURITY_INDEX_BODY).expect("should parse");

        assert!(topic.intro_html.contains("indexes the 13 articles"));

        let sh = topic.start_here.expect("start-here block");
        assert_eq!(sh.href, "/wiki/capability-based-security");
        assert_eq!(sh.label, "Capability-based security");
        assert!(sh.prose_html.contains("access-control"));
        assert!(!sh.prose_html.contains("START-HERE"));
        assert!(!sh.prose_html.contains("<!--"), "leaked comment marker: {}", sh.prose_html);
        assert!(!topic.intro_html.contains("<!--"), "leaked comment marker: {}", topic.intro_html);

        assert_eq!(topic.groups.len(), 5);

        let g0 = &topic.groups[0];
        assert_eq!(g0.title, "Identity and permissions");
        assert!(!g0.title.contains("group-count"));
        assert_eq!(g0.count(), 5);
        let g0_intro = g0.intro_html.as_deref().unwrap();
        assert!(g0_intro.contains("Who is known"));
        assert!(!g0_intro.contains("<!--"), "leaked comment marker: {}", g0_intro);
        assert_eq!(g0.members[0].href, "/wiki/capability-based-security");
        assert_eq!(g0.members[0].label, "Capability-based security");
        assert!(g0.members[0].annotation_html.contains("access-control model"));
        assert!(!g0.members[0].annotation_html.starts_with('\u{2014}'));
        assert!(
            !g0.members[4].annotation_html.contains("<!--"),
            "leaked comment marker: {}",
            g0.members[4].annotation_html
        ); // last member in the group, right before END AUTO-GENERATED

        let g3 = &topic.groups[3]; // "Data handling and privacy" — no prose intro
        assert_eq!(g3.title, "Data handling and privacy");
        assert!(g3.intro_html.is_none());
        assert_eq!(g3.count(), 1);

        assert!(topic.tail_html.contains("What this is not"));
        assert!(topic.tail_html.contains("See also"));
        assert!(topic.tail_html.contains(r#"href="/architecture/""#));
        assert!(!topic.tail_html.contains("AUTO-GENERATED"));
    }

    #[test]
    fn returns_none_for_body_with_no_structure_at_all() {
        assert!(parse_index_topic("Just a plain paragraph, no markers at all.\n").is_none());
    }

    #[test]
    fn returns_none_for_unterminated_start_here() {
        let body = "<!-- START-HERE-HIGHLIGHT -->\n**Start here:** [[foo|Foo]] — text\n";
        assert!(parse_index_topic(body).is_none());
    }

    #[test]
    fn returns_none_for_unterminated_group() {
        let body = concat!(
            "## Group one\n\n",
            "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW -->\n",
            "- [[foo|Foo]] — text\n",
        );
        assert!(parse_index_topic(body).is_none());
    }

    #[test]
    fn non_group_heading_stops_structured_parsing_and_becomes_tail() {
        let body = concat!(
            "<!-- START-HERE-HIGHLIGHT -->\n",
            "**Start here:** [[foo|Foo]] — text\n",
            "<!-- END-START-HERE-HIGHLIGHT -->\n\n",
            "## Group one\n\n",
            "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW -->\n",
            "- [[foo|Foo]] — text\n",
            "<!-- END AUTO-GENERATED -->\n\n",
            "## Not a group\n\nJust prose, no membership block.\n",
        );
        let topic = parse_index_topic(body).expect("should parse (start-here + 1 group)");
        assert_eq!(topic.groups.len(), 1);
        assert!(topic.tail_html.contains("Not a group"));
        assert!(topic.tail_html.contains("Just prose"));
    }

    #[test]
    fn start_here_block_with_no_wikilink_is_malformed() {
        let body = "<!-- START-HERE-HIGHLIGHT -->\n**Start here:** no link here.\n<!-- END-START-HERE-HIGHLIGHT -->\n";
        assert!(parse_index_topic(body).is_none());
    }

    #[test]
    fn group_with_no_members_is_malformed() {
        let body = concat!(
            "## Group one\n\n",
            "<!-- AUTO-GENERATED MEMBERSHIP: DO NOT EDIT BELOW -->\n",
            "<!-- END AUTO-GENERATED -->\n",
        );
        assert!(parse_index_topic(body).is_none());
    }
}
