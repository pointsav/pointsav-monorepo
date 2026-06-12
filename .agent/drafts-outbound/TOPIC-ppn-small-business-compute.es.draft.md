---
artifact: topic-draft
foundry-draft-v1: true
language_protocol: PROSE
route_to: project-editorial
status: draft
created: 2026-06-11
archive: project-infrastructure
research_trail:
  session: totebox@project-infrastructure 2026-06-11
  sources: [BRIEF-totebox-transformation.md §13-§15, live test results 2026-06-11]
  claim_verification: pending editorial review
  bcsc_reviewed: false
  bilingual_pair: TOPIC-ppn-small-business-compute.md
---

# La Red de Plataforma Privada: cómputo agrupado a partir del hardware que ya posee

## Qué es una PPN

Una Red de Plataforma Privada de Pointsav (PPN, por sus siglas en inglés) es una red de cómputo privada y cifrada, ensamblada a partir de máquinas que una empresa posee o arrienda. Cada máquina — una computadora portátil antigua en una oficina, un servidor arrendado en un centro de datos, una máquina virtual en un proveedor de nube — ejecuta la misma capa operativa, os-infrastructure, y se une a la misma malla cifrada. Una vez unidas, las máquinas dejan de ser computadoras individuales y se convierten en nodos de un único conjunto de capacidad de cómputo.

El objetivo del diseño es sencillo de enunciar: las cargas de trabajo que se ejecutan dentro de la red no deberían ser accesibles para nadie fuera de ella — ni el proveedor de nube que aloja uno de los nodos, ni el centro de datos que arrienda otro, ni una persona con acceso físico al hardware. Parte de ese objetivo está en operación hoy; parte está planificada. Este documento es explícito sobre cuál es cuál.

## Tres tipos de nodo, una sola red

Un nodo de la PPN puede ser cualquiera de tres cosas:

1. **Hardware propio (bare metal)** — una máquina física que la empresa posee. En las pruebas de junio de 2026, fueron computadoras portátiles de consumo con varios años de antigüedad, ejecutando Linux convencional.
2. **Servidor arrendado** — un servidor dedicado o privado virtual rentado a un proveedor de alojamiento.
3. **Máquina virtual en la nube** — una instancia en un proveedor de nube pública, como Google Cloud.

Una vez instalado os-infrastructure, los tres son operativamente idénticos. La red no distingue una computadora portátil de una instancia en la nube; cada nodo reporta la misma información — memoria disponible, capacidad de virtualización, una señal periódica de actividad — y cada uno puede alojar cargas de trabajo. Las diferencias que permanecen son hechos físicos: una máquina virtual en la nube es alcanzable desde la internet pública y resulta útil como punto de retransmisión; una computadora portátil detrás de un enrutador doméstico no lo es, y en cambio aporta capacidad.

## Qué significa "recursos agrupados"

Una empresa que utiliza una PPN no asigna trabajo a máquinas específicas. Solicita una máquina virtual a la red, y un controlador de flota decide dónde se ejecuta. La lógica de colocación del controlador es consultiva: examina el estado actual de cada nodo — memoria reportada, disponibilidad de virtualización por hardware (KVM) — y selecciona el nodo mejor capacitado para asumir el trabajo. La solicitud se delega entonces a ese nodo, que crea la máquina virtual localmente e informa el resultado.

El solicitante nunca necesita saber qué máquina física fue elegida. En la prueba de junio de 2026, una solicitud enviada al controlador de flota en un nodo de nube fue colocada en una computadora portátil en otro edificio, porque la portátil tenía la mayor memoria libre y la virtualización por hardware de la que el nodo de nube carecía. El solicitante vio únicamente el registro de una nueva máquina virtual entrando en servicio.

Esto es lo que hace útil al hardware antiguo. Una computadora portátil demasiado lenta para el trabajo de escritorio diario aún tiene memoria y un procesador capaz. Agrupadas detrás de una interfaz única, varias de estas máquinas equivalen a una pequeña nube privada.

## El modelo de aislamiento — actual y previsto

El modelo de aislamiento tiene dos capas, y se encuentran en etapas de madurez distintas.

**Aislamiento de red — en operación hoy.** Todo el tráfico entre nodos viaja por WireGuard, un túnel cifrado a nivel de núcleo y auditado. Un proveedor de nube que aloja un nodo de la PPN ve únicamente paquetes UDP cifrados. No puede leer el contenido del tráfico entre nodos, observar qué cargas de trabajo existen, ni insertarse en la malla.

