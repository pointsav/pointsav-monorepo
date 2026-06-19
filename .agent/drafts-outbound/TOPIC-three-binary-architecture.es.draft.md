---
artifact: topic
schema: foundry-draft-v1
title: "La Arquitectura de Tres Binarios: os-console, os-totebox, os-orchestration"
lang: es
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, claim-43, claim-49, claim-23, claim-52]
research_trail:
  sources: [BRIEF-OS-FAMILY.md, BRIEF-sovereign-os-family-master-plan.md, BRIEF-os-console-hypervisor.md, conventions/architecture-layer-catalog.md]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: sesión de investigación de sustrato radical can-we-make-a-bubbly-quasar
  verification_method: investigación por agentes + revisión de DOCTRINA + verificación cruzada de registro de proyectos
---

# La Arquitectura de Tres Binarios: os-console, os-totebox, os-orchestration

Totebox Orchestration se entrega mediante tres entornos operativos binarios distintos.
Cada uno tiene un rol distinto, un objetivo de despliegue distinto y un conjunto distinto
de aplicaciones alojadas. Juntos forman un sistema completo para la gestión soberana de datos.

---

## Visión General

```
                    ┌──────────────────────────────────────┐
                    │           MÁQUINA HOST               │
                    │                                      │
                    │         os-console                   │
                    │   Superficie de Terminal del Operador│
                    │   cartridges: app-console-* (F2–F12)│
                    │   TUI orientada al teclado            │
                    │   autorización de máquina via F11    │
                    └──────────────┬───────────────────────┘
                                   │  token de capacidad de máquina
                                   │  (autorizado en el emparejamiento F11)
                                   ▼
                    ┌──────────────────────────────────────┐
                    │       HARDWARE / VM DEL CLIENTE      │
                    │                                      │
                    │         os-totebox                   │
                    │   Almacén de Datos WORM Soberano     │
                    │   servicios: service-* (Anillos 1+2) │
                    │   sin shell; sin proceso root        │
                    │   registro de solo adición WORM      │
                    │   IA local Nivel A (service-slm)     │
                    └──────────────┬───────────────────────┘
                                   │  federación restringida por capacidades
                                   │  (el Totebox otorga capacidades derivadas)
                                   ▼
                    ┌──────────────────────────────────────┐
                    │     INFRAESTRUCTURA DEL PROVEEDOR    │
                    │                                      │
                    │      os-orchestration                │
                    │   Capa de Agregación Sin Estado      │
                    │   apps: app-orchestration-*          │
                    │   flota de GPU Yo-Yo (Nivel B)       │
                    │   flujos comerciales multi-Totebox   │
                    │   no posee claves de archivo         │
                    └──────────────────────────────────────┘
```

---

## os-totebox: Almacén de Datos WORM Soberano

os-totebox es el despliegue del lado del cliente. Se ejecuta en hardware bajo el control
físico del cliente — una máquina tipo NUC, una VM de GCP o un servidor privado. Su función
es alojar los servicios de los Anillos 1 y 2:

**Anillo 1 — Ingesta en el Límite:**
- `service-fs` — sistema de archivos de solo adición WORM; cada escritura es una entrada
  de registro
- `service-input` — ingesta de entrada estructurada de operadores os-console
- `service-extraction` — pipeline de extracción de entidades de emails y documentos
- `service-egress` — flujo de datos de salida controlado

**Anillo 2 — Procesamiento Determinista:**
- `service-content` — DataGraph; grafo de conocimiento sobre todas las entidades ingeridas
- `service-people` — registro de personal e identidad
- `service-email` — puente de archivo de email (Microsoft Exchange → Maildir)
- `service-slm` — pasarela de inferencia Doorman (IA local Nivel A en OLMo 7B)

El Anillo 3 (IA opcional, inferencia Nivel B+) está alojado en os-orchestration, no en
os-totebox. Un Totebox se entrega con los Anillos 1 y 2. El Anillo 3 es un nivel de pago
opcional.

