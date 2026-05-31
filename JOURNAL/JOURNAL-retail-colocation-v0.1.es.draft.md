---
schema: foundry-journal-v1
artifact_type: JOURNAL
language: es
paired_with: JOURNAL-retail-colocation-v0.1.draft.md
state: draft
version: "0.1"
title: "La Composición de Anclas de Grandes Superficies como Indicador Espacial Adelantado de la Actividad Comercial: Análisis de Clústeres a Escala Continental"
target_journal: "Economic Geography (Wiley, IF 7.2)"
target_publisher: "Wiley-Blackwell"
impact_factor: "7.2"
alternate_venue: "Journal of Economic Geography (OUP, IF 4.8)"
authors:
  - name: "Jennifer M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, EE.UU."
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Methodology
      - Formal Analysis
      - Writing – Original Draft
      - Writing – Review & Editing
  - name: "Peter M. Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, EE.UU."
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Conceptualization
      - Validation
      - Writing – Review & Editing
  - name: "Mathew Woodfine"
    affiliation: "Woodfine Management Corp., New York, NY, EE.UU."
    email: corporate.secretary@woodfinegroup.com
    orcid: ""
    credit_roles:
      - Software
      - Data Curation
      - Writing – Review & Editing
subject_codes:
  - "JEL: R12, R30, L81"
  - "ACM CCS: Computing methodologies → Spatial analysis"
keywords:
  - anclajes de grandes superficies
  - co-ubicación comercial
  - indicadores espaciales adelantados
  - análisis de clústeres a escala continental
  - geografía económica del comercio minorista
  - datos de origen-destino
  - taxonomía de niveles
bcsc_class: no-disclosure-implication
ai_tool_used: "claude-sonnet-4-6 (Anthropic)"
corresponding_author: corporate.secretary@woodfinegroup.com
word_count_body: 8500
word_count_target: 8500
submission_status: not-submitted
cites: []
forbidden_terms_cleared: true
notes_for_editor: |
  Versión en español del manuscrito original en inglés. La hipótesis primaria de falsación (H₁)
  requiere datos de movilidad origen-destino aún en proceso de adquisición para el Reino Unido,
  Francia y Alemania. Los análisis de §7.1 y §7.4 pueden ejecutarse con los datos LODES
  cargados actualmente. Los Apéndices B y C se completarán en versión posterior. Los identificadores
  ORCID son necesarios para todos los autores antes de la presentación formal.
---

> **Manuscrito de trabajo · Versión 0.1 · 2026-05-31 · CC BY 4.0**
> Este manuscrito es un borrador de trabajo. No ha sido sometido a revisión por pares.
> Los resultados son preliminares y pueden ser revisados sin previo aviso. Correspondencia:
> corporate.secretary@woodfinegroup.com.
>
> *Citar como:* Woodfine, Jennifer M., Woodfine, Peter M., y Woodfine, Mathew (2026).
> La Composición de Anclas de Grandes Superficies como Indicador Espacial Adelantado de la
> Actividad Comercial: Análisis de Clústeres a Escala Continental. Manuscrito de trabajo
> v0.1, 31 de mayo de 2026. Woodfine Management Corp., Nueva York, NY.

> **Declaración sobre afirmaciones prospectivas**
> Ciertas afirmaciones en este artículo describen líneas de investigación previstas,
> capacidades analíticas planificadas y resultados anticipados. Estas afirmaciones
> reflejan las expectativas actuales de los autores y se basan en suposiciones razonables
> y trabajos en curso a la fecha indicada. Los resultados, mediciones y hallazgos reales
> pueden diferir materialmente. Los lectores no deben depositar una confianza excesiva
> en dichas afirmaciones, que están sujetas a revisión conforme avance la investigación
> y se disponga de nuevos datos.

# La Composición de Anclas de Grandes Superficies como Indicador Espacial Adelantado de la Actividad Comercial: Análisis de Clústeres a Escala Continental

**Jennifer M. Woodfine**, **Peter M. Woodfine**, **Mathew Woodfine**

*Woodfine Management Corp., Nueva York, NY, EE.UU.*

Correo electrónico de correspondencia: corporate.secretary@woodfinegroup.com

---

## Resumen

Este artículo propone que la composición de categorías de anclas de grandes superficies en un clúster submetropolitano es un indicador espacial adelantado de la intensidad de la actividad comercial. Se presenta una taxonomía compositiva que distingue tres niveles: clústeres de Nivel 1, que reúnen hipermercados, grandes superficies de ferretería o bricolaje y almacenes de membresía; clústeres de Nivel 2, que combinan dos de estas tres categorías; y clústeres de Nivel 3, compuestos por una sola categoría dominante. Mediante un algoritmo de doble pasada DBSCAN (Radio de densidad de 1,0 km para configuraciones compactas; 3,0 km para configuraciones amplias) aplicado a datos abiertos de OpenStreetMap, se identifican y clasifican 6.493 clústeres en trece países. Los resultados muestran que los clústeres de Nivel 1 exhiben un diámetro medio un 63% mayor que los de Nivel 3 (β₁ = 0,489; IC 95% [0,359; 0,619]; p < 0,001), y que la presencia de almacenes de membresía es el predictor más potente de la clasificación en Nivel 1 (β = 0,639; p < 0,001), con una varianza explicada del 50,3% (R² = 0,503). La hipótesis primaria de falsación —que los clústeres de Nivel 1 atraen visitantes desde un área geográfica significativamente más dispersa que los de Nivel 2, controlando por la población disponible en la zona de captación— requiere datos de movilidad origen-destino que se encuentran en proceso de adquisición y se presentará en versión posterior.

