# service-input

[ 🇬🇧 Read this document in English ](./README.md)

Backend de Input Machine para el despliegue `cluster-totebox-jennifer-2`.
Migra en lotes archivos heredados del despliegue jennifer-1 a través
de la canalización de ingesta actual hacia `service-fs` (o
directamente a un directorio CORPUS para extracción), y evalúa la
salida de extracción contra datos de referencia curados manualmente
para monitorear la calibración de la canalización con el tiempo.
Activo en el puerto 9106.

## Nota histórica

Un diseño anterior de este crate — un servicio genérico de ingesta de
límites del Ring 1 para análisis de documentos multi-formato
(PDF/DOCX/XLSX/Markdown) — fue completamente construido y probado
antes de la fusión de este archivo del 2026-06-20, pero esa
implementación nunca se incorporó. El propósito real y actual de este
crate ha estado en desarrollo continuo desde el 2026-06-14 y no está
relacionado con ese diseño anterior.

## Endpoints

| Endpoint | Método | Propósito |
|---|---|---|
| `/healthz` | GET | Disponibilidad + conteos de cola/completados |
| `/v1/status` | GET | Progreso de migración fase-1/fase-2 |
| `/v1/append` | POST | Reenvía una carga útil preformada a `service-fs` |
| `/v1/migrate` | POST | Migra en lotes archivos heredados de jennifer-1; reanudable |
| `/v1/eval/:stem` | GET | Evalúa la salida de extracción de un documento contra datos de referencia |
| `/v1/calibration-report` | GET | Puntuación de calibración agregada en todos los stems |

## Estado

Activo. En funcionamiento en el puerto 9106.

## Licencia

Consulte el archivo `LICENSE` del repositorio. La asignación de
licencias a nivel de componente está regida por
`LICENSE-MATRIX.md` en
`pointsav/factory-release-engineering`.
