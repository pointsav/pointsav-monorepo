---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-system
target_repo: vendor/content-wiki-documentation
target_path: ./
target_filename: topic-sel4-aarch64-qemu-substrate-target.es.md
audience: vendor-internal
bcsc_class: no-disclosure-implication
language_protocol: TRANSLATE-ES
authored: 2026-05-29T00:00:00Z
authored_by: task-project-system (session phase-1c-d)
authored_with: claude-sonnet-4-6
references:
  - topic-sel4-aarch64-qemu-substrate-target.md (canónico en inglés)
notes_for_editor: |
  Panorama estratégico per DOCTRINE.md §XII: adaptación, no traducción 1:1.
  El objetivo es que un lector en español pueda entender el alcance del TOPIC
  en inglés y decidir si leerlo completo.

  Vocabulario vigilado: "soberano" (descriptivo) es término prohibido; usar
  "bajo control del cliente" o "del cliente". "Blockchain" está prohibido.
  seL4 es un microkernel de terceros verificado formalmente; Foundry construye
  sobre él, no lo posee. Aplicar postura "Honest We Own It" en el resumen.
---

# El Objetivo de Sustrato seL4 AArch64 en QEMU

Las imágenes de unikernel de Foundry se ejecutan sobre el microkernel seL4
dirigido a la arquitectura de conjunto de instrucciones AArch64, con el modelo
de máquina `virt` de QEMU como entorno de emulación principal para el desarrollo,
las pruebas y la integración continua. Este objetivo fue seleccionado mediante
las decisiones de arquitectura del Grupo 3A y el Grupo 3D en mayo de 2026, y es
el sustrato de hardware para todo el trabajo de la Phase 1C y la Phase 2.


## Resumen

