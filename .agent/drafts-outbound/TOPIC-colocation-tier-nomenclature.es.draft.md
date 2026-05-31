---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "Nomenclatura de Niveles de Colocalización"
slug: topic-colocation-tier-nomenclature
language: es
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-colocation-tier-nomenclature.draft.md
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: taxonomy.py (fuente de la plataforma); config.py (ALPHA_CATEGORIES, ANCHOR_CATEGORIES); build-geometric-ranking.py (lógica DBSCAN de dos pasadas); clusters-meta.json Fase 23+Cambio B; commit 84b7fe7a (barrera de extensión Cambio B)
research_inline: true
created: 2026-05-31
---

# Nomenclatura de Niveles de Colocalización

La plataforma de colocalización clasifica cada clúster comercial identificado en
uno de tres niveles: T1 Regional, T2 Distrito y T3 Local. Este artículo define
qué significa cada nivel, explica cómo se determina la clasificación y documenta
la evolución de la convención de nomenclatura.

## Los Tres Niveles

**T1 Regional** designa los clústeres de colocalización de mayor jerarquía: los
anclados por los formatos comerciales más grandes del mercado. Un clúster alcanza
T1 cuando contiene al menos un club de membresía mayorista — como Costco, Sam's
Club o PriceSmart — o cuando contiene un hipermercado de formato completo
acompañado de un ancla de ferretería. La combinación de hipermercado más
ferretería es necesaria para la designación T1, porque un hipermercado sin un
complemento de ferretería indica un área de captación más reducida. Con los datos
de la Fase 23+Cambio B, la plataforma identifica 1.746 clústeres T1 en 18 países.

**T2 Distrito** designa los clústeres que cumplen el umbral de ancla de
hipermercado, pero no califican para T1. Esto abarca dos situaciones distintas.
Un clúster que contiene un hipermercado y un ancla de ferretería, pero cuyos
miembros están separados más de 2,5 km desde el centro geométrico hasta el
miembro más lejano, se clasifica como T2 en lugar de T1. La barrera de extensión
geométrica — introducida en el Cambio B — refleja que los clústeres muy dispersos
representan una dinámica de captación de consumidores diferente. Los clústeres más
compactos indican efectos de colocalización más intensos. Un clúster que contiene
un hipermercado pero carece de un ancla de ferretería también se clasifica como
T2, independientemente de su extensión geométrica. La plataforma identifica
actualmente 2.726 clústeres T2.

**T3 Local** designa los clústeres que contienen colocalización comercial
significativa, pero que carecen de formatos ancla suficientes para la designación
T1 o T2. Los clústeres T3 típicamente contienen una combinación de supermercados
de formato mediano, tiendas de electrónica y minoristas de estilo de vida sin una
presencia de hipermercado completo. Representan nodos comerciales localmente
importantes. La plataforma identifica actualmente 2.021 clústeres T3.

## El Algoritmo DBSCAN de Dos Pasadas

La clasificación de nivel no se asigna de forma independiente al descubrimiento
de clústeres: el nivel emerge de la geometría y composición de los clústeres
identificados por el algoritmo DBSCAN de dos pasadas.

La primera pasada utiliza tiendas ancla como semillas. DBSCAN agrupa los puntos
que están dentro de un radio definido entre sí en clústeres. En la primera pasada,
solo se usan como puntos semilla las tiendas de categoría ancla: clubs mayoristas
e hipermercados de formato completo. Esta pasada identifica los núcleos en torno
a los que se forman los clústeres comerciales de alto orden.

La segunda pasada rellena cada núcleo de clúster identificado con ubicaciones
comerciales adicionales dentro del alcance geométrico del clúster. Las anclas de
ferretería, minoristas de estilo de vida, tiendas de electrónica y otros miembros
de categoría se añaden cuando caen dentro del radio de un clúster semilla
existente. La composición del clúster relleno — qué categorías ancla están
presentes — determina el nivel.

## La Barrera de Extensión: Cambio B

La barrera de extensión de 2,5 km se introdujo en el Cambio B (commit `84b7fe7a`,
28 de mayo de 2026). Antes del Cambio B, el nivel se determinaba únicamente por
la composición de las anclas. La barrera corrigió el hallazgo de que un subconjunto
de clústeres con composición T2 tenía valores de extensión muy superiores a los
2,5 km y representaba distribuciones comerciales geográficamente difusas en lugar
de los patrones de colocalización compacta que definen el caso de uso del análisis
de mercado.

La barrera del Cambio B relegó aproximadamente 667 clústeres de T2 a T3,
reduciendo el recuento de T2 de aproximadamente 3.393 (Fase 22) a 2.726
(Fase 23+Cambio B) y aumentando el de T3 de aproximadamente 1.354 a 2.021.

## Historia de la Nomenclatura

La nomenclatura T1/T2/T3 fue adoptada para proporcionar un sistema de etiquetado
estable y no jerárquico que transmita la función de mercado sin implicar una
simple ordenación por rango. Un desarrollo interno anterior utilizó etiquetas
provisionales — Alfa, Beta y Gamma — que fueron retiradas antes de que se
produjera cualquier resultado público. Las etiquetas T1/T2/T3 son canónicas
desde la Fase 18 y aparecen en todos los archivos de datos publicados, artículos
TOPIC y manuscritos JOURNAL.

Los descriptores — Regional, Distrito, Local — pretenden transmitir la geografía
de captación, no una jerarquía administrativa. Un clúster T1 Regional suele
servir a un área de captación regional que se extiende entre 20 y 50 km; un
clúster T2 Distrito a una captación más localizada; y un clúster T3 Local a un
área de barrio o ciudad pequeña. Estas son generalizaciones empíricas, no
restricciones de definición.

---

*Procedencia de los datos:* clusters-meta.json Fase 23+Cambio B (reconstrucción
2026-05-29T05:00Z); definiciones de taxonomía en `app-orchestration-gis/taxonomy.py`
y `config.py`.
