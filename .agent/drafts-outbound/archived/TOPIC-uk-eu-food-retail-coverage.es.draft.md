---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "Cobertura de Comercio Minorista de Alimentación en el Reino Unido y la UE"
slug: topic-uk-eu-food-retail-coverage
language: es
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-uk-eu-food-retail-coverage.draft.md
research_done_count: 6
research_suggested_count: 1
open_questions_count: 0
research_provenance: archivos JSONL en service-business/ (conteos de registros por cadena); config.py (ALPHA_HYPERMARKET EU, REGION_CONFIG); taxonomy.py Fases 17+18; entradas del registro de limpieza de sesión Fases 13/15/16/18; QIDs Wikidata por cadena
research_inline: true
created: 2026-05-31
---

# Cobertura de Comercio Minorista de Alimentación en el Reino Unido y la UE

La plataforma de colocalización utiliza datos de puntos de interés de
OpenStreetMap, enriquecidos con identificadores de entidades Wikidata, para
construir su inventario de ubicaciones comerciales. Este artículo documenta qué
cadenas de supermercados y comercio minorista de alimentación están cubiertas en
el Reino Unido y la Unión Europea, cuántas ubicaciones aporta cada cadena al
conjunto de datos y dónde la cobertura sigue siendo escasa.

## Resumen de Cobertura

A partir de la Fase 18 (22 de mayo de 2026), la plataforma cubre ubicaciones de
comercio minorista de alimentación en 18 países. El Reino Unido, Alemania y
Francia tienen la cobertura de cadenas más profunda, lo que refleja tanto la
densidad de los formatos comerciales de gran superficie en esos mercados como la
exhaustividad del mapeo OSM para tiendas de gran formato. Islandia (3 clústeres),
Noruega (10) y Finlandia (55) tienen la cobertura más delgada, consistente con
mercados de menor tamaño y mapeo OSM menos completo.

## Reino Unido

El Reino Unido fue objeto de una campaña de reingestión enfocada en la Fase 13
(17 de mayo de 2026) que expandió sustancialmente la cobertura.

**Tesco** (Wikidata Q487494) es la mayor cadena de supermercados del Reino Unido
por número de tiendas y aporta 3.872 registros a la plataforma. La Fase 13
expandió la cobertura de Tesco de 784 a 3.872 registros.

**Sainsbury's** (Q153417) aporta 1.903 registros tras la expansión de la Fase 13
desde 672 registros.

**ASDA** (Q297410) aporta 1.051 registros, ingestados en la Fase 12
(17 de mayo de 2026). ASDA opera principalmente en el rango de hipermercados y
grandes superficies relevante para la formación de clústeres T1 y T2.

**Morrisons** (Q922344) aporta 620 registros, ingestados en la Fase 12.

**Wickes** (Q7998350) aporta 236 registros. Wickes es un minorista de bricolaje
y mejora del hogar (Grupo Travis Perkins) clasificado como formato ancla de
ferretería. Su presencia en un clúster contribuye al requisito de ancla de
ferretería para la designación T1 y T2.

## Alemania

Alemania tiene el mayor recuento de clústeres de cualquier país europeo en la
plataforma, con 722 clústeres (227 T1, 338 T2, 157 T3) según la Fase 23+Cambio B.

El mercado alemán se caracteriza por una concentración de operadores de alimentación
y bricolaje de gran formato. Cadenas establecidas — Aldi, Lidl, Rewe y Edeka —
se ingestaron en fases de compilación anteriores y proporcionan una amplia cobertura.
La Fase 18 (22 de mayo de 2026) añadió dos cadenas significativas.

**Kaufland** (Q685967, Grupo Schwarz) aporta 253 registros en Alemania (registros
adicionales en Polonia). Kaufland opera hipermercados de línea completa.

**Globus** (Q528681, Globus Holding) aporta 125 registros. Globus opera
hipermercados de gran formato en Alemania, clasificado como ALPHA_HYPERMARKET EU.

## Francia

Francia tiene 624 clústeres (247 T1, 161 T2, 216 T3). La cobertura de hipermercados
franceses está anclada por Carrefour. La Fase 18 añadió:

**Intermarché Hyper** (Q2029154, Les Mousquetaires) — 56 registros de tiendas en
formato hipermercado.

**Géant Casino** (Q2901839, Casino Group) — 10 registros. Géant Casino es la
división de hipermercados del Grupo Casino.

**Bricomarché** (Q2896882, Les Mousquetaires) — 497 registros. Formato de
ferretería y bricolaje que contribuye a la capa de anclas de ferretería.

**Brico Dépôt** (Q3007003, Kingfisher) — 137 registros. La cadena francesa de
bricolaje de Kingfisher.

## Otros Mercados Europeos

**Austria:** Interspar (Q1364056, SPAR Austria) aporta 85 registros del formato
hipermercado Interspar. Billa+ (Q806085) aporta registros del formato hipermercado
ampliado de Billa.

**Países Bajos:** Albert Heijn XL (sin entidad Wikidata; marca de Ahold Delhaize)
aporta 43 registros. Jumbo Foodmarkt (Q14716185) aporta 8 registros de las tiendas
insignia de gran formato.

**Polonia:** Kaufland (Q685967) aporta 253 registros. E.Leclerc (Q1273376) aporta
36 registros.

**Italia:** Esselunga (Q1377048) aporta 259 registros. Esselunga opera
supermercados e hipermercados de gran formato en el norte de Italia.

**Grecia:** Sklavenitis (Q7536996) aporta 406 registros. Los datos de ubicación
se obtuvieron mediante consulta de nombre en griego (Σκλαβενίτης) en OSM.

**Portugal:** Continente (Q5164541, Sonae) aporta 57 registros.

**Mercados nórdicos (SE/DK/NO/FI):** IKEA (entidades por país) y Bauhaus (Q532716)
proporcionan la cobertura principal de anclas de gran formato. Føtex (Q3093871,
Salling Group) aporta 103 registros DK. Bauhaus está presente en SE, DK
(20 registros), NO (2 registros) y FI (6 registros).

## Brechas de Cobertura

Varias geografías dentro del ámbito de 18 países de la plataforma tienen brechas
de cobertura conocidas.

**Islandia** solo tiene 3 clústeres, lo que refleja un mapeo OSM limitado de
ubicaciones comerciales y un mercado pequeño.

**La Iberia atlántica** (España occidental y Portugal más allá de Lisboa) y el
**sur de Italia** tienen una cobertura T1 más delgada de la que sugeriría su
tamaño de población.

**Escocia** tiene una cobertura más dispersa que Inglaterra y Gales, principalmente
porque la densidad de ASDA y Morrisons es menor en los mercados escoceses.

Los formatos de descuento Aldi y Lidl, presentes en toda Europa, están registrados
en OSM pero no se clasifican como anclas ALPHA_HYPERMARKET porque sus tamaños de
tienda están por debajo del umbral de hipermercado de pleno formato utilizado para
la designación de anclas T1/T2. Contribuyen a la formación de clústeres T3 como
miembros de comercio minorista general.

## Procedencia de los Datos y Atribución

Todos los registros de ubicación proceden de OpenStreetMap, que pone sus datos a
disposición bajo la Licencia de Base de Datos Abierta (ODbL). Los identificadores
de entidades Wikidata utilizados para relacionar los registros OSM con identidades
de cadenas canónicas son CC0.

---

*Procedencia de los datos:* archivos JSONL de cadenas en `service-business/`
(registros por cadena); commits de ingestión Fases 12–18 (17 de mayo de 2026 al
22 de mayo de 2026). OSM ODbL; Wikidata CC0.
