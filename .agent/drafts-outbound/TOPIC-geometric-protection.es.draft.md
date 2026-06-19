---
artifact: topic
schema: foundry-draft-v1
title: "Protección Geométrica: Autorización por Capacidades seL4 en Totebox Orchestration"
lang: es
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-34, claim-43, claim-49, SYS-ADR-10]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, BRIEF-OS-FAMILY.md, system-core-v1.0.0, system-ledger-v1.0.0]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: sesión de investigación de sustrato radical can-we-make-a-bubbly-quasar
  verification_method: investigación por agentes + revisión de código fuente system-core/system-ledger
---

# Protección Geométrica: Autorización por Capacidades seL4 en Totebox Orchestration

**Protección Geométrica™** es un término de PointSav para la aplicación del modelo de
capacidades de seL4 a la autorización de Totebox. No es el nombre de una funcionalidad
de producto ni una afirmación de marketing. Describe un enfoque matemáticamente distinto
al control de acceso que cambia la estructura de la autorización en lugar de añadir
capas de seguridad a un modelo existente.

---

## Por Qué Falla la Seguridad por Capas

La respuesta estándar ante una brecha de seguridad es añadir otra capa.

```
Firewall → WAF → IAM → VPN → TLS → 2FA → SIEM → EDR → CASB → Zero Trust
```

Cada nueva capa es una nueva superficie de ataque. Un adversario que aprende a operar
dentro de la capa más reciente puede alcanzar lo que esa capa intentaba proteger. Más
importante aún, cada capa hace la misma suposición fundamental: **autentícate y tendrás
acceso**. El modelo de acceso —un sujeto que presenta credenciales para acceder a un
objeto— no cambia. La geometría sigue siendo la misma. Un adversario determinado está
aprendiendo el laberinto, no perdiendo la capacidad de recorrerlo.

La seguridad por capas aumenta el costo y el tiempo de un ataque. No cambia la
estructura lógica de la superficie de ataque.

---

## El Modelo de Capacidades de seL4: Autorización como un DAG Formal

El microkernel seL4 implementa un modelo diferente. El acceso a cualquier recurso
—una región de memoria, un endpoint de red, un dispositivo de almacenamiento— requiere
poseer un token de capacidad infalsificable para ese recurso. No una credencial. No un
token de sesión. Un objeto matemático emitido por el propio kernel.

El conjunto de todas las capacidades en un sistema seL4 en ejecución forma un **grafo
dirigido acíclico (DAG)**: cada arista es una capacidad; cada nodo es un recurso o un
contenedor de capacidades (CNode). Para acceder a un recurso, un proceso debe poseer
una capacidad que forme una ruta hacia él en este grafo. No existe otra ruta.

Propiedades clave de este modelo:

**Infalsificabilidad.** Una capacidad no puede construirse a partir de bits aleatorios.
Es un objeto del kernel. Un proceso no puede adivinar una capacidad que no le fue otorgada.

**Sin autoridad ambiental.** En un sistema operativo convencional, un proceso root puede
acceder a cualquier recurso. En seL4, incluso un proceso privilegiado solo puede acceder
a los recursos para los que posee capacidades explícitas. No existe un "root" que eluda
el grafo.

**Prueba formal.** El kernel seL4 ha sido verificado formalmente mediante el asistente
de pruebas Isabelle/HOL. Las pruebas establecen que el modelo de capacidades es
correctamente aplicado por la MMU del hardware en todo momento. Esto no es una aserción
de ingeniería — es una prueba matemática verificada por máquina.

**La revocación se propaga.** Revocar una capacidad elimina una arista del grafo. La
prueba garantiza que esta propagación es completa: ninguna capacidad derivada permanece
utilizable después de la revocación.

---

## Protección Geométrica Definida

Protección Geométrica es la condición en la que el modelo de autorización es un DAG
formalmente probado y acotado de tokens infalsificables, en lugar de una lista de
control de acceso mutable verificada en tiempo de ejecución.

En la seguridad convencional, la "geometría" del acceso —qué sujetos pueden alcanzar
qué objetos— es un estado mutable en tiempo de ejecución. Un adversario que corrompe
el estado cambia la geometría. En el modelo de capacidades de seL4, la geometría es
un invariante aplicado por el kernel. La prueba de corrección del kernel significa que
la geometría no es mutable por software que se ejecute por debajo del límite del kernel.

