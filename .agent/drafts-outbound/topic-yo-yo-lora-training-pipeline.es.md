---
schema: foundry-draft-v1
state: draft-pending-language-pass
originating_cluster: project-intelligence
target_repo: content-wiki-documentation
target_path: ./
target_filename: topic-yo-yo-lora-training-pipeline.es.md
audience: vendor-public
bcsc_class: current-fact
language_protocol: PROSE-TOPIC
authored: 2026-05-11
authored_by: command@claude-code (session 2026-05-11)
authored_with: claude-sonnet-4-6
references:
  - service-slm/scripts/nightly-run.sh
  - service-slm/compute/packer/scripts/lora-training.sh
  - service-slm/scripts/corpus-threshold.py
  - service-slm/CLAUDE.md
  - conventions/apprenticeship-substrate.md §12 (CPT Trigger Discipline)
  - conventions/four-tier-slm-substrate.md
notes_for_editor: |
  Traducción completa al español de topic-yo-yo-lora-training-pipeline.md.
  Los términos técnicos (QLoRA, LoRA, Yo-Yo #1, L4 GPU, GCS, LadybugDB,
  vLLM, OLMo, peft, bitsandbytes, trl, SFT, DPO, NF4) se mantienen en inglés.
  Verificar precisión técnica y fluidez del español antes de publicar.
---

# TOPIC — Pipeline de Entrenamiento LoRA Nocturno en Yo-Yo #1

## Descripción general

Yo-Yo #1 es una instancia spot g2-standard-4 de Google Cloud equipada con
una GPU NVIDIA L4 de 24 GB de VRAM. Cada noche ejecuta un pipeline de dos
fases y cuatro horas de duración que produce pesos adaptadores ajustados para
el modelo de lenguaje del espacio de trabajo. La Fase 1 extrae entidades de
negocio estructuradas del corpus de datos de jennifer y las escribe en un
grafo de propiedades. La Fase 2 lee los pares de entrenamiento de ingeniería
y aprendizaje acumulados, verifica si el corpus ha superado un umbral mínimo,
y ejecuta un ciclo de entrenamiento eficiente en parámetros sobre el modelo
base. Las dos fases son obligatorias y secuenciales — no pueden superponerse
porque ambas requieren acceso exclusivo a la GPU L4.

## Por qué las fases son separadas

La GPU L4 sirve dos cargas de trabajo incompatibles dentro de la ventana
nocturna. Durante la Fase 1, vLLM carga OLMo 3 32B Think (cuantizado a
4 bits) para ejecutar la inferencia de extracción de entidades. Durante la
Fase 2, el ciclo de entrenamiento QLoRA carga los safetensors de OLMo 3 7B
Think para el cómputo de gradientes. Una GPU no puede servir simultáneamente
un proceso de inferencia vLLM activo y un ciclo de entrenamiento PyTorch —
las direcciones de memoria entran en conflicto y el cambio de contexto entre
kernels CUDA a esta escala no está soportado. `nightly-run.sh` establece el
límite de forma explícita: la Fase 1 termina con `stop-yoyo.sh`, que drena
el proceso vLLM y libera la GPU antes de que comience la Fase 2. Cada fase
tiene un presupuesto de tiempo configurable, con un valor predeterminado de
7200 segundos (dos horas) cada una.

## Fase 1 — Reconstrucción del DataGraph

Al inicio de la ventana nocturna, `start-yoyo.sh` arranca la VM de Yo-Yo #1
y espera hasta 90 minutos a que vLLM señale su disponibilidad. Una vez que
el servidor de inferencia está activo, `jennifer-datagraph-rebuild.sh` procesa
tres flujos de documentos del despliegue de jennifer: archivos markdown de
transcripciones de reuniones, archivos YAML y markdown de investigación de
agentes, y registros JSON de fuentes de contactos. Para cada documento, el
script llama a `POST :9080/v1/chat/completions` a través de Doorman, que
enruta la carga útil al modelo 32B Think en la VM de Yo-Yo. El modelo devuelve
un arreglo JSON estructurado de entidades nombradas — personas, empresas,
proyectos, cuentas y ubicaciones — restringido por una gramática JSON Schema
para que la salida sea procesable por máquina sin postprocesamiento. El script
luego llama a `POST :9081/v1/graph/mutate` en service-content para escribir
esas entidades en LadybugDB. Un registro local de hashes de documentos
procesados garantiza que cada documento se procese exactamente una vez en
múltiples ejecuciones nocturnas.

Al finalizar la Fase 1, vLLM se detiene y la GPU queda libre.

## Fase 2 — Entrenamiento del adaptador

`corpus-threshold.py` se ejecuta al inicio de la Fase 2. Cuenta los pares
JSONL en dos buckets del corpus — `engineering-pointsav` (pares SFT extraídos
de commits de ingeniería entre todos los clústeres) y `apprenticeship-pointsav`
(pares DPO generados por el sustrato de enrutamiento de aprendizaje). Cuando
alguno de los buckets alcanza 50 pares, el script escribe un archivo marcador
de entrenamiento pendiente y, si la variable de entorno
`SLM_YOYO_WEIGHTS_GCS_BUCKET` está configurada, sincroniza el directorio del
corpus relevante con el bucket de GCS configurado.

