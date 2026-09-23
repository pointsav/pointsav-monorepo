# app-privategit-software

Tienda de software PointSav. Sirve `software.pointsav.com`.

Todos los productos están actualmente en `$0`/BETA sin importar su nivel de licencia —
el precio se registra en el catálogo, pero ningún producto cobra por ahora. Los clientes
que no deseen compilar desde el código fuente eventualmente pagarán una tarifa de
licencia en USDC (Polygon PoS) y recibirán un binario precompilado y firmado. La ruta de
código abierto permanece gratuita en `github.com/pointsav/pointsav-monorepo`.

## Superficies

- **Explorar** (`/software`, `/software/:id`) — catálogo renderizado en vivo desde
  `products.yaml`, agrupado por nivel de licencia
- **Precios** (`/pricing`), **Licencia** (`/licensing`) y páginas legales
  (`/page/contact`, `/page/disclaimer`, `/page/privacy`, `/page/accessibility`)
- **Pagar** — USDC en Polygon PoS; pagos verificados mediante `tool-wallet`
- **Licencia** — un token de licencia portador emitido tras la confirmación del pago
  en cadena, válido hasta el final del día UTC en que se emitió (se remite de nuevo si
  se visita `/order/:tx_hash` en un día posterior, no es una ventana fija de 24 h)
- **Descargar** — el token de licencia autentica una descarga contra
  `app-privategit-release`

Ver `README.md` para la lista completa de variables de entorno.
