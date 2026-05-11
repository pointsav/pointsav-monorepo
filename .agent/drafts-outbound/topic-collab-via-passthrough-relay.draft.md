---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: architecture/   # candidates: architecture/, applications/ — project-language decides
target_filename: collab-via-passthrough-relay.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-04-28T00:30:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: sonnet-4-6
draft_shape: bulk-draft
references:
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md §1 Step 7
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md §11
  - vendor/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md §4.7
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/STEP-7-COLLAB-SMOKE.md (commit ea26118)
  - https://github.com/yjs/yjs
  - https://codemirror.net/
  - https://docs.rs/tokio/latest/tokio/sync/broadcast/
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #16 (Optional Intelligence Layer)
notes_for_editor: |
  SKELETON ONLY per project-tetrad-discipline.md §4 backfill procedure.
  Substance follows in milestone N+1 (next time this cluster touches Step 7
  or a related collab topic). Authored 2026-04-28 to demonstrate Tetrad
  intent for the project-knowledge cluster's wiki leg.

  When substance lands, the section structure below should hold; the
  `(draft-pending — substance follows in milestone N+1)` placeholders
  swap for actual prose. The Spanish overview (`.es.md` sibling) is
  authored at the same skeleton stage in this commit; the bilingual
  pair tracks together.

  Topic angle: the substrate pattern is general — server holds NO
  document state; CRDT carries session-ephemeral overlay; canonical
  storage is git. The wiki engine is one application; the same pattern
  applies to other substrate services that want collab semantics
  without a parallel authoritative record (e.g., service-extraction
  multi-author review, app-workplace-presentation deck-collab,
  app-workplace-proforma table-collab). project-language has
  authority on whether to keep the section enumeration that draws
  these parallels or pare to the wiki-only case.

  target_path candidates listed; project-language decides per the
  taxonomy decision still pending operator ratification on naming-
  convention.md.
---

# Real-time collaboration via passthrough relay — a substrate pattern

## §1 The pattern in one paragraph

The passthrough relay pattern inverts the normal assumption about where a collaborative editing server sits in the authority chain: the relay holds no document state at all, so the canonical git tree remains the sole authoritative record of every topic's content at every point in time. Concurrent editors connect over WebSocket to a `tokio::sync::broadcast` channel keyed by slug — one broadcast room per document — and the server's only job is to forward Yjs CRDT update messages between those clients; it neither decodes nor stores the document state those messages encode. The sole persistence boundary in the entire system is the `POST /edit/{slug}` atomic-write path, which is unchanged from the single-author case: when an editor saves, the client serialises its local Yjs document to Markdown, sends it over HTTP, and the server atomically renames the new file into place on disk — exactly the same operation as a non-collab save.

## §2 Why a passthrough relay (vs a CRDT server)

Tools like Etherpad and HackMD operate on a server-authoritative document model: the collaborative editing server holds a live, mutable document object — an Operational Transformation log in Etherpad's case, a Y.Doc in HackMD's — and that object is the primary record of current content. A git export is a snapshot taken from that server record, not the other way around. The consequence is a permanent second authoritative state: two places in the system hold an answer to the question "what is the current text of this document," and they can drift apart if the export mechanism fails, the server crashes before a save, or the OT/CRDT log diverges from the git history due to a merge edge case.

The passthrough design eliminates that second record entirely. The server is a message conduit, not a store. When a Yjs client sends a binary update message to `GET /ws/collab/{slug}`, the Rust handler receives the raw bytes from the WebSocket and broadcasts them to all other clients in the same slug room via `tokio::sync::broadcast`. The server never deserialises the Yjs protocol; it never constructs a Y.Doc; it never writes anything to disk as a side effect of a relay operation. The only thing the server knows about the document's state is whatever the client sends through `POST /edit` — the HTTP save path.

This matters specifically for the disclosure-substrate posture described in DOCTRINE.md claim #29 (Substrate Substitution). The canonical disclosure record in this system is the git tree: every topic's content history is a sequence of signed commits, and that sequence is what would be produced in a regulatory audit. A server-authoritative CRDT store would exist in parallel with that sequence, would not be signed, and would represent content states that never appeared in git. Under the passthrough design, no such parallel record exists: in-flight CRDT state is definitionally not part of the disclosure record because it is never written anywhere. The record closes at `POST /edit` time, not before.

