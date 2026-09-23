// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Disclaimer content for software.pointsav.com — a self-contained page, not
//! a link out to the wiki's or marketing sites' own disclaimer (operator
//! instruction, 2026-07-02: this site has its own content, no cross-site
//! links, because it does its own thing — software binary distribution and
//! a USDC-payment license marketplace, not an investment offering).
//!
//! Adapted from `factory-release-engineering/policies/DISCLAIMER.md` and the
//! `app-mediakit-marketing` tenant disclaimers, but rewritten for what this
//! site actually does: the LP-investment-offering sections (accredited
//! investor exemptions, Private Placement Memorandum, per-jurisdiction
//! exemption table) are dropped — no partnership units are sold here. Added:
//! a license-terms section and an on-chain USDC payment-risk section, which
//! is genuinely new content with no directly equivalent precedent elsewhere
//! in the corpus (flagged for counsel review before this is final, same as
//! the source DISCLAIMER.md's own closing note).

use crate::ui::Lang;
use maud::{html, Markup};

/// The condensed "Important information" disclosure-slot content, rendered
/// inside the footer's collapsed-by-default accordion
/// (`layout::footer`) — matches the `app-mediakit-marketing-2`
/// `DisclosureSlot` pattern: on-page and readable without JS, but collapsed
/// so it doesn't read as a second copy of the trademark paragraph directly
/// below it. Links to `disclaimer_markup` (`/page/disclaimer` /
/// `/es/page/disclaimer`) for the full text.
///
/// Translated for ES (2026-07-13, full-site-parity pass) — this is this
/// crate's own bespoke payment-risk copy, not the shared Woodfine/PointSav
/// legal source text `surface::trademark_line` still defers on; home.pointsav.com
/// verified to translate its own equivalent footer disclosure text too.
pub fn disclosure_body(lang: Lang) -> Markup {
    match lang {
        Lang::En => html! {
            p {
                "This site sells software licenses only — not an offer of securities or "
                "investment. See the full " a href="/page/disclaimer" { "Disclaimer" } "."
            }
            p {
                strong { "Payments are made in USDC on the Polygon network." }
                " Verify the address and network before sending — on-chain payments are "
                "irreversible and cannot be refunded if sent to the wrong address, wrong "
                "network, or wrong amount. License issuance depends on blockchain "
                "confirmation and may take a few minutes."
            }
        },
        Lang::Es => html! {
            p {
                "Este sitio vende \u{fa}nicamente licencias de software \u{2014} no es una oferta de "
                "valores ni de inversi\u{f3}n. Consulte el "
                a href="/es/page/disclaimer" { "Aviso legal" } " completo."
            }
            p {
                strong { "Los pagos se realizan en USDC sobre la red Polygon." }
                " Verifique la direcci\u{f3}n y la red antes de enviar \u{2014} los pagos en cadena "
                "son irreversibles y no pueden reembolsarse si se env\u{ed}an a la direcci\u{f3}n, "
                "red o monto incorrectos. La emisi\u{f3}n de la licencia depende de la "
                "confirmaci\u{f3}n en la cadena de bloques y puede tardar algunos minutos."
            }
        },
    }
}

/// The full self-contained disclaimer page (`GET /page/disclaimer`,
/// `GET /es/page/disclaimer`).
pub fn disclaimer_markup(lang: Lang) -> Markup {
    match lang {
        Lang::En => disclaimer_markup_en(),
        Lang::Es => disclaimer_markup_es(),
    }
}

