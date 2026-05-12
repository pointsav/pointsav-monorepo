# Woodfine Management Corp. — Monthly Bookkeeping Pipeline
## Operations Guide

---

## Overview

One script processes a raw monthly bank export and appends it to the master ledger (`Data_2026.xlsx`).  
The original files are never modified. All output is written to new files for review before anything is renamed.

---

## Every month — one command

```bash
cd /home/jennifer/sandbox/inputs/project-bookkeeping/journal-entries

python3 run_monthly_pipeline.py "May Transactions.xlsx" "Data_2026.xlsx"
```

Replace `May Transactions.xlsx` with the actual filename from the bank export.  
Always point at `Data_2026.xlsx` — the script writes a safe copy and leaves the original untouched.

---

## Output files

| File | What it is |
|---|---|
| `<Month>_PROCESSED.xlsx` | Fully processed transactions — open this to fill in manual fields |
| `<Month>_SUMMARY.csv` | Totals by bank, FX count, WEEKENDS breakdown |
| `Data_2026_updated.xlsx` | Master ledger with new month appended — review before renaming |

---

## What the pipeline does automatically

| Step | What happens |
|---|---|
| Fix text dates | Converts `MM/DD/YYYY` text strings to proper Excel date values |
| WEEKENDS | Classifies every date as `Business Day`, `Weekend`, or `Holiday` (BC + federal calendar) |
| INVOICE YEAR / QTR | Auto-detected from the data — flags any spill-over rows from the prior month |
| Foreign currency | Extracts `100 USD @ 1.42` patterns from PAYEE into dedicated columns |
| COMPANY / TYPE / CATEGORY | Filled for any payee already seen in `Data_2026.xlsx` |
| Bank names | Standardises: `TD` → `TD - JMW`, `TD Visa` → `TD - Visa` |
| Duplicate removal | Spill-over rows from the prior bank statement are detected and removed |
| Summary CSV | Written automatically |
| Append | Clean rows appended to `Data_2026_updated.xlsx` |

---

## What still needs manual work in `_PROCESSED.xlsx`

| Field | Why manual |
|---|---|
| RECEIPT | You match up whether you have the receipt, invoice, or CC slip |
| DESCRIPTION | Varies per transaction even for the same payee |
| COMPANY / TYPE / CATEGORY | For new payees not yet in Data_2026 |
| HST / PST / CDN before GST / GST | Tax treatment per purchase |
| TRAVEL DAYS | Manual entry |

---

## After reviewing `Data_2026_updated.xlsx`

When satisfied, rename the files:

```bash
mv "Data_2026.xlsx" "Data_2026_backup_May.xlsx"
mv "Data_2026_updated.xlsx" "Data_2026.xlsx"
```

---

## Scripts in this folder

| Script | Purpose |
|---|---|
| `run_monthly_pipeline.py` | **Main script — run this every month** |
| `fill_weekends.py` | Fill WEEKENDS column (used by pipeline; also runnable standalone) |
| `fill_year_qtr.py` | Fill INVOICE YEAR and QTR (standalone) |
| `tool-excel-currency extractor.py` | Extract FX from PAYEE (standalone, use `--input` flag) |
| `fix_text_dates.py` | Fix text-formatted dates in place (standalone) |
| `duplicate_check.py` | Check for duplicate date+CDN pairs between two files (standalone) |
| `remove_duplicates.py` | Remove duplicate rows from the April file (standalone) |
| `monthly_summary.py` | Generate summary CSV (standalone) |
| `append_to_data2026.py` | Append processed rows to master ledger (standalone) |
| `fill_lookup_fields.py` | Fill COMPANY/TYPE/CATEGORY + fix bank names (standalone) |

---

## Quarter reference

| Months | Quarter |
|---|---|
| January, February, March | Q1 |
| April, May, June | Q2 |
| July, August, September | Q3 |
| October, November, December | Q4 |

---

## Notes

- The pipeline lookup grows each month — the more payees you fill in manually, the more it auto-fills next time.
- Spill-over rows (prior month transactions in the current bank export) are detected automatically by duplicate check and removed before appending.
- If a new bank account appears that needs a name standardisation, add it to the `BANK_RENAMES` dictionary near the top of `run_monthly_pipeline.py`.
- The holiday calendar covers 2023–2029 and does not need updating until 2030.
