---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: infrastructure/
target_filename: sovereign-mesh.es.md
audience: operadores técnicos e ingenieros que trabajan en el despliegue y administración de PPN
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - infrastructure/sovereign-mesh.md
  - systems/infrastructure-os.md
  - systems/os-network-admin.md
  - architecture/diode-standard.md
  - architecture/machine-based-auth.md
notes_for_editor: >
  Traducción paralela del borrador en inglés topic-sovereign-mesh.draft.md.
  Mismo contenido, mismas secciones, mismo orden. Los slugs de citas y las
  referencias cruzadas se mantienen en inglés (nombres canónicos del sistema).
  Los términos de producto (Sovereign Mesh, Genesis Protocol, Diode Standard,
  F8 Terminal, Noise Protocol, WireGuard) no se traducen.
  Frontmatter del artículo al hacer commit: title "Malla Soberana",
  category "infrastructure", status "active", quality "review",
  cites [infrastructure-os, os-network-admin, diode-standard, machine-based-auth].
research_done_count: 5
research_suggested_count: 2
open_questions_count: 2
research_provenance: >
  Derivado del borrador en inglés (topic-sovereign-mesh.draft.md), a su vez
  basado en cinco documentos fuente leídos directamente del árbol de trabajo:
  (1) infrastructure/sovereign-mesh.md, (2) systems/infrastructure-os.md,
  (3) systems/os-network-admin.md, (4) architecture/diode-standard.md,
  (5) architecture/machine-based-auth.md.
  No se realizó investigación adicional para la traducción.
research_inline: true
---

# Malla Soberana

La **malla soberana** es la capa de red a nivel de aplicación que conecta todos los nodos de la flota de la Red Privada PointSav (PPN). Funciona sobre túneles criptográficos WireGuard a través de una interfaz `ppn0` dedicada y entrega comandos binarios firmados sin depender de un intermediario de mensajes centralizado. Cada nodo se comunica directamente con sus pares autorizados; la capa de malla aplica la misma jerarquía de autoridad que el [[Diode Standard]] como propiedad estructural, no como opción de configuración.

## Topología en hub y radios

La malla utiliza una disposición de hub central con radios. El nodo de retransmisión en la nube se sitúa en el centro y retransmite paquetes entre nodos radio que pueden no tener una ruta directa entre sí.

| Rol | Nodo | Dirección prevista | Paquete |
|---|---|---|---|
| Hub | Retransmisión en la nube (GCP) | `10.50.0.1` | `app-infrastructure-cloud` |
| Radio | Nodo en instalaciones propias | `10.50.0.2` | `app-infrastructure-onprem` |
| Radio | Nodo arrendado | `10.50.0.3` | `app-infrastructure-leased` |

La subred `10.50.0.0/24` es el rango de direcciones previsto para la PPN. Todo el tráfico de la malla queda encapsulado dentro de WireGuard antes de salir de un nodo; el transporte subyacente — internet público, LAN privada o red interna de GCP — es irrelevante para la capa de malla.

## Superposición WireGuard

Cada nodo levanta una interfaz WireGuard `ppn0` como parte de su secuencia de arranque. WireGuard proporciona:

- **Acuerdo de claves** — intercambio Noise Protocol IK; el par de claves a largo plazo de cada nodo es generado y almacenado en el primer ingreso a la malla por `os-network-admin` en el nodo de plano de control, o a través del Genesis Protocol en nodos de borde bare-metal
- **Cifrado e integridad** — ChaCha20-Poly1305 por paquete; ningún tráfico de malla en texto plano abandona nunca un nodo
- **Alcanzabilidad entre pares** — el retransmisor en la nube es el único par con dirección estática; los nodos en instalaciones propias y arrendados se localizan entre sí a través del retransmisor hasta que se disponga de una ruta directa

La configuración WireGuard de cada nodo se almacena en `~/Foundry/deployments/<instancia>/` (local únicamente, excluido de git). Los pares de claves nunca se guardan en ningún repositorio.

## Protocolo de comandos

Todos los comandos de la malla utilizan un formato de paquete binario de 16 bytes entregado por UDP en el puerto 8090. El tamaño reducido es deliberado: el paquete contiene un token de intención, un selector de destino, un nonce y una firma de autoridad truncada — suficiente para identificar el comando, verificar su procedencia y detectar ataques de repetición sin necesitar una sesión TLS completa por cada comando.

El flujo de comandos desde el operador hasta el nodo de destino es:

```
Intención del operador (lenguaje natural)
      ↓
Terminal F8  —  os-network-admin  HTTP :8085
      ↓
Enrutador semántico service-slm
      ↓
Comando binario de 16 bytes (autorizado y firmado)
      ↓
Difusión service-udp  →  ppn0  →  Túnel WireGuard
      ↓
Nodo destino  —  UDP puerto 8090
```

Los comandos fluyen en una sola dirección — desde `os-network-admin` hacia la malla, nunca en sentido inverso — restricción aplicada por `service-pointsav-link` en la capa de aplicación. Véase [[Diode Standard]] para la jerarquía de autoridad completa.

## Roles de los nodos en la malla

### os-infrastructure — ancla de borde

El nodo bare-metal `os-infrastructure` es un par de la malla, no un controlador. Escucha en el puerto 8090 los comandos binarios firmados dirigidos a él y los ejecuta; no inicia comandos. La tarjeta de red Broadcom 14e4:16b4 del nodo transporta el tráfico de la malla a través de la interfaz `ppn0` una vez que concluye la secuencia de ingreso del Genesis Protocol.

