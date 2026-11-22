# tool-precedent-core

El núcleo genérico de registros planos de precedentes. Cero dependencias. Contiene lo que necesita cada `tool-precedent-*`, de modo que cada herramienta recibe cada mejora: registro de cambios (`precedent/changes.csv`), linaje de versiones (`precedent/versions.csv`), términos (`precedent/terms.csv`), Registro de Derivaciones (`precedent/draws.csv`) y fichas con sello de integridad (FNV-1a 64). No hay un registro maestro.

Comandos (todos aceptan `--root DIR`, por defecto `.`, y `--file` para nombrar otro CSV): `changes-add`, `changes`, `versions [--dir outputs] [--out F]`, `terms`, `terms-add`, `draws`, `draw-add`, `apply <file.html> [--check]`, `catalog`, `registry <check|classes|css|dtcg|lint>`, `shared-hash`.

Hermano: `tool-forecasting-core` (project-proforma) lleva los mismos seis módulos (changes, csv, draws, fence, terms, versions), copiados. `shared-hash` imprime el FNV-1a 64 de cada uno; las dos listas deben coincidir línea por línea. Cuando cambia un módulo compartido, se copia al otro núcleo y se comparan.

## Comandos

La lista completa se genera desde una sola tabla en `src/commands.rs` (`tool-precedent-core describe --markdown`); `help`, `<comando> --help`, `man` y `describe --json` salen de la misma tabla. Cada comando acepta `--root DIR`.

- **Lectura (read):** nunca escribe. `find`, `lineage-report`, `language-report`, `catalog`, `changes`, `terms`, `draws`, `shared-hash`, `help`, `describe`, `man`.
- **Añadir (append):** añade una fila a un registro bajo `precedent/`. `changes-add`, `terms-add`, `draw-add`.
- **Escritura (write):** crea o reescribe un archivo. `versions` (reconstruye el índice de versiones) y `apply` (alinea el bloque de estilo de un documento con su ficha de familia y sus componentes; `--check` solo informa). `registry css` y `registry dtcg` escriben un archivo solo cuando se da `--out`.

## La puerta de aprobación (SYS-ADR-07)

En la línea de comandos la puerta es de procedimiento. Los comandos de añadir y escribir se ejecutan por instrucción de una persona, y la persona que aprueba el comando en la sesión es la puerta: una CLI no puede distinguir a una persona de un modelo, así que no lo aparenta. La vía de la biblioteca es distinta: `store::EditStore::commit` aplica `adr07::check` al aprobador. Lo que la CLI debe hacer cumplir de forma mecánica es el confinamiento: todo destino de escritura permanece dentro de `--root` (cambio previsto; hasta que se publique, `--file` y `--out` aceptan cualquier ruta). Las opciones de lectura (`--registry`, `--entities`, `--parties-file`) pueden nombrar archivos fuera de la raíz, por diseño.

## Para una IA

Ejecute `tool-precedent-core describe --json` para leer cada comando, sus opciones y si solo lee o también escribe. Los comandos de lectura pueden ejecutarse libremente. Los de añadir y escribir se ejecutan por instrucción de una persona: la persona que instruye la sesión es quien actúa. `lineage-report`, `language-report` y `find --counts` imprimen solo recuentos y son seguros de mostrar.
