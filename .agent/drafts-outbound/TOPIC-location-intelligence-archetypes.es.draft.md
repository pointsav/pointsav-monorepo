---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Arquetipos de Co-localización en Inteligencia de Ubicación"
slug: topic-location-intelligence-archetypes
language: es
status: draft
paired_with: TOPIC-location-intelligence-archetypes.draft.md
target_repo: content-wiki-projects
target_path: topics/topic-location-intelligence-archetypes.es.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 5
research_suggested_count: 0
open_questions_count: 0
research_provenance: "TOPIC-location-intelligence-archetypes.draft.md (versión EN; proyecto-gis 2026-06-11); BRIEF-location-intelligence-archetypes-2026-06-01.md"
research_inline: true
created: 2026-06-11
author: totebox@project-gis (claude-sonnet-4-6)
---

# Arquetipos de Co-localización en Inteligencia de Ubicación

La plataforma de Inteligencia de Ubicación identifica la gravedad comercial y
minorista mediante tres arquetipos de co-localización: Centros Comerciales
(PRO), Franja Urbana (VWH) y Área de Acceso (PKS). Cada arquetipo describe un
patrón de agrupación distinto que refleja un tipo diferente de actividad
comercial y una relación diferente con la geografía urbana circundante.

Los códigos de tres letras fueron ratificados el 1 de junio de 2026.

## Los tres arquetipos

| Código | Nombre | Tipo de ancla | Estado |
|--------|--------|---------------|--------|
| **PRO** | Centros Comerciales | Hipermercado de alimentación con ferretería y al menos uno de: club de precio, estilo de vida o electrónica | Activo — canal de co-localización T1/T2/T3 |
| **VWH** | Franja Urbana | Ferretería + ecosistema de suministro industrial (MRO, alquiler de herramientas, distribuidores de construcción, recambios de auto) | Activo — 6,368 clústeres (T1=852 / T2=1,327 / T3=4,189) |
| **PKS** | Área de Acceso | Ancla de tránsito regional (aeropuerto, tren, bus) + aparcamiento disuasorio + alquiler de vehículos/hotel | Activo — 6,953 clústeres (T1=691 / T2=2,658 / T3=3,604) |

PRO es el producto de mapa base — la base del conjunto de datos de selección
de ubicaciones. VWH y PKS son arquetipos superpuestos que identifican
estructuras de mercado adyacentes no capturadas por la agrupación anclada en
alimentación.

---

## PRO — Centros Comerciales

Los clústeres PRO representan co-localizaciones comerciales ancladas en
alimentación a tres escalas. El canal agrupa ubicaciones de categorías ancla
que se encuentran dentro de una distancia de extensión definida y asigna cada
grupo a uno de tres niveles según la composición de anclas.

### Definiciones de nivel

**T1 — Regional:** Un clúster que contiene un hipermercado de alimentación y
un establecimiento de ferretería, más al menos uno de los siguientes: club de
precio, establecimiento de estilo de vida o retailer de electrónica.
Alternativamente: cuatro o más minoristas de categoría ancla en un clúster
compacto (extensión ≤ 1 km), o tres o más anclas en cualquier clúster
compacto.

**T2 — Distrito:** Un clúster que contiene un hipermercado de alimentación y
un establecimiento de ferretería, con una extensión máxima de 2,5 km.

**T3 — Local:** Todos los pares de anclas restantes que no cumplen los
criterios de T1 ni T2.

### Conjunto de datos actual (Fase 23 + Cambio B, 28 de mayo de 2026)

| Nivel | Clústeres | Países |
|-------|-----------|--------|
| T1 | 1,746 | 17 |
| T2 | 2,726 | 17 |
| T3 | 2,021 | 17 |
| **Total** | **6,493** | |

El conjunto de datos abarca 17 países en Norteamérica y Europa.
El parámetro SPAN_T2_MAX_KM se estableció en 2,5 km en la reconstrucción del
Cambio B, estrechando el límite T2 con respecto a fases anteriores.

---

## VWH — Franja Urbana

