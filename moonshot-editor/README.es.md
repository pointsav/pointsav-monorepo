<div align="center">

# Iniciativa de Reemplazo Soberano | Sovereign Replacement Initiative

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

### *Una iniciativa de ingeniería activa para reemplazar arquitectura extranjera de terceros.*

</div>

<br/>

> [!WARNING]
> **DECLARACIÓN DEL MARCO SOBERANO**
> Este repositorio es una implementación de referencia del Protocolo de Datos Soberano. Impone aislamiento absoluto de datos. No contiene cargas de red propietarias activas.

| Nivel de Arquitectura | Función del Componente | Ancla de Gobernanza |
| :--- | :--- | :--- |
| 🔴 Investigación | Superficie de Widgets: Editor / Visor / Árbol de Archivos | Fundación de Datos Soberana (previsto) |

## 📖 Marcador de Auditoría Arquitectónica

Este directorio es un marcador estructural en el registro **MOONSHOTS / PROYECTOS ESPECIALES**.
Registra la intención arquitectónica de reemplazar widgets de editor y de virtualización de
árbol de terceros con una superficie de widgets nativa, verificada y orientada a Rust. El plano
existe; la implementación está prevista.

## I. QUÉ REEMPLAZA ESTE MOONSHOT

`moonshot-editor` apunta a los widgets interactivos de terceros que un banco de trabajo de otro
modo arrastraría: los editores de código **CodeMirror 6** / **Monaco** y las bibliotecas de
virtualización de árbol **react-arborist** / **react-window**. También retira el editor actual de
`app-privategit-workbench` —un `<textarea>` transparente superpuesto sobre un `<pre>` de
resaltado— y su árbol de archivos no virtualizado, que renderiza un nodo DOM por archivo y es la
causa del retraso de navegación en árboles grandes.

## II. QUÉ SE ESTÁ CONSTRUYENDO

Una **superficie de widgets de editor / visor / árbol de archivos** nativa (Rust → WebAssembly):

- Un núcleo de editor de texto: modelo de cursor y selección, margen de numeración,
  buscar/reemplazar, multicursor — impulsado por
  [`moonshot-docengine`](../moonshot-docengine) y resaltado por
  [`moonshot-parser`](../moonshot-parser).
- Un **árbol de archivos virtualizado** que renderiza solo las filas visibles y carga los hijos
  de directorio bajo demanda, eliminando el comportamiento de DOM-O(n) y de obtención síncrona
  por carpeta que causa el retraso actual.
- Un modelo de selección que produce un **identificador de sección estable**, la unidad sobre la
  que opera el puente de edición por IA.

La misma superficie de widgets se prevé que sirva tanto al prototipo de navegador como a la
superficie nativa `os-workplace` sin una segunda implementación.

---
*© 2026 PointSav Digital Systems.*
*Plano Arquitectónico Público. Regido por el Protocolo de Datos Soberano.*