The 256-message lag buffer configured on each `tokio::sync::broadcast` channel addresses the one race this design must handle: a client that joins a collab session after other editors have already made changes will not have received those earlier Yjs update messages. Because Yjs CRDTs are convergent, a client that receives all prior update messages in any order will arrive at the same document state. The 256-message buffer ensures a late joiner can catch up on recent activity without the server needing to materialise or store the full document state. If the buffer fills before a late joiner connects, Yjs's sync protocol handles the gap through its awareness/state-vector exchange mechanism on connection open — the server's job remains forwarding, not resolving.

## §3 The implementation in `app-mediakit-knowledge`

The collab relay shipped in commit `05f1dab` as `src/collab.rs`, gated entirely behind the `--enable-collab` CLI flag. When the flag is absent — which is the default and the current production posture as of v0.1.29 — the WebSocket route `GET /ws/collab/{slug}` is not registered in the axum router, the `window.WIKI_COLLAB_ENABLED` template variable is set to `false`, and the client-side JavaScript bundle never loads. Zero collab code paths execute in the default-off configuration.

The server-side relay is implemented using `tokio::sync::broadcast`, the standard Tokio multi-producer multi-consumer channel. Each slug gets its own broadcast channel with a buffer capacity of 256 messages, created on first connection and stored in a `DashMap<String, broadcast::Sender<Bytes>>` held in `AppState`. When a WebSocket client sends a Yjs update message, the handler reads the raw bytes and calls `sender.send(bytes)` — a single line that fans the message out to all other receivers on that channel. There is no `yrs` crate dependency; the `yrs` Rust port of Yjs was specified in the original PHASE-2-PLAN.md Step 7 brief but was not used in the as-shipped implementation. The server relays raw Yjs binary protocol messages without deserialising them, which means the server carries no document state at any point.

The WebSocket upgrade uses axum's built-in WS support (`axum::extract::ws`). The route handler upgrades the HTTP connection, spawns a task per client that loops on incoming WebSocket frames and rebroadcasts each, and removes the client from the room's receiver set on disconnect. If the last client disconnects, the broadcast channel's receiver count drops to zero; the `DashMap` entry persists but holds an empty channel, ready for the next connection. No disk write occurs on last-client-disconnect, which is a deliberate departure from the PHASE-2-PLAN.md spec that originally called for snapshotting the Y.Doc to disk on last disconnect. The passthrough design makes that snapshot impossible because the server never holds the Y.Doc.

The client bundle `cm-collab.bundle.js` (~302 KB as shipped, larger than the ~100 KB originally estimated in PHASE-2-PLAN.md §2) is built from three npm packages vendored in `vendor-js/`: `yjs`, `y-codemirror.next`, and `y-websocket`. The bundle is committed to `static/vendor/` as a pre-built artefact, so no npm toolchain is required at runtime or in the Rust build path. The `static/saa-init.js` initialisation script checks `window.WIKI_COLLAB_ENABLED` at load time; if the flag is `false`, the import of `cm-collab.bundle.js` is never executed. The CodeMirror editor loads fully in either case — collab is additive to the editing surface, not a prerequisite.

The `src/server.rs` template layer injects the flag: when `--enable-collab` is set, the HTML template for `/edit/{slug}` includes `<script>window.WIKI_COLLAB_ENABLED = true;</script>` immediately before the editor initialisation script tag. When the flag is absent, that script tag is omitted entirely rather than set to `false` — preventing the JS engine from even evaluating the collab code path during bundle parse.

Test coverage at commit `05f1dab` added 7 tests (3 unit + 4 integration) to bring the total from 90 to 97. The unit tests cover: the WebSocket route accepts a connection when `--enable-collab` is set; `tokio::sync::broadcast` rooms multiplex correctly across two clients on the same slug; the 256-message buffer drains to completion without panic; and the client bundle loads (the HTTP 200 for `cm-collab.bundle.js` is reachable) only when the flag is set. The 4 integration tests cover the flag-gated script-injection behaviour in rendered HTML. The unit + integration suite does not cover the visual cursor-presence property — that requires a two-headed browser smoke with DOM inspection, which was judged out-of-scope for Phase 2 and is covered instead by the manual smoke procedure documented in STEP-7-COLLAB-SMOKE.md.

