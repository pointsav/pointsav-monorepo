---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "app-console-slm — consola de monitorización de infraestructura de inferencia"
slug: app-console-slm
target_repo: media-knowledge-documentation
target_path: media-knowledge-documentation/applications/app-console-slm.es.md
paired_with: app-console-slm.md
category: applications
quality: complete
bcsc_class: public-disclosure-safe
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — traducción del TOPIC EN"
research_inline: true
---

# app-console-slm — consola de monitorización de infraestructura de inferencia

app-console-slm es un cartucho de interfaz de usuario de terminal (TUI) para la
consola del operador que muestra el estado en tiempo real de la infraestructura de
inferencia de IA. Muestra el estado del modelo de inferencia local, el estado de los
nodos GPU remotos, la profundidad de la cola de prioridad, el recuento de entidades
del grafo organizativo y el gasto del día en curso. Proporciona controles de teclado
para ajustar la política de enrutamiento y activar interruptores de emergencia por nivel.

La consola se ejecuta en una ventana de terminal en el mismo nodo que la pasarela de
inferencia. No requiere navegador, conexión de red a un servicio externo ni autenticación
más allá del acceso local al shell. Es el panel de control principal del operador para
comprender y controlar la capa de inferencia.

## Paneles de visualización

La consola organiza la información en cinco paneles que se actualizan automáticamente
cada diez segundos. El operador puede activar una actualización inmediata en cualquier
momento con la tecla R.

### Panel de pasarela

El panel de pasarela muestra el estado actual del enrutador de inferencia: si está
en funcionamiento, la política de enrutamiento activa (balanced, drain-batch,
drain-express o local-only) y la disponibilidad de cada nivel. Un indicador verde
marca un nivel como disponible. Un indicador amarillo marca un nivel como degradado.
Un indicador gris marca un nivel como desconectado.

### Panel de flota de nodos GPU

El panel de flota muestra cada nodo GPU remoto configurado con su estado actual:
detenido, arrancando, disponible, fallido o zombie. Para los nodos disponibles,
el panel muestra la latencia de la sonda más reciente en milisegundos.

### Panel de grafo organizativo

El panel del grafo muestra el recuento total de entidades en el grafo de conocimiento
organizativo, el número de tipos de aristas distintos presentes y la marca de tiempo
de la extracción más reciente. También muestra el estado del disyuntor para el servicio
del grafo.

### Panel de cola

El panel de cola muestra la profundidad actual de cada nivel de cola de prioridad.
P0 contiene tareas de clasificación en segundo plano. P1 contiene tareas de extracción
a la espera de un nodo GPU. P2 contiene generación de corpus de entrenamiento y trabajo
de aprendizaje. El panel también muestra el total completado y el recuento de poison
actual.

### Panel de costes

El panel de costes muestra el gasto del día en curso en todos los niveles. El panel
desglosa el gasto por etiqueta de nodo: el nodo batch, el nodo express y la API externa
(si está configurada).

## Controles de teclado

| Tecla | Acción |
|---|---|
| R | Actualización inmediata |
| K | Diálogo de interruptor de emergencia — activar/desactivar por nivel o global |
| P | Diálogo de política — seleccionar política de enrutamiento |
| G | Detalle del grafo — mostrar desglose de tipos de entidades |
| ? | Ayuda — mostrar todos los atajos de teclado |
| Q | Salir |

## Características técnicas

La consola es un crate de biblioteca que implementa el trait Cartridge para el chasis
de la consola del operador. Se carga en el slot F9. La comunicación con la pasarela
de inferencia utiliza HTTP estándar contra los endpoints de monitorización de la
pasarela. El modo de texto plano está disponible mediante la bandera `--plain` para
entornos de terminal sin soporte unicode.

## Relación con la pasarela de inferencia

La consola es un observador principalmente de lectura de la pasarela de inferencia.
No participa en las decisiones de enrutamiento. Los comandos de interruptor de
emergencia y política enviados a través de la consola surten efecto inmediatamente en
la pasarela.
