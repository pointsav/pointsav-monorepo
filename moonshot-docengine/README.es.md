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
| 🔴 Investigación | Modelo de Documento y Mapeo Bidireccional | Fundación de Datos Soberana (previsto) |

## 📖 Marcador de Auditoría Arquitectónica

Este directorio es un marcador estructural en el registro **MOONSHOTS / PROYECTOS ESPECIALES**.
Registra la intención arquitectónica de reemplazar bibliotecas de modelo de documento de
terceros con un motor nativo, verificado y orientado a Rust. El plano existe; la implementación
está prevista.

## I. QUÉ REEMPLAZA ESTE MOONSHOT

`moonshot-docengine` apunta a las dependencias de modelo de documento enriquecido de terceros
que el banco de trabajo de otro modo arrastraría: **ProseMirror**, **Lexical** y **TipTap**
(modelos de documento en JavaScript), junto con el analizador y renderizador de markdown hecho
a mano actualmente incrustado en `app-privategit-workbench`. Son extranjeros, atados a
JavaScript, y no pueden alcanzar la superficie nativa de `os-workplace` sin una segunda
implementación.

## II. QUÉ SE ESTÁ CONSTRUYENDO

Un único **motor de documento canónico escrito en Rust y compilado a WebAssembly**:

- Un modelo de documento canónico (árbol de nodos tipados + marcas) como única fuente de verdad.
- **Mapeo bidireccional preciso a nivel de AST** entre una vista renderizada "lo que ves es lo
  que obtienes" y la forma de origen serializada — cada nodo renderizado lleva su rango de
  origen exacto, de modo que un resaltado en el visor se mapea a un tramo de origen preciso y
  viceversa. Esto reemplaza la sincronización actual por coincidencia de texto, que es imprecisa.
- Ida y vuelta determinista (render → origen → render) sin deriva.
- Un motor compartido por dos interfaces: el prototipo de navegador hoy, la superficie nativa
  `os-workplace` mañana — se prevé que el motor permanezca sin cambios en ambos.

---
*© 2026 PointSav Digital Systems.*
*Plano Arquitectónico Público. Regido por el Protocolo de Datos Soberano.*
