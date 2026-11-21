# tool-collate

El lector de páginas compartido. Recorre una carpeta de páginas Markdown, lee el frontmatter de cada página (opcional: correcto, ausente o inválido) y su estructura (títulos, enlaces, tablas, bloques de código) y presenta recuentos. Solo lectura; no escribe nada y no contiene reglas de ningún tipo de página en particular.

```
tool-collate scan <carpeta> [--keep <carpeta-oculta>]...
```

El informe contiene solo recuentos: páginas encontradas y leídas, archivos ilegibles, frontmatter correcto / ausente / inválido, enlaces relativos revisados y rotos, enlaces no revisados (un esquema, un `#ancla` o una ruta `/sitio/ruta`) y carpetas omitidas por regla. Las carpetas ocultas, `target`, `node_modules` y `vendor-*` se omiten salvo que `--keep` nombre la carpeta oculta.

Las herramientas para tipos concretos de página (wiki, investigación, agentes) dependen de este crate para la lectura y aportan sus propias reglas. El recorrido de carpetas es el `walk` compartido de `tool-precedent-core`. La única dependencia de terceros es `serde_yaml`, porque un analizador escrito a mano interpreta mal el frontmatter real (listas anidadas, bloques literales, escapes).

Las pruebas usan solo páginas inventadas en `tests/fixtures/pages`.

Licencia: Apache-2.0.
