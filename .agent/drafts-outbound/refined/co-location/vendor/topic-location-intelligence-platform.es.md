---
schema: foundry-doc-v1
title: "Plataforma de Inteligencia de Ubicación"
slug: location-intelligence-platform
category: applications
type: topic
quality: complete
status: active
audience: public
bcsc_class: public-disclosure-safe
language_protocol: TRANSLATE-ES
last_edited: 2026-05-02
editor: pointsav-engineering
paired_with: location-intelligence-platform.md
cites:
  - osm-odbl
  - overture-maps-cdla-2-0
  - ni-51-102
  - osc-sn-51-721
---

# Plataforma de Inteligencia de Ubicación

La plataforma de Inteligencia de Ubicación de PointSav es una aplicación GIS soberana, basada en archivos planos, diseñada para el análisis de clústeres minoristas y la selección estratégica de sitios. Construida como una alternativa de alto rendimiento a los productos SaaS comerciales, la plataforma permite a los clientes mantener el control total sobre sus conjuntos de datos geográficos, algoritmos e infraestructura de visualización.

## Capacidades Operativas

La plataforma transforma los datos brutos de ubicación de tiendas en nodos comerciales accionables mediante la ejecución de la [Metodología de Co-ubicación](co-location-methodology). Responde a una pregunta comercial fundamental: *¿qué nodos geográficos poseen la densidad validada por capital necesaria para sustentar un desarrollo inmobiliario adyacente?*

### 1. Identificación de Clústeres
La plataforma calcula clústeres de co-ubicación alrededor de anclas principales (ej. Walmart Supercentres, IKEA) utilizando un algoritmo espacial determinista. Cada clúster se califica según la convergencia de operadores independientes y la infraestructura cívica de apoyo.

### 2. Interfaz Interactiva Multicapa
El mapa interactivo en [gis.woodfinegroup.com](https://gis.woodfinegroup.com) utiliza una arquitectura de tres capas:
-   **Capa 1 — Puntos de Interés Globales:** Vista de más de 31,000 ubicaciones minoristas individuales.
-   **Capa 2 — Clústeres de Co-ubicación:** La vista analítica principal, que codifica la fuerza del clúster mediante saturación visual y tamaño.
-   **Capa 3 — Radios de Captación:** Visualización de los límites de proximidad (predeterminado 3.0 km) que definen el alcance del análisis de área de influencia.

## Arquitectura Soberana

La plataforma se adhiere a los principios de soberanía de datos del [Motor GIS de PointSav](pointsav-gis-engine):
-   **Operación basada en Archivos Planos:** Los datos persisten como archivos JSONL y GeoParquet versionados en un Totebox Archive.
-   **Visualización con Estándares Abiertos:** Utiliza PMTiles y MapLibre GL JS para servir mapas vectoriales sin dependencias de APIs de terceros.

---
## Procedencia
- **Adaptación Estratégica:** Basada en el documento inglés `location-intelligence-platform.md`.
- **Refinement:** 2026-05-02 por project-language Task.
