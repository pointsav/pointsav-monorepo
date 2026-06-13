# app-mediakit-marketing

El motor de la plataforma de marketing para la familia `os-mediakit`. Sirve
`home.woodfinegroup.com` y `home.pointsav.com`.

[ 🇬🇧 Read this document in English ](./README.md)

> **Estado:** Activo (reescritura desde cero, andamiaje P1, 2026-06).
> Fila del registro: `pointsav-monorepo/.agent/rules/project-registry.md`.
> Arquitectura y hoja de ruta: `.agent/briefs/BRIEF-marketing-platform-master.md`.

## Qué es

Un único binario de Rust (axum 0.8) que renderiza páginas de marketing en el
**servidor** a partir de manifiestos de secciones tipadas. Reemplaza el antiguo
monolito HTML de 1,2 MB en un solo archivo y su frágil intercambio de DOM
mediante bundler/plantilla en el cliente (la causa del error de viewport en iOS
Safari) por una ruta limpia renderizada en el servidor.

Es **primero el agente**: los autores de IA componen páginas emitiendo un
manifiesto tipado a través del servidor MCP; las propuestas se ponen en una cola
de revisión humana; una persona aprueba (F12) antes de que algo persista. No hay
ruta de publicación automatizada (SYS-ADR-10, SYS-ADR-19).

- **Cromo y componentes:** `app-mediakit-shell` (chasis compartido). El
  encabezado, el pie y cada componente de sección — incluido todo el CSS
  adaptable — viven allí. Los manifiestos de contenido no llevan CSS.
- **Modelo de contenido:** `<content_dir>/<slug>/page.yaml` — una lista ordenada
  de secciones tipadas. El esquema es el contrato.

## Ejecutar

```
cargo run -p app-mediakit-marketing -- serve \
  --content-dir app-mediakit-marketing/content \
  --state-dir /tmp/marketing-state \
  --module-id woodfine \
  --bind 127.0.0.1:9109 \
  --enable-mcp
```

Luego `curl http://127.0.0.1:9109/` (HTML renderizado en el servidor),
`curl http://127.0.0.1:9109/page/contact`, `curl http://127.0.0.1:9109/healthz`.

## Compilar y probar

```
cd app-mediakit-marketing
cargo test
cargo clippy --all-targets -- -D warnings
```

## Estado

Andamiaje P1: conjunto mínimo de secciones (`hero`, `prose`, `cta`), cola de
revisión basada en archivos. El catálogo completo de secciones, la vista de
diferencias unificada real, la migración del contenido de las páginas de inicio
en vivo y la transición del despliegue son fases posteriores — ver el BRIEF
maestro.