Los clústeres VWH identifican concentraciones de minoristas de ferretería y
suministros industriales en ausencia de anclas de alimentación. Estos
emplazamientos ocupan la franja urbana — ubicaciones entre 5 y 80 km del
centro de una gran área metropolitana — y tienden a agruparse cerca de
intercambiadores de autopista en zonas con uso del suelo industrial adyacente.

### Definición

Un candidato VWH es una ubicación donde hay uno o más establecimientos de
ferretería, no existe ningún hipermercado de alimentación dentro del radio
del clúster, y el emplazamiento se encuentra a una distancia metropolitana de
5–80 km. La forma construida típica es un edificio de almacén o fabricación
ligera de 3 a 6 plantas, distinto del formato de caja grande de una planta
del parque comercial.

Las ubicaciones VWH prestan servicio a contratistas del sector de la
construcción, operadores de fabricación ligera y arrendatarios de logística
de aprovisionamiento inmediato — no a consumidores minoristas generales.

### Señales de co-localización

**Esenciales:**

| Señal | Justificación |
|-------|--------------|
| Intercambiador de autopista ≤ 2 km | Acceso de camiones y salida de mercancías |
| Población ≥ 300,000 en radio de 30 minutos | Mano de obra para fabricación y logística |
| Uso del suelo industrial adyacente | Compatibilidad de zonificación |

**Significativas:**

| Señal | Justificación |
|-------|--------------|
| Aeropuerto de carga ≤ 20 km | Electrónica y componentes, reposición rápida |
| Ferrocarril de mercancías ≤ 2 km | Entrega de componentes justo a tiempo |
| Corredor de transporte público ≤ 500 m | Acceso de la mano de obra |

**Descalificadoras:** Zona residencial densa inmediatamente adyacente; llanura
de inundación; zona de conservación del patrimonio; ubicación dentro de un
clúster PRO.

### Resultados de producción (11 de junio de 2026)

El canal VWH es de calidad productiva. Se perfilaron 10,338 establecimientos de
ferretería (45 cadenas) como anclas proxy; el DBSCAN se ejecutó sobre POIs de
suministro industrial sin la ferretería (validación fuera de muestra); las reglas
de nivel utilizan lógica de colapso de grupos, validada con una co-localización
hardware del 73,4 % en clústeres T1+T2 (umbral de aceptación: 55 %).

| País | Clústeres |
|------|----------|
| Estados Unidos | 3,167 |
| Alemania | 648 |
| Reino Unido | 543 |
| Canadá | 506 |
| Francia | 420 |
| Países Bajos | 240 |
| Italia | 226 |
| Polonia | 171 |
| **Total (17 países)** | **6,368** |

Distribución por nivel: T1 (Hub comercial completo) = 852 (13,4 %), T2 (Establecido) =
1,327 (20,8 %), T3 (Emergente / Reducido) = 4,189 (65,8 %). La distribución con
predominio de T3 es esperada: los hubs de suministro completo que combinan MRO,
alquiler de herramientas, distribuidor de construcción y recambios son legítimamente
poco frecuentes.

El indicador `retail_contamination` señala los clústeres con un hipermercado de
alimentación a menos de 1 km del centroide (3,048 clústeres; 47,9 %). Son parques
comerciales de uso mixto — co-localizaciones VWH válidas que también incluyen
comercio de alimentación.

---

## PKS — Área de Acceso

Los clústeres PKS identifican concentraciones comerciales cerca de aeropuertos
regionales y estaciones de tren interurbano situados en una corona de acceso
a 15–150 km del centro de una gran área metropolitana. El patrón de demanda
definitorio es el desplazamiento de estacionamiento y vuelo, o estacionamiento
y tren: los residentes de un Mercado Regional conducen hasta un nodo de
transporte, estacionan y viajan al Mercado Metropolitano.

### Definición

Un candidato PKS es un nodo de transporte regional — aeropuerto o estación de
tren interurbano — a distancia metropolitana de 15–150 km. Los nodos a menos
de 15 km del centro metropolitano se clasifican como suburbanos en lugar de
regionales; los nodos a más de 150 km se consideran mercados independientes
con una relación metropolitana propia.

