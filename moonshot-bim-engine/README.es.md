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
| 🔴 Investigación | Analizador IFC y Motor de Render BIM | Fundación de Datos Soberana (previsto) |

## 📖 Marcador de Auditoría Arquitectónica

Este directorio es un marcador estructural en el registro **MOONSHOTS / PROYECTOS ESPECIALES**.
Registra la intención arquitectónica de reemplazar la infraestructura de renderizado BIM de
terceros con un motor nativo, verificado y orientado a Rust. El plano existe; la implementación
está prevista.

## I. QUÉ REEMPLAZA ESTE MOONSHOT

`moonshot-bim-engine` apunta a la pila de Modelado de Información de Construcción de terceros de
la que el banco de trabajo de otro modo dependería: **web-ifc** (analizador IFC con licencia MPL)
y **xeokit** (visor BIM con licencia comercial). Esta dependencia es la barrera de licenciamiento
registrada contra la superficie `app-workplace-bim` en el registro de arquitectura — poseerla
elimina la barrera.

## II. QUÉ SE ESTÁ CONSTRUYENDO

Un **analizador IFC y motor de geometría/render BIM escrito en Rust** (compilable a WebAssembly):

- Un **analizador IFC (ISO 16739-1:2024)** para el formato de modelo de construcción canónico y
  abierto — el registro fiduciario es plano, estándar y nunca un binario propietario.
- Una tubería de geometría y render para modelos BIM que funciona sin un visor de terceros.
- El elemento de mayor complejidad de esta iniciativa; escalonado detrás de los motores
  centrados en documentos.

> [!NOTE]
> Este es el moonshot más ambicioso del conjunto del banco de trabajo y conlleva el mayor riesgo
> de cronograma. Puede usarse una vía intermedia verificada para demostraciones tempranas
> mientras este motor madura; el objetivo sigue siendo la propiedad total y el retiro de la
> dependencia BIM externa.

---
*© 2026 PointSav Digital Systems.*
*Plano Arquitectónico Público. Regido por el Protocolo de Datos Soberano.*
