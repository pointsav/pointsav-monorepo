<div align="center">

# 🏗️ SYSTEM-LEDGER

[ 🇬🇧 Read this document in English ](./README.md)

</div>

---

## Resumen

`system-ledger` es el **consumidor-máquina-de-estados** de capa
sustrato para las primitivas del Sustrato del Libro Mayor de
Capacidades definidas en `system-core`. Posee la lógica de decisión
del lado del kernel que determina si honrar una invocación de
capacidad: caché de checkpoints, conjunto de capacidades revocadas,
historia del ápice con invariante post-traspaso, verificación de
firmas de registros-testigo.

Creado el 27-04-2026 por directiva de Master Claude (resolución
Opción B). Refleja el patrón `service-fs` de
`worm-ledger-design.md`: mismo formato C2SP signed-note,
desacoplado por capa.

**Estado:** Activo — esqueleto en su lugar; implementaciones de
módulo aterrizan en commits subsiguientes según la lista de tareas
del clúster.

**Ancla constitucional:** Afirmación #33 de DOCTRINE.md.
**Especificación operativa:**
`conventions/system-substrate-doctrine.md` §3.1 + §4 + §5.

La documentación técnica detallada — capas de módulos, esquema de
flujo de decisión, instrucciones de compilación y prueba — vive en
el documento canónico en inglés ([README.md](./README.md)) por
convención bilingüe del espacio de trabajo (`~/Foundry/CLAUDE.md`
§6: el español es panorámica estratégica, no traducción literal).