### os-network-admin — plano de control

`os-network-admin` posee la autoridad de comandos sobre la malla. La Terminal F8 — una interfaz de comandos en lenguaje natural en el puerto HTTP 8085 — acepta la intención del operador y la enruta a través de `service-slm` para producir un comando binario firmado de 16 bytes. El comando se difunde luego a través de `service-udp` en el puerto 8090 a uno o más pares de la malla. `os-network-admin` también alberga el registro de emparejamiento y gestiona la admisión de nuevos nodos mediante el protocolo [[machine-based auth]].

### Retransmisor en la nube — hub

El nodo de retransmisión en GCP retransmite paquetes encapsulados en WireGuard entre nodos radio. No interpreta comandos de la malla; es únicamente una capa de transporte. La IP pública fija del retransmisor y su configuración WireGuard estática lo convierten en el punto de anclaje que permite a los nodos en instalaciones propias y arrendados localizarse mutuamente sin depender de DNS ni DHCP.

## Integración con el Genesis Protocol

Un nodo bare-metal se incorpora a la malla a través del [[infrastructure-os#genesis-protocol|Genesis Protocol]], no mediante aprovisionamiento WireGuard manual. En el primer arranque:

1. seL4 genera un par de claves sembrado con entropía de fuentes de hardware
2. El nodo entra en modo de arranque ciego — ignorando todo DHCP y DNS — y explora el balizamiento de `os-network-admin` en el puerto 8090
3. Si se encuentra la baliza, `os-network-admin` guía al nodo a través del proceso de ingreso a la malla: registro del par WireGuard, asignación de IP y vinculación del par de claves al registro de emparejamiento
4. Si no se encuentra ninguna baliza dentro de la ventana de exploración, el nodo realiza su auto-génesis: escribe su par de claves en el almacenamiento de variables seguras UEFI y entra en un patrón de espera en el puerto 9443, aguardando una reclamación de administrador

Este mecanismo garantiza que ningún nodo se incorpore nunca a la malla sin un protocolo de autoridad verificado. Los flujos de trabajo manuales con `wg genkey` aplican únicamente durante el aprovisionamiento inicial de la flota; no constituyen la ruta de ingreso en tiempo de ejecución para nodos en producción.

## Relación con el Diode Standard

El [[Diode Standard]] define tres categorías de tráfico en la malla: comandos de autoridad, telemetría y sincronización entre nodos. Las tres fluyen a través de la malla soberana, pero solo los comandos de autoridad utilizan el formato binario de 16 bytes en el puerto 8090. El tráfico de telemetría y sincronización utiliza TCP o UDP encapsulado en WireGuard en otros puertos.

La restricción de unidireccionalidad del Diode Standard — los comandos de autoridad fluyen desde `os-network-admin` hacia los nodos, nunca en sentido inverso — se implementa en la capa de la malla mediante `service-pointsav-link`, un adaptador conectable en caliente que aplica la dirección del flujo sin requerir cambios en la política de WireGuard.

## Véase también

- [[infrastructure-os]] — posiciones de despliegue, secuencia del Genesis Protocol, sustrato NIC Broadcom
- [[os-network-admin]] — Terminal F8, integración con service-slm, propiedad de la política de malla
- [[diode-standard]] — jerarquía de autoridad y definiciones de categorías de tráfico
- [[machine-based-auth]] — gestión de pares de claves Noise Protocol y tipos de emparejamiento

---

## Rastro de investigación

### Realizado

1. Lectura de `infrastructure/sovereign-mesh.md` — confirmado contenido de una sola frase; identificado alcance completo de secciones faltantes
2. Lectura de `systems/infrastructure-os.md` — secuencia de cinco pasos del Genesis Protocol, tres posiciones de despliegue, rol de la NIC Broadcom 14e4:16b4, topología hub-radio
3. Lectura de `systems/os-network-admin.md` — Terminal F8, flujo service-slm → binario 16 bytes → service-udp → puerto 8090, rol de plano de control
4. Lectura de `architecture/diode-standard.md` — tres categorías de tráfico, adaptador service-pointsav-link, mecanismo de aplicación de unidireccionalidad
5. Lectura de `architecture/machine-based-auth.md` — protocolo Noise Protocol IK, cuatro tipos de emparejamiento (ADMIN/INPUT/USER/INTERFACE), propiedad del registro de emparejamiento

### Sugerido (no realizado en esta sesión)

1. Leer el código fuente del paquete `service-vpn/` para confirmar las herramientas WireGuard, la rotación de claves y la secuencia de activación de `ppn0`
2. Leer `os-network-admin/scripts/mesh_status.sh` para verificar que las asignaciones de IP, los nombres de interfaz y las referencias a puertos coinciden con este artículo

### Preguntas abiertas

1. **Subred PPN canónica** — el código de producción tiene codificados de forma fija `10.50.0.1/2/3`; los guías usan `10.x.x.x/24` (rango no especificado). Este artículo usa `10.50.0.0/24` y `.1/.2/.3` como direcciones previstas con lenguaje condicional. El operador debe ratificar antes de publicar este artículo.
2. **Estado de implementación del Genesis Protocol** — los artículos describen el protocolo seL4 completo en cinco pasos; el `os-infrastructure/src/main.rs` actual contiene código de modo monitor EAPOL, no código del Genesis Protocol. Este artículo describe la arquitectura prevista según los artículos. La alineación código/artículo es una tarea separada sujeta a la decisión del operador sobre EAPOL frente a Genesis.
