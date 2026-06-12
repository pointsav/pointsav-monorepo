---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-development
target_repo: woodfine-fleet-deployment
target_path: vault-privategit-source/
target_filename: guide-workbench-setup.md
audience: customer-internal
bcsc_class: no-disclosure-implication
language_protocol: PROSE-GUIDE
authored: 2026-05-23T17:00:00Z
authored_by: totebox@project-development claude-code
authored_with: sonnet-4-6
references:
  - app-privategit-workbench/src/main.rs
  - infrastructure/local-workbench/config.toml
  - infrastructure/local-workbench/local-workbench.service
notes_for_editor: |
  Operational guide for the workbench deployment. Covers setup,
  configuration, and daily use. Does not cover crate development.
  Keyboard shortcut table should remain as-is. Bilingual pair not
  required (operational files are English-only per CLAUDE.md §14).
research_done_count: 0
research_suggested_count: 0
open_questions_count: 0
research_provenance: direct-consultation
research_inline: false
---

# Guide — app-privategit-workbench Setup and Operation

`app-privategit-workbench` is a browser-based file editor served from
a local HTTP endpoint on the Totebox Orchestration workspace VM. It
presents a three-column interface — file tree, viewer, and editor —
for working with markdown, TOML, shell scripts, and other plain-text
files across the cluster archive tree.

This guide covers deployment on a `vault-privategit-source` instance.

---

## Requirements

- `app-privategit-workbench` binary installed at `/usr/local/bin/app-privategit-workbench`
- nginx configured to proxy `/_api/edit/` to the workbench port
- Systemd unit `app-privategit-workbench.service` enabled and running

The binary and service are provisioned by `project-development` via the
standard Stage 6 + binary-ledger discipline. Verify the binary ledger
entry before operating: `tail -1 /srv/foundry/data/binary-ledger/app-privategit-workbench.jsonl`.

---

## Configuration — config.toml

The workbench reads its configuration from the path given as the first
argument to the binary (default: `config.toml` in the working directory).
The service file passes the absolute path explicitly.

```toml
bind = "127.0.0.1:9210"   # Internal only — never expose this port directly
max_bytes = 2097152         # 2 MiB per file
module_id = "development"   # Instance identifier injected into the SPA

# Each [[root]] block declares one writable root the SPA can access.
# url_prefix is the path the browser uses; fs_path is the filesystem target.

[[root]]
url_prefix = "_command"
fs_path = "/srv/foundry"
writable = false            # Command workspace — read-only view

[[root]]
url_prefix = "_sandbox-jennifer"
fs_path = "/home/jennifer/sandbox"
writable = true

[[root]]
url_prefix = "_clones/project-development"
fs_path = "/srv/foundry/clones/project-development"
writable = true
```

**Adding a writable root:**

1. Append a `[[root]]` block to `config.toml` at `/srv/foundry/infrastructure/local-workbench/config.toml`.
2. Add the matching `fs_path` to `ReadWritePaths=` in the systemd service file.
3. Restart the service: `sudo systemctl restart app-privategit-workbench`.

Missing the `ReadWritePaths` update causes a silent permission failure at write time —
the service will start but `PUT /file` will return a filesystem error.

---

## nginx integration

The workbench write API proxies through nginx. The intranet nginx config
(`nginx-intranet.conf`) must have:

```nginx
location /_api/edit/ {
    proxy_pass http://127.0.0.1:9210/;
    proxy_set_header Host $host;
    proxy_read_timeout 10s;
    proxy_send_timeout 10s;
    client_max_body_size 2m;
}
```

Directory listings (file tree) use nginx autoindex routes:

```nginx
location /_api/command/ {
    alias /srv/foundry/;
    autoindex on; autoindex_format json;
}
location /_api/clones/ {
    alias /srv/foundry/clones/;
    autoindex on; autoindex_format json;
}
```

Add a matching `/_api/<prefix>/` block for every new `[[root]]` whose
`url_prefix` starts with `_clones/` or `_sandbox-*`. Without the nginx
route, the file tree for that root will show as empty.

---

## Service management

```bash
systemctl status app-privategit-workbench   # Check state
sudo systemctl restart app-privategit-workbench   # Restart after config change
journalctl -u app-privategit-workbench -n 50      # Recent logs
```

The service binds only to `127.0.0.1:9210`. The intranet nginx (port 9200 / 9211)
is the access point for browser clients.

---

## Using the workbench

Open the intranet in a browser on the WireGuard subnet (`10.8.0.9:9200`).
The workbench SPA loads automatically.

**File tree (left column):** Expandable sections for each configured root.
Click a folder to expand it. Click a file to open it in the viewer.

**Viewer (centre column):** Renders markdown, displays HTML in an iframe,
shows images inline, and falls back to plain text for all other types.
Click the × button in the viewer bar to close the current file.

**Editor (right column):** Appears for files in writable roots. The editor
panel includes:

| Control | Function |
|---|---|
| Outline button | Open document outline (Cmd+Shift+O) |
| Comment button | Toggle line comment for selected lines |
| Save button | Write file to disk; Cmd+S also works |
| Cursor indicator | Shows Ln/Col; updates on caret move |
| Dirty dot (●) | Orange — unsaved changes present |

**Keyboard shortcuts:**

| Shortcut | Action |
|---|---|
| Cmd+S | Save current file |
| Cmd+F | Find / Replace bar |
| Cmd+Shift+O | Document outline panel |
| Ctrl+G | Go to line |
| Alt+↑ / Alt+↓ | Move line up / down |
| Shift+Alt+↓ | Duplicate line |
| Tab | Insert 2 spaces |

---

## Security model

- **Root containment:** All file paths are canonicalized and verified to remain
  within the declared `fs_path` before any read or write. Path traversal
  (`../`) is rejected with HTTP 400.
- **CSRF guard:** Every `PUT /file` request requires the `X-Foundry-Editor: 1`
  header. Browser cross-origin requests do not send this header.
- **Extension allowlist:** Only the following extensions may be written:
  `md txt html css js ts json toml yaml yml sh rs py rb go conf ini env lock svg`.
  Binary files and unknown extensions are read-only.
- **Writable flag:** Each root declares `writable = true` or `false`. Reads are
  permitted from all roots; writes are rejected with HTTP 403 from read-only roots.
- **mtime conflict detection:** `PUT /file` accepts an optional
  `X-Foundry-Mtime` header containing the last-read mtime. If the file has
  been modified on disk since the browser read it, the service returns HTTP 409
  and the editor shows a conflict dialog.
- **Systemd hardening:** The service unit sets `ProtectSystem=strict`,
  `PrivateTmp=true`, and `ReadWritePaths=` scoped to declared roots.

---

## Troubleshooting

**File tree shows empty for a root I just added.**
The nginx `/_api/<prefix>/` autoindex route is missing. Add the block and
reload nginx: `sudo nginx -s reload`.

**Save fails silently or returns a system error.**
The new root's `fs_path` is not in `ReadWritePaths=` in the service unit.
Add it, reload the daemon, and restart the service.

**"File extension not allowed for writes."**
The file type is not in the extension allowlist. It can be read but not written
through the workbench. Edit directly in a terminal session instead.

**"File modified on disk since last read" (HTTP 409).**
Another process wrote the file after the workbench loaded it. Reload the file
from the tree to pick up the latest content, then re-apply your edits.
