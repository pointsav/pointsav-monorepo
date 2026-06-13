---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Arquitectura de pagos con criptomonedas y emisión de licencias"
slug: topic-crypto-license-sales-architecture
language: es
status: draft
paired_with: TOPIC-crypto-license-sales-architecture.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-crypto-license-sales-architecture.es.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-marketplace/src/main.rs; tool-wallet/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# Arquitectura de pagos con criptomonedas y emisión de licencias

La compra de una licencia de software en `software.pointsav.com` fluye desde una
transferencia de USDC en la red Polygon hasta un token firmado con Ed25519 que autoriza
la descarga de binarios. El diseño es intencionalmente libre de custodios: el cliente
nunca crea una cuenta, el proveedor nunca retiene los fondos del cliente más allá del
momento de la liquidación, y no se requiere ningún intermediario para enrutar el pago.
La arquitectura consta de tres componentes: un vigilante de pagos, una tienda virtual y
un servidor de distribución; aquí se describen a nivel de sus interacciones.

## Por qué USDC en Polygon

USDC es una moneda estable vinculada al dólar estadounidense emitida por Circle. Su valor
está anclado al dólar, lo que hace práctica su uso para compras de software a precio fijo
sin exponer a ninguna de las partes a la volatilidad del tipo de cambio. Polygon PoS es
una cadena compatible con EVM de prueba de participación con tarifas de transacción más
bajas que la red principal de Ethereum, lo que la hace económica para compras en el rango
de pocos dólares. El sistema de pago opera como un observador de solo lectura del estado
público de la cadena de bloques: monitorea los eventos de registro ERC-20 Transfer en el
contrato USDC dirigidos a la billetera del proveedor. No se requiere ningún contrato
inteligente del lado del proveedor. Cualquier explorador de bloques puede verificar
independientemente un pago utilizando el hash de la transacción.

## Mecánica del vigilante de pagos

El vigilante de pagos sondea el punto de acceso JSON-RPC de Polygon a un intervalo
configurable, recorriendo los bloques confirmados e inspeccionando las entradas del
registro ERC-20 Transfer. Cuando encuentra una transferencia a la billetera del proveedor,
inspecciona el monto transferido para determinar a qué nivel de licencia corresponde el
pago — los dos niveles tienen cada uno un monto de USDC distinto. Por cada coincidencia
confirmada, escribe un recibo estructurado en un archivo plano que contiene el hash de la
transacción, la dirección del remitente, el número de bloque, la marca de tiempo de
confirmación y el identificador de producto derivado. También agrega una entrada a un
registro de transacciones en formato JSONL para fines contables y de auditoría.

Los recibos son autorizados. La tienda virtual no emitirá un token de licencia sin un
recibo correspondiente en disco. El diseño en dos etapas — el vigilante escribe el recibo,
la tienda lo lee — significa que la tienda nunca consulta la cadena directamente; delega
esa responsabilidad por completo al vigilante.

El vigilante también admite una URL de RPC alternativa para mayor resiliencia: si el punto
de acceso RPC principal no está disponible, vuelve a intentarlo con la alternativa antes
de fallar.

## Derivación de direcciones por pedido

Por defecto, todos los pagos se dirigen a una única dirección de billetera estática del
proveedor, y el hash de transacción sirve como identificador de pedido. Para los clientes
que prefieren una dirección de recepción dedicada para el seguimiento de pedidos o fines
contables, la tienda virtual puede derivar una a partir de la semilla maestra BIP-39 del
proveedor utilizando la derivación de clave jerárquica determinista (HD) BIP-32 a lo largo
de la ruta de derivación estándar de Ethereum. Cada pedido recibe un índice único, y la
asignación del identificador de pedido al índice de derivación se almacena localmente. Los
pagos a direcciones derivadas son monitoreados y registrados por el mismo vigilante que
supervisa la dirección estática. El flujo de dirección derivada es opcional; el flujo
estándar de dirección única sigue siendo el predeterminado.

## Emisión del token de licencia

Una vez que existe un recibo para un hash de transacción, la tienda virtual emite un token
de licencia firmado con Ed25519. La clave de firma está en posesión exclusiva de la tienda
virtual y nunca la abandona. La clave pública de verificación correspondiente está en
posesión exclusiva del servidor de distribución. Ninguno de los componentes tiene el
material de clave del otro, y ningún material de clave se transmite al cliente.

La carga útil del token registra el identificador del producto, una fecha de vencimiento
(un año a partir del momento de emisión en la configuración actual) y el nivel de licencia
como una lista de derechos. El token se forma anteponiendo los 64 bytes de la firma Ed25519
a los bytes en bruto de la carga útil y codificando el resultado como una cadena
base64url. El resultado es una cadena opaca única que el cliente almacena y presenta al
servidor de distribución.

## Verificación en el servidor de distribución

La verificación no tiene estado y no requiere ninguna llamada de red. Cuando llega una
solicitud de descarga con un token, el servidor de distribución decodifica la cadena
en base64url, separa los primeros 64 bytes como la firma Ed25519, verifica la firma
sobre los bytes restantes utilizando la clave pública almacenada, analiza la carga útil
y comprueba que el producto coincida con el producto solicitado y que la fecha de
vencimiento no haya pasado. Un producto no coincidente devuelve 403; una firma inválida
devuelve 401; un token vencido devuelve 403 con una cadena de motivo que indica que el
canal ha vencido. Dado que el servidor de distribución no tiene ninguna clave de firma,
una vulneración del servidor de distribución no permite a un atacante crear nuevos tokens.

El servidor de distribución expone la clave de verificación pública en un punto de acceso
bien conocido. Las herramientas externas — como el propio script instalador de un cliente —
pueden descargar la clave pública una vez y posteriormente verificar los tokens sin
conexión sin contactar al servidor de distribución en tiempo de ejecución.

## Idempotencia del recibo y flujo de reclamación

El punto de acceso de emisión de licencias de la tienda virtual es idempotente: consultar
el mismo hash de transacción varias veces siempre devuelve el mismo token. Si ya existe
un recibo en disco, el token se emite de inmediato. En caso contrario, la tienda delega
una consulta a la cadena al subcomando de verificación del vigilante de pagos y, tras la
confirmación, escribe el recibo antes de emitir el token. La primera llamada para una
nueva transacción puede implicar un ida y vuelta a la cadena; cada llamada posterior se
sirve desde disco sin latencia adicional más allá de la E/S local.

Un punto de acceso de reclamación independiente registra una asociación fuera de la cadena
entre un hash SHA-256 de binario y la dirección de billetera del comprador. Esto constituye
la base para una futura atestación de propiedad en la cadena. La acuñación de NFT en la
cadena queda diferida para una versión futura del sistema; el registro de reclamación se
escribe ahora para que los datos estén disponibles cuando se agregue esa capacidad.
