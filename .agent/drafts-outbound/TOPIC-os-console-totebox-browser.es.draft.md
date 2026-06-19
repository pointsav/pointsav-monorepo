---
artifact: topic
schema: foundry-draft-v1
title: "os-console: El Navegador de Totebox Orchestration"
lang: es
route: project-editorial
status: draft
created: 2026-06-19
updated: 2026-06-19
brief-id: project-console-os-console-hypervisor
doctrine_anchors: [claim-49, claim-43, claim-34]
research_trail:
  sources: [BRIEF-os-console-hypervisor.md, BRIEF-OS-FAMILY.md, BRIEF-sovereign-os-family-master-plan.md]
  reviewed_by: totebox@project-console
  research_date: 2026-06-19
  session_context: sesión de investigación de sustrato radical can-we-make-a-bubbly-quasar
  verification_method: investigación por agentes + revisión de Cargo.toml de os-console
---

# os-console: El Navegador de Totebox Orchestration

os-console es la superficie de terminal del operador para Totebox Orchestration. Se
ejecuta en la máquina host del operador —un ordenador personal, una estación de trabajo
o un cliente ligero— y presenta una interfaz orientada al teclado para los servicios
que se ejecutan en el Totebox del operador.

La analogía más cercana es un navegador web. La analogía es estructural, no metafórica:
os-console y un navegador resuelven el mismo problema arquitectónico por los mismos medios.

---

## La Analogía del Navegador

| Navegador Web | os-console |
|---|---|
| Renderiza HTML de servidores web | Renderiza vistas TUI de cartridges desde servicios Totebox |
| Pestañas del navegador — procesos de renderizado aislados | Cartridges de teclas F — Dominios de Protección seL4 (previsto Fase H2) |
| Almacén de certificados — identidades de servidor confiables | Emparejamiento de máquina (F11) — máquina host como ancla de confianza |
| HTTP + DNS — protocolo de transporte universal | Protocolo de servicio Totebox — contrato cartridge-servicio |
| Service Workers — caché sin conexión | Caché de cartridge sin conexión (previsto Fase H3) |
| Política de mismo origen — aislamiento de pestañas | Límite de capacidades seL4 entre PDs de cartridge (previsto) |
| La red de tu proveedor de Internet | Tu Totebox — soberano, en las instalaciones del cliente, sin dependencia de la nube |
| Navegador como imagen de arranque | os-console como imagen de VM arrancable (previsto Fase H2) |

La distinción clave respecto a un navegador web: os-console se conecta a hardware bajo
el control físico del operador. El Totebox no es un servicio en la nube. Los datos no
salen de las instalaciones del operador.

---

## Cartridges de Teclas F como Pestañas del Navegador

Una pestaña del navegador es un proceso de renderizado con un origen definido. Puede
acceder a recursos de su origen y no puede leer la memoria de otras pestañas. El
navegador aplica este límite.

En os-console, cada cartridge de tecla F es una aplicación distinta:

| Tecla | Cartridge | Servicio |
|---|---|---|
| F2 | Personas | service-people :9091 |
| F3 | Email | service-email |
| F4 | Contenido | service-content :9081 |
| F6 | Contabilidad | service-bookkeeper |
| F9 | SLM | service-slm :9080 Doorman |
| F11 | Sistema / Emparejamiento | pairing-server |
| F12 | Entrada / Auditoría | service-input |

En el sustrato seL4 Microkit previsto (Fase H2), cada cartridge se ejecuta como un
Dominio de Protección seL4. Un PD no puede leer la memoria de otro PD. El kernel seL4
aplica este límite con la misma garantía con que un navegador aplica el mismo origen:
de forma formal, no por convención.

Un cartridge F9 (SLM) comprometido no puede leer los datos del cartridge de Personas.
No porque haya un firewall entre ellos — sino porque no existe ninguna ruta de capacidad
seL4.

---

