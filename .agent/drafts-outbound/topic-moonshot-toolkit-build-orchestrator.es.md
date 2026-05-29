---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-moonshot-toolkit-build-orchestrator.es.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: TRANSLATE-ES
authored: 2026-05-29T00:00:00Z
authored_by: task-project-system (session phase-1c-d)
authored_with: claude-sonnet-4-6
references:
  - topic-moonshot-toolkit-build-orchestrator.md (canónico en inglés)
notes_for_editor: |
  Panorama estratégico per DOCTRINE.md §XII: adaptación, no traducción 1:1.
  El objetivo es que un lector en español pueda entender el alcance del TOPIC
  en inglés y decidir si leerlo completo.

  Vocabulario vigilado: "soberano" (descriptivo) es término prohibido; usar
  "bajo control del cliente" o "del cliente" en su lugar. "Blockchain" está
  prohibido — moonshot-toolkit produce imágenes de arranque, no registros
  distribuidos. "Determinista" y "auditabilidad" son términos correctos.
---

# El Orquestador de Construcción moonshot-toolkit

moonshot-toolkit es la herramienta de compilación escrita íntegramente en Rust
que produce las imágenes de arranque de los sistemas seL4 de Foundry. Lee una
especificación de sistema en formato TOML, genera un manifiesto de construcción
con contenido direccionado, y orquesta todo el proceso — desde la compilación de
los dominios de protección hasta el ensamblaje del binario de arranque — sin
Python, CMake ni scripts de shell en la cadena crítica.


## Resumen

El ecosistema seL4 Microkit incluye un script Python de ensamblaje de imágenes
y un sistema de construcción en CMake. Esas herramientas son adecuadas para el
desarrollo embebido general, pero presentan tres problemas para una disciplina de
construcción reproducible: el determinismo está sujeto a variaciones de versión e
implementación; la superficie de auditoría es multi-lenguaje y difícil de razonar
formalmente; y las herramientas requieren acceso a la red durante la ejecución.
moonshot-toolkit reemplaza esa pila con un único binario Rust compilado desde
dependencias incluidas en el repositorio.

El manifiesto que genera moonshot-toolkit — el `plan_hash` — es el punto exacto
al que se adjunta la cofirma del apex del cliente (Sigstore Cosign con la clave
apex según `system-substrate-doctrine.md §6.1`). Un cliente que recibe un binario
puede reconstruir el plan a partir de la especificación y verificar que el
`plan_hash` coincide con el valor cofirmado, y opcionalmente reconstruir el
binario byte-a-byte en su propia infraestructura.

Phase 1C complete (moonshot-toolkit v0.3.0, 2026-05-29): el comando completo
de construcción — desde especificación TOML hasta imagen de arranque seL4 en QEMU
— se ejecuta en una sola invocación de `cargo run`, sin dependencias externas en
tiempo de ejecución más allá del compilador cruzado de sistema.


## Secciones del TOPIC en inglés

### §1 — Por qué solo Rust

Explica los tres problemas de la pila Python/CMake de Microkit: determinismo
(el ordenamiento de diccionarios de Python es definido por implementación; el
descubrimiento de dependencias de CMake varía entre versiones), auditabilidad
(una pila multi-lenguaje compone una superficie de auditoría difícil de razonar
formalmente), y aislamiento de red (los comandos `pip install` y `find_package`
de CMake son superficies de red activas). Explica cómo un binario Rust con
dependencias incluidas en el repositorio elimina los tres.

### §2 — SystemSpec — La entrada

Documenta el formato de especificación TOML: dominios de protección (nombre,
ruta de código fuente, prioridad de planificación, tamaño de pila), canales
(enlaces de comunicación punto-a-punto entre dominios), regiones de memoria
(mapeos físicos con atributos de caché y permisos, opcionalmente precargadas
desde un blob binario), y entrega de IRQ (enlace de líneas de interrupción a
dominios específicos). Describe los invariantes que se aplican durante el
análisis: sin nombres de dominio duplicados, todos los extremos de canal y
destinos de IRQ deben referenciar dominios declarados, las regiones de memoria
no deben superponerse.

### §3 — BuildPlan — El manifiesto

Explica el campo `spec_hash` (SHA-256 del TOML canónico de la SystemSpec), la
lista ordenada de pasos (un paso CompilePd por dominio de protección, seguido de
un único paso AssembleImage), y el `plan_hash` (SHA-256 del JSON canónico de
`(spec_hash, pasos)`). Describe la garantía de determinismo: la misma
especificación produce siempre el mismo `plan_hash`. Explica que el `plan_hash`
es el valor al que se adjunta la cofirma del apex del cliente, enlazando el
binario entregado a la especificación que lo produjo.

### §4 — Comandos de construcción

Documenta los dos tipos de paso. CompilePd invoca `aarch64-linux-gnu-gcc` con
los indicadores apropiados para dominios de protección seL4: sin biblioteca C
estándar (`-nostdlib -nostartfiles`), sin suposiciones de entorno alojado
(`-ffreestanding`), binario estático sin código independiente de posición
(`-static -no-pie`), ISA AArch64 (`-march=armv8-a`), sin registros FPU/SIMD
(`-mgeneral-regs-only`). AssembleImage ejecuta en cinco etapas en Rust puro:
genera el archivo CPIO con un escritor propio (`src/cpio.rs`); embebe el archivo
en un stub de ensamblador con `.incbin`; compila las 44 fuentes C y ASM del
elfloader de seL4 junto con la biblioteca libcpio auxiliar; enlaza todos los
objetos con `-nostdlib -static -lgcc`; copia el binario resultante a la ruta de
salida.

### §5 — Reproducibilidad y cofirma

Describe el flujo de verificación del cliente: reconstruir la SystemSpec desde
el TOML entregado; ejecutar `moonshot-toolkit plan` para derivar el BuildPlan;
comparar el `plan_hash` calculado contra el valor en el manifiesto cofirmado del
proveedor; opcionalmente reejecutar `moonshot-toolkit build` en la propia
infraestructura del cliente para verificar el binario byte-a-byte. Enlaza con
la propiedad de la afirmación doctrinal #33: decisiones de control de acceso
criptográficamente auditables, ancladas en registros que controla el cliente.

### §6 — Estado de Phase 1C

Documenta los cuatro hitos completados. Phase 1C.a (v0.2.0, 2026-05-27): CompilePd
invoca el compilador cruzado real y produce un ELF verificado (`build/hello.elf`,
punto de entrada `0x40010c`). Phase 1C.b (2026-05-27): kernel seL4 AArch64
compilado desde fuente con `KernelPlatform=qemu-arm-virt`. Phase 1C.c
(2026-05-28): arranque completo en QEMU confirmado con salida del kernel visible.
Phase 1C.d (v0.3.0, 2026-05-29): AssembleImage completamente implementado en Rust;
`moonshot-toolkit build` produce `build/system-image.bin` que arranca en QEMU
con `-m 1G -kernel build/system-image.bin`.


---

*(El TOPIC canónico en inglés está en `topic-moonshot-toolkit-build-orchestrator.md`.
Esta versión en español es un panorama estratégico, no una traducción
palabra-por-palabra, según DOCTRINE.md §XII.)*