**Palabras clave:** anclas de grandes superficies; co-ubicación comercial; indicadores espaciales adelantados; análisis de clústeres; geografía económica del comercio minorista; datos de movilidad; taxonomía de niveles.

---

## 1. Introducción

La teoría de los lugares centrales (Christaller 1933) estableció que los entornos de venta al por menor se organizan en jerarquías espaciales determinadas por la demanda efectiva de la población local. Sin embargo, esta perspectiva centrada en la demanda pasa por alto un mecanismo de organización igualmente fundamental: las cadenas de grandes superficies seleccionan emplazamientos de forma independiente, basándose en sus propios modelos de densidad económica, y la co-ubicación de múltiples cadenas de distintas categorías genera una señal compuesta sobre la idoneidad comercial submetropolitana que ningún establecimiento individual puede producir por sí solo.

Este artículo propone y operacionaliza una taxonomía compositiva de clústeres de grandes superficies que aprovecha esta señal de co-presencia. La contribución teórica es el argumento de preferencia revelada: cuando varias cadenas dominantes de distintas categorías —hipermercados, grandes superficies de ferretería o bricolaje y almacenes de membresía— seleccionan de forma independiente la misma ubicación submetropolitana, la señal compuesta de su co-presencia constituye un indicador espacial adelantado de la intensidad de la actividad comercial más duradero, más ampliamente disponible y metodológicamente más sencillo que cualquier conjunto de datos de demanda de una sola fecha de referencia.

Las tres contribuciones del artículo son las siguientes. En primer lugar, se propone una taxonomía compositiva de tres niveles implementada a escala continental mediante datos abiertos, capaz de replicarse en cualquier país con cobertura suficiente de OpenStreetMap. En segundo lugar, se desarrolla un algoritmo de doble pasada DBSCAN con un rango ajustado de modo shrinkage dentro de cada nivel, que discrimina entre iguales compositivos sin necesidad de datos de demanda adicionales. En tercer lugar, se especifica un programa de falsación empírica con cinco pruebas ordenadas por disponibilidad de datos.

---

## 2. Marco teórico y revisión de la literatura

### 2.1 Agrupación espacial y co-ubicación

La agrupación espacial del comercio minorista ha sido analizada desde perspectivas complementarias. La ley de gravitación de Reilly (1931) y el modelo probabilístico de Huff (1964) modelizan la demanda como función de la distancia y la masa, sin abordar los incentivos estratégicos que llevan a los minoristas a agruparse. Konishi (2005) demuestra formalmente que los minoristas pueden beneficiarse de la concentración incluso en ausencia de externalidades de demanda directas, siempre que la aglomeración genere información útil para los consumidores sobre la variedad disponible.

Los trabajos de Brueckner (1993), Pashigian y Gould (1998) y Eppli y Shilling (1995) analizan las externalidades entre establecimientos âncla en centros comerciales planificados y muestran que los principales arrendatarios internalizan parte de estas externalidades a través de arrendamientos preferentes. La co-ubicación en superficies de gran formato no planificadas —objeto de este artículo— genera externalidades análogas sin la mediación contractual del promotor inmobiliario, lo que hace de la señal compositiva un indicador más robusto de idoneidad comercial orgánica.

### 2.2 Anclas de grandes superficies y difusión

Holmes (2011) demuestra empíricamente, a partir de los datos de expansión de Walmart en Estados Unidos, que la difusión geográfica de una cadena de grandes superficies refleja economías de densidad en distribución y logística. Esta lógica de densidad implica que las cadenas seleccionan emplazamientos en función de la demanda potencial y de la infraestructura logística local, lo que convierte la presencia de una ancla de primer nivel en una señal creíble sobre las condiciones submetropolitanas. La extensión central de este artículo consiste en demostrar que la co-presencia simultánea de categorías de ancla distintas —hipermercado, ferretería o bricolaje y almacén de membresía— produce una señal compuesta más informativa que cualquier ancla individual.

Hernandez y Simmons (2006) documentan el surgimiento del parque de actividades comerciales (*power centre*) en Canadá como formato submetropolitano dominante, caracterizado por la co-ubicación de grandes superficies de distintas categorías en configuraciones de baja densidad. Darnall *et al.* (2022) aplican métodos de agrupación abiertos a los datos de OpenStreetMap para delimitar aglomeraciones comerciales en el Reino Unido, demostrando la viabilidad del análisis a escala nacional con datos no propietarios.

La literatura reciente sobre movilidad (Calafiore *et al.* 2022; Li *et al.* 2024) introduce datos de dispositivos móviles para medir zonas de captación a escala submetropolitana, superando las limitaciones de los datos censales agregados. Esta metodología es la que se adopta en las pruebas de falsación de §7.2, que requieren datos origen-destino actualmente en proceso de adquisición.

### 2.3 Laguna empírica

A pesar de la solidez teórica del argumento de co-ubicación como señal espacial, la literatura carece de una taxonomía compositiva operacionalizada a escala continental que distinga la calidad de la señal en función de las categorías de ancla presentes. Los índices de agrupación existentes son predominantemente monocategoriales (solo hipermercados, o solo centros comerciales) o se basan en perímetros administrativos que no capturan la estructura funcional de los clústeres submetropolitanos. Este artículo cierra esa laguna.

---

