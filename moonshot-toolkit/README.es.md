<div align="center">

# 🚀 MOONSHOT-TOOLKIT

[ 🇬🇧 Read this document in English ](./README.md)

</div>

---

## Resumen

`moonshot-toolkit` es el orquestador de construcción exclusivo en
Rust que reemplaza la cadena de herramientas Microkit (Python +
CMake) según MEMO §7 ("Microkit (Python/CMake) → moonshot-toolkit
(Rust-Only Toolchain)"). Lee un `system-spec.toml` nativo en Rust
(equivalente del XML de descripción de sistema de Microkit),
genera un `BuildPlan` determinista direccionado por contenido, y
orquesta la compilación de seL4 + system-* + os-* para producir
una imagen arrancable.

Fundamental para la Fase 1B: sin `moonshot-toolkit`, ningún otro
proyecto `moonshot-*` se entrega.

**Estado:** Activo (Fase 1B — orquestador de construcción
exclusivo en Rust).

**Anclas constitucionales:** Afirmaciones #33 y #34 de
DOCTRINE.md. **Especificación operativa:**
`conventions/system-substrate-doctrine.md` §6
(Verificación-Reproducible-Sobre-Metal-del-Cliente).

La documentación técnica detallada — capas de módulos, esquema
del CLI, instrucciones de compilación y prueba — vive en el
documento canónico en inglés ([README.md](./README.md)) por
convención bilingüe del espacio de trabajo (`~/Foundry/CLAUDE.md`
§6: el español es panorámica estratégica, no traducción literal).
