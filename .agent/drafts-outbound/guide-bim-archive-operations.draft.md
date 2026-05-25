---
schema: foundry-draft-v1
draft_id: guide-bim-archive-operations
language_protocol: PROSE-GUIDE
state: ready-for-sweep
target_path: customer/woodfine-fleet-deployment/cluster-totebox-property/guide-bim-archive-operations.md
created: 2026-05-06T19:10:00Z
author: task@project-bim
cites: [ifc-4-3, cobiev3, bcf-3-0, ifctester, ids-1-0, ifccsv]
research_done_count: 2
research_suggested_count: 0
open_questions_count: 1
research_provenance: |
  Plan Part 3 (Totebox Archive vault layout + workflows): /home/mathew/.claude/plans/1-we-need-to-frolicking-taco.md
  Cluster manifest: /srv/foundry/clones/project-bim/.agent/manifest.md
  Sub-agent C (COBie/IDS exchange workflows): C-bim-regulatory-acceptance-2026-04-28.md
research_inline: false
---

# Guide: BIM Archive Operations

This guide covers operating a Totebox Archive instance — the long-term IFC model and documentation vault for a building asset. All commands run on the deployment host as a user with access to the archive vault directory.

All paths below use `<vault>/` as the vault root (e.g., `/opt/foundry/vaults/cluster-totebox-property-1/`).

---

## Vault Layout

A Totebox Archive vault has a fixed directory structure. Do not add or move top-level directories; tooling reads from fixed paths.

```
<vault>/
├── MANIFEST.md                ← Vault identity, asset metadata
├── README.md                  ← English description
├── README.es.md               ← Spanish description
├── ifc/                       ← IFC model files by discipline
│   ├── architectural/
│   ├── structural/
│   ├── mechanical/
│   └── combined/
├── elements/                  ← Per-element JSON records (IfcGloballyUniqueId index)
├── bcf/                       ← BCF 3.0 issue files
├── ids/                       ← IDS constraint files for this asset
├── materials/                 ← Material specification sheets
├── codes/                     ← Regulatory documentation (PDFs, web archives)
├── geometry/                  ← Auxiliary geometry (DXF, DWG, survey)
├── drawings/                  ← 2D production drawings (PDF)
├── objects/                   ← BIM object specifications
├── refs/                      ← External references and permits
└── logs/                      ← Operation logs (auto-generated)
```

**MANIFEST.md** declares:
```yaml
asset_id: cluster-totebox-property-1
asset_name: "Woodfine Property Asset — Site 1"
ifc_schema: IFC4X3
created: <ISO date>
last_ingestion: <ISO date>
model_guids:
  architectural: <IfcProject GUID>
  structural: <IfcProject GUID>
```

---

## Ingestion Workflow

Ingestion adds a new IFC model or model update to the vault.

### Prerequisites

- `IfcOpenShell` installed: `python3 -m pip install ifcopenshell`
- `ifctester` installed: `python3 -m pip install ifctester`
- Source IFC file in IFC 4.3 format (IFC4X3 schema)

### Steps

**1. Validate the incoming IFC file.**

Before ingesting, validate the file against the vault's registered IDS constraints:

```bash
ifctester --ids <vault>/ids/<overlay-id>.ids \
          --ifc <path-to-incoming.ifc> \
          --report json \
          --output <vault>/logs/validation-<YYYY-MM-DD>.json
```

Check the output for failures. A pass is required for ingestion. If failures exist, return the model to the design team with the validation report.

**2. Extract per-element records.**

Parse the IFC file and write one JSON record per element to `elements/`:

```python
import ifcopenshell, json, os

model = ifcopenshell.open("incoming.ifc")
elements_dir = "<vault>/elements/"

for element in model.by_type("IfcElement"):
    record = {
        "guid": element.GlobalId,
        "type": element.is_a(),
        "name": element.Name,
        "psets": {}
    }
    # collect property sets
    for rel in element.IsDefinedBy:
        if rel.is_a("IfcRelDefinesByProperties"):
            pset = rel.RelatingPropertyDefinition
            if pset.is_a("IfcPropertySet"):
                props = {p.Name: str(p.NominalValue) for p in pset.HasProperties
                         if hasattr(p, "NominalValue")}
                record["psets"][pset.Name] = props

    with open(os.path.join(elements_dir, f"{element.GlobalId}.json"), "w") as f:
        json.dump(record, f, indent=2)
```

