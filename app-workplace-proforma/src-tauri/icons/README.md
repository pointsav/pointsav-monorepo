# Application Icons — Workplace✦Memo

Tauri requires application icons in multiple sizes and formats.
They are NOT committed to the repository.

---

## Generating icons

Tauri provides an icon generation command that takes a single 1024×1024
source PNG and produces all required formats:

```bash
# From the app-workplace-memo/ root directory:
npx tauri icon path/to/icon-1024.png
```

This will generate:
- `icons/32x32.png`
- `icons/128x128.png`
- `icons/128x128@2x.png`
- `icons/icon.icns` (macOS)
- `icons/icon.ico` (Windows)

The source icon for Workplace✦Memo uses the ✦ glyph (U+2726)
on the PointSav gold (#c8a96e) background.

---

## Placeholder icons for development

For initial development builds, create placeholder icons using ImageMagick:

```bash
# Install ImageMagick (macOS):
brew install imagemagick

# Generate placeholder icons:
convert -size 1024x1024 xc:'#1e1e24' \
  -fill '#c8a96e' -pointsize 600 \
  -gravity Center -annotate 0 '✦' \
  icon-source.png

npx tauri icon icon-source.png
rm icon-source.png
```

---

## Icon files required by tauri.conf.json

```json
"icon": [
  "icons/32x32.png",
  "icons/128x128.png",
  "icons/128x128@2x.png",
  "icons/icon.icns",
  "icons/icon.ico"
]
```

The build will fail if these files do not exist.
For a first dev build, run the placeholder generation above.
