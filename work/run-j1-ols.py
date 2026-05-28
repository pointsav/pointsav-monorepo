#!/usr/bin/env python3
"""
J1 OLS regression — JOURNAL-retail-colocation §7.2 (partial specification)

Two regressions with available Phase 22 data:

  Model 1 — Geometric tier characterisation:
      log(span_km) ~ T1_dummy + T2_dummy + C(country)
      T3 is the reference tier; country FE absorbed.
      Tests: are T1/T2 clusters geometrically larger than T3 after controlling
      for cross-country composition differences?

  Model 2 — Composition classification LPM (linear probability model):
      T1_dummy ~ has_price_club + has_electronics + has_lifestyle + has_sport
                 + log(span_km) + tight + C(country)
      Tests: do composition features jointly predict T1 membership beyond
      the tier-defining anchor pair (hypermarket + hardware), controlling for
      geometry and country?

Note: §7.2 primary specification (log(od_work) ~ tier + log(pop_150km) + country FE)
is pending O-D data + Kontur population join (Phase 24B).

Outputs:
  work/figures/F6-ols-coefficients.png  — coefficient forest plot
  work/ols-results.txt                  — full regression summary
"""

import numpy as np
import pandas as pd
import statsmodels.formula.api as smf
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import matplotlib.patches as mpatches
import os, warnings
warnings.filterwarnings('ignore')

DATA_DIR = '/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work'
OUT_DIR  = os.path.join(os.path.dirname(__file__), 'figures')
os.makedirs(OUT_DIR, exist_ok=True)

# ── Load data ─────────────────────────────────────────────────────────────────
df = pd.read_csv(f'{DATA_DIR}/clusters-ols.csv')
print(f"Loaded: {len(df):,} clusters, {df['country'].nunique()} countries")

# Drop very small clusters (span_km < 0.05 km = likely data quality)
df = df[df['span_km'] >= 0.05].copy()
df['log_span_km'] = np.log(df['span_km'])
print(f"After span filter: {len(df):,} clusters")

# Country counts for SE filter — need >=2 obs per country for FE
country_counts = df['country'].value_counts()
valid_countries = country_counts[country_counts >= 10].index
df = df[df['country'].isin(valid_countries)].copy()
print(f"After country min-10 filter: {len(df):,} clusters, {df['country'].nunique()} countries")
print(f"Tier distribution: T1={df.t1_dummy.sum():,} T2={df.t2_dummy.sum():,} T3={(len(df)-df.t1_dummy.sum()-df.t2_dummy.sum()):,}")

# ── Model 1 — Geometric tier characterisation ─────────────────────────────────
print("\n" + "="*70)
print("MODEL 1: log(span_km) ~ T1 + T2 + C(country)")
print("="*70)

formula1 = 'log_span_km ~ t1_dummy + t2_dummy + C(country)'
m1 = smf.ols(formula1, data=df).fit(cov_type='cluster', cov_kwds={'groups': df['country']})
print(m1.summary())

# ── Model 2 — Composition classification LPM ──────────────────────────────────
print("\n" + "="*70)
print("MODEL 2: T1_dummy ~ composition + log(span_km) + tight + C(country)")
print("="*70)

# Exclude clusters where T1 is mechanically determined (hypermarket + hardware
# both present) — test only the distinguishing composition features.
# Actually: run on full sample; has_hypermarket and has_hardware nearly constant
# at T1/T2, so drop to avoid near-perfect multicollinearity.
formula2 = ('t1_dummy ~ has_price_club + has_lifestyle + has_electronics + has_sport '
            '+ log_span_km + tight + C(country)')
m2 = smf.ols(formula2, data=df).fit(cov_type='cluster', cov_kwds={'groups': df['country']})
print(m2.summary())

# ── Write text summary ─────────────────────────────────────────────────────────
txt_path = os.path.join(os.path.dirname(__file__), 'ols-results.txt')
with open(txt_path, 'w') as f:
    f.write("J1 OLS REGRESSION RESULTS — Phase 22 clusters\n")
    f.write(f"N = {len(df):,} clusters | {df['country'].nunique()} countries\n")
    f.write(f"Tier: T1={df.t1_dummy.sum():,} T2={df.t2_dummy.sum():,} T3={(len(df)-df.t1_dummy.sum()-df.t2_dummy.sum()):,}\n\n")
    f.write("MODEL 1: log(span_km) ~ T1 + T2 + country FE\n")
    f.write(str(m1.summary()) + "\n\n")
    f.write("MODEL 2: T1_dummy ~ composition + log(span_km) + tight + country FE\n")
    f.write(str(m2.summary()) + "\n")
print(f"\nFull summary written to: {txt_path}")

# ── Build coefficient table for F6 ────────────────────────────────────────────
def extract_coefs(model, keep_vars, labels):
    rows = []
    for var, label in zip(keep_vars, labels):
        if var in model.params:
            coef = model.params[var]
            ci_lo, ci_hi = model.conf_int().loc[var]
            pval = model.pvalues[var]
            rows.append({'label': label, 'coef': coef, 'ci_lo': ci_lo, 'ci_hi': ci_hi,
                         'pval': pval, 'sig': '***' if pval < 0.001 else ('**' if pval < 0.01 else ('*' if pval < 0.05 else ''))})
    return pd.DataFrame(rows)

m1_vars   = ['t1_dummy', 't2_dummy']
m1_labels = ['T1 tier (vs T3)', 'T2 tier (vs T3)']

