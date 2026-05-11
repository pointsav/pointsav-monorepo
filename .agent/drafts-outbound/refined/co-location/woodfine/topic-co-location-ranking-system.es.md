---
schema: foundry-doc-v1
title: "Sistema de Clasificación de Co-ubicación Minorista"
slug: co-location-ranking-system
category: governance
type: topic
quality: complete
status: active
audience: customer-woodfine
bcsc_class: current-fact
language_protocol: TRANSLATE-ES
last_edited: 2026-05-02
editor: pointsav-engineering
paired_with: co-location-ranking-system.md
cites:
  - ni-51-102
  - osc-sn-51-721
---

# Sistema de Clasificación de Co-ubicación Minorista

El sistema de clasificación de co-ubicación de Woodfine evalúa los sitios minoristas mediante una **matriz de combinación de anclas nominadas**. Este algoritmo determinista califica cada ubicación de hipermercado basándose en la convergencia de categorías secundarias y terciarias (comerciales y cívicas) dentro de radios de captación definidos.

El sistema genera un índice de 12 rangos agrupados en cinco niveles de calidad (Tiers), visualizados en la plataforma GIS mediante una escala de colores que va desde el ámbar intenso (★★★★★ Tier 5, el más alto) hasta el azul pálido (★ Tier 1, el más bajo).

## El Modelo de Anclas Nominadas

Cada sitio en el índice está anclado por un único operador de hipermercado o mercancía general de gran formato (como Walmart o IKEA). El sistema clasifica a los operadores secundarios en cuatro categorías deterministas:

1.  **Hardware (Secundario-1):** Tiendas de mejoras para el hogar (Home Depot, Leroy Merlin).
2.  **Warehouse Club (Secundario-2):** Clubes de precios (Costco, Sam's Club, Makro).
3.  **Healthcare (Terciario-A):** Hospitales y centros médicos.
4.  **Higher Education (Terciario-B):** Universidades y colegios.

## Matriz de 12 Rangos

La combinación de categorías presentes determina el rango del sitio. El Tier 5 (★★★★★) representa la convergencia absoluta de las cuatro categorías con el ancla principal. Este nivel de validación indica que cuatro procesos independientes de selección de sitios han coincidido en el mismo nodo geográfico.

## Calibración y Escasez

Para mantener la relevancia del índice, el sistema aplica una **regla de calibración** automática: si los sitios de nivel superior exceden el 10% del total de anclas, el radio de proximidad se ajusta para mantener la escasez y la calidad del índice. Actualmente, los sitios Tier 5 representan el 3.7% del total, cumpliendo con los estándares de selectividad de la plataforma.

---
## Procedencia
- **Adaptación Estratégica:** Basada en el documento inglés `co-location-ranking-system.md`.
- **Refinamiento:** 2026-05-02 por project-language Task.