En la VM de Yo-Yo, `lora-training.sh` verifica el directorio de entrenamiento
pendiente cada 30 segundos. Cuando aparece un marcador, lo reclama mediante un
renombrado atómico (añadiendo `.claimed`), descarga el corpus desde GCS y
ejecuta QLoRA usando las bibliotecas peft, bitsandbytes y trl.

## Qué es QLoRA

QLoRA (Quantised Low-Rank Adaptation) es un método de ajuste fino eficiente
en parámetros que carga un modelo base en cuantización 4-bit NF4 y entrena un
conjunto pequeño de matrices de peso adicionales — llamadas adaptador — en
lugar de actualizar el modelo completo. Para un modelo de 7 mil millones de
parámetros como OLMo 3 7B Think, la cuantización a 4 bits reduce la demanda
de GPU de aproximadamente 14 GB (en bfloat16) a cerca de 6 GB, dejando margen
suficiente en la L4 de 24 GB para el ciclo de entrenamiento. El adaptador
apunta a siete capas de proyección lineal: `q_proj`, `v_proj`, `k_proj`,
`o_proj`, `gate_proj`, `up_proj` y `down_proj`. El entrenamiento se ejecuta
durante dos épocas con rango 16 (`r=16`), alpha 32 (`lora_alpha=32`), una
longitud de secuencia máxima de 512 tokens y verificación de gradientes
activada para gestionar la memoria de activaciones.

La configuración de entrenamiento es intencionalmente conservadora. El
objetivo es desplazar el modelo base hacia el vocabulario, los patrones de
formato y las convenciones estructurales que aparecen en los corpus de
ingeniería y aprendizaje — no reentrenar el modelo en una tarea general. Dos
épocas sobre cientos de pares son suficientes para este desplazamiento acotado.

## Los dos flujos del corpus

**Pares de ingeniería** son pares SFT (ajuste fino supervisado) extraídos de
diffs de commits reales, mensajes de commit y resúmenes de revisión de todos
los clústeres del espacio de trabajo. Enseñan al modelo el vocabulario técnico
preciso y los patrones estructurales utilizados en el flujo de trabajo de
ingeniería: cómo se describen los diffs, cómo se formulan los comentarios de
revisión y cómo se documentan las decisiones de implementación.

**Pares de aprendizaje** son pares DPO (optimización de preferencias directas)
producidos por el sustrato de enrutamiento de aprendizaje. Cada par consiste
en una respuesta en sombra (la salida sin guía del modelo) y una respuesta
veredicto (la formulación preferida confirmada por el operador). El
entrenamiento DPO sobre estos pares mueve el modelo hacia la distribución de
respuestas preferidas sin requerir etiquetas explícitas para cada token.

## Salida del adaptador y publicación

Cuando el entrenamiento finaliza, el adaptador se guarda en
`/data/weights/adapters/<tenant>/<role>/v<N>/` en la VM de Yo-Yo. El directorio
del adaptador contiene los archivos de pesos LoRA y la configuración del
tokenizador — el tamaño total es típicamente de 1 a 3 GB. Luego,
`lora-training.sh` indica a `adapter-publish.service` que cargue el directorio
del adaptador al bucket de GCS configurado. El adaptador queda disponible
para que Doorman del espacio de trabajo lo cargue como una superposición de
pesos en tiempo de inferencia sobre el modelo base. El archivo marcador se
renombra a `.completed` cuando todos los pasos se completan correctamente.

## Entrenamiento de adaptadores versus preentrenamiento continuo

El proceso LoRA nocturno es entrenamiento de adaptadores. Produce un delta de
pesos — unos pocos gigabytes de parámetros — que el modelo base carga en
tiempo de inferencia. Se ejecuta en aproximadamente dos horas en una sola GPU
L4 y opera sobre cientos a miles bajos de pares de entrenamiento. El modelo
base en sí no se modifica.

El preentrenamiento continuo (CPT) es una operación distinta a una escala
fundamentalmente diferente. El CPT produciría un nuevo checkpoint de modelo
base entrenando sobre 50 a 100 mil millones de tokens en 8 a 32 GPUs de clase
H100 durante una a cuatro semanas. El costo por ciclo de CPT asciende a decenas
de miles de dólares. El CPT es activado por el operador, nunca automatizado, y
nunca programado como parte del pipeline nocturno. El objetivo de primer corte
para CPT es el primer trimestre de 2027, sujeto a volumen del corpus y decisión
del operador. Hasta que se tome esa decisión, todo el entrenamiento nocturno
es exclusivamente de adaptadores.

## Estado actual

El código del pipeline nocturno está completo. El servicio de modelo de
lenguaje del espacio de trabajo pasa 177 de 177 pruebas. La reconstrucción
de la imagen Packer que incorpora el stack Python de entrenamiento (peft,
bitsandbytes, trl) a la VM de Yo-Yo es la siguiente acción prevista del
operador. Una vez desplegada esa imagen, se habilitará `lora-training.service`
en la VM de Yo-Yo con `systemctl enable --now lora-training.service`. Hasta
que la imagen sea reconstruida, la fase de entrenamiento opera en modo solo
marcador: `corpus-threshold.py` escribe y despacha el marcador de GCS, pero
`lora-training.sh` aún no está activo en la imagen de VM en producción.
