---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-knowledge
target_repo: content-wiki-documentation
target_path: architecture/   # candidates: architecture/, applications/ — project-language decides
target_filename: collab-via-passthrough-relay.es.md
audience: vendor-public
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
language: es
companion: topic-collab-via-passthrough-relay.md (English canonical)
authored: 2026-04-28T00:30:00Z
authored_by: task-project-knowledge (session 619abe3eff24497e)
authored_with: sonnet-4-6
draft_shape: bulk-draft
references:                          # mirrors English sibling per Brief 6 audit fix 2026-04-28
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md §1 Step 7
  - vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md §11
  - vendor/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md §4.7
  - vendor/pointsav-monorepo/app-mediakit-knowledge/docs/STEP-7-COLLAB-SMOKE.md (commit ea26118)
  - https://github.com/yjs/yjs
  - https://codemirror.net/
  - https://docs.rs/tokio/latest/tokio/sync/broadcast/
  - DOCTRINE.md claim #29 (Substrate Substitution)
  - DOCTRINE.md claim #16 (Optional Intelligence Layer)
notes_for_editor: |
  SKELETON ONLY — Spanish overview sibling to topic-collab-via-passthrough-relay.md.
  Per DOCTRINE §XII strategic-adaptation pattern: the Spanish overview is
  not a 1:1 translation — it adapts the core idea for Spanish-language
  readers, preserves the structural points, and lets project-language's
  PROSE-TOPIC adapter pare for register at refinement time.

  Section structure mirrors the English skeleton because the structural
  points are language-independent; the Spanish prose at refinement time
  can compress sections §3 + §4 into a single technical-implementation
  section if the Spanish-reader audience benefits from a tighter
  treatment. project-language has authority on the compression decision.
---

# Colaboración en tiempo real mediante relé de paso — un patrón de sustrato

## §1 El patrón en un párrafo

El patrón de relé de paso invierte la suposición habitual sobre dónde reside la autoridad en un sistema de edición colaborativa: el servidor de relé no conserva ningún estado de documento, por lo que el árbol git canónico sigue siendo el único registro autoritativo del contenido de cada tema en todo momento. Los editores concurrentes se conectan mediante WebSocket a un canal `tokio::sync::broadcast` identificado por slug — una sala de difusión por documento — y la única responsabilidad del servidor es reenviar mensajes de actualización del protocolo CRDT de Yjs entre esos clientes; el servidor nunca decodifica ni almacena el estado del documento que esos mensajes codifican. El único límite de persistencia en todo el sistema es la ruta de escritura atómica `POST /edit/{slug}`: cuando un editor guarda, el cliente serializa su documento Yjs local a Markdown, lo envía por HTTP y el servidor mueve atómicamente el nuevo archivo a su lugar en disco.

## §2 Por qué un relé de paso (en lugar de un servidor CRDT)

Herramientas como Etherpad y HackMD operan bajo un modelo de documento autoritativo en el servidor: el servidor de edición colaborativa mantiene un objeto de documento vivo y mutable — un registro de Transformación Operacional en el caso de Etherpad, un Y.Doc en el de HackMD — y ese objeto es el registro principal del contenido actual. Una exportación a git es una instantánea tomada de ese registro del servidor, no al revés. La consecuencia es un segundo estado autoritativo permanente: hay dos lugares en el sistema que contienen una respuesta a la pregunta "¿cuál es el texto actual de este documento?", y pueden divergir si el mecanismo de exportación falla o el registro CRDT del servidor difiere del historial de git.

El diseño de relé de paso elimina ese segundo registro por completo. El servidor es un conducto de mensajes, no un almacén. Cuando un cliente Yjs envía un mensaje de actualización binario a `GET /ws/collab/{slug}`, el manejador Rust recibe los bytes en bruto del WebSocket y los difunde a todos los demás clientes de la misma sala de slug mediante `tokio::sync::broadcast`. El servidor nunca deserializa el protocolo Yjs; nunca construye un Y.Doc; nunca escribe nada en disco como efecto secundario de una operación de relé. El único estado del documento que el servidor conoce es lo que el cliente envía a través de `POST /edit` — la ruta de guardado HTTP.

Esto importa específicamente para la postura de divulgación descrita en la reclamación #29 de DOCTRINE.md (Sustitución de Sustrato). El registro de divulgación canónico en este sistema es el árbol git. Bajo el diseño de relé de paso, no existe ningún registro paralelo: el estado CRDT en curso no es parte del registro de divulgación por construcción, porque nunca se escribe en ningún lugar. El registro se cierra en el momento de `POST /edit`, no antes.

El búfer de 256 mensajes configurado en cada canal `tokio::sync::broadcast` resuelve la única condición de carrera que este diseño debe manejar: un cliente que se une a una sesión de colaboración después de que otros editores ya han realizado cambios podrá ponerse al día con la actividad reciente sin que el servidor necesite materializar o almacenar el estado completo del documento.

