use serde_json::Value;

use super::shell::esc;

pub fn render_editor_panel(slug: &str, token_json: &Value) -> String {
    let pretty = serde_json::to_string_pretty(token_json).unwrap_or_default();
    let escaped_json = esc(&pretty);

    format!(
        r#"<div class="bim-editor" data-slug="{slug}">
  <div class="bim-editor-header">
    <h1>Edit: <code>{slug}</code></h1>
    <cds-content-switcher id="bim-mode-switcher">
      <cds-content-switcher-item value="visual" selected>Visual</cds-content-switcher-item>
      <cds-content-switcher-item value="code">Code</cds-content-switcher-item>
    </cds-content-switcher>
  </div>

  <div id="bim-visual-pane" class="bim-editor-pane">
    <div class="bim-editor-visual-notice">
      <cds-inline-notification kind="info" subtitle="Visual editor — Phase 5 implementation">
      </cds-inline-notification>
    </div>
    <pre class="bim-editor-preview">{escaped_json}</pre>
  </div>

  <div id="bim-code-pane" class="bim-editor-pane" hidden>
    <div id="bim-codemirror" class="bim-codemirror-host"></div>
    <script type="module">
      import {{ EditorState }} from '/static/codemirror.bundle.js';
      import {{ EditorView, keymap }} from '/static/codemirror.bundle.js';
      import {{ json }} from '/static/codemirror.bundle.js';

      const view = new EditorView({{
        state: EditorState.create({{
          doc: {json_literal},
          extensions: [json(), EditorView.lineWrapping]
        }}),
        parent: document.getElementById('bim-codemirror')
      }});

      window.BimEditor = {{ view, slug: '{slug}' }};
    </script>
  </div>

  <div class="bim-editor-actions">
    <cds-button id="bim-save-btn" kind="primary">Save</cds-button>
    <cds-inline-notification id="bim-save-status" style="display:none"></cds-inline-notification>
  </div>

  <script type="module" src="/static/bim-editor.js"></script>
</div>"#,
        slug = esc(slug),
        escaped_json = escaped_json,
        json_literal = serde_json::to_string(&pretty).unwrap_or_else(|_| r#""""#.into()),
    )
}
