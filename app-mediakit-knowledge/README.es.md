# app-mediakit-knowledge

<div align="center">

[ 🇬🇧 Read this document in English ](./README.md)

</div>

Motor wiki para la plataforma de conocimiento de PointSav. Un solo
binario en Rust que sirve un directorio de archivos Markdown
(CommonMark con wikilinks) como una superficie de lectura y edición
de estilo Wikipedia.

## Estado

Fase 1 — renderizado. El motor lee un directorio de contenido,
analiza Markdown con la extensión de wikilinks de comrak, y sirve
páginas renderizadas por HTTP en una dirección de loopback.

Consulte [`ARCHITECTURE.md`](./ARCHITECTURE.md) para el plan
completo de las fases de construcción (hasta la Fase 8).

## Principio de diseño

**Los archivos Markdown en un árbol Git son la fuente de verdad.**
Cualquier base de datos, índice o caché es estado derivado,
reconstruible mediante `git checkout && reindex`. La identidad de
una página es una ruta; el historial de revisiones es `git log`;
los metadatos son frontmatter YAML; los espacios multi-contenido
son archivos hermanos. No hay escalera de migración de esquema
porque no hay esquema canónico en la base de datos — la base de
datos es un índice regenerable del árbol de archivos.

## Ejecutar (Fase 1)

```
cargo run -- serve --content-dir tests/fixtures/content
```

El servidor enlaza `127.0.0.1:9090` por defecto.

## Posicionamiento previsto

La Fase 8 está prevista como el foso del producto: la combinación
de autoría nativa en Markdown, extracción de datos estructurados
para los bloques de estado financiero requeridos por reguladores,
sellado de tiempo criptográfico, y adaptadores de exportación
por jurisdicción. Información prospectiva; sujeta a supuestos
materiales y decisiones del operador.

## Licencia

Apache-2.0.
