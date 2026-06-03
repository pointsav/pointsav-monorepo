---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: TOPIC
status: staged-pending-editorial
title: "Nivel B Yo-Yo — Sustrato Cloud Run"
target_path: media-knowledge-documentation/substrate/yoyo-cloud-run-substrate.es.md
paired_with: yoyo-cloud-run-substrate.md
bcsc_class: no-disclosure-implication
---

# Nivel B Yo-Yo — Sustrato Cloud Run

El sustrato Yo-Yo es el segundo nivel de inferencia en la arquitectura de enrutamiento
Doorman de tres niveles. El Nivel A gestiona solicitudes localmente en el nodo de trabajo
utilizando un modelo compacto. El Nivel B enruta hacia un modelo de mayor capacidad en
hardware GPU cuando la complejidad de una solicitud lo justifica. El Nivel C enruta hacia
proveedores externos de API comerciales. Este documento describe la implementación actual
del Nivel B: un servicio de Google Cloud Run que ejecuta el modelo OLMo 3 32B Think.

## Arquitectura

El servicio del Nivel B se ejecuta como una instancia de Cloud Run en la región
`europe-west4` (Países Bajos), alojada en el proyecto `woodfine-node-gcp-free`.

| Componente | Valor |
|---|---|
| Servicio | `yoyo-tier-b` |
| Imagen | `docker.io/ollama/ollama:0.24.0` |
| GPU | NVIDIA L4 (1×) |
| CPU | 8 vCPU |
| Memoria | 32 GiB |
| Escalado | 0–1 instancias (escala a cero en reposo) |
| Concurrencia | 4 solicitudes por instancia |
| Región | europe-west4 |

El modelo — OLMo 3 32B Think, cuantizado en formato Q3 GGUF con 15,6 GiB — se almacena
en un bucket de Google Cloud Storage (`woodfine-node-gcp-free-foundry-substrate`) bajo
la ruta `ollama-store/blobs/`. El proceso de Ollama lee el modelo desde este bucket
al iniciar en frío mediante el controlador de sistema de archivos GCS FUSE.

Una copia adicional de los pesos se almacena en `base-models/olmo-3-32b-think-q3.gguf`
en el mismo bucket. Ambas copias comparten un resumen SHA-256 idéntico (`06c420f9...`),
lo que permite verificación independiente.

## Perfil de inicio en frío

El inicio en frío es el período que transcurre desde que una solicitud llega a una
instancia escalada a cero hasta que se devuelve el primer token de inferencia. La
restricción principal es que 15,6 GiB de pesos del modelo deben cargarse desde
almacenamiento en red hacia la memoria GPU antes de que pueda comenzar la inferencia.

El enfoque directo — montar el bucket de GCS directamente y permitir que el proceso
de Ollama mapee en memoria el archivo GGUF — produce un tiempo de carga de
aproximadamente 30 minutos. Esto ocurre porque las lecturas mapeadas en memoria sobre
GCS FUSE emiten una solicitud HTTP GET de 4 KB por fallo de página, logrando un
rendimiento efectivo de aproximadamente 18 MB/s contra un archivo de 15,6 GiB.

La configuración actual elimina este problema mediante la función de caché de archivos
de GCS FUSE:

1. Se monta un volumen en memoria (tmpfs de 20 GiB) como destino de la caché.
2. El controlador GCS FUSE se configura para descargar el archivo GGUF en este volumen
   usando 16 trabajadores paralelos con fragmentos de 200 MB antes de que el proceso
   de Ollama lo lea.
3. La prueba de inicio (TCP en el puerto 8080) espera hasta 900 segundos para que el
   servicio esté listo, cubriendo la ventana de descarga y carga del modelo.

El resultado es un tiempo de inicio en frío de aproximadamente 5 minutos, dominado por
la descarga paralela de 15,6 GiB desde GCS. Una vez que el modelo está en el volumen
en memoria, Ollama lo lee a velocidad de RAM, cargándolo en la VRAM de la GPU en segundos.

La variable de entorno `OLLAMA_KEEP_ALIVE=-1` indica a Ollama que conserve el modelo
cargado en VRAM indefinidamente, de modo que las solicitudes posteriores en una instancia
caliente responden sin ningún retraso de recarga.

