---
artifact: topic
schema: foundry-draft-v1
title: "Sustrato Unikernel seL4 para os-console"
lang: es
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, SYS-ADR-10, SYS-ADR-19]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, moonshot-toolkit-v0.3.1, seL4-Microkit-2.2.0-manual, vendor-sel4-kernel-v15.0.0-dev]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: sesión de investigación de sustrato radical can-we-make-a-bubbly-quasar
  verification_method: moonshot-toolkit 35 pruebas superadas; seL4 AArch64 arranca hasta espacio de usuario en QEMU
---

# Sustrato Unikernel seL4 para os-console

os-console está previsto para ejecutarse como una imagen unikernel seL4 Microkit en su
forma de producción final (previsto Fase H2). Este artículo explica qué significa eso,
qué ya funciona y qué queda por construir.

---

## Qué Es un Unikernel

Un unikernel es una aplicación compilada directamente con los primitivos del sistema
operativo que necesita, produciendo un único binario arrancable. No hay sistema operativo
de propósito general, ni shell, ni gestor de paquetes, ni sistema de cuentas de usuario,
ni superficie de ataque más allá del propio código de la aplicación y el kernel mínimo
del que depende.

La distinción respecto a una VM convencional:

| VM Convencional | Unikernel |
|---|---|
| SO invitado (Linux/BSD) + aplicación | Aplicación + kernel mínimo |
| SO de propósito general: shell, usuarios, paquetes | Propósito único: solo una aplicación |
| Superficie de ataque del kernel compartido | Sin kernel compartido; sin autoridad ambiental |
| Huella típica: 500 MB a 2 GB | Huella típica: 10–50 MB |
| Tiempo de arranque: 5–30 segundos | Tiempo de arranque: < 1 segundo |

Un unikernel no puede ser comprometido con escalada de privilegios convencional porque
no hay raíz a la que escalar. No hay shell al que acceder. Solo existe la aplicación y
su conjunto de capacidades formalmente acotado.

---

## seL4 Microkit

seL4 es un microkernel formalmente verificado. El kernel seL4 ha sido verificado mediante
pruebas verificadas por máquina (Isabelle/HOL) que establecen la corrección del modelo de
capacidades del kernel, la gestión de memoria y los mecanismos de IPC.

seL4 Microkit es el entorno operativo mínimo de seL4 para aplicaciones embebidas y
unikernel. Define:

**Dominios de Protección (PDs):** La unidad de aislamiento. Cada PD tiene su propio
espacio de nombres de capacidades y no puede leer ni escribir la memoria de otro PD.
Los PDs se declaran estáticamente en tiempo de compilación en una especificación de
sistema TOML.

**Llamada a Procedimiento Protegido (PPC):** IPC síncrono entre PDs. Un PD invoca un
endpoint PPC. El kernel cambia el contexto de ejecución. El receptor retorna. El llamante
reanuda. No se involucra memoria compartida a menos que se mapee explícitamente con una
capacidad.

---

## Qué Ya Funciona

moonshot-toolkit v0.3.1 (nuestro orquestador de compilación, 35 pruebas superadas) ya
produce imágenes seL4 AArch64 arrancables:

```toml
# examples/hello-world.toml — funciona hoy
[system]
kernel = "vendor-sel4-kernel/build/aarch64-qemu/kernel.elf"
elfloader = "vendor-sel4-tools/elfloader-tool"

[[pd]]
name = "hello-pd"
binary = "examples/hello.elf"
priority = 100
```

Ejecutar `cargo run -- build examples/hello-world.toml` produce un ELF del cargador.
QEMU lo arranca hasta: `Booting all finished, dropped to user space`.

vendor-sel4-kernel (v15.0.0-dev, BSD-2-Clause) está incluido como código fuente propio
en el monorepo y se compila desde el código fuente. vendor-sel4-tools (cargador ELF,
44 fuentes C/ASM) está incluido y compilado por moonshot-toolkit.

El kernel arranca. La infraestructura está en su lugar.

---

## El Diseño de 3 Dominios de Protección para os-console

La imagen seL4 del sistema prevista para os-console contiene tres Dominios de Protección:

