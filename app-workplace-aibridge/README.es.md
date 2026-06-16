# app-workplace-aibridge

[ 🇬🇧 Read in English ](./README.md)

El **núcleo del puente de edición por secciones con IA** para el banco de trabajo
Workplace. Permite resaltar una sección de un documento, entregar **solo esa sección**
a una sesión de IA externa y aplicar el resultado — en lugar de pasar un archivo
completo por un modelo.

Este crate es el núcleo determinista en Rust que un servidor de Protocolo de Contexto
de Modelo (MCP) envuelve. Compone:

- [`moonshot-docengine`](../moonshot-docengine) — ajusta una selección arbitraria a su
  sección de documento contenedora y direcciona el origen por rango de bytes.
- [`moonshot-crdt`](../moonshot-crdt) — aplica el reemplazo de la IA como una edición
  reversible que incrementa la versión.

## Superficie de herramientas

| Herramienta | Método | Propósito |
|---|---|---|
| `read_selection` | `Bridge::read_selection(span)` | Ajusta una selección a su sección; devuelve el texto aislado entregado a la IA. |
| `propose_edit` | `Bridge::propose_edit(span, new)` | Previsualiza el texto resultante sin confirmar. |
| `commit_edit` | `Bridge::commit_edit(span, new)` | Aplica el reemplazo como edición reversible; devuelve la nueva versión. |

## SYS-ADR-07

Los esquemas estructurados fiduciarios/geométricos — proforma, schedule, GIS, BIM — nunca
deben pasar por una capa de IA. `Bridge` los rechaza en cada punto de entrada
(`BridgeError::SchemaForbidden`). Solo el contenido de prosa, código y presentación es elegible.

## Alcance

Este crate es solo el núcleo determinista. El protocolo de cable MCP y la conexión con
la sesión de Claude en vivo son la capa de integración, verificada contra un banco de
trabajo en ejecución (no headless). Sin dependencias de tiempo de ejecución más allá de
los dos núcleos hermanos.
