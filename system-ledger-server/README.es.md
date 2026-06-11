# system-ledger-server

Daemon de socket Unix que expone la consulta de `InMemoryLedger` en el fondo de compatibilidad NetBSD.

Comparte el formato de wire `system-ledger-proto` con `system-ledger-pd` (PD Microkit seL4). El transporte cambia entre ambos; el protocolo y la lógica de negocio no.

## Formato de trama

Cada mensaje lleva un prefijo de longitud LE de 4 bytes seguido de una carga útil postcard. El PD seL4 utiliza el mismo encabezado en su anillo de memoria compartida.

## Configuración

| Variable | Valor predeterminado | Propósito |
|---|---|---|
| `LEDGER_SOCK` | `/run/system-ledger/ledger.sock` | Ruta del socket Unix |

## rc.d NetBSD

Ver `scripts/rc.d/system_ledger` en `os-totebox/`.

## Compilación

```
cargo build --release -p system-ledger-server
```

## Pruebas

```
cargo test -p system-ledger-server
```

## Licencia

AGPL-3.0-or-later. Consulte [LICENSE](../LICENSE).
