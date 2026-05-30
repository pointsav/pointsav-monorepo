# app-workplace-http-prototype

Prototipo HTTP para `os-workplace`. Sirve las 8 superficies de Workplace
desde un único binario axum accesible vía WireGuard PPN mientras las
compilaciones nativas de Tauri esperan un host macOS. La Etapa 1 incluye
el editor de Memos.

## Ejecución

```sh
cargo build --release
WORKPLACE_PROTO_WORKSPACE=/home/jennifer/workbench \
WORKPLACE_PROTO_PORT=9110 \
./target/release/app-workplace-http-prototype
```

Abrir `http://10.8.0.9:9200` en la red WireGuard PPN (proxy nginx; el prototipo escucha internamente en el puerto 9110).

## Superficies

| Superficie | Etapa | Estado |
|------------|-------|--------|
| Memo | 1 | Activo |
| Proforma | 2 | Pendiente |
| Presentación | 3 | Pendiente |
| Cronograma | 4 | Pendiente |
| Código | 5 | Pendiente |
| PDF | 6 | Pendiente |
| GIS | 7 | Pendiente |
| BIM | 8 | Pendiente |

## Variables de entorno

| Variable | Predeterminado | Propósito |
|----------|----------------|-----------|
| `WORKPLACE_PROTO_WORKSPACE` | `/home/jennifer/workbench` | Directorio raíz del espacio de trabajo |
| `WORKPLACE_PROTO_PORT` | `9110` | Puerto HTTP |
