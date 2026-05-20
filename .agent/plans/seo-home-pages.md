# SEO — home page head blocks

Drafted: 2026-05-20 | Status: ready to apply | Operator confirmed preview; paused for next session.

## Open questions (resolve before applying)

1. Any LinkedIn or other social profiles to add to `sameAs`?
2. Scope: also update disclaimer/contact subpages + software.pointsav.com pages in same pass?

---

## home.pointsav.com

**File:** `/srv/foundry/deployments/media-marketing-landing-2/content/index.html`
**Format:** bundler — inner HTML is JSON string in `<script type="__bundler/template">`. Use Python to edit (not Edit tool).
**Insert after:** `<title>PointSav, Inc. — Home</title>`

```html
<meta name="description" content="A fully transferable data management and customer service platform for the procurement, development, and management of real properties.">
<meta name="robots" content="index, follow">
<link rel="canonical" href="https://home.pointsav.com/">
<meta property="og:type" content="website">
<meta property="og:site_name" content="PointSav, Inc.">
<meta property="og:title" content="PointSav, Inc. — Home">
<meta property="og:description" content="A fully transferable data management and customer service platform for the procurement, development, and management of real properties.">
<meta property="og:url" content="https://home.pointsav.com/">
<meta name="twitter:card" content="summary">
<meta name="twitter:title" content="PointSav, Inc.">
<meta name="twitter:description" content="A fully transferable data management and customer service platform for the procurement, development, and management of real properties.">
<script type="application/ld+json">
{"@context":"https://schema.org","@type":"Organization","name":"PointSav, Inc.","url":"https://home.pointsav.com","description":"A fully transferable data management and customer service platform for the procurement, development, and management of real properties.","sameAs":["https://github.com/pointsav","https://software.pointsav.com"]}
</script>
```

---

## home.woodfinegroup.com

**File:** `/srv/foundry/deployments/media-marketing-landing-1/content/index.html`
**Format:** bundler — same as above.
**Insert after:** `<title>Woodfine Capital Projects — Home</title>`

```html
<meta name="description" content="A real property developer with 40 years' experience in the procurement, development, and management of real property.">
<meta name="robots" content="index, follow">
<link rel="canonical" href="https://home.woodfinegroup.com/">
<meta property="og:type" content="website">
<meta property="og:site_name" content="Woodfine Capital Projects">
<meta property="og:title" content="Woodfine Capital Projects — Home">
<meta property="og:description" content="A real property developer with 40 years' experience in the procurement, development, and management of real property.">
<meta property="og:url" content="https://home.woodfinegroup.com/">
<meta name="twitter:card" content="summary">
<meta name="twitter:title" content="Woodfine Capital Projects">
<meta name="twitter:description" content="A real property developer with 40 years' experience in the procurement, development, and management of real property.">
<script type="application/ld+json">
{"@context":"https://schema.org","@type":"Organization","name":"Woodfine Capital Projects","url":"https://home.woodfinegroup.com","description":"A real property developer with 40 years' experience in the procurement, development, and management of real property.","sameAs":["https://github.com/woodfine"]}
</script>
```

---

## Apply script (Python)

Run once per file, substituting `TITLE` and `SEO_BLOCK` values above:

```python
import json, re

path = "/srv/foundry/deployments/media-marketing-landing-2/content/index.html"
title_tag = "<title>PointSav, Inc. — Home</title>"
seo_block = """
<meta name="description" content="A fully transferable data management and customer service platform for the procurement, development, and management of real properties.">
...
"""

with open(path) as f:
    raw = f.read()

m = re.search(r'(<script type="__bundler/template">)(.*?)(</script>)', raw, re.DOTALL)
inner_html = json.loads(m.group(2))
inner_html = inner_html.replace(title_tag, title_tag + seo_block, 1)
new_script = m.group(1) + json.dumps(inner_html) + m.group(3)
new_raw = raw[:m.start()] + new_script + raw[m.end():]

with open(path, "w") as f:
    f.write(new_raw)
```

After editing: verify with `curl -s http://127.0.0.1:9101/ | grep -A5 'description'`
