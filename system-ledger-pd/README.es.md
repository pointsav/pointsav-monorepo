# system-ledger-pd

Dominio de Protección seL4 Microkit que envuelve `InMemoryLedger`. Recibe
`ConsultRequest` a través del anillo de memoria compartida PPC en el canal 1;
devuelve `ConsultResponse` por el mismo anillo.

Mismo formato de transferencia que `system-ledger-server` (fondo de compatibilidad
Unix socket): prefijo de longitud LE de 4 bytes + carga útil postcard. El transporte
cambia; el protocolo no.

## Compilación

Requiere Microkit SDK v2.1.0 (placa x86\_64\_generic):

```sh
SDK_PATH=~/microkit-sdk-2.1.0 make
```

Verificación de integración continua (sin SDK — verifica que no haya filtración de std):

```sh
cargo build --no-default-features --features sel4 --target x86_64-unknown-none
```

## Ejecución

```sh
qemu-system-x86_64 -kernel build/final_image.elf -nographic -m 512M
```

Salida esperada: `LEDGER PD: online` seguido de `VERDICT: Allow` del cliente de prueba.

## Arquitectura

```
client_pd  (prioridad 200)
  │  escribe ConsultRequest en cap_request_mr (0x4001000, 16 KiB)
  │  emite microkit_ppcall(1, msginfo)
  ↓
system_ledger  (prioridad 254)
  │  lee ConsultRequest desde cap_request_mr
  │  llama InMemoryLedger::consult_capability
  │  escribe ConsultResponse en cap_response_mr (0x4005000, 4 KiB)
  │  retorna desde protected()
  ↓
client_pd
  │  lee ConsultResponse desde cap_response_mr
```

Consulte `ledger.system` para la descripción del sistema Microkit.
