---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Punto de acceso privado para descarga binaria de clientes con licencia"
slug: topic-private-git-paid-customer-endpoint
language: es
status: draft
paired_with: TOPIC-private-git-paid-customer-endpoint.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-private-git-paid-customer-endpoint.es.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "app-privategit-source/src/main.rs; .agent/manifest.md"
research_inline: true
created: 2026-06-12
author: totebox@project-software (claude-sonnet-4-6)
---

# Punto de acceso privado para descarga binaria de clientes con licencia

El servidor de distribución de binarios es el componente de `software.pointsav.com`
que entrega binarios compilados a los clientes con licencia. Es una compuerta delgada
y sin estado: no guarda registros de pago, datos del cliente ni claves de firma. Su
única responsabilidad es verificar que el token de licencia presentado sea auténtico y
autorice el producto solicitado, y luego transmitir el archivo binario. Un cliente que
haya completado una compra y tenga un token válido puede descargar el binario
correspondiente sin ninguna interacción adicional con la infraestructura de pago.

## Estructura de rutas

El servidor de distribución organiza sus puntos de acceso en cuatro categorías:

**Descubrimiento de productos y versiones.** Un índice de productos no autenticado lista
todos los productos cuyos lanzamientos están disponibles en el servidor, y un índice de
versiones lista todas las versiones disponibles para un producto dado. Estos puntos de
acceso no requieren token de licencia y están diseñados para ser consumidos por
herramientas como gestores de paquetes, scripts de instalación y pipelines de integración
continua.

**Descarga de binario versionado.** El punto de acceso gated principal sirve un binario
para un producto, versión y plataforma específicos. Las solicitudes a este punto de acceso
requieren un token de licencia válido. Un archivo de firma Ed25519 separada para cada
binario está disponible en una ruta correspondiente y siempre es no autenticado — las
firmas separadas son públicas por diseño, lo que permite a cualquier parte verificar la
autenticidad de un binario sin tener una licencia.

**Redirección a la última versión.** Un punto de acceso de conveniencia resuelve la
versión más alta disponible para un producto y plataforma dados, y emite una redirección
a la ruta de descarga versionada. El token de licencia se pasa a través de la
redirección. La redirección solo apunta a combinaciones de plataforma y versión para las
que realmente existe un lanzamiento; no redirigirá a una versión que carezca de un
binario para la plataforma solicitada.

**Manifiesto de lanzamiento.** Un punto de acceso de metadatos por versión sirve un
manifiesto estructurado que describe el contenido de un lanzamiento. No se requiere
autenticación. Este punto de acceso es útil para herramientas que necesitan inspeccionar
qué contiene un lanzamiento antes de iniciar una descarga.

## Autenticación

El servidor de distribución acepta un token de licencia de dos formas:

**Encabezado HTTP de autorización.** El token se pasa como credencial Bearer en el
encabezado `Authorization`. Esta es la forma estándar para clientes programáticos,
instaladores automatizados y herramientas de línea de comandos que pueden establecer
encabezados de solicitud arbitrarios.

**Parámetro de consulta.** El token se pasa como parámetro `token` adjunto a la URL. Esta
forma existe específicamente para habilitar enlaces de descarga con un solo clic iniciados
desde el navegador: una tienda virtual puede generar una URL que incluya el token,
permitiendo a un cliente descargar un binario directamente desde su navegador sin
configurar ningún encabezado HTTP. Ambas formas son igualmente seguras — ni el encabezado
ni la forma de parámetro de consulta expone el token a partes adicionales más allá del
cliente y el servidor.

## Lógica de verificación

El servidor decodifica la cadena del token en base64url, separa los primeros 64 bytes como
una firma Ed25519, y verifica la firma sobre los bytes restantes utilizando la clave pública
de verificación almacenada en el servidor. Luego analiza la carga útil y comprueba dos
cosas: que el campo de producto en la carga útil coincida con el producto solicitado, y
que la fecha de vencimiento no haya pasado. Un token para un producto diferente devuelve
403. Un token cuya firma no se verifica devuelve 401. Un token vencido devuelve 403 con
un motivo que indica que el canal ha vencido. Una descripción detallada del formato del
token se encuentra en el TOPIC complementario sobre la arquitectura de pagos con
criptomonedas y emisión de licencias.

## Cadenas de plataforma

Las cadenas de plataforma siguen la convención de triples de destino de Rust. Los ejemplos
incluyen `x86_64-unknown-linux-gnu` para Linux de 64 bits en x86, `aarch64-unknown-linux-gnu`
para Linux ARM de 64 bits, y `x86_64-apple-darwin` para macOS en Intel. El servidor
mapea el nombre del producto, la cadena de versión y el triple de plataforma directamente
a una ruta de archivo en el directorio de lanzamientos. Si no se ha construido ningún
binario para la combinación solicitada de producto, versión y plataforma, el servidor
devuelve 404 con una nota indicando que la canalización de compilación aún no ha producido
ese lanzamiento. El punto de acceso de redirección a la última versión solo redirige a
cadenas de plataforma para las que realmente existe un archivo de lanzamiento.

## Gestión de claves y comportamiento a prueba de fallos

El servidor carga la clave pública de verificación Ed25519 en el inicio desde una fuente
de configuración. Si no se configura ninguna clave, el servidor no acepta silenciosamente
todos los tokens: en cambio, los puntos de acceso de descarga y verificación devuelven una
respuesta de servicio no disponible. Este comportamiento a prueba de fallos significa que
una instancia mal configurada o recién desplegada que aún no ha recibido una clave de
verificación rechazará todas las solicitudes en lugar de conceder acceso accidentalmente.
Una instancia correctamente configurada con una clave válida aceptará los tokens firmados
por la clave privada correspondiente y rechazará todos los demás.

## Lo que el servidor no hace

El servidor de distribución no registra las descargas individuales ni mantiene ningún
historial de descargas. No implementa la revocación de tokens: una vez que se emite un
token, sigue siendo válido hasta su fecha de vencimiento, y actualmente no existe una
lista de revocación. Los clientes que necesitan impedir que un token comprometido sea
utilizado deben esperar a que el token venza; la rotación de claves por parte de la
tienda virtual invalida todos los tokens emitidos anteriormente al costo de requerir
que los clientes existentes vuelvan a emitir sus tokens.

El servidor no sirve código fuente. No actúa como un servidor Git en vivo: existe un
punto de acceso stub en la ruta del protocolo Git que devuelve una redirección al
repositorio público de GitHub en lugar de intentar hacer proxy de las operaciones Git.
Esta ruta está reservada para una versión futura que puede ofrecer acceso Git autenticado
a repositorios privados.