fn disclaimer_markup_en() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Disclaimer" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " — a trade name of Woodfine Capital "
                "Projects Inc."
            }
            p {
                "This site distributes software binaries and issues product licenses. "
                "By using this site, you acknowledge the following."
            }

            h2 { "1. No warranty" }
            p {
                "Software we make available on this site is provided \"as is,\" without "
                "warranty of any kind, express or implied, including but not limited to "
                "warranties of merchantability, fitness for a particular purpose, and "
                "non-infringement. PointSav Digital Systems does not guarantee that any "
                "binary distributed here is free of defects or uninterrupted in operation. "
                "Product descriptions, including BETA status, reflect intended "
                "capabilities and may change without notice."
            }

            h2 { "2. Not an offer of securities" }
            p {
                "Nothing on this site constitutes an offer to sell, or a solicitation of "
                "an offer to buy, any security, partnership interest, or investment "
                "product. This site sells software licenses only. Woodfine Capital "
                "Projects Inc.'s securities offerings are described separately at its own "
                "investor-facing properties and are not made through this site."
            }

            h2 { "3. License terms" }
            p {
                "Each product distributed here is governed by its own license terms, "
                "referenced from that product's catalog entry: open-source licenses for "
                "products distributed under one of these (Apache-2.0, AGPL-3.0-or-later, "
                "or as otherwise stated for proprietary components), and the applicable "
                "commercial license terms for paid tiers. Purchasing a license here does "
                "not transfer ownership of, or any right in, Woodfine Capital Projects "
                "Inc. or its affiliates."
            }

            h2 { "4. Payment, on the Polygon network, in USDC" }
            p {
                "You purchase licenses here by sending USDC on the Polygon "
                "network to the address shown at time of purchase. You are solely "
                "responsible for verifying that address, the network (Polygon, not "
                "Ethereum mainnet or any other chain), and the payment amount before "
                "sending funds. On-chain transactions are irreversible. PointSav Digital "
                "Systems cannot reverse, refund, or recover a payment sent to the wrong "
                "address, on the wrong network, or for the wrong amount. License "
                "issuance depends on confirmation of your transaction on the Polygon "
                "network, which is outside PointSav Digital Systems' control and may be "
                "delayed by network conditions."
            }

            h2 { "5. Restrictions on use of materials" }
            p {
                "Information on this site may be reproduced in hard copy for personal "
                "reference only, provided that all copyright and proprietary notices are "
                "retained. Other reproduction or distribution, in any form or by any "
                "means, without the express written permission of Woodfine Capital "
                "Projects Inc. is prohibited."
            }

            h2 { "6. Forward-looking information" }
            p {
                "We may describe planned or intended product capabilities, "
                "including BETA features and pricing. Words such as \"plans,\" "
                "\"intends,\" \"expects,\" \"may,\" \"will,\" and \"targets\" are intended "
                "to identify forward-looking statements. Actual availability, "
                "functionality, and pricing may differ materially from what is "
                "described, and PointSav Digital Systems disclaims any obligation to "
                "update these statements except where required by applicable law."
            }

            h2 { "7. Jurisdictional restrictions" }
            p {
                "Access to and use of this site, including the ability to purchase a "
                "license or send cryptocurrency payment, may be restricted or unlawful "
                "in some jurisdictions. You are responsible for determining whether "
                "your use of this site complies with the laws of your jurisdiction."
            }
        }
    }
}