**3. Copy the IFC file to the vault.**

```bash
cp incoming.ifc <vault>/ifc/architectural/architectural-v<n>.ifc
```

Use sequential version numbers (`v1`, `v2`, etc.) not dates — the git commit history provides the date record.

**4. Commit the ingestion.**

```bash
cd <vault>
git add ifc/ elements/ logs/
git commit -m "ingest: architectural model v<n> — <brief description>"
```

**5. Update MANIFEST.md.**

Update `last_ingestion` and the relevant `model_guids` entry, then commit:

```bash
git add MANIFEST.md
git commit -m "manifest: update last_ingestion + model GUID for architectural v<n>"
```

---

## Export Workflows

### COBie Export

COBie (Construction Operations Building Information Exchange) is the facility handover data format required by US and UK government clients. Export COBie from an IFC model using `ifccsv` (IfcOpenShell CSV tools):

```bash
python3 -m ifccsv \
    -i <vault>/ifc/combined/combined-v<n>.ifc \
    -s COBie \
    -o <vault>/refs/COBie-v<n>.xlsx
```

The `-s COBie` flag selects the COBie mapping schema. Verify the output spreadsheet contains:
- Facility, Floor, Space, Type, Component, System, Connection, Attribute, Document, Coordinate sheets.
- No empty `ExternalIdentifier` cells in the Component sheet.

Commit the COBie file:
```bash
git add refs/COBie-v<n>.xlsx
git commit -m "export: COBie v<n> from combined model"
```

### IDS Validation Export

Generate a full validation report for regulatory submission:

```bash
ifctester \
    --ids <vault>/ids/<jurisdiction>-<overlay-id>.ids \
    --ifc <vault>/ifc/combined/combined-v<n>.ifc \
    --report html \
    --output <vault>/refs/ids-validation-<jurisdiction>-<YYYY-MM-DD>.html
```

The HTML report is suitable for direct submission to building permit authorities in jurisdictions that accept IDS-based compliance reports. Commit the report alongside the model version.

### BCF 3.0 Issue Export

BCF (BIM Collaboration Format) files record design issues referencing specific model elements by GUID. To export all open issues from the `bcf/` directory as a single BCF 3.0 zip file for exchange with a consultant:

```bash
# Requires bcfpython: pip install bcfpython
python3 - <<'EOF'
import bcf.v3.bcf_file as bcf_file
import zipfile, os, glob

b = bcf_file.BcfFile()
for f in glob.glob("<vault>/bcf/*.bcf"):
    b.add_topic_from_file(f)
b.save("<vault>/refs/issues-export-<YYYY-MM-DD>.bcfzip")
EOF
```

---

## Adding Issues (BCF 3.0)

To record a new design issue referencing a model element:

```bash
# Requires bcfpython
python3 - <<'EOF'
import bcf.v3.bcf_file as bcf_file
from bcf.v3.model.topic import BimSnippet
import datetime

b = bcf_file.BcfFile.load("<vault>/bcf/issues.bcf")  # or create new
topic = b.add_topic(
    title="Wall ThermalTransmittance below BCBC 2024 requirement",
    description="IfcWall GUID=<guid> has ThermalTransmittance=0.300, limit is 0.210",
    author="task@project-bim",
    date=datetime.datetime.now().isoformat()
)
topic.reference_object(guid="<ifcGloballyUniqueId>")
b.save("<vault>/bcf/issues.bcf")
EOF

git add bcf/
git commit -m "bcf: add wall thermal transmittance issue <short-id>"
```

---

## Log Rotation

Validation logs accumulate in `logs/`. Rotate logs older than 90 days to `logs/archive/`:

```bash
find <vault>/logs/ -maxdepth 1 -name "*.json" -mtime +90 \
    -exec mv {} <vault>/logs/archive/ \;
git add logs/
git commit -m "logs: rotate validation logs older than 90 days"
```

---

## Open Questions

1. The `ifccsv` COBie export requires `ifcopenshell` ≥ 0.7.0 for IFC4X3 model support. Older versions may silently downgrade the schema. Verify `python3 -c "import ifcopenshell; print(ifcopenshell.version)"` returns 0.7.0 or higher before running COBie exports on IFC4X3 models.
