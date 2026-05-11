---
schema: foundry-draft-v1
state: draft-refined
originating_cluster: project-language
target_repo: content-wiki-documentation
target_path: index.es.md
target_filename: index.es.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: TRANSLATE-ES
category: root
authored: 2026-04-29T01:30:00Z
authored_by: sonnet-4-6 sub-agent (refinement pass; parent task-project-language session 12376c0e4bc33ea7; operator: ps-administrator)
authored_with: claude-sonnet-4-6
references:
  - TOPIC-HOME.draft.md (companion English source, refinement pass)
  - content-contract.md §4 (category: root)
  - clones/project-knowledge/.claude/outbox.md (Q1+Q2 answers relayed via Master)
notes_for_editor: |
  Spanish strategic adaptation per DOCTRINE §XII — substantive overview,
  not a 1:1 translation. Q1+Q2 now closed; category: root added for
  consistency with English (engine reads same field for /es routing per
  planned iteration-2 feature). Rename from cluster-outbound
  TOPIC-HOME.es.draft.md to index.es.md happens at gateway-commit time.

  Refinement disciplines applied: Bloomberg-grade register tightening;
  analogous register discipline applied to Spanish (no marketing-fluffy
  equivalents of the English banned terms). No Provenance footer in
  Spanish — English carries the research trail per draft-research-trail-
  discipline.md §3 (TRANSLATE-ES: Optional).

  State moved to draft-refined.
research_done_count: 0
research_suggested_count: 0
open_questions_count: 0
research_provenance: none
research_inline: false
---

# documentation.pointsav.com

La documentación de la plataforma PointSav abarca la arquitectura, los
servicios, los sistemas operativos y las convenciones de gobernanza del
sustrato PointSav. Los artículos están dirigidos a ingenieros, escritores,
diseñadores y lectores con interés financiero en la plataforma. Todo el
contenido se publica bajo [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

## Áreas principales

La wiki está organizada en nueve áreas temáticas:

- **Arquitectura** — principios de diseño, patrones de sustrato e invariantes
  transversales que rigen la construcción de la plataforma
- **Servicios** — servicios autónomos de ingestión, procesamiento, búsqueda y
  egreso que implementan el modelo de tres anillos
- **Sistemas** — ToteboxOS y su modelo de capacidades basado en seL4,
  orquestación y aislamiento de inquilinos
- **Aplicaciones** — aplicaciones orientadas al usuario e internas construidas
  sobre el sustrato de la plataforma
- **Gobernanza** — registros de decisiones de arquitectura, postura de licencias
  y convenciones de cumplimiento
- **Infraestructura** — despliegue de flota y topología operacional (en preparación)
- **Empresa** — entidades corporativas, estructura organizativa y divulgaciones
  públicas
- **Referencia** — glosario, matriz de nomenclatura y guías de estilo para
  colaboradores
- **Ayuda** — guías de incorporación para ingenieros, escritores y diseñadores

## Artículo destacado

El motor del sitio lee el archivo `featured-topic.yaml` en la raíz del
repositorio para determinar qué artículo se destaca en la página de inicio.
Si el archivo no está presente, esta sección no se renderiza.

## Contribuir

El contenido se publica bajo [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).
Las contribuciones siguen el flujo de confirmación por niveles descrito en
[[style-guide-topic]]. La wiki incluye pares bilingües: cada artículo técnico
en inglés tiene un resumen estratégico en español adaptado para lectores
hispanohablantes, no una traducción literal.

El motor que sirve este sitio, [[app-mediakit-knowledge]], está construido
en Rust y consume este repositorio como su única fuente de contenido. Está
previsto que futuras instancias sirvan wikis de clientes bajo el patrón de
hostabilidad del cliente.