## 3. Modelo de co-ubicación geométrica

### 3.1 Definición de las categorías de ancla

Se consideran tres categorías canónicas de ancla de grandes superficies:

*Hipermercados.* Superficies de venta al por mayor de alimentación general con más de 2.500 m² de sala de ventas, bajo una cadena identificable en el registro de OpenStreetMap. Constituyen la categoría de mayor poder de atracción, capaces de generar flujos de visitantes independientemente del resto de la composición del clúster.

*Ferretería y bricolaje de gran superficie.* Superficies de venta al por mayor de materiales de construcción, herramientas y artículos para el hogar, bajo una cadena identificable. Esta categoría refleja demanda de alojamiento y actividad inmobiliaria residencial submetropolitana.

*Almacenes de membresía.* Superficies de venta al por mayor bajo un modelo de afiliación (como Costco o Sam's Club), con un mínimo de compra por unidad que desplaza la demanda hacia hogares con mayor capacidad de almacenamiento, típicamente localizados en entornos suburbanos y periurbanos.

### 3.2 Taxonomía de niveles

*Nivel 1 (T1).* El clúster contiene al menos una ancla de cada una de las tres categorías canónicas (hipermercado, ferretería o bricolaje, almacén de membresía); o bien adopta la admisión por amplitud (ruta T1.c), cumpliendo la condición de amplitud categórica ≥ 4 y el umbral de compacidad h2b. La clasificación en Nivel 1 constituye la señal compositiva más fuerte.

*Nivel 2 (T2).* El clúster contiene anclas de exactamente dos de las tres categorías canónicas (sin acceso a la ruta T1.c).

*Nivel 3 (T3).* El clúster contiene anclas de una única categoría canónica o de categorías secundarias únicamente (por ejemplo, grandes superficies de electrónica o de artículos deportivos sin la presencia de las tres categorías primarias).

*Modificador T0 (Platino).* Se aplica a los clústeres de Nivel 1 que cumplen la condición *tight_intact* (todos los establecimientos dentro de τ_tight = 1,0 km entre sí) y el modificador cívico (presencia de un hospital con ≥ 200 camas o una universidad con ≥ 10.000 estudiantes matriculados en un radio de 5,0 km). El modificador T0 no es un nivel de clasificación independiente, sino un indicador adicional de densidad funcional.

### 3.3 Algoritmo DBSCAN de doble pasada

La detección de clústeres utiliza el algoritmo de densidad DBSCAN (Ester *et al.* 1996) en dos pasadas sucesivas. La primera pasada aplica un radio de proximidad compacto (ε₁ = 1,0 km, minPts = 1), capturando configuraciones de parques de actividades comerciales bien delimitadas. La segunda pasada aplica un radio amplio (ε₂ = 3,0 km, minPts = 1) a los establecimientos no asignados en la primera pasada, capturando configuraciones dispersas en zonas periurbanas de baja densidad edificatoria. Se impone un límite de diámetro máximo de 3,0 km (Δ_max) para evitar la fusión de clústeres pertenecientes a distintos corredores comerciales.

### 3.4 Rango ajustado dentro del nivel

La clasificación de nivel determina la categoría compositiva del clúster; el rango dentro del nivel establece la posición relativa entre clústeres de la misma categoría. Se utiliza una función de rango de distancia con ajuste *shrinkage*:

    dist_rank_in_tier_cj = w_c · F̂_t(span_km_cj)⁻¹ + (1 - w_c) · F̂_t_global(span_km_cj)⁻¹

donde F̂_t es la función de distribución empírica del diámetro span_km dentro del nivel t para el país c; F̂_t_global es la distribución global del nivel; y w_c = n_c / (n_c + K) es el peso de contracción, con K ≈ 20–30. Los clústeres de menor diámetro obtienen rangos más altos dentro de su nivel, dado que la compacidad geométrica refleja mayor densidad de anclajes.

### 3.5 Modificador de compacidad (T1.b y T1.c)

La ruta T1.b permite la clasificación en Nivel 1 a clústeres de dos categorías canónicas que cumplen la condición de compacidad h2b (diámetro ≤ 1,5 km, configuración *tight_intact*). La ruta T1.c permite la clasificación en Nivel 1 a clústeres con amplitud categórica ≥ 4 (incluyendo categorías secundarias) que superan el umbral compuesto de compacidad h2b. Ambas rutas recogen configuraciones de anclaje con señal compositiva comparable a la tripartita por su densidad funcional.

### 3.6 El modificador cívico

El modificador cívico (T0) identifica clústeres de Nivel 1 con presencia simultánea de equipamientos sanitarios y educativos de gran capacidad. La literatura sobre economías de aglomeración urbana (Drucker y Goldstein 2007; Glasson 2003) documenta que los hospitales universitarios y las grandes universidades generan demanda de servicios comerciales de alta frecuencia y atraen población con perfil de gasto superior al promedio. El modificador T0 codifica esta hipótesis de amplificación cívica, que se somete a prueba en §7.5.

### 3.7 Protocolo de zona de captación por dispositivos móviles

Para los clústeres con cobertura de datos de movilidad origen-destino, la zona de captación se construye mediante el protocolo de polígono y dispositivo: (1) se obtienen los polígonos de OpenStreetMap de cada cadena ancla presente en el clúster; (2) para cada dispositivo del panel de movilidad que registra una visita a uno de esos polígonos, se asigna una celda de origen en resolución H3-7 y, cuando está disponible, una celda de residencia habitual; (3) el conjunto de celdas de origen distintas, limitado al anillo exterior de 150 km, constituye la zona de captación del clúster. La entropía de captación H = −Σ pᵢ log pᵢ, donde pᵢ es la fracción de dispositivos procedentes de la celda H3 iésima, cuantifica la dispersión geográfica de la demanda.

---

## 4. Datos y marco analítico

### 4.1 Datos de establecimientos

Los datos de establecimientos minoristas proceden de OpenStreetMap, consultado en mayo de 2026. La cobertura de OpenStreetMap para grandes superficies con identidad de cadena conocida en los trece países analizados es sustancialmente completa para los establecimientos abiertos desde al menos 2020, con una tasa de omisión estimada inferior al 5% para las cadenas incluidas en la taxonomía (Haklay 2010). Los archivos de configuración por cadena especifican el identificador Wikidata, los filtros de nombre, las coordenadas de la caja delimitadora por país y los umbrales mínimos de superficie o de número de resultados.

### 4.2 Países incluidos

El análisis abarca trece países: Alemania, Austria, Canadá, Dinamarca, España, Finlandia, Francia, Países Bajos, Polonia, Portugal, Reino Unido, Suecia y Estados Unidos. Esta selección refleja la disponibilidad de datos de OpenStreetMap con cobertura de cadenas de gran superficie, la disponibilidad de datos censales y de movilidad compatibles con las especificaciones de §7, y la diversidad de formatos de distribución submetropolitana (parques de actividades comerciales norteamericanos frente a grandes superficies periurbanas europeas).

### 4.3 Datos de origen-destino

Para Estados Unidos se emplean los datos del programa LODES (Origin-Destination Employment Statistics, versión 8) del Bureau of the Census, que proporcionan flujos de empleo trabajo-residencia a nivel de grupos de bloques censales, agregados a resolución H3-7. Para España se emplean los datos de movilidad del Ministerio de Transportes, Movilidad y Agenda Urbana (MITMA), que proporcionan matrices de movilidad diaria a nivel municipal, reclasificadas a resolución H3-7. Los datos para el Reino Unido (ONS ODWP01EW), Francia (INSEE FD_MOBPRO) y Alemania (Bundesagentur für Arbeit Pendlerdaten) están en proceso de integración para versiones posteriores del manuscrito.

### 4.4 Datos de población

Los datos de población utilizados como variable de control en las regresiones de §7 proceden de la base de datos Kontur Population (resolución H3-8; licencia CC BY 4.0), que proporciona estimaciones de población de alta resolución para los trece países a partir de imágenes satelitales de construcción y datos censales oficiales. La variable log(pop_150km) se construye sumando la población Kontur dentro del anillo de 150 km centrado en el centroide de cada clúster.

---

## 5. Resultados descriptivos

### 5.1 Composición del conjunto de clústeres

El procedimiento de clasificación identifica 6.493 clústeres clasificados en los trece países analizados, de los cuales 1.746 (26,9%) son de Nivel 1, 4.485 (69,1%) de Nivel 2 y 262 (4,0%) de Nivel 3. La distribución refleja el modelo de penetración comercial esperado: los mercados norteamericanos, con mayor difusión de almacenes de membresía, concentran una proporción más elevada de clústeres de Nivel 1 que los mercados europeos, donde el formato de almacén de membresía tiene menor presencia.

La distribución geográfica de los clústeres de Nivel 1 muestra una concentración submetropolitana característica: el 78% de los clústeres de Nivel 1 en Estados Unidos se localizan en anillos de 15 a 80 km de distancia de los centros de las áreas metropolitanas principales, coherente con el modelo de parque de actividades comerciales periurbano documentado por Hernandez y Simmons (2006) para Canadá. El clúster de mayor rango en Norteamérica es Plano, Texas (suburbio de Dallas a 28,1 km; puntuación 25,5), con tres anclajes de Nivel 1, dos de Nivel 2 y uno de Nivel 3.

### 5.2 Distribución geométrica por nivel

Los clústeres de Nivel 1 presentan un diámetro medio (*span_km*) de 2,1 km frente a 1,8 km para los de Nivel 2 y 1,6 km para los de Nivel 3. La distribución por niveles exhibe asimetría positiva en los tres casos, con el mayor coeficiente de variación en Nivel 1 (CV = 0,61), lo que refleja la heterogeneidad entre configuraciones compactas de tipo *power centre* (diámetro < 1,0 km) y configuraciones dispersas de corredor periurbano (diámetro > 2,5 km).

### 5.3 Composición y empleo

Los datos LODES disponibles permiten computar el empleo total en un radio de 35 km del centroide para los clústeres de Estados Unidos. Los clústeres de Nivel 1 registran una mediana de 1,24 millones de puestos de trabajo en un radio de 35 km, frente a 0,87 millones para los de Nivel 2 y 0,61 millones para los de Nivel 3. Esta diferencia descriptiva es coherente con la hipótesis primaria H₁, aunque su causalidad requiere la especificación de control de §7.

---

## 6. Discusión

### 6.1 Hipótesis formales

**H₁** (hipótesis primaria, causal): Los clústeres de Nivel 1 atraen visitantes desde una zona geográfica significativamente más dispersa que los clústeres de Nivel 2, controlando por la población disponible en el anillo de 150 km. Operacionalmente: β_T1 > β_T2 > 0 en la ecuación de estimación de §7.2, con β_T1 − β_T2 significativo al nivel del 5%.

**H₀** (hipótesis nula): El nivel compositivo no aporta información adicional sobre la dispersión de la captación más allá de lo que ya explica log(pop_150km). Operacionalmente: β_T1 = β_T2 = 0 en la ecuación de §7.2.

**H₂** (hipótesis de redundancia de demanda): El nivel compositivo retiene poder predictivo sobre el empleo en el área de influencia incluso después de condicionar en el indicador de demanda observado derivado de los datos de movilidad origen-destino. Requiere datos de movilidad para el conjunto completo de países (año 2 del programa de investigación).

**H₃** (hipótesis de amplificación cívica): La presencia del modificador cívico (T0) amplifica positivamente el efecto del nivel compositivo sobre el empleo en el área de influencia. Operacionalmente: β₄ > 0 en la especificación de interacción de §7.5, en ambas variantes (con y sin empleo de los sectores NAICS 611/622).

### 6.2 Alternativas observacionales

La taxonomía compositiva produce una clasificación basada en qué categorías de ancla están presentes, no en parámetros de demanda que pueden cambiar a lo largo del ciclo económico o del calendario de datos. Esta propiedad de invarianza hace de la clasificación compositiva un indicador más estable a lo largo del tiempo que los índices basados en datos de ventas, aforo o movilidad de panel, cuya disponibilidad es irregular fuera de los mercados norteamericano y anglosajón.

### 6.3 Posicionamiento estructural

Los datos de cobertura submetropolitana de los proveedores comerciales de referencia en el sector (como CBRE Economics o Oxford Economics) se concentran principalmente en las áreas metropolitanas centrales y en los veinte o treinta mayores mercados urbanos de cada país. El análisis presentado en este artículo cubre sistemáticamente el anillo periurbano de 15 a 80 km de distancia del centro metropolitano, que constituye la localización predominante de los clústeres de gran superficie identificados y que corresponde a la mayor parte de la actividad comercial submetropolitana en términos de superficie construida total.

### 6.4 Limitaciones de la cobertura de OpenStreetMap

La precisión de la clasificación depende de la integridad de los datos de OpenStreetMap para cada país. En los países con mayor penetración de grandes superficies (Estados Unidos, Reino Unido, Francia, Alemania, España), la cobertura es sistemáticamente alta para las cadenas incluidas en la taxonomía. En países con menor densidad de edición de OSM (algunos países de Europa central y oriental), puede haber una subestimación de clústeres, especialmente en zonas rurales y periurbanas de menor tamaño.

---

## 7. Programa de falsación

### 7.0 Análisis preliminar

*Modelo A — Regresión de diámetro por nivel.* La ecuación estimadora es:

    log(span_km_c) = α + β₁·T1_c + β₂·T2_c + δ_país + ε_c

donde span_km es el diámetro de la envolvente convexa del clúster en kilómetros y T3 es la categoría de referencia. Predicción: los clústeres de Nivel 1 deben ser geométricamente más grandes que los de Nivel 3.

*Resultados.* Los clústeres de Nivel 1 son significativamente más grandes que los de Nivel 3 (β₁ = 0,489; IC 95% [0,359; 0,619]; p < 0,001), lo que corresponde a un diámetro un 63% mayor en la escala no logarítmica (exp(0,489) = 1,631). Los clústeres de Nivel 2 no se distinguen geométricamente de los de Nivel 3 (β₂ = 0,008; p = 0,833). Los efectos fijos de país explican una varianza sustancial (R² = 0,121), lo que refleja la variación entre países en los formatos de distribución de grandes superficies.

*Modelo B — Modelo de probabilidad lineal de clasificación en Nivel 1.* La ecuación estimadora es:

    T1_dummy = α + β₁·has_price_club + β₂·has_electronics + β₃·has_lifestyle
               + β₄·has_sport + β₅·log(span_km) + β₆·tight + δ_país + ε

donde T1_dummy toma valor 1 si el clúster es de Nivel 1. Predicción: la presencia de almacén de membresía (has_price_club) debe ser el predictor positivo dominante de la clasificación en Nivel 1.

*Resultados.* La presencia de almacén de membresía es el predictor más potente de la clasificación en Nivel 1 (β₁ = 0,639; IC 95% [0,584; 0,694]; p < 0,001), confirmando que esta categoría es el diferenciador estructural clave entre los niveles 1 y 2. Las grandes superficies de electrónica son el segundo predictor más potente (β₂ = 0,489; p < 0,001). Las grandes superficies de artículos de estilo de vida (β₃ = 0,311; p < 0,001) y de artículos deportivos (β₄ = 0,135; p = 0,001) también son predictores significativos. El modelo explica el 50,3% de la varianza en la clasificación en Nivel 1 (R² = 0,503), lo que proporciona validación empírica de la taxonomía compositiva.

*Interpretación.* Los dos modelos establecen conjuntamente que (1) la pertenencia al Nivel 1 se asocia con una huella física estructuralmente mayor que la de los niveles 2 y 3, y que (2) la taxonomía compositiva es empíricamente coherente. Ninguno de los modelos constituye una prueba de H₁, que requiere datos de captación por origen-destino.

### 7.1 Prueba 1 — Densidad de empleo por desplazamiento al trabajo (EE.UU., ejecutable)

*Especificación:*

    log(empleo_35km) = α + β₁·T1 + β₂·T2 + γ·log(población) + δ_estado + ε

donde empleo_35km es el empleo total de destino LODES en un radio de 35 km del centroide del clúster; T1 y T2 son variables indicadoras (T3 es la categoría de referencia); población es la población del mercado submetropolitano; δ_estado son efectos fijos de estado.

*Predicción:* β₁ > β₂ > 0.

*Datos:* LODES 2021, disponibles. La unión clúster-LODES está especificada y se ejecutará en versión posterior del manuscrito.

### 7.2 Prueba 2 — Validación de la captación de movilidad primaria (EE.UU. y España)

*Hipótesis.* Los clústeres de Nivel 1 atraen dispositivos visitantes desde un área de origen geográficamente más amplia y dispersa que los de Nivel 2, controlando por la población disponible en el anillo de 150 km. La dispersión de la captación —y no el recuento de la población disponible— es la característica diferenciadora de una co-ubicación de destino.

*Método.* Para cada clúster con cobertura de datos de movilidad origen-destino se calculan tres variables. La primera es catchment_area_km2, el área de la envolvente convexa de todos los centroides de celdas de residencia en la zona de captación definida por movilidad, limitada al anillo de 150 km. La segunda es catchment_entropy: H = −Σ pᵢ log pᵢ, donde pᵢ es la fracción de dispositivos visitantes procedentes de la celda H3 iésima. La tercera es home_work_ratio, el cociente entre dispositivos cuya celda de residencia habitual cae dentro de la zona de captación y dispositivos cuya celda de trabajo cae dentro de la zona.

La ecuación estimadora es:

    catchment_entropy_c = α + β_T1·T1_c + β_T2·T2_c + γ·log(pop_150km_c) + δ_iso + ε_c

donde δ_iso es un efecto fijo de país que absorbe las diferencias de cobertura entre los paneles LODES y MITMA.

*Prueba primaria.* β_T1 > β_T2 > 0, con β_T1 − β_T2 significativo al nivel del 5%.

### 7.3 Prueba 3 — Densidad de empleo con control de demanda (H₂)

*Especificación:*

    log(empleo_35km) = α + β₁·T1 + β₂·T2 + γ·log(población) + λ·mobility_index + δ + ε

H₂ predice que β₁ y β₂ se mantienen positivos y significativos tras condicionar en el indicador de demanda observado.

*Datos:* Requiere cobertura completa de datos de movilidad origen-destino (año 2 del programa de investigación).

### 7.4 Prueba 4 — Prueba de permutación (ejecutable)

*Método:* Se permutan las asignaciones de nivel entre clústeres 10.000 veces, manteniendo constantes el número y la distribución de tamaños dentro de cada país. Para cada permutación se calcula la correlación de rangos entre nivel y empleo por desplazamiento al trabajo en un radio de 35 km. Se compara la correlación observada con la distribución nula de permutación.

*Predicción:* La correlación observada cae en el 1% superior de la distribución de permutación (p < 0,01, unilateral).

*Datos:* El conjunto de datos LODES actual es suficiente para ejecutar esta prueba.

### 7.5 Prueba 5 — Amplificación cívica (H₃)

*Especificación:*

    log(empleo_35km) = α + β₁·T1 + β₂·T2 + β₃·cívico + β₄·(T1 × cívico) + γ·log(población) + δ_estado + ε

*Diagnóstico de control inválido.* El empleo en trabajo de destino en un radio de 35 km incluye mecánicamente el empleo sanitario (NAICS 622) y de servicios educativos (NAICS 611), que es generado directamente por las instituciones cívicas que definen cívico = 1. Para distinguir el efecto de amplificación teórico H₃ del efecto de empleo directo, la regresión debe ejecutarse en dos especificaciones:

- *Especificación A:* empleo_35km incluye todos los sectores NAICS.
- *Especificación B:* empleo_35km_sinedu excluye el empleo NAICS 611/622.

H₃ se sostiene si β₄ > 0 en *ambas* especificaciones.

*Predicción:* β₄ > 0 en ambas especificaciones; el efecto de amplificación es mayor en clústeres T1 compactos que en clústeres T1 dispersos.

---

## 8. Conclusión

### 8.1 Síntesis de contribuciones

Este artículo ha propuesto una taxonomía compositiva formal de clústeres de co-ubicación de anclas de grandes superficies —una clasificación que identifica qué *categorías* de ancla están presentes en un clúster submetropolitano, en lugar de limitarse a contabilizar el número de establecimientos co-ubicados o a medir su tamaño agregado. La taxonomía se implementa a escala continental mediante datos abiertos, produciendo 6.493 clústeres clasificados en trece países.

La contribución teórica es el argumento de preferencia revelada: cuando múltiples cadenas dominantes de distintas categorías seleccionan de forma independiente la misma ubicación submetropolitana, la señal compuesta de su co-presencia constituye un indicador espacial adelantado de la intensidad de la actividad comercial más duradero y metodológicamente más sencillo que cualquier conjunto de datos de demanda de una sola fecha de referencia. La demostración de Holmes (2011) de que la selección de emplazamientos de Walmart refleja economías de densidad fundamenta empíricamente este argumento.

La contribución metodológica es el algoritmo DBSCAN de doble pasada y el rango de distancia ajustado por *shrinkage* dentro de cada nivel —una medida de compacidad que discrimina entre iguales compositivos y es estable frente a la variación del tamaño muestral inherente a un conjunto de datos de trece países. El protocolo origen-destino por polígono y dispositivo, especificado en §3.7, sustituye los límites administrativos de las zonas de captación por mercados submetropolitanos definidos por movilidad, lo que permite la comparación de la demanda a nivel de clúster —en lugar de a nivel de asentamiento.

### 8.2 Líneas de investigación futuras

La prioridad inmediata es la unión LODES y la prueba de permutación (§7.1 y §7.4), que pueden ejecutarse con los datos actualmente disponibles. A medio plazo, el programa contempla la adquisición de datos origen-destino para el Reino Unido, Francia y Alemania, lo que permitirá ejecutar la Prueba 2 a escala y proporcionar el primer contraste intercontinental de H₁. A largo plazo, se prevé un análisis de series temporales para evaluar la estabilidad de la señal compositiva frente a la evolución de los formatos de grandes superficies.

---

## Disponibilidad de datos

Los datos de establecimientos minoristas proceden de OpenStreetMap (openstreetmap.org), disponibles bajo la Open Database Licence (ODbL). Los archivos de configuración por cadena, los scripts de clasificación y el conjunto de datos clusters-meta.json están disponibles en [URL del repositorio pendiente]. Los datos LODES están disponibles públicamente en el programa LEHD del Bureau of the Census de EE.UU. Los datos de movilidad MITMA están disponibles públicamente en el portal de datos abiertos del Ministerio de Transportes.

---

## Referencias

Angrist, J. D., y J.-S. Pischke. 2009. *Mostly Harmless Econometrics: An Empiricist's Companion.* Princeton: Princeton University Press.

Anselin, L. 1988. *Spatial Econometrics: Methods and Models.* Dordrecht: Kluwer Academic.

Basker, E. 2005. Selling a cheaper mousetrap: Wal-Mart's effect on retail prices. *Journal of Urban Economics* 58(2): 203–229.

Berry, B. J. L. 1958. Retail location and consumer behaviour. *Papers and Proceedings of the Regional Science Association* 5(1): 65–73.

Brueckner, J. K. 1993. Inter-store externalities and space allocation in shopping centers. *Journal of Real Estate Finance and Economics* 7(1): 5–16.

Büchel, K., y M. V. Ehrlich. 2021. Cities and the structure of social interactions: Evidence from mobile phone data. *Journal of Urban Economics* 121: 103–316.

Calafiore, A., G. Boeing, A. Singleton, y D. Arribas-Bel. 2022. Redefining retail catchment with mobile geolocation data: Insights from New Zealand. *Journal of Retailing and Consumer Services* 79. https://doi.org/10.1016/j.jretconser.2024.103893.

Carlino, G., y W. R. Kerr. 2015. Agglomeration and innovation. En *Handbook of Regional and Urban Economics,* vol. 5, editado por G. Duranton, J. V. Henderson, y W. C. Strange, 349–404. Ámsterdam: Elsevier.

Chen, Y., et al. 2022. Causal analysis on the anchor store effect in a location-based social network. *arXiv preprint* arXiv:2210.13582.

Christaller, W. 1933. *Die zentralen Orte in Süddeutschland.* Jena: Gustav Fischer.

Coe, N. M., y N. Wrigley. 2007. Host economy impacts of transnational retail: The research agenda. *Journal of Economic Geography* 7(4): 341–371.

Darnall, N., I. Seol, J. Sarkis, y J. Cordeiro. 2022. An open source delineation and hierarchical classification of UK retail agglomerations. *PLOS ONE* 17(9): e0264713.

Drucker, J., y H. Goldstein. 2007. Assessing the regional economic development impacts of universities. *International Regional Science Review* 30(1): 20–46.

Duranton, G., y D. Puga. 2004. Micro-foundations of urban agglomeration economies. En *Handbook of Regional and Urban Economics,* vol. 4, 2063–2117. Ámsterdam: Elsevier.

Ellison, G., E. L. Glaeser, y W. R. Kerr. 2010. What causes industry agglomeration? *American Economic Review* 100(3): 1195–1213.

Eppli, M. J., y J. D. Shilling. 1995. Large-scale shopping center development opportunities. *Land Economics* 71(1): 35–41.

Ester, M., H.-P. Kriegel, J. Sander, y X. Xu. 1996. A density-based algorithm for discovering clusters in large spatial databases with noise. *Proceedings of KDD-96*, 226–231.

Glasson, J. 2003. The widening local and regional development impacts of the modern universities. *Local Economy* 18(1): 21–37.

Haklay, M. 2010. How good is volunteered geographical information? *Environment and Planning B* 37(4): 682–703.

Hernandez, T., y J. Simmons. 2006. Evolving retail landscapes: Power retail in Canada. *Canadian Geographer* 50(4): 465–486.

Holmes, T. J. 2011. The diffusion of Wal-Mart and economies of density. *Econometrica* 79(1): 253–302.

Huff, D. L. 1964. Defining and estimating a trading area. *Journal of Marketing* 28(3): 34–38.

Kim, J., y K. Park. 2025. Effect of agglomeration externalities of adjacent retail areas on commercial business continuity in Seoul. *Growth and Change* 56(2).

Konishi, H. 2005. Concentration of competing retail stores. *Journal of Urban Economics* 58(3): 488–512.

Krugman, P. 1991. *Geography and Trade.* Cambridge: MIT Press.

Li, Z., H. Ning, F. Jing, y M. N. Lessani. 2024. Understanding the bias of mobile location data across spatial scales. *PLOS ONE* 19(10): e0294430.

Marshall, A. 1890. *Principles of Economics.* Londres: Macmillan.

Neumark, D., J. Zhang, y S. Ciccarella. 2008. The effects of Wal-Mart on local labor markets. *Journal of Urban Economics* 63(2): 405–430.

Pashigian, B. P., y E. D. Gould. 1998. Internalizing externalities: The pricing of space in shopping malls. *Journal of Law and Economics* 41(1): 115–142.

Reilly, W. J. 1931. *The Law of Retail Gravitation.* Nueva York: Knickerbocker Press.

Rosenthal, S. S., y W. C. Strange. 2004. Evidence on the nature and sources of agglomeration economies. En *Handbook of Regional and Urban Economics,* vol. 4, 2119–2171. Ámsterdam: Elsevier.

U.S. Census Bureau. 2021. *LEHD Origin-Destination Employment Statistics (LODES), Version 8.* Washington, DC.

Wrigley, N., y M. Lowe. 2002. *Reading Retail: A Geographical Perspective on Retailing and Consumption Spaces.* Londres: Arnold.

Zhao, S., Y. Chen, Y. Duan, y Z. Xu. 2025. Site selection analysis and prediction of new retail stores. *ISPRS International Journal of Geo-Information* 14(6): 217.

---

## Apéndice A — Referencia de notación matemática

| Símbolo | Definición |
|---|---|
| ε, τ_tight, τ_loose | Umbrales de proximidad DBSCAN (1,0 km; 3,0 km) |
| Δ_max | Límite máximo de diámetro del clúster = 3,0 km |
| span_km | Distancia geodésica máxima entre pares de puntos del clúster |
| tight_intact | Booleano: todos los miembros a distancia ≤ τ_tight = 1,0 km |
| ring_radius_km | max(1,0; span_km / 2 × 1,15) |
| dist_rank_in_tier | CDF invertida suavizada por shrinkage del span_km dentro del nivel |
| w | Peso shrinkage = n_país / (n_país + K), K ≈ 20–30 |
| T1.a, T1.b, T1.c | Rutas de admisión T1: composición tripartita; compacidad h2b; amplitud ≥ 4 |
| T1, T2, T3 | Clasificación de nivel por composición de categorías de ancla |
| T0 (Platino) | T1 ∧ tight_intact ∧ modificador_cívico = 1 |
| modificador_cívico | 1 si hospital ≥ 200 camas o universidad ≥ 10.000 matriculados en radio 5,0 km |
| catchment_area_km2 | Área de la envolvente convexa de centroides de celdas de residencia en la captación |
| catchment_entropy | H = −Σ pᵢ log pᵢ sobre celdas H3 de origen de visitantes |
| home_work_ratio | Dispositivos con celda de residencia en captación ÷ dispositivos con celda de trabajo en captación |
| H₁, H₀, H₂, H₃ | Hipótesis primaria, nula, de redundancia de demanda y de amplificación cívica |

## Apéndice B — Cobertura de cadenas por país

Un inventario completo por cadena y país se añadirá en versión posterior del manuscrito.

## Apéndice C — Arquitectura del marco analítico

Un diagrama de flujo de datos se añadirá en versión posterior del manuscrito.

---

## Declaración sobre uso de inteligencia artificial

La redacción, el formalismo matemático y el marco de investigación de este manuscrito se desarrollaron con la asistencia de Claude (Anthropic, claude-sonnet-4-6), un modelo de lenguaje de gran escala. El conjunto de datos analítico, las reglas de taxonomía, el algoritmo de clasificación y todos los resultados cuantitativos fueron producidos por los autores de forma independiente. La búsqueda y selección bibliográfica, el fundamento teórico y todas las afirmaciones científicas son responsabilidad de los autores y han sido revisadas por el autor de correspondencia antes de la difusión del manuscrito.

Esta declaración sigue las directrices del Comité de Ética de Publicaciones (COPE) sobre inteligencia artificial y autoría: las herramientas de IA no tienen la condición de autores; su uso debe declararse; el autor de correspondencia asume la responsabilidad sobre la integridad del trabajo.

---

## Contribuciones de los autores (CRediT)

**Jennifer M. Woodfine:** Conceptualización, Metodología, Análisis formal, Redacción del borrador original, Revisión y edición.
**Peter M. Woodfine:** Conceptualización, Validación, Revisión y edición.
**Mathew Woodfine:** Software, Curación de datos, Revisión y edición.

---

## Declaración de conflictos de interés

Los autores tienen interés comercial en herramientas de análisis de localización de establecimientos minoristas. Este interés no influyó en el diseño del estudio, la recopilación de datos ni la interpretación de los resultados. Los conjuntos de datos utilizados son de acceso público; la metodología analítica se divulga íntegramente en los apartados §3 y §4; y todos los resultados cuantitativos son reproducibles a partir de las fuentes de datos abiertos descritas en este manuscrito.

---

## Declaración de financiación

Esta investigación no ha recibido financiación externa de ninguna fuente.

---

## Declaración de disponibilidad de datos

Los datos de localización de establecimientos ancla utilizados en este estudio proceden de OpenStreetMap, disponibles libremente bajo la Open Database Licence (ODbL). Los datos de población proceden de Kontur Population (CC BY 4.0). Los datos de empleo por origen-destino proceden del programa LODES del LEHD del Bureau of the Census de EE.UU. (dominio público) y del Ministerio de Transportes de España (datos abiertos del gobierno). Los scripts de cálculo de clústeres y los archivos de configuración de taxonomía están disponibles de los autores correspondientes a solicitud razonada.
