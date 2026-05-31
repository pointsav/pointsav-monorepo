---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "GIS como Sustrato BIM"
slug: topic-gis-as-bim-substrate
language: es
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-gis-as-bim-substrate.draft.md
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: bim-product-family.md (.agent/rules/); documentación del pipeline de compilación AEC (build-aec-*.sh); esquema clusters-meta.json; DOCTRINE.md §IV (fundamentos del dominio BIM)
research_inline: true
created: 2026-05-31
---

# GIS como Sustrato BIM

Los Sistemas de Información Geográfica (GIS) y el Modelado de Información de
Construcción (BIM) sirven a propósitos diferentes pero complementarios en la
planificación y operación de entornos construidos. GIS identifica dónde se
concentra la actividad y qué condiciones ambientales caracterizan una ubicación.
BIM modela cómo se diseña, construye y mantiene un edificio en esa ubicación.
Este artículo describe cómo los datos de colocalización GIS están previstos para
servir como capa de sustrato que informa los flujos de trabajo BIM — aportando
contexto de emplazamiento que la familia de productos BIM está planificada para
consumir.

## Dos Disciplinas Distintas

GIS y BIM son diferentes en escala, propósito y tipo de datos. GIS opera a la
escala de ciudades, regiones y continentes. Responde preguntas sobre la distribución
de la actividad, los patrones demográficos, los flujos económicos y las condiciones
ambientales a través de la geografía. La plataforma de colocalización es una
aplicación GIS: analiza la distribución espacial de establecimientos ancla
comerciales a escala continental para identificar clústeres de actividad
colocalizada que indican fortaleza de mercado.

BIM opera a la escala de un solo edificio o campus. Modela la geometría física
de una estructura, sus sistemas mecánicos y eléctricos, sus especificaciones de
materiales y su historial de mantenimiento. BIM se ocupa de lo que está dentro
de un límite de parcela específico.

Estas escalas rara vez se superponen en la práctica. Un análisis GIS identifica
un emplazamiento como comercialmente significativo; BIM modela lo que se construye
en ese emplazamiento. La relación de sustrato es unidireccional: los datos GIS
fluyen hacia BIM como contexto, no al revés.

## La Conexión con las Capas de Datos AEC

Las capas de datos de Arquitectura, Ingeniería y Construcción (AEC) de la
plataforma forman el puente entre el análisis GIS y el contexto de emplazamiento
BIM. Cada clúster de colocalización lleva un conjunto de atributos AEC derivados
del pipeline de compilación AEC nocturno:

*Zona climática (ASHRAE 169 / NECB / UE).* La norma de diseño de calefacción y
refrigeración aplicable para la ubicación del clúster. Una ubicación en la Zona
Climática ASHRAE 3A (Cálido-Húmedo) requiere especificaciones HVAC
fundamentalmente diferentes a una en la Zona 6A (Frío-Húmedo).

*Clasificación climática de Köppen.* El tipo climático a largo plazo en la
ubicación del clúster, extraído del conjunto de datos global de resolución de
1 km de Beck et al. La clasificación Köppen informa las estrategias de diseño
pasivo: orientación, proporciones de acristalamiento, potencial de ventilación
natural y requisitos de protección solar.

*Peligrosidad sísmica.* Valores de aceleración pico del suelo derivados del
Modelo Nacional de Peligrosidad Sísmica del USGS (Norteamérica), los mapas
sísmicos de Recursos Naturales de Canadá y el Modelo Europeo de Peligrosidad
Sísmica (ESHM20).

*Riesgo de inundación.* Categorización del peligro de inundación derivada de los
datos de inundación fluvial WRI Aqueduct, la Capa Nacional de Peligro de
Inundación de FEMA y los conjuntos de datos de zonas de inundación INSPIRE de la
UE.

Estas cuatro capas AEC se calculan en el centroide de cada clúster y se adjuntan
al registro de metadatos del clúster.

## Flujo de Datos Previsto

La relación prevista entre los datos de clústeres GIS y la familia de productos
BIM es la siguiente. Un analista de mercado utiliza la plataforma de colocalización
para identificar un clúster T1 Regional en una geografía objetivo. El registro del
clúster proporciona las coordenadas del centroide, la composición de colocalización,
el nivel (T1/T2/T3) y los atributos AEC. Este registro constituye un *resumen
ambiental del emplazamiento* — un compendio estructurado de las condiciones de
mercado y ambientales en esa ubicación.

La familia de productos BIM está planificada para consumir el resumen ambiental
del emplazamiento como punto de partida para el análisis del sitio y el diseño.
El demonio de archivo `service-bim` está previsto para leer los registros de
metadatos de clústeres como una de sus fuentes de datos de entrada. El editor
`app-workplace-bim` está planificado para mostrar los atributos AEC en un panel
de condiciones del emplazamiento.

Esta integración está planificada y prevista. El sustrato de datos — metadatos de
clústeres con atributos AEC — está construido y en funcionamiento. Los componentes
de la familia de productos BIM que lo consumen están en desarrollo activo.

## El Contrato de Datos

Los campos AEC en los metadatos del clúster — `ashrae_zone`, `koppen_class`,
`seismic_hazard`, `flood_risk` — son la interfaz definida entre GIS y BIM.
Representan una traducción de datos ráster sin procesar en valores categóricos
discretos que un sistema BIM puede interpretar sin necesidad de realizar consultas
espaciales de ráster.

## Nota sobre el Alcance

Este artículo describe una integración de sustrato planificada. La plataforma de
colocalización produce los metadatos del clúster con atributos AEC; la familia
de productos BIM está prevista para consumirlos. La integración de extremo a
extremo entre un registro de clúster de colocalización en producción y un modelo
BIM activo no está aún implementada.

---

*Relacionado:* `bim-product-family.md` — mapa de la familia de productos,
contrato de datos y decisiones sobre el formato IFC para la familia de
productos BIM.
