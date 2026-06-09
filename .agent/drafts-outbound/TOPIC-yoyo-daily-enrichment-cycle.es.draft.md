---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Ciclo Diario de Enriquecimiento Yo-Yo"
slug: topic-yoyo-daily-enrichment-cycle
language: es
status: draft
paired_with: TOPIC-yoyo-daily-enrichment-cycle.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-yoyo-daily-enrichment-cycle.es.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 1
research_provenance: "BRIEF-slm-learning-loop.md §11 §13 §14; bin/yoyo-daily-cycle.sh; registros de ciclo en /srv/foundry/data/yoyo-cycle-logs/"
research_inline: true
created: 2026-06-09
author: totebox@project-intelligence (claude-sonnet-4-6)
---

# Ciclo Diario de Enriquecimiento Yo-Yo

El ciclo diario de enriquecimiento Yo-Yo es la ventana de procesamiento por lotes que
arranca una máquina virtual con GPU una vez al día para enriquecer el DataGraph y
acumular datos de entrenamiento para el modelo de lenguaje local. El ciclo se ejecuta
a una hora fija, aplica un límite de costo máximo y termina la VM tanto si el trabajo
concluye antes como si alcanza el límite.

## Propósito

La VM de trabajo ejecuta un modelo de lenguaje de 7.000 millones de parámetros (OLMo 2 7B)
en CPU para uso interactivo. Este modelo funciona correctamente para instrucciones cortas,
pero extrae entidades de documentos con menor precisión que un modelo más grande
en GPU. El ciclo diario aborda esta limitación arrancando una VM de lotes independiente
— el nodo Yo-Yo — que carga un modelo de 32.000 millones de parámetros y procesa una
cola de documentos acumulados durante el día.

Los productos de cada ciclo son:
- Nuevas entidades nombradas añadidas al DataGraph (almacén de grafos LadybugDB)
- Pares de entrenamiento por Optimización de Preferencias Directas (DPO) escritos en el corpus de enriquecimiento

Cada par DPO registra lo que extrajo el modelo de 32B como salida preferida y lo que
extrajo el modelo de 7B como referencia, permitiendo afinar el modelo de 7B hacia la
calidad de extracción del modelo mayor en sucesivas ejecuciones de entrenamiento.

## Las ocho fases

El ciclo es un script Bash (`yoyo-daily-cycle.sh`) que ejecuta ocho fases secuenciales.
El script escribe un archivo de registro con marca de tiempo para cada ejecución.

**Fase 1 — Arranque de la VM.** Si la VM de lotes no está ya en ejecución, se emite
un comando `gcloud instances start`. La VM arranca desde un disco persistente que
conserva los pesos del modelo y la configuración del servidor de inferencia del ciclo
anterior.

**Fase 2 — Estado del servidor de inferencia.** El script consulta el endpoint de estado
de llama-server (`/health`) cada diez segundos hasta recibir `{"status":"ok"}`. El
arranque tarda sistemáticamente unos 170 segundos desde el encendido hasta la primera
respuesta positiva. Si el servidor no responde en diez minutos, el ciclo se interrumpe
y la VM se detiene.

**Fase 3 — Circuito Tier B.** El Doorman (la pasarela de inferencia local) mantiene un
interruptor de circuito para el nodo Yo-Yo. El script espera hasta dos minutos a que
el circuito se cierre, confirmando que el Doorman ha registrado la VM como accesible.
Si el circuito no se cierra, el ciclo continúa con una advertencia de respaldo a Tier A.

**Fase 4 — Drenaje de enriquecimiento.** Durante el 40 por ciento del presupuesto del
ciclo (18 minutos con el límite de 45 minutos), el script espera mientras el Doorman
procesa la cola de enriquecimiento pendiente. En este período, el proceso service-content
envía fragmentos de documentos al nodo Yo-Yo para la extracción de entidades y escribe
los pares DPO en el corpus de enriquecimiento. El progreso se registra cada 60 segundos
con conteos de entidades, pares DPO, utilización de la GPU y uso de VRAM.

**Fase 5 — Verificación del umbral del corpus.** Tras el enriquecimiento, se ejecuta
`corpus-threshold.py` para contar los datos listos para entrenamiento acumulados. Si los
conteos superan el umbral configurado, el script escribe archivos marcadores de
entrenamiento con fecha en `data/training-pending/`. Estos marcadores son la entrada
de la Fase 6.

