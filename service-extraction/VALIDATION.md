# service-extraction — Validation

Measurement record for `service-extraction` against the test corpus.

**Claude Code:** update this file after every build that changes extraction behaviour. Append a new version entry — do not edit prior entries.

---

## Test corpus

**Location:** `samples/` in this directory.

The current corpus is 10 real `.eml` files spanning:

| Filename | Type | Notes |
|---|---|---|
| `Culinary_Highlights_at_Schloss_Elmau.eml` | HTML-heavy marketing | German hotel, embedded imagery |
| `Nvidia_s_Rack_Era___SaaS_Faces_Agents__Enterprise_AI_Hardens.eml` | Newsletter | TLDR IT digest format |
| `Strahov_Akropolis_a_most_v_Ostrave.eml` | Newsletter | Czech language, Stavbaweb |
| `Tender_Invitation_Market_Screen_Global_Real_Estate_unlisted.eml` | Business direct | Unlisted real estate tender |
| `The_Earthen_Towers_of_Shibam_A_Vertical_City_in_the_Yemeni_Desert.eml` | Article digest | ArchDaily editorial |
| `The_space_age_started_100_years_ago_today.eml` | Newsletter | The Conversation |
| `This_week_Technical_Foundations_for_Content_Designers.eml` | Newsletter | UX Playbook — interview format |
| `__Check_out_these_Perfect_Pairings_for_Trade_Gothic_Next_.eml` | Marketing | MyFonts product promo |
| `___This_week_on_How_I_AI__From_Figma_to_Claude_Code_and_back___From_journalist_to_iOS_developer.eml` | Newsletter | Lenny's Newsletter |
| `ask_Ryan_anything.eml` | Interview digest | Long-form, body-prose names (Ryan Nguyen) |

Ground-truth extraction targets are in `samples/expected.yaml` (to be created during v0.4 development — see Reproduction section).

---

## Extraction targets per email

For each `.eml`, the ground-truth extraction target is defined as:

1. **Sender name** — from header display name or signature, whichever is more specific.
2. **Sender organisation** — from signature, email domain, or body-mentioned.
3. **Sender contact details** — email address (always), phone number (when present in signature).
4. **Recipients** — from `To` and `Cc` headers only. `Bcc` is never present in stored `.eml`.
5. **Body-mentioned people with role or affiliation** — e.g. "Ryan Rumsey (CDO School)" in the `ask_Ryan` thread (the guest of sender Chris Nguyen, UX Playbook). This is the hardest category and the primary v0.4 gap.
6. **Body-mentioned organisations** — company names referenced in prose.
7. **First-party content URLs** — actual article links, not tracking URLs.
8. **Classification** — one of: Newsletter, Business, Transactional, Personal.

## Scoring methodology

**Per-email score** = `(correctly extracted items) / (ground-truth items) × 100`

A "correctly extracted item" means the record appears in the output with the correct canonical form. Case-insensitive matching on names; exact matching on email addresses.

**Corpus score** = unweighted arithmetic mean of per-email scores.

**Thresholds:**

| Range | Meaning |
|---|---|
| <70% | Reject — worse than v0.1, do not ship |
| 70–80% | Regression — investigate before shipping |
| 80–90% | Baseline — ship only if no prior version was higher |
| **>90%** | **v0.4 Core target** |
| >95% | Stretch target, not required for v0.4 Core |

---

## Version results

### v0.1 — regex only, no body-prose extraction

- **Date:** 2026-04-17
- **Score:** ~68% (estimated, no formal measurement recorded)
- **Pipeline:** regex + title-case heuristic
- **Notes:** Starting point. Known case-bleed problem — lowercase verbs like "writing" and "disrupted" occasionally picked up as names. Retired.

### v0.2 — regex + signature detection + URL filtering

- **Date:** 2026-04-18
- **Score:** Not formally measured. Qualitatively validated.
- **Pipeline:** `mailparse` + `html2text` + regex. Tracking URL filter. Signature block heuristic.
- **Verified qualitatively:**
  - 10/10 newsletters classified correctly
  - Melina Herzig (PPCmetrics) signature parsed end-to-end: name, role, phone, company
  - Czech "Mariana ze Stavby" handled correctly
  - "Chris from UX Playbook" pattern split into person + company
  - 125 tracking URLs filtered across the 10-email corpus
- **Known gaps:** body-prose names (Ryan Nguyen case), multi-word organisations in prose, titles separated from the person they describe.
- **Status:** current production baseline.

### v0.4 — v0.2 + Aho-Corasick + CGM + Shannon entropy + signature boundary

- **Date:** [pending build]
- **Score:** [pending measurement]
- **Target:** >90%
- **Techniques added:** see ROADMAP.md #2, #3, #4, #5.
- **Notes:** [to be filled in during build]

---

## Reproducing measurements

The ground-truth file `samples/expected.yaml` and the scoring script `scripts/score_against_expected.sh` are part of the v0.4 development scope. Expected structure of the ground-truth file:

```yaml
# samples/expected.yaml
Culinary_Highlights_at_Schloss_Elmau.eml:
  classification: Marketing
  sender_name: [expected name]
  sender_org: Schloss Elmau
  sender_email: [expected address]
  recipients: [list]
  body_mentioned_people: []
  body_mentioned_orgs: [list]
  first_party_urls: [list]
# ... one block per .eml
```

Once `expected.yaml` exists and the scoring script is written, measurements run as:

```sh
./build.sh
mkdir -p /tmp/validation-totebox
for eml in samples/*.eml; do
  ./target/release/service-extraction "$eml" /tmp/validation-totebox
done
./scripts/score_against_expected.sh /tmp/validation-totebox samples/expected.yaml
```

The scoring script should output per-email scores and a corpus mean. It should also write a machine-readable result file that can be appended to this document's version table.

---

## Known limitations of this methodology

- **Sample size.** 10 emails is a small corpus. A v0.4 that scores 91% against 10 emails is less statistically reliable than a v0.5 that scores 89% against 200 emails. The corpus should grow before tightening the threshold.
- **Ground-truth subjectivity.** Some extractions are genuinely ambiguous. "Ryan Nguyen" — is he the sender's guest, the subject of the thread, or both? The ground-truth file should document judgment calls alongside the target values.
- **Precision vs recall.** The current scoring counts only correctly extracted items against the ground-truth count. It does not penalise false positives (extracting something the ground truth did not list). v0.5 scoring should add a precision term to guard against that.

These are known issues, not reasons to delay v0.4 measurement. A flawed measurement is still more informative than no measurement.
