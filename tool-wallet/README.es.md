# tool-wallet

Observador de pagos USDC en Polygon y escritor de recibos para la tienda de software PointSav.

Distinto de `service-wallet` (Doctrina claim #53), que es el servicio Ring 2 de libro mayor
de ingresos por tenant para el Sustrato de Flujo Inverso en el lado del cliente. `tool-wallet`
es una utilidad de proveedor de un solo tenant: PointSav recibe pagos de licencias entrantes.

## Variables de entorno

| Variable | Propósito |
|---|---|
| `POLYGON_RPC_URL` | Punto de acceso JSON-RPC de Polygon |
| `POLYGON_WALLET_ADDRESS` | Dirección de la cartera receptora de PointSav |
| `WALLET_SEED_PATH` | Ruta al archivo de semilla HD — aprovisionado por el operador, nunca en git |
| `FS_ENDPOINT` | Punto de acceso del libro mayor WORM de service-fs |
