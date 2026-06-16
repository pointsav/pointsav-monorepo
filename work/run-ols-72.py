#!/usr/bin/env python3
"""
run-ols-72.py — J1 §7.2 primary OLS specification

  catchment_entropy ~ tier + log(pop_150km) + C(country)

Uses O-D work flow entropy (od_work_entropy) as the dependent variable.
O-D data is currently available for US (LODES) and ES (MITMA) only;
the sample is limited to clusters with both od_work_entropy and pop_150km.

Outputs:
  work/ols-72-results.txt  — full regression summary + key coefficients
"""

import numpy as np
import pandas as pd
import statsmodels.formula.api as smf
import warnings
import os

warnings.filterwarnings('ignore')

WORK = '/srv/foundry/clones/project-gis/pointsav-monorepo/app-orchestration-gis/work'
OUT  = os.path.dirname(__file__)

df_full = pd.read_csv(f'{WORK}/clusters-ols.csv')
print(f"Full dataset: {len(df_full):,} clusters, {df_full['country'].nunique()} countries")

# Filter to clusters with O-D entropy + population
df = df_full[(df_full['od_work_entropy'].notna()) & (df_full['pop_150km'] > 0)].copy()
print(f"With O-D entropy + pop_150km: {len(df):,} clusters, {df['country'].nunique()} countries")

if len(df) < 30:
    print("WARN: fewer than 30 observations with O-D data — §7.2 not yet runnable (Phase 24B)")
    raise SystemExit(0)

# Drop span_km outliers (data quality guard)
df = df[df['span_km'] >= 0.05].copy()

# Country FE requires ≥10 obs per country
country_counts = df['country'].value_counts()
valid_countries = country_counts[country_counts >= 10].index
df = df[df['country'].isin(valid_countries)].copy()
print(f"After country min-10 filter: {len(df):,} clusters, {df['country'].nunique()} countries")
print(f"Tier: T1={df.t1_dummy.sum():,} T2={df.t2_dummy.sum():,} T3={(len(df)-df.t1_dummy.sum()-df.t2_dummy.sum()):,}")
print(f"Entropy: mean={df['od_work_entropy'].mean():.3f} sd={df['od_work_entropy'].std():.3f}")

# Log-transform population
df['log_pop_150km'] = np.log(df['pop_150km'])

# ── §7.2 primary specification ─────────────────────────────────────────────────
print("\n" + "="*70)
print("§7.2 SPEC: catchment_entropy ~ T1 + T2 + log(pop_150km) + C(country)")
print("="*70)

formula = 'od_work_entropy ~ t1_dummy + t2_dummy + log_pop_150km + C(country)'
m = smf.ols(formula, data=df).fit(cov_type='cluster', cov_kwds={'groups': df['country']})
print(m.summary())

# ── Key results for paper body ────────────────────────────────────────────────
t1     = m.params.get('t1_dummy', float('nan'))
t2     = m.params.get('t2_dummy', float('nan'))
pop    = m.params.get('log_pop_150km', float('nan'))
t1_ci  = m.conf_int().loc['t1_dummy'] if 't1_dummy' in m.params else (float('nan'), float('nan'))
t2_ci  = m.conf_int().loc['t2_dummy'] if 't2_dummy' in m.params else (float('nan'), float('nan'))
pop_ci = m.conf_int().loc['log_pop_150km'] if 'log_pop_150km' in m.params else (float('nan'), float('nan'))
t1_p   = m.pvalues.get('t1_dummy', float('nan'))
t2_p   = m.pvalues.get('t2_dummy', float('nan'))
pop_p  = m.pvalues.get('log_pop_150km', float('nan'))

print("\n── KEY RESULTS FOR §7.2 BODY TEXT ──")
print(f"N = {int(m.nobs):,}  R² = {m.rsquared:.3f}")
print(f"T1 coef: {t1:+.3f} [{t1_ci[0]:+.3f}, {t1_ci[1]:+.3f}]  p={t1_p:.4f}")
print(f"T2 coef: {t2:+.3f} [{t2_ci[0]:+.3f}, {t2_ci[1]:+.3f}]  p={t2_p:.4f}")
print(f"log(pop) coef: {pop:+.3f} [{pop_ci[0]:+.3f}, {pop_ci[1]:+.3f}]  p={pop_p:.4f}")

# ── Write text output ─────────────────────────────────────────────────────────
out_path = os.path.join(OUT, 'ols-72-results.txt')
with open(out_path, 'w') as f:
    f.write("J1 §7.2 OLS — catchment_entropy ~ tier + log(pop_150km) + country FE\n")
    f.write(f"Data: Phase 23+Change B | O-D: US LODES + ES MITMA only\n")
    f.write(f"N = {int(m.nobs):,} clusters | {df['country'].nunique()} countries\n")
    f.write(f"Tier: T1={df.t1_dummy.sum():,} T2={df.t2_dummy.sum():,} T3={(len(df)-df.t1_dummy.sum()-df.t2_dummy.sum()):,}\n\n")
    f.write(str(m.summary()) + "\n\n")
    f.write("── KEY COEFFICIENTS ──\n")
    f.write(f"T1 coef: {t1:+.3f} [{t1_ci[0]:+.3f}, {t1_ci[1]:+.3f}]  p={t1_p:.4f}\n")
    f.write(f"T2 coef: {t2:+.3f} [{t2_ci[0]:+.3f}, {t2_ci[1]:+.3f}]  p={t2_p:.4f}\n")
    f.write(f"log(pop) coef: {pop:+.3f} [{pop_ci[0]:+.3f}, {pop_ci[1]:+.3f}]  p={pop_p:.4f}\n")
    f.write(f"R² = {m.rsquared:.3f}  Adj-R² = {m.rsquared_adj:.3f}\n")
print(f"Full summary written → {out_path}")
