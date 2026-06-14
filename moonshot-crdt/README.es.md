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
| 🔴 Investigación | Estado Colaborativo y Linaje de Versiones | Fundación de Datos Soberana (previsto) |

## 📖 Marcador de Auditoría Arquitectónica

Este directorio es un marcador estructural en el registro **MOONSHOTS / PROYECTOS ESPECIALES**.
Registra la intención arquitectónica de reemplazar bibliotecas de estado colaborativo de
terceros con un tipo de dato replicado nativo, verificado y orientado a Rust. El plano existe;
la implementación está prevista.

## I. QUÉ REEMPLAZA ESTE MOONSHOT

`moonshot-crdt` apunta a las bibliotecas de Tipos de Datos Replicados Sin Conflictos de terceros
de las que un banco de trabajo colaborativo de otro modo dependería: **Loro**, **Yjs** y
**Automerge**. De estas, solo Loro es nativa de Rust; las demás están atadas a JavaScript y no
pueden servir a la superficie nativa `os-workplace` sin reimplementación.

## II. QUÉ SE ESTÁ CONSTRUYENDO

Un **tipo de dato replicado nativo escrito en Rust** para colaboración e historial de documentos:

- Fusión determinista y con prioridad sin conexión de ediciones concurrentes, sin requerir un
  coordinador central.
- **Historial de deshacer/rehacer y linaje de versiones** de primera clase como capacidad del
  producto, no como agregado.
- Un registro de cambios que se prevé **anclable al libro mayor Merkle con raíz en el cliente**
  (afirmaciones de Doctrina #33/#34), de modo que el historial del documento sea verificable.
- Opera sobre el modelo canónico de [`moonshot-docengine`](../moonshot-docengine).

El horizonte de reemplazo es **de adentro hacia afuera**: comenzar sobre una base de Rust
verificada y bifurcable, y migrar a este motor a medida que madura, retirando la dependencia externa.

---
*© 2026 PointSav Digital Systems.*
*Plano Arquitectónico Público. Regido por el Protocolo de Datos Soberano.*