seL4 es un microkernel de la familia L4 verificado formalmente, desarrollado por
CSIRO's Data61 y mantenido por la Fundación seL4. Su propiedad definitoria es
una prueba verificada mecánicamente de corrección funcional: la implementación en
C del kernel está demostrada contra una especificación formal en Isabelle/HOL. seL4
usa un modelo de control de acceso basado en capacidades: cada recurso del kernel —
memoria, hilos, endpoints de IPC, manejadores de interrupciones — es accesible
únicamente a través de una capacidad, un token tipado inforjable. Este modelo es
la base sobre la cual se construye el Sustrato del Libro de Capacidades (afirmación
doctrinal #33): el libro extiende el control de acceso aplicado por el kernel con
un registro criptográficamente auditable de cada decisión de capacidad.

AArch64 fue seleccionado como objetivo primario por dos razones: el portafolio de
pruebas (seL4 cubre AArch64 como objetivo de primera clase con el historial más
largo de mantenimiento continuo de pruebas) y la trayectoria de hardware
(procesadores de servidor AArch64 de Ampere Altra, AWS Graviton y Neoverse N1/V1
están disponibles en los proveedores de nube relevantes para los objetivos de
despliegue de Foundry).

El modelo de máquina `virt` de QEMU para AArch64 es una plataforma sintética
diseñada específicamente para el desarrollo de software. La CPU por defecto es
Cortex-A53 (`-cpu cortex-a53`). El controlador de interrupciones es GIC versión 2.
La UART es PL011 en dirección física `0x09000000`, a la que seL4 enruta su salida
de depuración cuando `KernelPrinting=ON`. La memoria física se describe como una
ventana de 1 GiB desde `0x40000000` hasta `0x80000000`; QEMU debe lanzarse con
al menos `-m 1G`.


## Secciones del TOPIC en inglés

### §1 — seL4 como base del microkernel

Explica la prueba de corrección funcional verificada mecánicamente de seL4 en
Isabelle/HOL, el modelo de control de acceso basado en capacidades (tokens
inforjables, sin autoridad ambiental, sin usuario root), y la relación con la
afirmación doctrinal #33: el kernel proporciona la aplicación en tiempo real; el
Sustrato del Libro añade la capa de auditoría. Aclara que Foundry no modifica el
kernel seL4 — construye sobre su API publicada y el modelo de dominio de protección
de Microkit.

### §2 — AArch64 primero

Documenta las dos razones de la selección: el portafolio de pruebas (AArch64 es
un objetivo de primera clase con el historial más largo de mantenimiento continuo
en el proyecto seL4; se usa en los propios pipelines de integración continua de la
Fundación seL4) y la trayectoria de hardware (procesadores de servidor AArch64
disponibles en los proveedores de nube relevantes). Aclara que x86-64 no está
excluido — seL4 lo soporta — pero el enfoque en AArch64 primero calibra la
inversión en la cadena de herramientas desde el inicio.

### §3 — El modelo de máquina virt de QEMU

Documenta las características del modelo `virt` relevantes para seL4: CPU
Cortex-A53, controlador GIC v2, UART PL011 a `0x09000000`, árbol de dispositivos
generado desde QEMU en tiempo de configuración del kernel, y ventana de memoria
física de 1 GiB (`-m 1G` requerido). Explica por qué `qemu-system-aarch64` debe
estar presente antes de que el kernel seL4 pueda configurarse: el sistema de
construcción CMake extrae el árbol de dispositivos de la máquina de QEMU en tiempo
de configuración.

### §4 — Configuración de compilación del kernel

Documenta las opciones de configuración CMake críticas para el objetivo QEMU
AArch64: `KernelPlatform=qemu-arm-virt`, `KernelSel4Arch=aarch64`,
`KernelVerificationBuild=OFF` (requerido para que `KernelPrinting=ON` surta efecto —
cuando `KernelVerificationBuild=ON`, el CMake desactiva silenciosamente
`CONFIG_PRINTING`), `KernelPrinting=ON`, `KernelDebugBuild=ON`. Explica que el
kernel resultante (`kernel.elf`) es un ELF estático AArch64 con punto de entrada
`0xffffff8040000000` — la dirección virtual prevista del kernel una vez configurada
la MMU AArch64.

### §5 — El elfloader y la cadena de arranque

Explica por qué el `kernel.elf` de seL4 para AArch64 no puede cargarse
directamente en QEMU: el punto de entrada `0xffffff8040000000` no existe en el
espacio de direcciones físico de la máquina `virt`. Documenta el rol del elfloader
(de `vendor-sel4-tools/elfloader-tool/`): se ejecuta desde la dirección física
`0x40400000`, desempaqueta el kernel y el servidor raíz desde un archivo CPIO
embebido, configura las tablas de páginas de la MMU AArch64 para mapear el espacio
de direcciones virtuales del kernel, y salta al punto de entrada del kernel. Aclara
que, desde Phase 1C.d, el paso AssembleImage de moonshot-toolkit compila y enlaza
el elfloader automáticamente en Rust puro, sin Python, CMake ni scripts de shell.

### §6 — Cadena de arranque de Phase 1C — verificada

Documenta los cuatro hitos completados. Phase 1C.b (2026-05-27): kernel seL4
compilado desde fuente; `kernel.elf` verificado como ELF AArch64 estático, punto
de entrada `0xffffff8040000000`. Phase 1C.c (2026-05-28): arranque completo en
QEMU confirmado; elfloader → kernel seL4 → servidor raíz produce salida serial.
Phase 1C.d (2026-05-29): moonshot-toolkit v0.3.0 automatiza el pipeline completo;
`moonshot-toolkit build` produce `build/system-image.bin` (punto de entrada
`0x40400000`); el comando de arranque es `qemu-system-aarch64 -machine virt,secure=off
-cpu cortex-a53 -m 1G -nographic -kernel build/system-image.bin`; salida verificada:
"Booting all finished, dropped to user space".


---

*(El TOPIC canónico en inglés está en `topic-sel4-aarch64-qemu-substrate-target.md`.
Esta versión en español es un panorama estratégico, no una traducción
palabra-por-palabra, según DOCTRINE.md §XII.)*
