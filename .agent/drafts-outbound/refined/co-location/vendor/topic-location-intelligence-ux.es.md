---
schema: foundry-doc-v1
title: "Filosofía de Diseño UX de Inteligencia de Ubicación"
slug: location-intelligence-ux
category: applications
type: topic
quality: complete
status: active
audience: public
bcsc_class: internal
language_protocol: TRANSLATE-ES
last_edited: 2026-05-02
editor: pointsav-engineering
paired_with: location-intelligence-ux.md
---

# Filosofía de Diseño UX de Inteligencia de Ubicación

La interfaz de Inteligencia de Ubicación de PointSav está diseñada para priorizar la claridad en la toma de decisiones por encima del volumen de datos brutos. Utilizando una filosofía de "conclusión primero", la plataforma comunica la confianza en la selección de sitios mediante jerarquía visual y una clasificación estructural.

## Diferenciación: El Grado de Clúster como Unidad Primaria

A diferencia de los productos GIS comerciales que muestran puntos individuales por defecto, la plataforma de PointSav utiliza el **Grado de Clúster** como la unidad visual y analítica principal:

1.  **Rampa de Confianza:** Los sitios se codifican mediante una rampa de color (desde el ámbar pálido hasta el intenso). Los marcadores más oscuros y grandes indican mayores niveles de convergencia de capital.
2.  **Jerarquía Estructural:** La interfaz guía al usuario hacia los nodos comerciales más defendibles, haciendo que los sitios Tier 5 y Tier 4 dominen la vista nacional.
3.  **Tarjetas de Índice Contextuales:** Al hacer clic en un clúster, se activa un panel lateral que proporciona metadatos inmediatos (ranking municipal, operadores presentes) sin perder el contexto del mapa.

---
## Procedencia
- **Adaptación Estratégica:** Basada en el documento inglés `location-intelligence-ux.md`.
- **Refinement:** 2026-05-02 por project-language Task.