/// **Translation note (2026-07-13):** this is this crate's own self-authored
/// disclaimer, not a translation of `factory-release-engineering`'s shared
/// Woodfine LP-offering `DISCLAIMER.md` (that document's own canonical Spanish
/// version is separately staged/unratified — see `ui::lang` module docs). This
/// page's content never depended on that shared source, so translating it here
/// carries none of that risk; ship-now-refine-later applies only to the
/// trademark/copyright line still deferring to the shared source.
fn disclaimer_markup_es() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Aviso legal" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " \u{2014} nombre comercial de Woodfine "
                "Capital Projects Inc."
            }
            p {
                "Este sitio distribuye binarios de software y emite licencias de "
                "producto. Al usar este sitio, usted reconoce lo siguiente."
            }

            h2 { "1. Sin garant\u{ed}a" }
            p {
                "El software que ponemos a disposici\u{f3}n en este sitio se proporciona \u{ab}tal cual\u{bb}, sin "
                "garant\u{ed}a de ning\u{fa}n tipo, expresa o impl\u{ed}cita, incluyendo pero sin "
                "limitarse a garant\u{ed}as de comerciabilidad, idoneidad para un prop\u{f3}sito "
                "particular y no infracci\u{f3}n. PointSav Digital Systems no garantiza que "
                "ning\u{fa}n binario distribuido aqu\u{ed} est\u{e9} libre de defectos o funcione sin "
                "interrupciones. Las descripciones de producto, incluido el estado BETA, "
                "reflejan capacidades previstas y pueden cambiar sin previo aviso."
            }

            h2 { "2. No es una oferta de valores" }
            p {
                "Nada en este sitio constituye una oferta de venta, ni una solicitud de "
                "oferta de compra, de ning\u{fa}n valor, participaci\u{f3}n societaria o producto "
                "de inversi\u{f3}n. Este sitio vende \u{fa}nicamente licencias de software. Las "
                "ofertas de valores de Woodfine Capital Projects Inc. se describen por "
                "separado en sus propias plataformas dirigidas a inversionistas y no se "
                "realizan a trav\u{e9}s de este sitio."
            }

            h2 { "3. T\u{e9}rminos de licencia" }
            p {
                "Cada producto distribuido aqu\u{ed} se rige por sus propios t\u{e9}rminos de "
                "licencia, referenciados desde la ficha de cat\u{e1}logo de ese producto: "
                "licencias de c\u{f3}digo abierto para los productos distribuidos bajo una de "
                "ellas (Apache-2.0, AGPL-3.0-or-later, u otra indicada para los componentes "
                "propietarios), y los t\u{e9}rminos de licencia comercial aplicables para los "
                "niveles de pago. Comprar una licencia aqu\u{ed} no transfiere la propiedad "
                "de, ni ning\u{fa}n derecho sobre, Woodfine Capital Projects Inc. ni sus "
                "afiliadas."
            }

            h2 { "4. Pago, en la red Polygon, en USDC" }
            p {
                "Usted compra las licencias aqu\u{ed} enviando USDC en la red Polygon "
                "a la direcci\u{f3}n mostrada al momento de la compra. Usted es el \u{fa}nico "
                "responsable de verificar esa direcci\u{f3}n, la red (Polygon, no Ethereum "
                "mainnet ni ninguna otra cadena) y el monto del pago antes de enviar los "
                "fondos. Las transacciones en cadena son irreversibles. PointSav Digital "
                "Systems no puede revertir, reembolsar ni recuperar un pago enviado a la "
                "direcci\u{f3}n incorrecta, en la red incorrecta o por el monto incorrecto. "
                "La emisi\u{f3}n de la licencia depende de la confirmaci\u{f3}n de su transacci\u{f3}n "
                "en la red Polygon, lo cual est\u{e1} fuera del control de PointSav Digital "
                "Systems y puede retrasarse por condiciones de la red."
            }

            h2 { "5. Restricciones sobre el uso de materiales" }
            p {
                "La informaci\u{f3}n de este sitio puede reproducirse en copia impresa "
                "\u{fa}nicamente para referencia personal, siempre que se conserven todos los "
                "avisos de copyright y de propiedad. Queda prohibida cualquier otra "
                "reproducci\u{f3}n o distribuci\u{f3}n, en cualquier forma o por cualquier medio, "
                "sin el permiso expreso y por escrito de Woodfine Capital Projects Inc."
            }

            h2 { "6. Informaci\u{f3}n prospectiva" }
            p {
                "Podemos describir capacidades de producto planeadas o "
                "previstas, incluyendo funciones y precios en fase BETA. Palabras como "
                "\u{ab}planea\u{bb}, \u{ab}pretende\u{bb}, \u{ab}espera\u{bb}, \u{ab}puede\u{bb}, \u{ab}ser\u{e1}\u{bb} y "
                "\u{ab}tiene como meta\u{bb} tienen como fin identificar declaraciones "
                "prospectivas. La disponibilidad, funcionalidad y precios reales pueden "
                "diferir materialmente de lo descrito, y PointSav Digital Systems "
                "renuncia a cualquier obligaci\u{f3}n de actualizar estas declaraciones salvo "
                "cuando lo exija la ley aplicable."
            }

            h2 { "7. Restricciones jurisdiccionales" }
            p {
                "El acceso y uso de este sitio, incluida la posibilidad de comprar una "
                "licencia o enviar un pago en criptomoneda, puede estar restringido o "
                "prohibido en algunas jurisdicciones. Usted es responsable de determinar "
                "si su uso de este sitio cumple con las leyes de su jurisdicci\u{f3}n."
            }
        }
    }
}
