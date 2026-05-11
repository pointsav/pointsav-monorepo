# Fonts — Workplace✦Memo

All font families bundled with Workplace✦Memo are licensed under the
**SIL Open Font Licence 1.1 (OFL)**, which permits:

- Using the fonts in documents and software
- Embedding fonts in software binaries and exported files
- Modifying and redistributing the fonts
- Commercial use without royalty

The OFL requires that modified versions use a different name and that the
fonts are not sold standalone.

Full licence text: https://scripts.sil.org/OFL

---

## Downloading font files

Font WOFF2 binaries are **not committed to the repository** — they are
downloaded at setup time and generated into `src/js/font-data.js` as
base64-encoded data. This keeps the repository lean.

```bash
# Download all font files and Paged.js
./scripts/download-deps.sh

# Embed fonts into src/js/font-data.js
./scripts/embed-fonts.sh
```

---

## Font inventory

| Family | Weight(s) | Designer / Foundry | Source |
|---|---|---|---|
| EB Garamond | 400, 400i, 600 | Georg Duffner / Octavio Pardo | [Google Fonts](https://fonts.google.com/specimen/EB+Garamond) |
| Source Serif 4 | 400, 600 | Frank Grießhammer / Adobe | [Google Fonts](https://fonts.google.com/specimen/Source+Serif+4) |
| Lora | 400, 400i, 600 | Cyreal | [Google Fonts](https://fonts.google.com/specimen/Lora) |
| Playfair Display | 400, 700 | Claus Eggers Sørensen | [Google Fonts](https://fonts.google.com/specimen/Playfair+Display) |
| Fraunces | 400, 600 | Undercase Type | [Google Fonts](https://fonts.google.com/specimen/Fraunces) |
| DM Sans | 400, 500, 700 | Colophon Foundry | [Google Fonts](https://fonts.google.com/specimen/DM+Sans) |
| IBM Plex Sans | 400, 600 | Bold Monday / IBM | [Google Fonts](https://fonts.google.com/specimen/IBM+Plex+Sans) |
| Source Code Pro | 400 | Paul D. Hunt / Adobe | [Google Fonts](https://fonts.google.com/specimen/Source+Code+Pro) |

---

## Directory structure

```
fonts/
├── README.md               ← this file
├── EB-Garamond/
│   ├── EBGaramond-Regular.woff2
│   ├── EBGaramond-Italic.woff2
│   └── EBGaramond-SemiBold.woff2
├── Source-Serif-4/
│   ├── SourceSerif4-Regular.woff2
│   └── SourceSerif4-SemiBold.woff2
├── Lora/
│   ├── Lora-Regular.woff2
│   ├── Lora-Italic.woff2
│   └── Lora-SemiBold.woff2
├── Playfair-Display/
│   ├── PlayfairDisplay-Regular.woff2
│   └── PlayfairDisplay-Bold.woff2
├── Fraunces/
│   ├── Fraunces-Regular.woff2
│   └── Fraunces-SemiBold.woff2
├── DM-Sans/
│   ├── DMSans-Regular.woff2
│   ├── DMSans-Medium.woff2
│   └── DMSans-Bold.woff2
├── IBM-Plex-Sans/
│   ├── IBMPlexSans-Regular.woff2
│   └── IBMPlexSans-SemiBold.woff2
└── Source-Code-Pro/
    └── SourceCodePro-Regular.woff2
```

---

## Adding a new font

1. Verify the font is SIL OFL licensed
2. Download the WOFF2 file(s) and place in `fonts/<FamilyName>/`
3. Use the naming convention: `<Family>-<Weight>[Italic].woff2`
   e.g. `Fraunces-SemiBoldItalic.woff2`
4. Add the family to `src/js/fonts.js` `BUILTIN_FONTS` array
5. Add `@font-face` entries to the relevant template in `src/js/templates.js`
6. Run `./scripts/embed-fonts.sh` to regenerate `font-data.js`
7. Document the font here with source URL and designer credit

---

## Licence confirmations

All fonts below have been individually verified as SIL OFL 1.1.
Verification date: April 2026.

| Family | Verification source |
|---|---|
| EB Garamond | https://github.com/octaviopardo/EBGaramond/blob/master/LICENSE |
| Source Serif 4 | https://github.com/adobe-fonts/source-serif/blob/release/LICENSE.md |
| Lora | https://fonts.google.com/specimen/Lora (OFL confirmed) |
| Playfair Display | https://fonts.google.com/specimen/Playfair+Display (OFL confirmed) |
| Fraunces | https://github.com/undercasetype/Fraunces/blob/main/OFL.txt |
| DM Sans | https://fonts.google.com/specimen/DM+Sans (OFL confirmed) |
| IBM Plex Sans | https://github.com/IBM/plex/blob/master/LICENSE.txt |
| Source Code Pro | https://github.com/adobe-fonts/source-code-pro/blob/release/LICENSE.md |
