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
| 🔴 Investigación | Analizador Sintáctico Incremental | Fundación de Datos Soberana (previsto) |

## 📖 Marcador de Auditoría Arquitectónica

Este directorio es un marcador estructural en el registro **MOONSHOTS / PROYECTOS ESPECIALES**.
Registra la intención arquitectónica de reemplazar la infraestructura de análisis de terceros
con un analizador incremental nativo, verificado y orientado a Rust. El plano existe; la
implementación está prevista.

## I. QUÉ REEMPLAZA ESTE MOONSHOT

`moonshot-parser` apunta a **tree-sitter** (la biblioteca de análisis incremental de terceros y
su corpus externo de gramáticas) y al tokenizador sintáctico por expresiones regulares hecho a
mano actualmente en `app-privategit-workbench`. El tokenizador actual es frágil con
construcciones anidadas y omite el resaltado por encima de un tamaño de archivo fijo.

## II. QUÉ SE ESTÁ CONSTRUYENDO

Un **analizador sintáctico incremental nativo escrito en Rust** (compilable a WebAssembly):

- Produce un árbol sintáctico concreto y vuelve a analizar solo el tramo afectado en cada
  edición, de modo que el costo de una pulsación permanece acotado sin importar el tamaño del
  archivo.
- Impulsa el resaltado de sintaxis y alimenta información de rango de origen a
  [`moonshot-docengine`](../moonshot-docengine) para el mapeo renderizado↔origen.
- Definiciones de gramática propias en el árbol, en lugar de extraídas de un registro externo.

El horizonte de reemplazo es **de adentro hacia afuera**: el banco de trabajo puede comenzar
sobre una base verificada y bifurcable, y migrar a este motor a medida que madura, retirando
por completo la dependencia externa.

---
*© 2026 PointSav Digital Systems.*
*Plano Arquitectónico Público. Regido por el Protocolo de Datos Soberano.*