## §3 Implementación y límites del servidor en `app-mediakit-knowledge`

El relé de colaboración se implementó en el commit `05f1dab` como `src/collab.rs`, completamente restringido por el indicador CLI `--enable-collab`. Cuando el indicador está ausente — que es la configuración predeterminada y la postura de producción actual en v0.1.29 — la ruta WebSocket `GET /ws/collab/{slug}` no se registra en el enrutador axum, la variable de plantilla `window.WIKI_COLLAB_ENABLED` se establece en `false`, y el paquete JavaScript del lado del cliente nunca se carga.

El relé del lado del servidor se implementa mediante `tokio::sync::broadcast`, el canal estándar multi-productor multi-consumidor de Tokio. Cada slug obtiene su propio canal de difusión con una capacidad de búfer de 256 mensajes, creado en la primera conexión y almacenado en un `DashMap<String, broadcast::Sender<Bytes>>` en `AppState`. Cuando un cliente WebSocket envía un mensaje de actualización de Yjs, el manejador lee los bytes en bruto y llama a `sender.send(bytes)` — una sola línea que distribuye el mensaje a todos los demás receptores de ese canal. No hay dependencia del crate `yrs`: el servidor reenvía mensajes binarios del protocolo Yjs sin deserializarlos, por lo que el servidor no porta ningún estado de documento en ningún momento.

El paquete cliente `cm-collab.bundle.js` (~302 KB tal como fue entregado) está construido a partir de tres paquetes npm: `yjs`, `y-codemirror.next` e `y-websocket`. El paquete se confirma en `static/vendor/` como un artefacto precompilado, por lo que no se requiere ninguna cadena de herramientas npm en tiempo de ejecución. El script de inicialización `static/saa-init.js` verifica `window.WIKI_COLLAB_ENABLED` en tiempo de carga; si el indicador es `false`, la importación de `cm-collab.bundle.js` nunca se ejecuta.

En cuanto a los límites del servidor — lo que el relé deliberadamente omite: no hay crate `yrs` ni estado de documento en el servidor; no hay persistencia de estado CRDT en ningún punto de la ruta de relé; no hay identidad portadora de autenticación que dure más que la vida útil del WebSocket; no hay límite de velocidad en la ruta de relé más allá de los valores predeterminados de axum; y no hay historial de edición accesible desde el lado del servidor, porque no existe historial del lado del servidor que inspeccionar. El único historial de edición que existe es el registro de commits git producido por los guardados sucesivos de `POST /edit`.

La cobertura de pruebas en el commit `05f1dab` agregó 7 pruebas (3 unitarias + 4 de integración) para llevar el total de 90 a 97. Las pruebas unitarias cubren: la ruta WebSocket acepta una conexión cuando se establece `--enable-collab`; las salas `tokio::sync::broadcast` multiplexan correctamente entre dos clientes en el mismo slug; el búfer de 256 mensajes se vacía sin pánico; y el paquete del cliente se carga solo cuando se establece el indicador. La cobertura no incluye la propiedad visual de presencia del cursor, que requiere un proceso de verificación manual con dos navegadores documentado en STEP-7-COLLAB-SMOKE.md.

## §5 Lo que esto significa para la posición de divulgación

El estado CRDT en curso — la secuencia de mensajes de actualización de Yjs intercambiados entre clientes durante una sesión de colaboración — no forma parte del registro de divulgación y no puede serlo, por construcción. Dado que el relé nunca persiste esos mensajes, no existe ningún artefacto del lado del servidor que pudiera producirse posteriormente en respuesta a una obligación de divulgación. La sesión de colaboración no deja ningún estado en el servidor.

Las ediciones guardadas ingresan al registro de divulgación a través de la misma ruta que todas las demás ediciones: `POST /edit/{slug}` envía el texto Markdown completo del documento, el servidor realiza un renombrado atómico del archivo, y el siguiente commit git en la secuencia captura esa instantánea. Desde la perspectiva de git, un guardado editado en colaboración es idéntico a un guardado de un solo autor. El commit registra lo que el documento contenía en el momento del guardado; no registra quién escribió qué caracteres durante la sesión de colaboración. Esta es la postura correcta bajo la convención del sustrato de divulgación: la unidad de divulgación es el estado del documento confirmado, no la descomposición de autoría de cómo se alcanzó ese estado.

La ruta de reversión del indicador `--enable-collab` (documentada en STEP-7-COLLAB-SMOKE.md §5) no es destructiva en ninguna capa, precisamente debido a este diseño. La superposición CRDT de colaboración es efímera por construcción; su eliminación al reiniciar el servicio no es un evento de pérdida de datos. Cualquier contenido guardado antes del reinicio existe en el árbol git y es completamente recuperable; cualquier contenido en curso que no fue guardado antes del reinicio se pierde exactamente de la misma manera que el contenido no guardado en una sesión de editor de un solo autor.