Un adversario que compromete completamente un solo Dominio de Protección solo puede
acceder a lo que las pruebas de seL4 establecen que ese PD puede alcanzar. La topología
de acceso es un objeto matemático, no una política aplicada por software susceptible de
subversión.

---

## Implementación en PointSav: system-core y system-ledger

El sustrato de capacidades en Totebox Orchestration está implementado en dos crates de Rust:

**system-core v1.0.0** define el sistema de tipos de capacidades:

```rust
pub struct Capability {
    pub cap_type: CapabilityType,   // Endpoint | Memory | Irq | Notification | CNode
    pub rights: Vec<Right>,          // Read | Write | Invoke | Grant | Revoke
    pub expiry_t: Option<u64>,
    pub witness_pubkey: Option<String>,
    pub ledger_anchor: LedgerAnchor,
}
```

Una `Capability` no es un token de sesión. Es un objeto tipado, acotado por derechos
y opcionalmente limitado en el tiempo, anclado al registro de auditoría WORM.

**system-ledger v1.0.0** proporciona la función de veredicto:

```rust
pub enum Verdict {
    Allow,
    Refuse(RefuseReason),
    ExtendThenAllow { new_expiry_t: u64 },
}
```

`consult_capability()` sobre `InMemoryLedger` evalúa una invocación de capacidad contra
el estado actual del registro y devuelve un `Verdict`. El registro es de solo adición
(WORM) y está anclado mediante cadenas de prueba Merkle conforme a RFC 9162.

---

## Emparejamiento de Máquinas como Ceremonia de Acuñación de Capacidades

El emparejamiento de máquinas F11 en os-console es la ceremonia de acuñación de
capacidades prevista para el acceso a Totebox (planificado; Fase H3 de la hoja de ruta
del sustrato os-console):

1. La autoridad de emparejamiento del Totebox posee un `CapabilityType::CNode` — la
   raíz de su espacio de nombres de capacidades.
2. Cuando una máquina host se empareja mediante F11, la autoridad de emparejamiento
   deriva y otorga tokens `CapabilityType::Endpoint` para cada servicio de cartridge
   autorizado.
3. La instancia os-console de la máquina host posee estos tokens.
4. En cualquier momento, el operador del Totebox revoca un token. La llamada a
   `apply_revocation()` en `system-ledger` propaga la revocación. El siguiente intento
   de IPC desde esa instancia os-console devuelve `Verdict::Refuse`.

La máquina host está autorizada. No la cuenta de usuario — la máquina.

---

## Contraste con IAM / ACL Convencionales

| IAM / ACL Convencional | Protección Geométrica (seL4) |
|---|---|
| El sujeto presenta credenciales | El sujeto posee un token de capacidad infalsificable |
| Credencial verificada contra política en tiempo de ejecución | El kernel aplica la propiedad de capacidades; sin política en tiempo de ejecución |
| La política es estado mutable | El grafo de capacidades es un invariante del kernel |
| Escalada posible si el estado de política está corrupto | Sin ruta de escalada sin una arista de capacidad |
| La revocación requiere propagación de política (puede haber retraso) | La revocación elimina la arista a nivel de kernel; la prueba garantiza la propagación |
| Añadir seguridad = añadir capas de política | Añadir seguridad = eliminar aristas de capacidades |

---

## Alineación con Leapfrog 2030

Protección Geométrica es el modelo de seguridad de Totebox previsto para estar en
producción en el hito Leapfrog 2030. El sustrato seL4 Microkit (Doctrina, cláusula #34,
Sustrato Soberano de Dos Fondos) proporciona la capa del kernel. system-core y
system-ledger proporcionan el sustrato de capacidades en Rust. La arquitectura de tres
binarios (os-console, os-totebox, os-orchestration) implementa Protección Geométrica en
cada capa: por cartridge en os-console, por servicio en os-totebox, y mediante un PD
intermediario de capacidades en os-orchestration que posee las capacidades de endpoints
entre Toteboxes.
