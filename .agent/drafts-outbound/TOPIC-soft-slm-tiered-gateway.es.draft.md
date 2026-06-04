---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "La pasarela de inferencia por niveles — enrutamiento de IA local-primero"
slug: soft-slm-tiered-gateway
target_repo: media-knowledge-documentation
target_path: media-knowledge-documentation/substrate/soft-slm-tiered-gateway.es.md
paired_with: soft-slm-tiered-gateway.md
category: substrate
quality: complete
bcsc_class: public-disclosure-safe
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "2026-06-04 project-intelligence session — traducción del TOPIC EN"
research_inline: true
---

# La pasarela de inferencia por niveles — enrutamiento de IA local-primero

Una pasarela de inferencia por niveles enruta cada solicitud de IA a través de una
jerarquía de niveles de cómputo, seleccionando el nivel más económico y capaz para
cada solicitud. El trabajo rutinario se ejecuta en hardware propiedad de la
organización. La capacidad adicional en GPU arrendada gestiona el trabajo que supera
la capacidad local. Una API comercial externa proporciona el respaldo final. Cada nivel
se degrada de forma controlada hacia el nivel inferior; ningún nivel es un único punto
de fallo.

## Por qué importa el enfoque local-primero

Enrutar toda la inferencia a un servicio externo es operacionalmente simple, pero
conlleva costes estructurales. Cada solicitud cruza una frontera organizativa,
exponiendo el contenido de las solicitudes y respuestas a un proveedor tercero. El
coste es proporcional al uso sin amortización. La organización no tiene forma de
adaptar el modelo a su propio vocabulario, procesos o conocimiento acumulado.

Una pasarela local-primero elimina estos costes para la mayoría del trabajo. El modelo
local gestiona las solicitudes dentro de su capacidad. Los recursos externos gestionan
lo que no puede. Con el tiempo, el modelo local mejora a través del entrenamiento
derivado del uso, reduciendo el conjunto de solicitudes que requieren cómputo externo.

## Los tres niveles

### Nivel A — inferencia local

El Nivel A es un servidor de inferencia que se ejecuta en el hardware propio de la
organización. Siempre está en funcionamiento, produce respuestas en segundos y no tiene
coste por solicitud más allá del hardware amortizado. Es el destino predeterminado para
todas las solicitudes.

El modelo local está entrenado o ajustado específicamente para el dominio de la
organización. Es más pequeño y rápido que los modelos de niveles superiores. Responde
de forma competente la mayoría de las solicitudes rutinarias: resúmenes, clasificación,
extracción de entidades de tipos de documentos conocidos, generación de código en
patrones conocidos.

### Nivel B — nodo GPU de expansión

El Nivel B son uno o más nodos de inferencia remotos que ejecutan un modelo más grande
y capaz en hardware GPU dedicado. Los nodos arrancan bajo demanda y se detienen cuando
están inactivos, por lo que el coste es proporcional al uso real y no a la
disponibilidad.

La pasarela mantiene un disyuntor por nodo y una máquina de estados del ciclo de vida
de la VM. Cuando llega una solicitud al Nivel B y el nodo objetivo está detenido, la
pasarela lo inicia automáticamente. El llamante recibe una respuesta 202 Accepted con
un punto de consulta mientras arranca el nodo.

Los nodos del Nivel B se organizan por etiqueta. Una etiqueta `batch` gestiona trabajo
en segundo plano: extracción de corpus, procesamiento de datos de entrenamiento y
mantenimiento programado. Una etiqueta `express` gestiona trabajo sensible al tiempo
que no puede esperar un arranque en frío.

### Nivel C — proveedor de inferencia externo

El Nivel C es una conexión opcional a una API de modelo de lenguaje comercial. Sirve
como respaldo final cuando tanto el Nivel A como el Nivel B no están disponibles, y
como ruta directa para tareas que la organización ha designado explícitamente como
externas.

El Nivel C nunca se utiliza como fuente de datos de entrenamiento. Requiere una clave
API explícita para activarse.

## Enrutamiento de solicitudes

Cada solicitud lleva una indicación de complejidad y, opcionalmente, una etiqueta de
nivel. La pasarela selecciona el nivel usando esta secuencia de decisión:

1. Si hay un interruptor de emergencia cerrado para el nivel solicitado, la solicitud
   se rechaza o cae al siguiente nivel.
2. Si hay una etiqueta de nivel explícita, la solicitud se enruta a ese nivel.
3. Si no hay etiqueta, se aplica la política de enrutamiento:
   - `balanced`: complejidad baja y media → Nivel A; alta → Nivel B.
   - `drain-batch`: todo el trabajo no-express va al nodo batch.
   - `drain-express`: todo el trabajo va al nodo express para vaciar el backlog.
   - `local-only`: todo el trabajo va al Nivel A independientemente de la complejidad.

La política de enrutamiento es configurable en tiempo de ejecución sin reiniciar la
pasarela.

## El interruptor de emergencia

Cada nivel tiene un interruptor de emergencia independiente. Cerrar un interruptor
detiene inmediatamente todo nuevo despacho a ese nivel. Las solicitudes en vuelo se
completan; ninguna nueva comienza. El trabajo en cola se acumula y se vacía cuando se
reabre el interruptor.

El interruptor de emergencia es el control de facturación del operador. Cerrar el
interruptor del nodo express detiene el arranque de la GPU A100; el coste cae a cero.

## La memoria organizativa

Antes de despachar cualquier solicitud, la pasarela consulta el grafo de conocimiento
organizativo para obtener entidades relevantes. Las entidades coincidentes se inyectan
en el prompt del sistema como contexto estructurado. El modelo ve las relaciones,
decisiones y políticas conocidas de la organización sin necesidad de derivarlas de
nuevo mediante inferencia.

## El servidor MCP

La pasarela expone una interfaz de memoria organizativa mediante el Protocolo de
Contexto de Modelo en un segundo puerto. Cualquier cliente de IA compatible con MCP
puede conectarse a esta interfaz usando su suscripción integrada, sin necesidad de una
clave API adicional.