**Fase 6 — Activación del entrenamiento LoRA.** Se deben cumplir tres condiciones para
que se ejecute el entrenamiento: los marcadores de entrenamiento deben estar presentes,
las bibliotecas de aprendizaje automático deben estar instaladas en el entorno virtual de
entrenamiento de la VM de lotes, y debe existir una etiqueta de aprobación del operador
para la fecha actual. Si las tres se cumplen, el script detiene el servidor de inferencia
para liberar aproximadamente 16 gigabytes de VRAM e invoca `run-dpo-training.py` por SSH
con el 45 por ciento del presupuesto (20 minutos con el límite de 45 minutos). El indicador
`--resume` acumula puntos de control diarios para que cada ejecución extienda el
entrenamiento del día anterior en lugar de comenzar desde cero.

**Fase 7 — Sincronización con GCS.** Si la variable de entorno
`SLM_YOYO_WEIGHTS_GCS_BUCKET` está definida y hay marcadores de entrenamiento presentes,
el corpus de enriquecimiento se sincroniza con el bucket de Cloud Storage configurado.
Este paso está actualmente desactivado a la espera de una sesión futura que configure
el bucket.

**Fase 8 — Parada definitiva.** El servidor de inferencia se detiene por SSH, la VM se
detiene con `gcloud instances stop` y el script espera hasta tres minutos a que la VM
alcance el estado `TERMINATED`. Una línea de resumen registra el tiempo total transcurrido,
el delta de entidades, el delta de pares DPO y el estado final de la VM.

## Presupuesto y costo

El ciclo diario opera con un límite máximo de 45 minutos. La VM se detiene
incondicionalmente al final de la Fase 8, independientemente de si las fases se
completaron con normalidad.

| Elemento | Valor |
|---|---|
| Tipo de VM | g2-standard-4 con NVIDIA L4 24 GB |
| Zona | us-central1-a |
| Costo en ejecución | aproximadamente $0.71 por hora |
| Costo por ciclo (límite de 45 min) | aproximadamente $0.53 por ciclo |
| Costo en estado TERMINATED | $0.00 |
| Costo mensual (ciclos diarios) | aproximadamente $16 por mes |

Un archivo de interruptor de emergencia (`/srv/foundry/data/yoyo-disabled`) suprime todas
las operaciones de ciclo de vida de la VM de forma inmediata. Crear el archivo impide que
la Fase 1 emita un comando de arranque. Eliminarlo reanuda el funcionamiento normal en el
siguiente ciclo programado.

Un temporizador de monitor de inactividad comprueba cada cinco minutos si la VM lleva
más de 30 minutos en ejecución sin actividad. Si el ciclo diario no logra detener la VM,
el monitor de inactividad la detendrá como medida de seguridad, evitando la acumulación
de costos sin límite.

## Formato de los pares DPO

Cada par DPO de enriquecimiento es un archivo JSON escrito en el directorio de
retroalimentación. El formato es compatible con el DPOTrainer de TRL:

```json
{
  "prompt":      "<texto del fragmento del documento>",
  "chosen":      "[{\"classification\":\"Person\",\"entity_name\":\"...\"}]",
  "rejected":    "[{\"classification\":\"Person\",\"entity_name\":\"...\"}]",
  "source_type": "datagraph-enrichment",
  "worm_id":     "<identificador del documento>",
  "timestamp":   "<ISO 8601>"
}
```

`chosen` es la extracción del modelo de 32B. `rejected` es la extracción del modelo de 7B.
Un par solo se escribe cuando ambos modelos encontraron al menos una entidad y los
resultados difieren después de la normalización. Los pares en los que el modelo de 7B no
encontró nada se descartan, ya que no contienen señal de preferencia genuina.

## Resultados de prueba verificados (2026-06-09)

Tres ciclos de prueba de 10 minutos confirmaron que el pipeline funciona correctamente
de extremo a extremo.

| Ciclo | Duración | Delta de entidades | Pares DPO añadidos | Estado final de la VM |
|---|---|---|---|---|
| 1 | 10 min 43 s | +7 | +6 | TERMINATED |
| 2 | 9 min 12 s | +8 | +4 | TERMINATED |
| 3 | 10 min 38 s | +22 | +8 | TERMINATED |

Diagnósticos de GPU en el ciclo 3: 99% de utilización, 16.151 de 23.034 MB de VRAM en uso, 73 °C.

## Pregunta abierta

Los nombres de los módulos objetivo de LoRA en `run-dpo-training.py` utilizan nombres
del estilo LLaMA (`q_proj`, `gate_proj`, etc.). OLMo-2 puede utilizar nombres internos
diferentes. Esto debe verificarse con el modelo cargado antes de la primera ejecución de
entrenamiento. Una discrepancia silenciosa haría que LoRA entrenara cero parámetros.
