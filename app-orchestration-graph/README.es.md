# app-orchestration-graph

Puerta de enlace de federación de DataGraph entre múltiples archivos Totebox para la
capa de orquestación os-orchestration de PointSav.

**Estado: Carpeta reservada.** La función de federación de DataGraph reside
actualmente en `app-orchestration-slm` (`POST /v1/graph/federated`). Este proyecto
se activa cuando esa carga de trabajo crece lo suficiente como para justificar su
extracción en un servicio dedicado.

**Licencia:** Propietaria.

## Qué hará

`app-orchestration-graph` es la puerta de enlace dedicada a través de la cual
`os-orchestration` accede a los DataGraphs de múltiples archivos Totebox
simultáneamente. Cada archivo Totebox mantiene un DataGraph soberano de
`service-content` — entidades, relaciones y metadatos del corpus específicos al
dominio de ese archivo. Ningún archivo Totebox puede consultar directamente el
DataGraph de otro. Todo acceso al DataGraph entre archivos pasa por esta puerta
de enlace.

## Por qué un servicio separado

`app-orchestration-slm` es el intermediario de inferencia — su función es enrutar
solicitudes de inferencia al nivel de cómputo adecuado (Tier A local, Tier B Yo-Yo,
Tier C Anthropic). Actualmente también gestiona la federación de DataGraph mediante
`POST /v1/graph/federated`, que distribuye consultas a todos los Doormen Totebox
registrados y agrega los resultados.

Cuando la flota es pequeña (2–5 Toteboxes), combinar federación e inferencia en el
mismo proceso es adecuado. A medida que la flota crece, las consultas de distribución
a DataGraph se convierten en una carga de trabajo de alto volumen independiente, con
requisitos distintos de latencia, caché y pool de conexiones respecto a la inferencia.
En ese punto, mezclarlas degrada ambas.

`app-orchestration-graph` extrae la federación en su propio proceso:
- Mantiene conexiones persistentes a todos los endpoints `service-content` de Toteboxes
  registrados
- Gestiona la distribución, agregación de resultados, tolerancia a fallos parciales y
  caché de resultados
- Libera a `app-orchestration-slm` de la responsabilidad del DataGraph

## Lo que NO es

- No es un almacén de DataGraph. No mantiene datos de entidades propios. Todos los datos
  permanecen soberanos en el `service-content` de cada Totebox.
- No es un servicio de sincronización. No replica DataGraphs entre Toteboxes. Las
  consultas son bajo demanda; no se empuja ni recibe nada de la puerta de enlace.
- No es `app-orchestration-content`. Ese nombre fue considerado y descartado — generaría
  confusión con `service-content` (el almacén por Totebox). Este servicio es una puerta
  de enlace de federación de grafos, no un almacén de contenido.

## Relación con otros servicios

| Servicio | Función | Relación |
|---|---|---|
| `service-content` | Almacén DataGraph por Totebox | Fuente de datos — consultada por esta puerta de enlace |
| `app-orchestration-slm` | Intermediario de inferencia | Actualmente contiene la lógica de federación; la cede al activarse |
| `service-slm` (Doorman) | Enrutador de inferencia por Totebox | Tiene endpoints de consulta DataGraph gestionados por esta puerta |

## Puerto

`:9181` (planificado; `:9180` corresponde a `app-orchestration-slm`)

## Umbral de activación

Extraer de `app-orchestration-slm` cuando se cumpla alguna de estas condiciones:
- La flota supera ~10 archivos Totebox con DataGraphs activos
- Las consultas de distribución al DataGraph afectan de forma medible la latencia del
  intermediario de inferencia
- Surge un segundo consumidor de acceso al DataGraph entre archivos (más allá del
  camino de inferencia)

Hasta entonces, `POST /v1/graph/federated` en `app-orchestration-slm` cubre el caso
de uso.