## §4 What the server doesn't carry

The following is a precise enumeration of what the relay layer deliberately omits, relevant for security reviewers and architects evaluating the design:

- **No Yjs Rust port (`yrs` crate).** The `yrs` crate was specified in PHASE-2-PLAN.md Step 7 but is not present in the as-shipped `Cargo.toml`. The server forwards raw binary Yjs protocol messages without deserialising them. Yjs document semantics live entirely on the client side.
- **No document-state persistence at the relay layer.** No intermediate snapshot of CRDT state is written to disk at any point — not on connection open, not on broadcast, not on last-client-disconnect. The only write to disk in the entire collab path is the client-initiated `POST /edit/{slug}` save.
- **No auth-bearing identity longer than the WebSocket lifetime.** The relay does not assign persistent user identifiers, does not log which clients contributed which messages, and does not associate a WebSocket session with any account credential beyond the duration of that WebSocket connection. Cursor colour identifiers visible in the collab UI are generated client-side and are not stored.
- **No rate-limit on the relay path beyond axum defaults.** The broadcast handler applies no application-level rate limit on relay messages. Axum's default connection-handling and Tokio's scheduler provide the only backpressure. A production deployment behind a reverse proxy (nginx, in the current systemd unit configuration) inherits that proxy's connection limits; no relay-specific limit is enforced at the application layer.
- **No admin-accessible edit history on the server side.** Because the server stores no document state and no message log, there is no server-side edit history to inspect. The only edit history that exists is the git commit log produced by successive `POST /edit` saves. An administrator inspecting the server has no privileged view of what was typed during a collab session that was not subsequently saved.

## §5 What this means for disclosure posture

In-flight CRDT state — the sequence of Yjs update messages exchanged between clients during a collab session — is not part of the disclosure record and cannot be, by construction. Because the relay never persists those messages, there is no server-side artefact that could later be produced in response to a disclosure obligation. The collab session leaves no shadow state on the server. This is not a gap in the record; it is a design property that aligns the collab implementation with DOCTRINE.md claim #29's requirement that the substrate's canonical storage — in this case the git tree — be the sole authoritative record. A server-authoritative CRDT store would create a second authoritative record that is not git, is not signed, and is not suitable for regulatory production.

Saved edits enter the disclosure record through the same path as all other edits: `POST /edit/{slug}` sends the full Markdown text of the document, the server performs an atomic file rename, and the next git commit in the sequence captures that snapshot. From git's perspective, a collab-edited save is identical to a single-author save. The commit records what the document contained at save time; it does not record who typed which characters during the collab session. This is the correct posture under the disclosure-substrate convention: the disclosure unit is the committed document state, not the authorship decomposition of how that state was reached.

The `--enable-collab` flag's rollback path (documented in STEP-7-COLLAB-SMOKE.md §5) is non-destructive at every layer precisely because of this design. Removing the flag from the systemd unit's `ExecStart` line, running `systemctl daemon-reload` and `systemctl restart`, returns the service to default-off posture without losing any data — because no data was ever held on the server side. The collab CRDT overlay is ephemeral by construction; its removal on service restart is not a data loss event, it is an expected and explicitly designed consequence of keeping the server stateless. Any content that was saved before the restart exists in the git tree and is fully recoverable; any content that was in-flight and not saved before the restart is lost in exactly the same way as unsaved content in a single-author editor session would be lost.

## §6 Generalising beyond the wiki

The passthrough relay is a substrate pattern, not a wiki-specific feature. Any service that wants concurrent editing semantics faces the same architectural question: does the collab infrastructure need to hold document state on the server, or can that state live entirely on the clients and in canonical storage? The answer depends on what the canonical storage is and whether a CRDT server sitting between the clients and that storage would compete with it for authority. Three concrete candidates inside the Foundry substrate illustrate the generalisation:

