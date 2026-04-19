# service-extraction

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

**Proveedor:** PointSav Digital Systems
**Estándar:** SYS-ADR-07 (Ingestión Bifurcada)
**Nivel:** 5 — Lógica de Servicio

---

## Qué hace

`service-extraction` es el analizador determinista del Archivo Totebox. Lee archivos `.eml` en bruto desde `service-email` o archivos comprometidos a través de `service-input`, elimina el formato propietario (MIME multipart, Base64, HTML) y escribe registros estructurados en el ledger WORM.

El enrutamiento sigue SYS-ADR-07: los datos estructurados — encabezados, destinatarios, firmas, adjuntos CSV — se analizan de forma determinista sin dependencia de IA. El texto no estructurado del cuerpo se entrega a `service-slm` sólo cuando ese servicio está instalado en el nodo del archivo. El nivel base de ToteboxOS ejecuta este servicio sin ningún componente de IA.

## Construcción

```
./build.sh
```

Produce dos binarios: un binario nativo de depuración para pruebas locales y un binario MUSL enlazado estáticamente para el despliegue en nodos en la nube de uso general.

## Ejecución

```
./target/release/service-extraction <input.eml> <totebox_root>
```

La salida se dirige a cuatro ubicaciones bajo `<totebox_root>/service-fs/data/`, siguiendo el patrón de aislamiento WORM por servicio documentado en la Guía de Usuario (Parte VI):

| Ruta | Contenidos |
|---|---|
| `service-extraction/source/` | `.eml` original sellado con SHA-256 |
| `service-extraction/ledger/` | Índice de registros extraídos |
| `service-people/source/` | Registros de personas (un JSON por persona) |
| `service-content/source/` | Texto del cuerpo limpio para indexación posterior |

## Pruebas

```
./build.sh
mkdir -p /tmp/test-totebox
./target/release/service-extraction samples/sample.eml /tmp/test-totebox
ls /tmp/test-totebox/service-fs/data/service-people/source/
```

Para la validación completa contra el conjunto de 10 correos de prueba, véase `VALIDATION.md`.

## Estado

- **v0.2** — línea base actual. Basado en regex, 571 líneas. Validado cualitativamente contra 10 archivos `.eml` reales (boletines y correspondencia empresarial). Maneja firmas multilingües, filtrado de URLs de seguimiento y clasificación de boletines.
- **v0.4** — en desarrollo activo. Añade el diccionario Aho-Corasick, el clasificador del Modelo de Gravitación Cognitiva, el filtro de entropía de Shannon y la detección mejorada de límites de firma. Objetivo: fidelidad de extracción superior al 90% en el corpus de prueba.

Véase `ROADMAP.md` para el inventario completo de técnicas y el plan por fases.

## Integración

`service-extraction` es invocado por `spool-daemon.sh` supervisando `service-email/maildir/new/` y `service-input/source/`. Es un proceso puntual — un `.eml` entra, los registros estructurados salen, el proceso termina. Sin daemon persistente, sin llamadas a la red, sin dependencias externas.

El motor de extracción es el lado interno del proceso de la Máquina de Entrada F12 (SYS-ADR-10). Cada registro comprometido a través de este servicio es candidato al flujo de trabajo del Verification Surveyor en `service-people`.

## Dependencias

Todas compatibles con Apache 2.0 o MIT. Sin GPL. Sin SDK propietarios. Véase `Cargo.toml` para el conjunto actual de dependencias; los cambios en las dependencias están sujetos a las condiciones de detención en `ROADMAP.md`.

## Legal

Consulte el archivo `LICENSE` en la raíz del monorepo. Este software se encuentra actualmente bajo licencia de Fase de Incubación. Todos los derechos reservados por Woodfine Capital Projects Inc.

---

*© 2026 PointSav Digital Systems™.*
