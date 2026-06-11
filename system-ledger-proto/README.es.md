# system-ledger-proto

Tipos de protocolo independientes del transporte para la consulta al Registro de Capacidades.

`ConsultRequest` y `ConsultResponse` se serializan con postcard de manera idéntica tanto si el transporte es un socket Unix (fondo de compatibilidad NetBSD) como un anillo de memoria compartida PPC de seL4 Microkit (fondo nativo seL4). Los mismos bytes circulan por ambos portadores; la lógica de negocio en `system-ledger` se invoca de forma idéntica en ambos casos.

## Uso

```toml
# std (predeterminado)
system-ledger-proto = { path = "../system-ledger-proto" }

# no_std + alloc (PD seL4)
system-ledger-proto = { path = "../system-ledger-proto", default-features = false, features = ["sel4"] }
```

## Características

| Característica | Uso |
|---|---|
| `std` (predeterminado) | Biblioteca estándar — usar en `system-ledger-server` y pruebas |
| `alloc` | `no_std` + alloc — para contextos embebidos o rump |
| `sel4` | `no_std` + alloc + primitivas de compatibilidad seL4 de `system-core` |

## Formato de wire

Codificación `postcard` (little-endian, compacto, compatible con `no_std`). El prefijo de longitud es responsabilidad de la capa de transporte.

## Licencia

AGPL-3.0-or-later. Consulte [LICENSE](../LICENSE).
