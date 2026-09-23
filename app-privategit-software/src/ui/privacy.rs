// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Privacy page for software.pointsav.com — a self-contained page, not a link
//! out to the wiki's or marketing sites' own privacy pages (same operator
//! instruction as `disclaimer.rs`: this site has its own content, no
//! cross-site links, because it does its own thing — software binary
//! distribution and a USDC-payment license marketplace).
//!
//! Content is deliberately narrow: only what this crate actually does
//! (payment-verification data, on-chain USDC records, license issuance) is
//! described. No retention policy or physical mailing address is invented —
//! neither is documented anywhere in the corpus as of 2026-07-03.

use crate::ui::Lang;
use maud::{html, Markup};

/// The full self-contained privacy page (`GET /page/privacy`, `GET /es/page/privacy`).
pub fn privacy_markup(lang: Lang) -> Markup {
    match lang {
        Lang::En => privacy_markup_en(),
        Lang::Es => privacy_markup_es(),
    }
}

fn privacy_markup_en() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Privacy" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " — a trade name of Woodfine Capital "
                "Projects Inc."
            }
            p {
                "PointSav distributes software binaries and issues product licenses. "
                "This page describes what data we collect and how we use it."
            }

            h2 { "1. What we collect" }
            p {
                "On this site, we collect payment-verification data only: the transaction hash "
                "and wallet address you supply when purchasing a license, and the receipt "
                "and claim records generated from a confirmed payment. We don't "
                "use tracking cookies for analytics or advertising. We do use a session "
                "token for signed-in account/license-status functionality where "
                "applicable — a functional exception, not a tracking mechanism."
            }

            h2 { "2. On-chain data" }
            p {
                "Payments are made in USDC on the Polygon network. Once submitted, a "
                "transaction and its associated wallet address are recorded permanently "
                "and publicly on the Polygon blockchain. This is inherent to how "
                "on-chain payment works and cannot be retracted or deleted by PointSav "
                "Digital Systems."
            }

            h2 { "3. How we use it" }
            p {
                "We use payment-verification data to issue and validate product "
                "licenses, answer order-status lookups, and maintain the "
                "transaction-log bookkeeping our business is required to keep. We don't "
                "sell it or share it with third parties for marketing purposes."
            }

            h2 { "4. Data retention and access" }
            p {
                "We store receipt and claim records on our own "
                "infrastructure. We haven't documented a specific retention period "
                "for this site yet — a genuine open item, not an oversight we're "
                "concealing. If you have a question about a specific record, contact "
                "us using the details below."
            }

            h2 { "5. Contact" }
            p {
                "Send questions about this page to " a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }
        }
    }
}

fn privacy_markup_es() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Privacidad" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " \u{2014} nombre comercial de Woodfine "
                "Capital Projects Inc."
            }
            p {
                "PointSav distribuye binarios de software y emite licencias de "
                "producto. Esta p\u{e1}gina describe qu\u{e9} datos recopilamos y c\u{f3}mo los usamos."
            }

            h2 { "1. Qu\u{e9} recopilamos" }
            p {
                "En este sitio, recopilamos \u{fa}nicamente datos de verificaci\u{f3}n de pago: el hash "
                "de transacci\u{f3}n y la direcci\u{f3}n de billetera que usted proporciona al "
                "comprar una licencia, y los registros de recibo y reclamo generados a "
                "partir de un pago confirmado. No usamos cookies de seguimiento "
                "con fines de anal\u{ed}tica o publicidad. S\u{ed} usamos un token de sesi\u{f3}n para la "
                "funcionalidad de estado de cuenta/licencia cuando corresponde \u{2014} una "
                "excepci\u{f3}n funcional, no un mecanismo de rastreo."
            }

            h2 { "2. Datos en cadena" }
            p {
                "Los pagos se realizan en USDC sobre la red Polygon. Una vez enviada, "
                "una transacci\u{f3}n y su direcci\u{f3}n de billetera asociada quedan "
                "registradas de forma permanente y p\u{fa}blica en la cadena de bloques de "
                "Polygon. Esto es inherente a c\u{f3}mo funciona el pago en cadena y no puede "
                "ser retirado ni eliminado por PointSav Digital Systems."
            }

            h2 { "3. C\u{f3}mo los usamos" }
            p {
                "Usamos los datos de verificaci\u{f3}n de pago para emitir y validar "
                "licencias de producto, responder consultas de estado de pedido, y "
                "mantener el registro de transacciones que nuestro negocio est\u{e1} obligado "
                "a conservar. No los vendemos ni los compartimos con terceros con fines de "
                "marketing."
            }

            h2 { "4. Retenci\u{f3}n y acceso a los datos" }
            p {
                "Almacenamos los registros de recibo y reclamo en nuestra propia "
                "infraestructura. Todav\u{ed}a no hemos documentado un per\u{ed}odo de "
                "retenci\u{f3}n espec\u{ed}fico para este sitio \u{2014} un punto genuinamente "
                "pendiente, no un descuido que ocultamos. Si tiene alguna pregunta sobre un "
                "registro espec\u{ed}fico, cont\u{e1}ctenos usando los datos a continuaci\u{f3}n."
            }

            h2 { "5. Contacto" }
            p {
                "Env\u{ed}e sus preguntas sobre esta p\u{e1}gina a "
                a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }
        }
    }
}