**`service-extraction` multi-author review pipeline.** The canonical storage for extraction results in `service-extraction` is the deterministic parser-combinator output written to structured records, not a live editable document. Multi-author review — multiple people annotating or correcting an extraction result — would naturally benefit from presence awareness and live conflict resolution. The canonical storage is structured (not free-text Markdown), which means the CRDT overlay would need to operate over a structured document type. The framing question: would a stateful CRDT server compete with the structured record store for authority? If the CRDT server materialises partial corrections that have not yet been committed back to the structured store, the answer is yes — and the passthrough design would not directly apply without an adapter layer that serialises CRDT state to the canonical structure format on save. The passthrough pattern applies in its simplest form only when the CRDT document type maps cleanly to the canonical storage type.

**`app-workplace-presentation` deck-collab.** The canonical storage for a presentation is the slide deck file format (likely PPTX or an equivalent structured format). Concurrent slide editing is the canonical use case for collaborative office tools, and most existing solutions (Google Slides, Office 365) use a server-authoritative model where the presentation object lives on the server. The framing question: would a stateful CRDT server compete with the slide file for authority? Yes — a server-authoritative presentation service by definition holds the authoritative deck object. The passthrough design applies if and only if the presentation's on-disk file format is the canonical record and the CRDT overlay is treated as session-ephemeral overlay on top of it. For a slide deck stored as a committed git artefact, that posture is achievable; for a deck stored in a third-party format that regenerates from a server-side object, it is not.

**`app-workplace-proforma` table-collab.** The canonical storage for a proforma is structured tabular data — rows and columns with typed values, likely stored as TOML or structured JSON alongside the git tree. Collaborative editing of table cells is a different CRDT problem from collaborative text editing: cell-level conflict resolution (two people editing the same cell) requires a different CRDT type than character-level text merging. The `yjs` library handles this via its `Y.Map` and `Y.Array` types in addition to `Y.Text`, so the CRDT client library is general enough. The framing question: would a stateful CRDT server compete with the structured cell store for authority? If the proforma's canonical storage is git-committed structured data and the CRDT overlay is session-ephemeral, the passthrough design applies with the same logic as the wiki case — the save path serialises the CRDT state to the canonical format and commits. The difference from the text case is that the serialisation step must produce valid structured data, not free-text Markdown, adding a validation requirement at `POST /save` time.

## §7 References

- **Yjs** — Conflict-free replicated data type library for collaborative applications. Client-side CRDT engine used in `cm-collab.bundle.js`. https://github.com/yjs/yjs
- **CodeMirror collab integration** — `y-codemirror.next` binds a Yjs `Y.Text` to a CodeMirror 6 editor state; cursor presence is rendered via CodeMirror's `DecorationSet` API. https://codemirror.net/
- **`tokio::sync::broadcast`** — Multi-producer multi-consumer channel used as the per-slug relay room. 256-message buffer is the channel capacity argument passed to `broadcast::channel(256)`. https://docs.rs/tokio/latest/tokio/sync/broadcast/
- **`PHASE-2-PLAN.md` §1 Step 7** — Original Step 7 brief specifying the collab design (note: original spec called for `yrs`; as-shipped implementation is a passthrough relay without `yrs`). `vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md`
- **`STEP-7-COLLAB-SMOKE.md`** (commit `ea26118`) — Manual two-client smoke procedure, pre-staged systemd unit diff, and rollback runbook for production enable. `vendor/pointsav-monorepo/app-mediakit-knowledge/docs/STEP-7-COLLAB-SMOKE.md`
- **`ARCHITECTURE.md` §11** — Full API surface set table listing `/ws/collab/{slug}` as a Phase 2+ opt-in route alongside the REST, JSON-LD, feed, and MCP surfaces. `vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md`
- **`UX-DESIGN.md` §4.7** — Collab UX wireframe and cursor-presence pattern for the SAA editor surface. `vendor/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md`
- **DOCTRINE.md claim #29** (Substrate Substitution) — Establishes that the canonical disclosure record is git; no parallel server-side store may compete with it for authority. `~/Foundry/DOCTRINE.md`
