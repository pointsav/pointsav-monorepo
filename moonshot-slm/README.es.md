<div align="center">

# Sovereign Replacement Initiative | Iniciativa de Reemplazo Soberano

[ 🇬🇧 Read this document in English ](./README.md)

</div>

<br/>

> [!WARNING]
> **DECLARACIÓN DEL MARCO SOBERANO**
> Este repositorio es una implementación de referencia del Protocolo de Datos Soberano. Aplica aislamiento absoluto de datos. No contiene cargas útiles de red propietarias activas.

| Nivel de Arquitectura | Rol del Componente | Ancla de Gobernanza |
| :--- | :--- | :--- |
| 🔴 Investigación | Entrenamiento de Modelo Soberano | Fundación de Datos Soberana |

## I. QUÉ REEMPLAZA ESTE PROYECTO

`moonshot-slm` tiene como objetivo la dependencia de inferencia en API de inteligencia artificial externa — específicamente la ruta de inferencia Tier C de Anthropic enrutada a través de `service-slm` (Doorman). Este proyecto elimina esa dependencia entrenando, evaluando y desplegando un modelo de lenguaje soberano que opera íntegramente dentro de la infraestructura de Foundry.

## II. QUÉ SE ESTÁ CONSTRUYENDO

**Pipeline de entrenamiento.** Un pipeline automatizado de ajuste fino supervisado (SFT) y optimización de preferencias directas (DPO) sobre un modelo base OLMo (Apache 2.0). Los datos de entrenamiento provienen de `service-slm` (endpoint `/v1/shadow`) y de `service-content` (filas de extracción validadas por el grafo).

**Gestión del ciclo de vida de adaptadores.** Adaptadores LoRA separados por familia de tareas: `coding-lora` y `extraction-lora`. Cada adaptador sigue un ciclo de vida con atestación SLSA: entrenar → evaluar → promover → desplegar.

**Objetivo a largo plazo.** Un modelo de dominio específico entrenado exclusivamente con datos soberanos que gestione todas las tareas Tier B sin llamadas a APIs externas.

## III. ESTADO ACTUAL

Investigación / Marcador. La infraestructura de captura de datos de entrenamiento existe en `service-slm`. El sistema de gestión de adaptadores es el objetivo de implementación de este proyecto. No se han ejecutado entrenamientos.

---
*© 2026 PointSav Digital Systems™.*
