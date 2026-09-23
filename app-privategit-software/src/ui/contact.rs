// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Contact page for software.pointsav.com — a self-contained page, not a link
//! out to the wiki's or marketing sites' own pages (same operator instruction
//! as `disclaimer.rs`).
//!
//! Closes the highest-priority finding from the original Sovereign Editorial
//! audit (`/page/contact` returning HTTP 0 — a dead customer-facing link,
//! flagged twice: once in the original 2026-06-24 audit and once in an
//! operator dogfood-test escalation on 2026-06-30).
//!
//! `open.source@pointsav.com` is the only real, corpus-documented contact
//! channel found anywhere in this workspace (`~/Foundry/CLAUDE.md` §1, the
//! System Administrator contact). No ticketing system, phone number, or
//! physical mailing address is documented — none is invented here.

use crate::ui::Lang;
use maud::{html, Markup};

/// The full self-contained contact page (`GET /page/contact`, `GET /es/page/contact`).
pub fn contact_markup(lang: Lang) -> Markup {
    match lang {
        Lang::En => contact_markup_en(),
        Lang::Es => contact_markup_es(),
    }
}

fn contact_markup_en() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Contact us" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " — a trade name of Woodfine Capital "
                "Projects Inc."
            }

            h2 { "1. Support channel" }
            p {
                "The fastest way to reach us is " a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }

            h2 { "2. What to contact us about" }
            p {
                "License and order issues (reference your transaction hash), binary or "
                "download problems, accessibility reports, and security disclosures."
            }

            h2 { "3. Response expectations" }
            p {
                "We do not publish a guaranteed response-time commitment for this "
                "channel."
            }
        }
    }
}

fn contact_markup_es() -> Markup {
    html! {
        div."sw-legal" {
            h1 { "Cont\u{e1}ctenos" }
            p."sw-legal__lede" {
                strong { "PointSav Digital Systems" } " \u{2014} nombre comercial de Woodfine "
                "Capital Projects Inc."
            }

            h2 { "1. Canal de soporte" }
            p {
                "La forma m\u{e1}s r\u{e1}pida de contactarnos es "
                a href="mailto:open.source@pointsav.com" { "open.source@pointsav.com" } "."
            }

            h2 { "2. Sobre qu\u{e9} contactarnos" }
            p {
                "Problemas de licencias y pedidos (indique su hash de transacci\u{f3}n), "
                "problemas con binarios o descargas, reportes de accesibilidad y "
                "divulgaciones de seguridad."
            }

            h2 { "3. Expectativas de respuesta" }
            p {
                "No publicamos un compromiso garantizado de tiempo de respuesta para "
                "este canal."
            }
        }
    }
}
