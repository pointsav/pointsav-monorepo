#!/usr/bin/env bash
# fix-font-zoom.sh — Fix the "50% zoom" perception on both marketing sites.
#
# Root causes:
#   1. font-size: 14px base (vs web standard 16px) makes text look tiny
#   2. font-size: 11px navigation/tab text is unusually small
#   3. .doc { margin: 0 56px } inside max-width:1440px means on monitors
#      wider than ~2000px the white content card is only ~50% of screen width
#
# This script patches the __bundler/template JSON string inside each
# deployment index.html to update font sizes and add wide-screen layout fixes.
# It is idempotent — re-running does nothing if already patched.
#
# Usage: bash scripts/fix-font-zoom.sh

set -euo pipefail

DEPLOYMENTS=(
  "/srv/foundry/deployments/media-marketing-landing-1/content/index.html"
  "/srv/foundry/deployments/media-marketing-landing-2/content/index.html"
)

all_ok=0

for FILE in "${DEPLOYMENTS[@]}"; do
  if [[ ! -f "$FILE" ]]; then
    echo "SKIP  $FILE (not found)"
    continue
  fi

  # Check if already patched
  if grep -q 'font-size: 16px.*antialiased\|FONT-ZOOM-PATCH-APPLIED' "$FILE"; then
    echo "OK    $FILE (already patched)"
    continue
  fi

  echo "PATCH $FILE ..."

  python3 - "$FILE" <<'PYEOF'
import sys, re, json

path = sys.argv[1]
with open(path, 'r', encoding='utf-8') as f:
    src = f.read()

# Find the __bundler/template script and extract the JSON string
m = re.search(r'(<script type="__bundler/template">)(.*?)(</script>)', src, re.DOTALL)
if not m:
    print(f"  ERROR: __bundler/template not found in {path}")
    sys.exit(1)

prefix, json_content, suffix = m.group(1), m.group(2), m.group(3)
template = json.loads(json_content.strip())

original_len = len(template)

# ── 1. Base font size: 14px → 16px ──────────────────────────────────────────
# Affects readability across the whole page immediately.
OLD1 = 'font-size: 14px;\n    line-height: 1.55;\n    -webkit-font-smoothing: antialiased;'
NEW1 = 'font-size: 16px;\n    line-height: 1.55;\n    -webkit-font-smoothing: antialiased;'
count1 = template.count(OLD1)

# ── 2. Nav text: 11px → 13px ────────────────────────────────────────────────
# .topnav .left, .topnav .right
OLD2 = 'font-size: 11px;\n    font-weight: 600;\n    letter-spacing: 0.14em;\n    color: var(--ink-3);\n    text-transform: uppercase;'
NEW2 = 'font-size: 13px;\n    font-weight: 600;\n    letter-spacing: 0.10em;\n    color: var(--ink-3);\n    text-transform: uppercase;'
count2 = template.count(OLD2)

OLD2b = 'font-size: 11px;\n    font-weight: 600;\n    letter-spacing: 0.14em;\n    color: #164679;\n    text-transform: uppercase;'
NEW2b = 'font-size: 13px;\n    font-weight: 600;\n    letter-spacing: 0.10em;\n    color: #164679;\n    text-transform: uppercase;'
count2b = template.count(OLD2b)

# ── 3. Tab/button text: 11px → 12px ─────────────────────────────────────────
OLD3 = 'font-size: 11px;\n    font-weight: 700;\n    letter-spacing: 0.16em;\n    line-height: 1.25;\n    padding: 8px 14px;\n    min-width: 130px;'
NEW3 = 'font-size: 12px;\n    font-weight: 700;\n    letter-spacing: 0.14em;\n    line-height: 1.25;\n    padding: 8px 14px;\n    min-width: 130px;'
count3 = template.count(OLD3)

# ── 4. Wide-screen layout: add @media (min-width: 1600px) to reduce margins ─
# When monitor is wider than 1600px, reduce .doc side margins so content
# fills more of the screen (the "50% zoom" on large monitors).
WIDE_MEDIA = '\n  @media (min-width: 1600px) {\n    .doc { margin: 0 36px; }\n    .props { padding: 36px 80px 56px; }\n    .compliance { margin: 0 36px; padding: 28px 80px 32px; }\n  }\n  @media (min-width: 1920px) {\n    .doc { margin: 0 24px; }\n    .props { padding: 36px 48px 56px; }\n    .compliance { margin: 0 24px; padding: 28px 48px 32px; }\n  }'

# Inject before the closing </style> of the main CSS block (the tokens block)
# Find the last </style> that follows the .page rule
OLD4 = '\n  @media (max-width: 1200px) {\n    .topnav { padding: 0 36px;'
NEW4 = WIDE_MEDIA + '\n\n  @media (max-width: 1200px) {\n    .topnav { padding: 0 36px;'
count4 = template.count(OLD4)

# ── Apply patches ────────────────────────────────────────────────────────────
applied = []

if count1 > 0:
    template = template.replace(OLD1, NEW1, 1)
    applied.append(f"base font-size 14px→16px ({count1} occurrence)")

if count2 > 0:
    template = template.replace(OLD2, NEW2, 1)
    applied.append(f"nav font-size 11px→13px Woodfine ({count2} occurrence)")

if count2b > 0:
    template = template.replace(OLD2b, NEW2b, 1)
    applied.append(f"nav font-size 11px→13px PointSav ({count2b} occurrence)")

if count3 > 0:
    template = template.replace(OLD3, NEW3, count3)
    applied.append(f"tab font-size 11px→12px ({count3} occurrences)")

if count4 > 0:
    template = template.replace(OLD4, NEW4, 1)
    applied.append(f"added wide-screen layout @media (min-width: 1600px/1920px)")

if not applied:
    print(f"  WARN: no target patterns found — template may already be patched or differ from expected")
    sys.exit(0)

# Re-encode and splice back
new_json = json.dumps(template)
new_block = prefix + new_json + suffix

# Verify the substitution is correct
if src.count(m.group(0)) != 1:
    print(f"  ERROR: template block not unique in {path}")
    sys.exit(1)

new_src = src[:m.start()] + new_block + src[m.end():]

with open(path, 'w', encoding='utf-8') as f:
    f.write(new_src)

print(f"  patched successfully:")
for a in applied:
    print(f"    ✓ {a}")
print(f"  template length: {original_len} → {len(template)} chars")
PYEOF

  echo "DONE  $FILE"
done

if [[ $all_ok -ne 0 ]]; then
  echo ""
  echo "One or more files need manual inspection."
  exit 1
fi

echo ""
echo "=== Verification ==="
python3 - <<'VEOF'
import re, json

for fname, label in [
    ("/srv/foundry/deployments/media-marketing-landing-1/content/index.html", "Woodfine"),
    ("/srv/foundry/deployments/media-marketing-landing-2/content/index.html", "PointSav"),
]:
    with open(fname, 'r', encoding='utf-8') as f:
        src = f.read()
    m = re.search(r'<script type="__bundler/template">(.*?)</script>', src, re.DOTALL)
    if not m:
        print(f"  {label}: template not found"); continue
    t = json.loads(m.group(1).strip())
    base_ok   = 'font-size: 16px' in t and 'font-size: 14px' not in t
    wide_ok   = 'min-width: 1600px' in t
    nav13_ok  = 'font-size: 13px' in t
    print(f"  {label}: base=16px={base_ok} nav=13px={nav13_ok} wide-media={wide_ok}")
VEOF