**Forma final prevista (Fase H2, planificada):** os-totebox arranca en metal desnudo o
como VM en una imagen seL4 Microkit. Cada servicio se ejecuta como un Dominio de
Protección seL4. Sin shell de Linux. Sin gestor de paquetes. Sin sistema de inicio.

---

## os-console: Superficie de Terminal del Operador

os-console se ejecuta en la máquina personal del operador. Es la interfaz a través de la
cual el operador interactúa con los servicios de Totebox. No almacena datos — renderiza
vistas de datos que residen en os-totebox.

| Tecla | Cartridge | Servicio Backend |
|---|---|---|
| F2 | Personas | service-people :9091 |
| F3 | Email | service-email |
| F4 | Contenido | service-content :9081 |
| F6 | Contabilidad | service-bookkeeper |
| F9 | SLM | service-slm :9080 |
| F11 | Sistema / Emparejamiento | pairing-server |
| F12 | Entrada / Auditoría | service-input |

---

## os-orchestration: Capa de Agregación Sin Estado

os-orchestration (nombre heredado: os-interface, cambio de nombre en progreso) es el
nivel de pago del lado del proveedor. Agrega datos de múltiples Toteboxes de clientes y
proporciona servicios comerciales que requieren acceso a datos entre Toteboxes.

os-orchestration es **sin estado**: no posee claves de archivo, no escribe datos, no
gestiona ningún registro. Computa sobre vistas de datos de Totebox restringidas por
capacidades.

| App | Función | Puerto |
|---|---|---|
| app-orchestration-slm | Intermediario de GPU Yo-Yo (Nivel B) | :9180 |
| app-orchestration-market | IU del mercado de datos (previsto, Doctrina #52) | — |
| app-orchestration-exchange | Pasarela de intercambio de anuncios (previsto, Doctrina #52) | — |
| app-orchestration-gis | Procesamiento GIS a escala continental | desplegado |
| app-orchestration-bim | Federación de BIM multi-archivo | — |

**Modelo comercial (Doctrina #23):** Un único Totebox ejecuta los Anillos 1–2 sin costo
de licencia. Conectarse a os-orchestration para inferencia de Nivel B, mercado de datos
o agregación de BIM es el nivel de pago.

---

## Protección Geométrica en los Tres Niveles

El modelo de Protección Geométrica se aplica en cada capa con el mismo mecanismo pero
un alcance diferente:

| Capa | Límite de PD | Qué acota |
|---|---|---|
| os-console | Por cartridge (F2, F3, F4...) | Un cartridge no puede leer los datos de otro |
| os-totebox | Por servicio (service-people, service-slm...) | Un servicio no puede escalar a otro |
| os-orchestration | Por app + punto de control de capacidades | Los datos de una org no pueden llegar a otra |

El kernel seL4 proporciona la misma prueba formal en cada capa. El sustrato de capacidades
(system-core, system-ledger) proporciona la interfaz en Rust para estas pruebas.
moonshot-sel4-vmm proporciona el runtime de PD que hace que el código Rust se ejecute
dentro de cada Dominio de Protección.

La arquitectura de tres binarios es un único sistema — una prueba seL4, un sustrato de
capacidades, un pipeline de compilación moonshot-toolkit — desplegado en tres niveles
distintos.

---

## Qué Significa Esto para el Operador de Pequeña Empresa

Una pequeña empresa sin departamento de TI ejecuta:
- Un `cluster-totebox-1` — en un NUC bajo el escritorio de la sala de servidores o una
  VM de GCP de 7 $/mes
- Uno o más `node-console-*` — uno por miembro del personal que necesita acceso a Totebox
- Opcional: conexión a `gateway-orchestration-slm-1` para capacidad de IA de Nivel B

El operador no gestiona servidores. No configura firewalls. Arranca el Totebox, arranca
os-console, escanea un código QR y el sistema está en funcionamiento. Si cancela la
suscripción opcional de Nivel B, su Totebox (y todos sus datos) continúa operando de
forma independiente. No hay dependencia obligatoria de la nube.