## §6 Generalizando más allá del wiki

El relé de paso es un patrón de sustrato, no una característica específica del wiki. Cualquier servicio que desee semánticas de edición concurrente enfrenta la misma pregunta arquitectónica: ¿necesita la infraestructura de colaboración mantener el estado del documento en el servidor, o puede ese estado residir completamente en los clientes y en el almacenamiento canónico? La respuesta depende de cuál sea el almacenamiento canónico y de si un servidor CRDT sentado entre los clientes y ese almacenamiento competiría con él por la autoridad.

**Canalización de revisión multiautor de `service-extraction`.** El almacenamiento canónico para los resultados de extracción en `service-extraction` son los registros estructurados producidos por combinadores de análisis deterministas. La revisión multiautor de resultados de extracción se beneficiaría de la conciencia de presencia y la resolución de conflictos en tiempo real. La pregunta central: ¿un servidor CRDT con estado competiría con el almacén de registros estructurados por la autoridad? Si el servidor CRDT materializa correcciones parciales que aún no han sido confirmadas de vuelta al almacén estructurado, la respuesta es sí — y el patrón de relé de paso no aplicaría directamente sin una capa adaptadora que serialice el estado CRDT al formato de almacenamiento canónico en el momento del guardado.

**Colaboración en presentaciones de `app-workplace-presentation`.** El almacenamiento canónico para una presentación es el formato de archivo del deck de diapositivas confirmado en git. La mayoría de las soluciones existentes (Google Slides, Office 365) usan un modelo autoritativo en el servidor donde el objeto de presentación reside en el servidor. El patrón de relé de paso aplica si y solo si el archivo de presentación en disco es el registro canónico y la superposición CRDT se trata como efímera de sesión sobre él.

**Colaboración en tablas de `app-workplace-proforma`.** El almacenamiento canónico para un proforma son datos tabulares estructurados — filas y columnas con valores tipados, almacenados como TOML o JSON estructurado junto al árbol git. La edición colaborativa de celdas de tabla requiere un tipo CRDT diferente al de la edición de texto por caracteres (`Y.Map` y `Y.Array` en lugar de `Y.Text`), pero la biblioteca cliente `yjs` es suficientemente general para manejarlo. Si el almacenamiento canónico del proforma son datos confirmados en git y la superposición CRDT es efímera de sesión, el patrón de relé de paso aplica con la misma lógica que el caso del wiki. La diferencia es que el paso de serialización en el guardado debe producir datos estructurados válidos, no Markdown de texto libre, añadiendo un requisito de validación en el momento de `POST /save`.

## §7 Referencias

- **Yjs** — Biblioteca CRDT libre de conflictos para aplicaciones colaborativas. Motor CRDT del lado del cliente usado en `cm-collab.bundle.js`. https://github.com/yjs/yjs
- **Integración colaborativa de CodeMirror** — `y-codemirror.next` vincula un `Y.Text` de Yjs a un estado de editor CodeMirror 6; la presencia del cursor se renderiza mediante la API `DecorationSet` de CodeMirror. https://codemirror.net/
- **`tokio::sync::broadcast`** — Canal multi-productor multi-consumidor usado como sala de relé por slug. El búfer de 256 mensajes es el argumento de capacidad de canal pasado a `broadcast::channel(256)`. https://docs.rs/tokio/latest/tokio/sync/broadcast/
- **`PHASE-2-PLAN.md` §1 Step 7** — Especificación original del diseño de colaboración (nota: la especificación original llamaba a `yrs`; la implementación entregada es un relé de paso sin `yrs`). `vendor/pointsav-monorepo/app-mediakit-knowledge/docs/PHASE-2-PLAN.md`
- **`STEP-7-COLLAB-SMOKE.md`** (commit `ea26118`) — Procedimiento manual de prueba con dos clientes, diff de unidad systemd pre-preparado y runbook de reversión para habilitación en producción. `vendor/pointsav-monorepo/app-mediakit-knowledge/docs/STEP-7-COLLAB-SMOKE.md`
- **`ARCHITECTURE.md` §11** — Tabla completa de la superficie API que lista `/ws/collab/{slug}` como una ruta opt-in de Phase 2+. `vendor/pointsav-monorepo/app-mediakit-knowledge/ARCHITECTURE.md`
- **`UX-DESIGN.md` §4.7** — Wireframe de UX de colaboración y patrón de presencia del cursor para la superficie del editor SAA. `vendor/pointsav-monorepo/app-mediakit-knowledge/UX-DESIGN.md`
- **DOCTRINE.md reclamación #29** (Sustitución de Sustrato) — Establece que el registro de divulgación canónico es git; ningún almacén del lado del servidor puede competir con él por la autoridad. `~/Foundry/DOCTRINE.md`