```
Imagen del sistema seL4 de os-console
┌─────────────────────────────────────────┐
│ PD os-console          prioridad 100    │
│  Cartridges: F2 F3 F4 F6 F9 F11 F12   │
│  TUI ratatui; sin acceso directo a red │
│  Pila: 256 KiB; montón: 1 MiB          │
└──────────┬──────────────────┬───────────┘
           │ PPC (IPC sínc.)  │ PPC (IPC sínc.)
           ▼                  ▼
┌──────────────┐   ┌────────────────────┐
│ pd-red       │   │ pd-serie           │  prioridad 150/180
│ smoltcp      │   │ serie VirtIO       │
│ VirtIO-net   │   │ salida ratatui     │
│ HTTP/1.1     │   │ entrada teclado    │
└──────────────┘   └────────────────────┘
       ▲
       │ VirtIO-net (capacidad DMA)
       ▼
moonshot-hypervisor → red del host → servicios Totebox
```

El PD os-console realiza una solicitud HTTP llamando al pd-red mediante PPC. El pd-red
posee la capacidad del dispositivo VirtIO-net. Si el PD os-console se ve comprometido,
no puede exfiltrar datos directamente por la red — solo puede llamar al pd-red a través
de su interfaz PPC definida.

---

## moonshot-sel4-vmm: El Runtime de PD Soberano

seL4 Microkit requiere un pequeño runtime de Rust dentro de cada PD. El enfoque
previsto de Foundry es escribir el nuestro propio en moonshot-sel4-vmm.

La ABI de seL4 es pequeña y formalmente verificada — no cambia arbitrariamente porque
las pruebas la restringen. Escribir nuestros propios enlaces (~300 líneas de Rust) lleva
el mismo tiempo que integrar una biblioteca externa, pero deja el código bajo nuestro
control.

moonshot-sel4-vmm está previsto para proporcionar:
- `_start()` → inicialización de pila/montón → `pd_main()`
- Envoltorios de llamadas al sistema seL4 (`sel4_call`, `sel4_send`, `sel4_recv`)
- Tipo IPC `microkit_msginfo_t` conforme a la ABI de Microkit
- Callbacks `notified(ch: u64)` y `protected(ch: u64, msginfo)` según el protocolo Microkit

Esta crate es compartida entre los tres binarios del sistema operativo: PDs de os-console,
PDs de servicio de os-totebox y PDs de app-orchestration-* de os-orchestration.

---

## La Pila "Lo Poseemos Nosotros"

La cadena de dependencias de runtime prevista para os-console como unikernel seL4:

| Capa | Componente | Estado |
|---|---|---|
| Aplicación | Código de cartridges de os-console | Activo; nuestro |
| Orquestador de compilación | moonshot-toolkit v0.3.1 | Activo; nuestro |
| VMM del host | moonshot-hypervisor | Andamiaje; nuestro — a rellenar |
| Runtime de PD | moonshot-sel4-vmm | Andamiaje; nuestro — Fase H1 |
| Sustrato de capacidades | system-core, system-ledger v1.0.0 | Activo; nuestro |
| Kernel | vendor-sel4-kernel v15.0.0-dev | Código fuente propio BSD-2-Clause |
| Cargador ELF | vendor-sel4-tools | Código fuente propio BSD-2-Clause |
| PD de red | smoltcp | MIT; incluible como fuente propia |
| Arranque de desarrollo | QEMU | Solo herramienta de desarrollo |

Nanos (unikernel comercial) y hermit-os (arquitectura de mini-SO externa) no se utilizan.

---

## Hoja de Ruta de Fases

**Fase H0 (actual):** Alpine Linux en QEMU — valida la pila de servicios antes de
invertir en el sustrato seL4.

**Fase H1 (prevista, 4–6 semanas):** Rellenar moonshot-sel4-vmm. Arrancar os-console
como un único PD seL4. Renderizar la TUI mediante serie VirtIO. Portapapeles VirtIO
funcional (imprescindible para operadores de pequeña empresa).

**Fase H2 (prevista, 8–16 semanas):** Diseño completo de 3 PDs. moonshot-hypervisor
reemplaza QEMU. Imagen arrancable en menos de 1 segundo. Pila 100% soberana.

**Fase H3 (prevista, Leapfrog 2030):** El emparejamiento F11 se convierte en acuñación
de capacidades. Tokens de capacidad seL4 a nivel de máquina. Revocación mediante
system-ledger propagada al nivel del kernel.
