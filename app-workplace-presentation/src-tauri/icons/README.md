# Icons

The master file is `icon-source.png` — a 1024×1024 PNG in PointSav gold (`#c8a96e`).
It is the only icon file committed to git. All platform-specific formats
(`32x32.png`, `128x128.png`, `128x128@2x.png`, `icon.icns`, `icon.ico`, etc.)
are generated locally and gitignored.

## First-time generation (or after updating icon-source.png)

```bash
# If icon-source.png does not yet exist, create a gold square placeholder:
convert -size 1024x1024 xc:'#c8a96e' src-tauri/icons/icon-source.png

# Generate all platform-specific formats:
make icons
# (equivalent to: npx tauri icon src-tauri/icons/icon-source.png)
```

## Family icon direction

When real artwork is commissioned, the workplace family gets matching motifs
in PointSav gold:

- `app-workplace-memo` → document-with-fold motif
- `app-workplace-proforma` → grid-of-cells motif
- `app-workplace-presentation` → slide-stack motif (this app)

See `CLEANUP_LOG.md` for the open item tracking this commission.