La señal comercial definitoria en una ubicación PKS es el alquiler de
vehículos. Los recambios de automoción, las gasolineras, los restaurantes de
servicio rápido y las tiendas de conveniencia son señales secundarias.

### Señales de co-localización

**Esenciales:**

| Señal | Justificación |
|-------|--------------|
| Ancla de transporte regional ≤ 3 km | Aeropuerto o estación con servicio directo al área metropolitana |
| Aislamiento metropolitano de 15–150 km | Define la relación regional |
| Clúster T1 o T2 ≤ 10 km | La misma población genera demanda de estacionamiento |
| Población regional ≥ 150,000 | Demanda mínima para estacionamiento de varios pisos |

**Significativas:**

| Señal | Justificación |
|-------|--------------|
| Alquiler de vehículos ≤ 1 km | Los viajeros que llegan requieren transporte |
| Concentración hotelera ≤ 500 m | Viajes de negocios y estacionamiento de varios días |
| Segundo modo de transporte ≤ 5 km | Integración multimodal |

**Descalificadoras:** Gran hub a menos de 15 km; población inferior a 100,000;
sin servicio directo al área metropolitana.

### Resultados de producción (11 de junio de 2026)

El canal PKS es de calidad productiva. Los registros de aparcamiento disuasorio
(23,117 ubicaciones) son el ancla geográfica principal — puntos de transición
coche→tránsito distribuidos de forma independiente de la geometría de la red
ferroviaria. Las reglas de nivel utilizan colapso de grupos de modos (ferrocarril
interurbano + ferrocarril de cercanías se unifican en el grupo FERROCARRIL, evitando
la inflación artificial de multimodalidad).

| Nivel | Clústeres | % | Definición |
|-------|----------|---|-----------|
| T1 (Hub regional) | 691 | 9,9 % | Multimodal + ecosistema comercial completo |
| T2 (Intercambiador de tránsito) | 2,658 | 38,2 % | Tránsito + al menos una señal comercial |
| T3 (Nodo de tránsito) | 3,604 | 51,9 % | Tránsito presente; oportunidad comercial |
| **Total** | **6,953** | | |

Enriquecimiento comercial: cadenas de alquiler de vehículos (hertz-eu, avis-eu,
europcar-eu, sixt-eu y otras) y cadenas hoteleras (ibis-eu, premier-inn-gb,
holiday-inn-express-us y otras) están incorporadas y activas en la compilación
de producción.

### Filtro de grandes hubs

Los aeropuertos con un clúster T1 PRO en un radio de 5 km se excluyen como
probables grandes hubs comerciales. Los grandes aeropuertos generan su propia
gravedad minorista y no exhiben el patrón de estacionamiento y tránsito. El filtro
excluye correctamente LAX, JFK, LHR y CDG.

### Mejoras futuras

- Datos de pasajeros aeroportuarios (CAPA o IATA) para sustituir el proxy de
  adyacencia T1 por un clasificador basado directamente en el tráfico
- Directorio de operadores de estacionamiento: Q-Park, APCOA, NCP, Indigo/Vinci (UE); SP+ (EE. UU.)

---

## Integración en el mapa

VWH y PKS aparecen como capas superpuestas bajo la sección **★ Mercados
Regionales** en el panel de control de capas.

**Selector VWH** — muestra puntos naranja en las ubicaciones candidatas de
Franja Urbana. Cuando está activo, las burbujas de clúster se atenúan al
10 % de opacidad para reducir la interferencia visual.

**Selector PKS** — muestra puntos turquesa en los candidatos integrados
(clúster T1/T2 a menos de 10 km) y puntos grises en los candidatos
independientes. Se aplica la misma atenuación del 10 % de las burbujas.

Ambas capas persisten en las transiciones de vista: el estado de atenuación
se mantiene al cambiar entre la Vista Minorista y el panel de detalle de
mercado BentoBox.

Variables de estado: `vwhActive`, `psActive`. Funciones: `toggleVwhLayer(btn)`,
`togglePsLayer(btn)`. El comportamiento de atenuación se gestiona mediante
`applyLiOverlayStyle()`, que verifica todos los estados de capa superpuesta
activos antes de aplicar los cambios de opacidad.