## Economía de escalado a cero

Con `min-instances: 0`, el servicio Cloud Run escala a cero cuando no hay solicitudes
activas. Una instancia escalada a cero no genera cargos de cómputo. El servicio escala
automáticamente cuando llega una solicitud, iniciando la instancia en frío.

Con aproximadamente 2 horas de uso activo por día, el costo mensual de cómputo es de
aproximadamente 57 USD usando la configuración actual de 8 vCPU / 32 GiB / L4. El
enfoque anterior (una VM Spot GCE reservada permanentemente en una zona específica)
costaba aproximadamente 165 USD por mes y no estaba disponible durante eventos de
agotamiento de zona.

Para cargas de trabajo que requieren cero latencia de inicio en frío, el servicio
puede configurarse con `min-instances: 1`, lo que mantiene una instancia activa en
todo momento a un costo continuo de aproximadamente 0,67 USD por hora para la GPU L4.

## Autenticación

El servicio Cloud Run no utiliza tokens portadores estáticos. El acceso está controlado
por tokens de identidad de GCP emitidos por el servicio de metadatos de Compute Engine.
Cuando el Doorman en el nodo de trabajo envía una solicitud al Nivel B, obtiene un token
de identidad de corta duración del servicio de metadatos local y lo incluye en el
encabezado `Authorization: Bearer`. Los tokens caducan después de una hora; el Doorman
obtiene un token nuevo en cada solicitud.

La variable de entorno `SLM_YOYO_GCP_AUTH=true` habilita este comportamiento en el
binario Doorman. Cuando este indicador está activo, se utiliza un proveedor
`MetadataBearer` en lugar del `StaticBearer` estático.

## Sonda de salud

El Doorman mantiene una sonda de salud en segundo plano para cada nodo del Nivel B
configurado. La sonda consulta la ruta raíz de Ollama (`/`) cada 30 segundos usando
el mismo mecanismo de token de identidad que las solicitudes de inferencia. Una
respuesta de "Ollama is running" indica que la instancia está en buen estado.

Tres fallas consecutivas de la sonda marcan el nodo como no disponible. El interruptor
de circuito del Doorman entonces enruta todo el tráfico al Nivel A hasta que la sonda
se recupere. Dado que el servicio Cloud Run está configurado con `concurrency: 4`, las
sondas de salud pueden alcanzar la instancia durante solicitudes de inferencia activas
sin ser puestas en cola ni limitadas.

La variable de entorno `SLM_YOYO_HEALTH_PATH=/` configura la ruta de la sonda. El
valor predeterminado (`/health`) es correcto para el servidor llama.cpp pero incorrecto
para Ollama, que sirve su indicador de salud en la ruta raíz.

## Integración con Doorman

El Doorman enruta solicitudes al Nivel B cuando el encabezado
`X-Foundry-Complexity: high` está presente, o cuando el indicador `SLM_TIER_A_FIRST`
está desactivado. El nombre del modelo visible para el extremo de Ollama es `olmo3`,
configurado mediante `SLM_YOYO_MODEL=olmo3`.

La URL del servicio es:
```
https://yoyo-tier-b-369270631281.europe-west4.run.app
```

## Rendimiento observado

Una prueba de flujo de una hora realizada el 3 de junio de 2026 produjo 15 solicitudes
exitosas consecutivas enrutadas a través de la cadena completa Doorman → Cloud Run →
OLMo 3 32B Think:

| Métrica | Valor |
|---|---|
| Solicitudes | 15 |
| Tasa de éxito | 100% (15/15 HTTP 200) |
| `tier_used` | `yoyo` (las 15) |
| Respuesta más rápida | 26,5 s |
| Respuesta más lenta | 169,5 s |
| Promedio de respuesta | ~72 s |
| Costo total | ~0,41 USD |

La varianza en el tiempo de respuesta refleja el comportamiento de razonamiento del
modelo OLMo 3 Think: genera tokens de razonamiento interno antes de producir la
respuesta final, y la extensión de la cadena de razonamiento varía según la solicitud.
