---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Sustrato de distribución de software de PointSav"
slug: topic-software-distribution-substrate
language: es
status: draft
paired_with: TOPIC-software-distribution-substrate.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-software-distribution-substrate.es.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 3
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-source/src/main.rs; app-privategit-marketplace/src/main.rs; tool-wallet/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# Sustrato de distribución de software de PointSav

El sustrato de distribución de software de PointSav es un sistema de tres componentes
que gestiona el alojamiento de binarios, la emisión de licencias a través de la tienda
en línea, y la verificación de pagos en la cadena de bloques. Los tres componentes —
un servidor de distribución, una tienda virtual y un vigilante de pagos — funcionan
como servicios independientes accesibles en `software.pointsav.com`. Cada componente
tiene una responsabilidad única y delimitada; juntos conforman una cadena de distribución
sin custodios: no se requieren cuentas de cliente y el comprador puede pasar del pago
a la descarga del binario en una sola sesión.

## Los tres componentes

**Servidor de distribución.** El servidor de distribución entrega binarios compilados e
impone la verificación de un token de licencia Ed25519 antes de permitir cualquier
descarga. También expone índices de productos y versiones que permiten a las herramientas
automatizadas descubrir los lanzamientos disponibles, y puede redirigir las solicitudes
de la versión más reciente de un producto a la ruta versionada correspondiente. El
servidor es sin estado: no guarda registros de pago ni datos del cliente. Su única
responsabilidad es verificar que el token sea auténtico y que autorice el producto
solicitado, para luego transmitir el archivo.

**Tienda virtual.** La tienda virtual presenta un catálogo de productos en el navegador,
verifica los pagos entrantes en USDC de Polygon cotejando las transferencias confirmadas
en la cadena con un almacén local de recibos en archivos planos, y emite tokens de
licencia firmados con Ed25519 una vez que se confirma el pago. La tienda virtual es
CÓDIGO — forma parte de la infraestructura del proveedor. Los productos de software que
vende son SOFT — llevan claves de licencia Ed25519 y están listados en el catálogo.
Estas son categorías distintas: la tienda es infraestructura; la mercancía es el
producto con licencia.

**Vigilante de pagos.** El vigilante de pagos monitorea la red Polygon PoS en busca de
transferencias entrantes de USDC a la billetera del proveedor, escribe un recibo
estructurado por cada pago confirmado y proporciona utilidades de gestión de claves:
generación de pares de claves Ed25519, generación de semillas BIP-39 y derivación de
direcciones jerárquicas deterministas (HD) por pedido. Los recibos son el registro
autorizado de compra; la tienda virtual no emitirá un token de licencia sin uno.

## Formato del token de licencia

Un token de licencia es una firma Ed25519 sobre una carga útil JSON, codificada en
base64url. Los 64 bytes de la firma se anteponen a los bytes de la carga útil antes de
codificarlos, produciendo una cadena opaca única. La carga útil registra el identificador
del producto, una fecha de vencimiento y una lista de derechos que codifican el nivel de
licencia. El servidor de distribución solo tiene la mitad pública del par de claves de
firma. La verificación no requiere ninguna llamada de red ni estado compartido: el
servidor decodifica el token, verifica la firma y comprueba el producto y el vencimiento
directamente desde el token.

## Flujo de pago y licencia

Una compra transcurre en cinco etapas:

1. El cliente visita la tienda virtual, selecciona un nivel de licencia y envía USDC a
   la billetera del proveedor en Polygon PoS — ya sea a la dirección estática del
   proveedor o a una dirección HD derivada por pedido para seguimiento de órdenes.
2. El vigilante de pagos detecta la transferencia confirmada en la cadena y escribe un
   recibo que identifica el producto y el hash de transacción del comprador.
3. El cliente consulta el punto de acceso de licencias de la tienda virtual con el hash
   de transacción; la tienda localiza el recibo y emite un token de descarga firmado.
4. El cliente presenta el token al servidor de distribución mediante un encabezado HTTP
   o un parámetro de consulta; el servidor verifica el token y transmite el binario.
5. En usos posteriores, el binario o instalador puede volver a verificar su propio token
   en el punto de acceso de clave pública del servidor de distribución sin contactar
   la tienda virtual.

## Niveles de licencia

Están disponibles dos niveles. El nivel de código abierto se distribuye bajo la licencia
Apache 2.0. El nivel comercial se distribuye bajo la Licencia de Código Funcional (FSL,
por sus siglas en inglés). Ambos niveles son compras únicas y permanentes; no existen
suscripciones. El nivel queda registrado en el campo de derechos del token y en el recibo
de pago. Los precios actuales se publican en `software.pointsav.com`.

## Lo que este sistema no hace

El sustrato gestiona pagos, emisión de tokens y entrega de binarios. No administra
suscripciones — todas las compras son transacciones únicas y perpetuas. No crea cuentas
de cliente. No implementa gestión de derechos digitales (DRM) más allá de la verificación
del token en el momento de la descarga. No restringe el acceso al código fuente: el
repositorio de GitHub es públicamente accesible bajo la licencia Apache 2.0. Los clientes
que adquieren el nivel comercial FSL reciben una clave de licencia que desbloquea la
distribución del binario; los términos del código fuente están regidos por la licencia
correspondiente, no por este sistema.
