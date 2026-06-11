# system-substrate-netbsd

Capa de compatibilidad NetBSD 10.1 para los constructores de imágenes `os-totebox` y `os-orchestration`.

Expone constantes de tiempo de compilación (cadenas de versión, listas de binarios) y el tipo `VeriexecEntry` para generar manifiestos `/etc/signatures`.

## Rol del fondo de compatibilidad

NetBSD 10.1 es el sistema operativo canónico del fondo de compatibilidad para todas las imágenes de VM `os-*`. Proporciona:

- `wg(4)` — controlador de kernel WireGuard (integrado desde NetBSD 9.3).
- `Veriexec` — verificación de firma de binarios a nivel del SO.
- FFS2 — sistema de archivos canónico; construido con `nbmakefs` de las herramientas cruzadas de NetBSD.
- `build.sh tools` — compilación cruzada reproducible en el host Ubuntu de GCP.

## Compilación

```
cargo build -p system-substrate-netbsd
```

## Licencia

AGPL-3.0-or-later. Ver [LICENSE](../LICENSE).