m2_vars   = ['has_price_club', 'has_lifestyle', 'has_electronics', 'has_sport',
             'log_span_km', 'tight']
m2_labels = ['Warehouse-club anchor', 'Lifestyle anchor', 'Electronics anchor',
             'Sporting-goods anchor', 'log(span km)', 'Tight configuration']

df1 = extract_coefs(m1, m1_vars, m1_labels)
df2 = extract_coefs(m2, m2_vars, m2_labels)

# ── Draw F6 forest plot ────────────────────────────────────────────────────────
TIER_BLUE  = '#2166ac'
COMP_GREEN = '#1a9641'
NULL_GREY  = '#999999'

fig, axes = plt.subplots(1, 2, figsize=(9.8, 5.0))  # ~190 mm at 72 dpi

for ax, df_c, color, title, xlabel, refline in [
    (axes[0], df1, TIER_BLUE,
     'Model 1: Geometric tier characterisation\nDependent: log(span km)',
     'OLS coefficient (log scale)\n[country FE; SE clustered by country]', 0.0),
    (axes[1], df2, COMP_GREEN,
     'Model 2: T1 classification (LPM)\nDependent: T1 tier dummy',
     'OLS coefficient (probability scale)\n[country FE; SE clustered by country]', 0.0),
]:
    y_pos = list(range(len(df_c)))[::-1]  # top-to-bottom ordering
    for i, (_, row) in enumerate(df_c.iterrows()):
        y = y_pos[i]
        sig_color = color if row['pval'] < 0.05 else NULL_GREY
        ax.errorbar(row['coef'], y,
                    xerr=[[row['coef'] - row['ci_lo']], [row['ci_hi'] - row['coef']]],
                    fmt='o', color=sig_color, ecolor=sig_color,
                    capsize=4, markersize=6, linewidth=1.5, zorder=3)
        # significance star
        if row['sig']:
            ax.text(row['ci_hi'] + abs(row['ci_hi'] - row['ci_lo']) * 0.05, y,
                    row['sig'], va='center', ha='left', fontsize=7, color=sig_color)

    ax.axvline(refline, color='#444444', linewidth=0.8, linestyle='--', zorder=1)
    ax.set_yticks(y_pos)
    ax.set_yticklabels(df_c['label'].tolist(), fontsize=8)
    ax.set_xlabel(xlabel, fontsize=7.5)
    ax.set_title(title, fontsize=8.5, fontweight='bold', pad=6)
    ax.spines['top'].set_visible(False)
    ax.spines['right'].set_visible(False)
    ax.grid(axis='x', linestyle=':', linewidth=0.5, alpha=0.5, zorder=0)

# Stats annotations
def stats_note(ax, model, df_c):
    n = int(model.nobs)
    r2 = model.rsquared
    note = f'N={n:,}  R²={r2:.3f}'
    ax.text(0.97, 0.03, note, transform=ax.transAxes,
            ha='right', va='bottom', fontsize=7, color='#444444',
            style='italic')

stats_note(axes[0], m1, df1)
stats_note(axes[1], m2, df2)

# Shared caption note
fig.text(0.5, 0.01,
         'Note: 95% CI shown. Country fixed effects not displayed. '
         'SE clustered by country (n=15 clusters). '
         '§7.2 primary specification (log[od_work] ~ tier + log[pop]) pending Phase 24B.',
         ha='center', fontsize=6.5, color='#666666', style='italic')

fig.suptitle('Figure 6 — OLS Coefficient Plot: Tier Geometry and Composition Predictors',
             fontsize=9.5, fontweight='bold', y=1.01)

plt.tight_layout(rect=[0, 0.04, 1, 0.98])
out_path = os.path.join(OUT_DIR, 'F6-ols-coefficients.png')
fig.savefig(out_path, dpi=300, bbox_inches='tight')
print(f"F6 saved to: {out_path}")

# ── Print key results for paper ────────────────────────────────────────────────
print("\n── KEY RESULTS FOR §7.2 BODY TEXT ──")
t1 = m1.params['t1_dummy']
t2 = m1.params['t2_dummy']
t1_ci = m1.conf_int().loc['t1_dummy']
t2_ci = m1.conf_int().loc['t2_dummy']
t1_p  = m1.pvalues['t1_dummy']
t2_p  = m1.pvalues['t2_dummy']
print(f"Model 1 — T1 coef: {t1:+.3f} [{t1_ci[0]:+.3f}, {t1_ci[1]:+.3f}] p={t1_p:.4f}")
print(f"Model 1 — T2 coef: {t2:+.3f} [{t2_ci[0]:+.3f}, {t2_ci[1]:+.3f}] p={t2_p:.4f}")
print(f"Model 1 — R² = {m1.rsquared:.3f}  N = {int(m1.nobs):,}")

pc = m2.params['has_price_club']
pc_ci = m2.conf_int().loc['has_price_club']
pc_p  = m2.pvalues['has_price_club']
ls_val = m2.params.get('log_span_km', float('nan'))
ls_ci = m2.conf_int().loc['log_span_km'] if 'log_span_km' in m2.params else (float('nan'), float('nan'))
print(f"Model 2 — has_price_club coef: {pc:+.3f} [{pc_ci[0]:+.3f}, {pc_ci[1]:+.3f}] p={pc_p:.4f}")
print(f"Model 2 — log(span_km) coef:   {ls_val:+.3f} [{ls_ci[0]:+.3f}, {ls_ci[1]:+.3f}]")
print(f"Model 2 — R² = {m2.rsquared:.3f}  N = {int(m2.nobs):,}")

print("\nDone.")
