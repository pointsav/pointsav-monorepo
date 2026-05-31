---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC-*
title: "Arquitectura de Resolución de Nombres Regionales"
slug: topic-regional-name-resolution
language: es
status: staged
destination: media-knowledge-documentation
bcsc_class: no-disclosure-implication
paired_with: TOPIC-regional-name-resolution.draft.md
research_done_count: 4
research_suggested_count: 1
open_questions_count: 0
research_provenance: archivos fuente del pipeline (config.py, build-clusters.py, ca_places_nominatim.json); documentación TIGER 2023; documentación GISCO LAU 2021; documentación API Nominatim OSM
research_inline: true
created: 2026-05-31
---

# Arquitectura de Resolución de Nombres Regionales

Los clústeres de colocalización se identifican mediante geometría: un conjunto de
coordenadas de latitud y longitud derivadas de los registros de puntos de interés
de OpenStreetMap. La geometría no tiene nombre. Asignar a un clúster un nombre
útil y reconocible para los usuarios requiere un paso de resolución independiente
que hace coincidir el centroide del clúster con conjuntos de datos de nombres de
lugares con autoridad reconocida. Este artículo describe cómo funciona dicha
resolución, por qué es necesaria y dónde puede fallar.

## El Problema con los Límites Administrativos

OpenStreetMap y Wikidata organizan la geografía en jerarquías administrativas:
país, región, condado, municipio. Estas jerarquías están definidas legal y
políticamente. No siempre corresponden a los nombres que residentes, empresas e
investigadores de mercado utilizan para describir un lugar.

Consideremos un clúster de colocalización comercial en la comunidad de Sherwood
Park, Alberta. Sherwood Park es una comunidad no incorporada dentro del Condado
de Strathcona. Su límite administrativo en OSM es el condado — *Strathcona County*
— no la comunidad. Un algoritmo que resuelva nombres de clústeres únicamente a
partir de límites administrativos etiquetaría este clúster como "Strathcona
County", un nombre que aporta escasa información a un investigador que estudia
patrones comerciales suburbanos en el área metropolitana de Edmonton. El nombre
"Sherwood Park" es el que usan la comunidad, sus comercios y sus residentes. Es
el que debe aparecer en el título de un artículo TOPIC de Mercado Regional.

Esta desconexión entre la geografía administrativa formal y los nombres
coloquiales de los lugares no es un caso excepcional. Aparece en todo contexto
donde persisten comunidades no incorporadas, subdivisiones censales y nombres
históricos de ciudades junto a estructuras de condado o municipio más recientes.
La arquitectura de resolución existe para salvar esa brecha.

## Conjuntos de Datos de Límites

Cuatro conjuntos de datos proporcionan candidatos de nombres de lugares, cada
uno cubriendo una parte diferente del alcance geográfico de la plataforma.

**TIGER 2023 (Estados Unidos).** El conjunto de datos TIGER (Topologically
Integrated Geographic Encoding and Referencing) de la Oficina del Censo de EE.
UU. proporciona límites de lugares para los Estados Unidos. La versión 2023
incluye aproximadamente 32.000 lugares con nombre: ciudades y pueblos
incorporados, lugares designados por el censo (CDP) y algunas comunidades no
incorporadas con nombres reconocidos. Los lugares TIGER son la fuente de
resolución principal para todos los clústeres de EE. UU.

**GISCO LAU 2021 (Unión Europea y países asociados).** La Comisión Europea
publica los límites de Unidades Administrativas Locales (LAU) derivados de la
nomenclatura NUTS. La versión 2021 cubre aproximadamente 98.600 municipios en
estados miembros de la UE y países vecinos del marco de Eurostat. Los límites
LAU son la fuente de resolución principal para clústeres de la UE en Alemania,
Francia, España, Italia, Polonia, Países Bajos, Austria, Portugal, Grecia,
Suecia, Dinamarca, Finlandia y Noruega.

**GADM GBR (Reino Unido).** La base de datos GADM proporciona datos de límites
subnacionales para países no cubiertos por GISCO. Para el Reino Unido, GADM
proporciona límites administrativos de nivel 3 (parroquias y distritos electorales
en Inglaterra; comunidades en Gales; parroquias civiles en Escocia).

**Reemplazos Nominatim (Canadá).** Canadá presenta un desafío particular porque
las subdivisiones censales (CSD) a veces abarcan grandes áreas geográficas que
contienen múltiples comunidades distintas con nombres diferentes. Doce entradas
de reemplazo manual en `ca_places_nominatim.json` proporcionan nombres canónicos
de lugares para casos en los que el nombre CSD sería engañoso. Sherwood Park
(CSD Condado de Strathcona) es uno de estos doce reemplazos.

## Lógica de Resolución

Para cada centroide de clúster, el algoritmo de resolución procede como sigue:

*Coincidencia de nombre.* El algoritmo primero verifica si las ubicaciones
comerciales del clúster tienen una etiqueta `addr:city` o `addr:suburb`
coherente en OSM. Si la mayoría de los registros miembros coinciden en un nombre
de lugar, ese nombre se toma como candidato sin consultar los conjuntos de datos
de límites.

*Contención por límites.* Si no existe consenso en las etiquetas OSM, se prueba
si el centroide está contenido en el conjunto de datos de límites aplicable. Se
selecciona el polígono de menor área que contiene el centroide, y su campo de
nombre se convierte en el candidato de resolución.

*Respaldo de nivel administrativo.* Si ningún polígono al nivel administrativo
preferido contiene el centroide, el algoritmo sube al siguiente nivel
administrativo y repite la prueba de contención.

*Aplicación de reemplazos.* Después de identificar el candidato inicial, el
algoritmo verifica el nombre candidato frente a la lista de reemplazos. Para
Canadá, si el nombre CSD resuelto coincide con uno de los doce nombres
problemáticos conocidos, el reemplazo proporciona el nombre coloquial correcto.

## Por Qué Importan los Nombres Canónicos

El nombre resuelto no es simplemente una etiqueta de visualización. Es el
identificador principal utilizado en el sistema de puntuación de Mercados
Regionales. El nombre resuelto de un clúster determina qué cálculo de distancia
metropolitana se le aplica. Una resolución incorrecta causaría una clasificación
errónea del mercado y afectaría a la puntuación de esa ubicación.

El nombre resuelto también se convierte en el título de cualquier artículo TOPIC
de Mercado Regional escrito para ese clúster. La exactitud aquí es una cuestión
de integridad editorial.

## Limitaciones Conocidas

La arquitectura de resolución actual se apoya en conjuntos de datos de límites
con versiones fijas (TIGER 2023, GISCO LAU 2021). Los nombres que hayan cambiado
desde esas versiones no quedarán reflejados hasta que se actualicen los datos.
De igual modo, las comunidades de nueva creación posteriores a los conjuntos de
datos recurrirán a la resolución por nivel administrativo, que puede producir
nombres menos específicos.

Las doce entradas de reemplazo para Canadá representan los casos identificados
durante los ciclos de compilación de las Fases 14 y 15. Pueden existir otras
discordancias entre nombres CSD y comunitarios en áreas aún no cubiertas.

---

*Procedencia de los datos:* TIGER 2023 (Oficina del Censo de EE. UU., dominio público);
GISCO LAU 2021 (Eurostat/CE, CC BY 4.0); GADM GBR (GADM v4.1, licencia de
investigación no comercial); reemplazos Nominatim (originales, project-gis).
Datos OSM CC0.
