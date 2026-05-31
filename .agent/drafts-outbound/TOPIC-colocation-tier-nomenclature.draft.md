---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "Co-location Tier Nomenclature"
slug: topic-colocation-tier-nomenclature
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-colocation-tier-nomenclature.es.draft.md
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: taxonomy.py (platform source); config.py (ALPHA_CATEGORIES, ANCHOR_CATEGORIES); build-geometric-ranking.py (DBSCAN two-pass logic); clusters-meta.json Phase 23+Change B; commit 84b7fe7a (Change B span gate)
research_inline: true
created: 2026-05-31
---

# Co-location Tier Nomenclature

The co-location platform classifies every identified retail cluster into one of
three tiers: T1 Regional, T2 District, and T3 Local. This article defines what
each tier means, explains how the classification is determined, and documents
the evolution of the naming convention.

## The Three Tiers

**T1 Regional** denotes the highest-order co-location clusters: those anchored
by the largest retail formats in the market. A cluster reaches T1 when it
contains at least one warehouse club — a membership-based bulk retailer such
as Costco, Sam's Club, or PriceSmart — or when it contains a full-format
hypermarket accompanied by a hardware anchor. The hypermarket-plus-hardware
combination is required for T1 designation because a hypermarket without a
hardware complement indicates a smaller retail catchment than the combined
offer. As of Phase 23+Change B, the platform identifies 1,746 T1 clusters
across 18 countries.

**T2 District** denotes clusters that meet the hypermarket anchor threshold but
do not qualify for T1. This encompasses two distinct situations. A cluster that
contains a hypermarket and hardware anchor but whose member locations span more
than 2.5 km from the geometric centre to the most distant member is classified
as T2 rather than T1. The geometric span gate — introduced in Change B — reflects
the finding that very widely dispersed clusters, even when they contain equivalent
anchor composition to T1 clusters, represent different consumer catchment
dynamics. Tighter clusters indicate denser co-location effects. A cluster that
contains a hypermarket but lacks a hardware anchor is also classified as T2,
regardless of its geometric span. The platform currently identifies 2,726 T2
clusters.

**T3 Local** denotes clusters that contain significant retail co-location but
lack anchor formats sufficient for T1 or T2 designation. T3 clusters typically
contain a mix of mid-format grocery, electronics, and lifestyle retailers without
a full hypermarket presence. They represent locally important retail nodes rather
than the regional and district drawing-power markets of T1 and T2. The platform
currently identifies 2,021 T3 clusters.

## The Two-Pass DBSCAN Algorithm

Tier classification is not assigned independently of cluster discovery: the
tier emerges from the geometry and composition of clusters identified by the
two-pass DBSCAN algorithm.

The first pass uses anchor stores as seeds. DBSCAN (Density-Based Spatial
Clustering of Applications with Noise) groups points that are within a defined
radius of each other into clusters. In the first pass, only the anchor-category
stores — warehouse clubs and full hypermarkets — are used as seed points. This
pass identifies the cores around which high-order retail clusters form.

The second pass fills each identified core cluster with additional retail
locations within the cluster's geometric reach. Hardware anchors, lifestyle
retailers, electronics stores, and other category members are added when they
fall within the radius of an existing seed cluster. The composition of the filled
cluster — which anchor categories are present — determines the tier.

The geometric span measurement used in the Change B gate is taken after the
second pass, when the cluster's full member set is known. Span is measured as
the maximum distance between any two member locations within the cluster.

## The Span Gate: Change B

The 2.5 km span gate was introduced in Change B (commit `84b7fe7a`, 2026-05-28).
Prior to Change B, tier was determined solely by anchor composition: if a cluster
contained the right anchor mix, it was T1, regardless of how spread out its
members were. The gate corrected for a finding that a subset of T2-composition
clusters had span values well above 2.5 km and were representing geographically
diffuse retail distributions rather than the tightly co-located patterns that
define the market analysis use case.

The Change B span gate demoted approximately 667 clusters from T2 to T3, shifting
the T2 count from approximately 3,393 (Phase 22) to 2,726 (Phase 23+Change B) and
increasing T3 from approximately 1,354 to 2,021.

## Naming History

The T1/T2/T3 nomenclature was adopted to provide a stable, non-hierarchical
labelling system that conveys market function without implying a simple rank
ordering. Earlier internal development used placeholder labels — Alpha, Beta,
and Gamma — which were retired before any public output was produced. The T1/T2/T3
labels have been canonical since Phase 18 and appear in all published data files,
TOPIC articles, and JOURNAL manuscripts.

The descriptors — Regional, District, Local — are intended to convey catchment
geography, not administrative hierarchy. A T1 Regional cluster typically serves
a regional catchment extending 20–50 km; a T2 District cluster a more localised
catchment; a T3 Local cluster a neighbourhood or small-town catchment. These are
empirical generalisations, not definitional constraints.

## Counts by Geography (Phase 23+Change B)

The platform covers 6,493 clusters across 18 countries. T1 clusters are most
common in the United States (889), Germany (227), and France (247). T2 clusters
are most common in the United States (1,779), Germany (338), and Great Britain
(400). The full per-country breakdown appears in the co-location summary at
`gis.woodfinegroup.com/research-summary.html`.

---

*Data provenance:* clusters-meta.json Phase 23+Change B (2026-05-29T05:00Z rebuild);
taxonomy definitions in `app-orchestration-gis/taxonomy.py` and `config.py`.