## Emparejamiento de Máquina como Almacén de Certificados

El almacén de certificados de un navegador establece qué servidores son de confianza.
El emparejamiento de máquina (F11) es el equivalente en Totebox. El SystemCartridge F11
presenta un código QR. El operador del Totebox lo escanea. El Totebox emite tokens de
capacidad para los servicios de cartridge autorizados. Desde ese momento, la máquina
host está autorizada — no la cuenta de usuario, la máquina.

La revocación funciona de manera inmediata: eliminar un emparejamiento de máquina en el
Totebox se propaga de inmediato. La siguiente solicitud de esa instancia os-console
devuelve un fallo de autorización. No hay período de gracia.

---

## Más Allá del Portapapeles: Interacción Estructurada

Un navegador web no utiliza el portapapeles para interactuar con un sitio web. Envías
un formulario. Adjuntas un archivo. Arrastras un componente. La interacción es
estructurada.

os-console está diseñado siguiendo el mismo modelo. El portapapeles (Cmd+V / Ctrl+V
mediante arboard, previsto Fase H1) es la base. La superficie de interacción
estructurada prevista es VirtIO-fs:

1. La máquina host del operador tiene una **Carpeta de Seguimiento del Totebox** — un
   directorio local montado en la VM os-console mediante VirtIO-fs.
2. El operador deposita un archivo en la Carpeta de Seguimiento.
3. El cartridge apropiado (Corrector, Contenido, Entrada) detecta el archivo, lo lee
   como entrada estructurada y lo presenta como un envío de formulario en lugar de
   texto pegado sin formato.
4. El archivo nunca se transmite como bytes a través del portapapeles. Se direcciona
   como una entrada restringida por capacidades a un formulario de cartridge específico.

---

## Un Totebox o Varios: os-orchestration como Múltiples Sitios Web

Un navegador puede mostrar contenido de múltiples sitios web simultáneamente.
os-orchestration es el equivalente Totebox de un agregador multisitio.

Cuando la organización de un operador gestiona múltiples Toteboxes —uno por oficina, o
uno por entidad jurídica— os-orchestration los federa. La sesión os-console del operador
puede mostrar vistas de cartridge de múltiples Toteboxes sin cambiar de conexión. Cada
Totebox es un archivo de datos independiente con su propio espacio de nombres de
capacidades. os-orchestration posee tokens de capacidad derivados de cada uno.

---

## El Navegador Arrancable

La forma final prevista de os-console (previsto Fase H2) es una imagen de VM arrancable.
El operador la arranca — en una máquina dedicada, dentro de su sistema operativo
existente como VM, o como un dispositivo virtual. La imagen contiene el kernel seL4
Microkit, los PDs de cartridge de os-console, los controladores VirtIO y nada más. No
hay sistema operativo de propósito general por debajo. Sin shell. Sin gestor de
paquetes. Sin superficie de ataque que el operador no haya aprobado.

Para el operador de pequeña empresa sin departamento de TI: descarga la imagen, ejecútala,
escanea el código QR. El navegador Totebox está en funcionamiento.

---

## Experiencia del Operador No Técnico

La intención de diseño para os-console es que no requiera conocimientos técnicos:

1. **Arranque**: la imagen se lanza (en hardware o como VM); la TUI aparece en segundos.
2. **Descubrimiento**: os-console detecta automáticamente los Toteboxes en la red local.
3. **Emparejamiento**: F11 muestra un código QR; el administrador del Totebox lo escanea;
   los cartridges se activan.
4. **Uso**: las teclas F navegan entre Personas, Email, Contenido, Contabilidad, SLM.
5. **Pegar**: Cmd+V / Ctrl+V funciona en cualquier cartridge. Deposita archivos en la
   Carpeta de Seguimiento.
6. **Sin configuración**: sin direcciones IP que escribir, sin puertos que recordar, sin
   claves SSH que gestionar.