**Aislamiento del anfitrión — planificado.** El cifrado protege los datos en tránsito, pero el operador de la máquina física puede, en principio, inspeccionar lo que se ejecuta en ella: un proveedor de nube controla su hipervisor, y una persona con acceso físico controla una computadora portátil. La respuesta prevista es el micronúcleo seL4, un núcleo verificado formalmente y diseñado para imponer particiones estrictas entre las cargas de trabajo y el entorno anfitrión. El estado objetivo es que os-infrastructure arranque seL4 como su capa de aislamiento, de modo que el propietario del hardware — incluido un proveedor de nube o de alojamiento — no pueda inspeccionar la memoria de las cargas de trabajo huéspedes. Esta capacidad está planificada y no se ejecuta hoy en hardware físico. Hasta que esté disponible, el aislamiento a nivel de anfitrión descansa en las fronteras convencionales de Linux y QEMU/KVM, y debe asumirse que una parte que controla la máquina física puede acceder a las cargas de trabajo en esa máquina.

## Quién controla la admisión: el papel de os-network-admin

Una malla cifrada es tan confiable como su membresía. os-network-admin es la autoridad de enrutamiento de la PPN: decide qué máquinas pueden unirse a la malla, aprueba o deniega solicitudes de incorporación, y está previsto que administre automáticamente la configuración de pares de WireGuard en todos los nodos. El control de admisión de pares importa porque la amenaza a una red privada rara vez es criptográfica — es una máquina no autorizada aceptada como par. Centralizar la admisión en un componente auditable mantiene deliberada la lista de miembros.

Una pregunta de diseño permanece abierta: ¿dónde debe residir esta autoridad?

- **Modo externo.** os-network-admin se ejecuta en un nodo fuera de la PPN, típicamente una máquina en la nube, y administra la tabla de pares desde el exterior. Esto es más sencillo de iniciar, al costo de depender de una autoridad que la propia PPN no contiene.
- **Modo interno.** os-network-admin se ejecuta como la primera máquina virtual de la PPN, dentro de una partición de aislamiento en el nodo fundador — bajo la capa seL4 planificada, una partición a la que se prevé que el propietario del hardware no pueda acceder. La red gobernaría entonces su propia membresía sin dependencia externa. Esto es más difícil de iniciar, porque el protocolo fundacional debe designar al primer nodo como autoridad inicial.

Ambos modos se consideran válidos para perfiles de despliegue distintos, y la plataforma se está diseñando para permitir cualquiera de los dos.

## La economía para una pequeña empresa

La estructura de costos de una PPN difiere de rentar capacidad en la nube. La mayor parte de la red es hardware que la empresa ya posee: computadoras portátiles y de escritorio retiradas aportan cómputo real a un costo marginal cercano a cero más allá de la electricidad. El único costo recurrente en la configuración de referencia es una pequeña máquina virtual en la nube que actúa como punto de retransmisión públicamente alcanzable — del orden de US$15–20 mensuales a precios actuales. Todo lo demás — la capa operativa, la malla, el controlador de flota — se ejecuta en equipos propios.

Para una pequeña empresa, la propuesta práctica es esta: tres o cuatro máquinas que de otro modo serían recicladas pueden ensamblarse, con una instalación sencilla en cada una, en una red privada que aprovisiona máquinas virtuales bajo demanda. La intención es que esta red sea una que solo la empresa controle.

## Qué demostró la prueba de junio de 2026

En junio de 2026, se ensambló y ejercitó en vivo una PPN de tres nodos:

- Una máquina virtual en Google Cloud, una MacBook Pro con Linux y una MacBook Air con Linux formaron una malla WireGuard operativa, con las portátiles aportando la virtualización por hardware de la que el nodo de nube carecía.
- El controlador de flota realizó colocación consultiva a través de la malla: ante una solicitud de máquina virtual, seleccionó la portátil con mayor memoria disponible y virtualización por hardware, y delegó la creación a través de la frontera entre nodos. La portátil aceptó y creó la máquina virtual.
- La creación de máquinas virtuales se verificó de extremo a extremo: discos de copia-en-escritura respaldados por una imagen de nube estándar de Ubuntu, configuración automatizada de primer arranque y lanzamiento de QEMU, con dos máquinas virtuales confirmadas en ejecución simultánea en el nodo de nube.
- Convertir una computadora portátil de consumo en un nodo de la PPN requirió tres comandos manuales breves; el resto de la configuración se automatizó por SSH.

Lo que la prueba no demostró es igualmente importante de enunciar: el aislamiento a nivel de anfitrión mediante seL4 sigue planificado, y la admisión automatizada de pares mediante os-network-admin aún no está en servicio. La prueba de junio estableció el sustrato de cómputo agrupado; la capa de soberanía es el objetivo hacia el que el proyecto avanza.
