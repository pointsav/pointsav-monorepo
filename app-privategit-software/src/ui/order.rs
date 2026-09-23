// SPDX-License-Identifier: FSL-1.1-ALv2
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Order status/entitlement page — `GET /order/:tx_hash`.
//!
//! Phase 2 (paid-flow UI rework): renders the three states the payment state
//! machine already returns (`resolve_license` in `main.rs`), replacing the old
//! flow where a customer had to manually poll a bare JSON endpoint and never saw
//! their license key on any page. This page is permanent and re-visitable at any
//! time from the same tx_hash — the durable ownership record.

use crate::ui::Lang;
use maud::{html, Markup, PreEscaped};

fn order_style() -> Markup {
    let css = r#".sw-or-wrap{max-width:640px;margin:0 auto;padding:40px 24px 64px;box-sizing:border-box;}
.sw-or-title{margin:0 0 24px;font-family:Georgia,"Times New Roman",serif;font-size:26px;color:#111827;}
.sw-or-card{border:1px solid #e4e7ec;border-radius:10px;padding:24px;background:#fff;box-shadow:0 1px 2px rgba(16,24,40,.04);}
.sw-or-status{display:inline-block;font-size:11px;font-weight:600;letter-spacing:.06em;text-transform:uppercase;padding:4px 10px;border-radius:999px;margin:0 0 16px;}
.sw-or-status--pending{background:#fef3c7;color:#92400e;}
.sw-or-status--confirmed{background:#ecfdf3;color:#067647;}
.sw-or-status--notfound{background:#fee4e2;color:#912018;}
.sw-or-receipt{margin:16px 0;padding:14px;border:1px dashed #d0d5dd;border-radius:8px;background:#fcfcfd;}
.sw-or-receipt__label{display:block;font-size:11px;letter-spacing:.08em;text-transform:uppercase;color:#667085;margin:0 0 6px;}
.sw-or-receipt__key{font-family:ui-monospace,SFMono-Regular,Menlo,monospace;font-size:15px;color:#234ed8;word-break:break-all;}
.sw-or-receipt__note{margin:10px 0 0;font-size:12px;color:#667085;line-height:1.5;}
.sw-or-download{display:inline-block;margin-top:8px;padding:12px 20px;border-radius:8px;background:#234ed8;color:#fff;font-size:14px;font-weight:600;text-decoration:none;}
.sw-or-download:hover{background:#0c2785;}
.sw-or-hint{font-size:13px;line-height:1.55;color:#475467;margin:16px 0 0;}
.sw-or-back{display:inline-block;margin-top:16px;font-size:13px;color:#234ed8;}"#;
    html! { style { (PreEscaped(css)) } }
}

/// Client-side auto-refresh for the pending state — the payment-confirmation moment
/// is the highest-anxiety point in a crypto purchase, so this page shouldn't be the
/// site's least responsive one (M3). `render_page` has no head-injection hook for a
/// `<meta http-equiv="refresh">`, so this uses the same inline-`<script>` pattern
/// already established elsewhere in this crate (SHA256 verify fetch, install-command
/// copy) — CSP already allows it (`script-src 'self' 'unsafe-inline'`). A single
/// one-shot `setTimeout` reload, not a poll loop: the page re-fetches the whole
/// state machine on reload anyway, so there's no separate poll endpoint to call.
fn auto_refresh_script(retry_after: u64) -> Markup {
    let ms = retry_after.saturating_mul(1000);
    html! {
        script { (PreEscaped(format!("setTimeout(function(){{location.reload();}},{ms});"))) }
    }
}

pub fn order_pending_markup(tx_hash: &str, retry_after: u64, lang: Lang) -> Markup {
    match lang {
        Lang::En => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Order status" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--pending" { "Pending" }
                    p { "Your transaction " code { (tx_hash) } " has not yet confirmed on Polygon." }
                    p."sw-or-hint" {
                        "We check your transaction automatically — this page refreshes itself "
                        "in about " (retry_after) " seconds. It stays valid forever once your \
                         payment confirms, so it's safe to leave open or come back to later."
                    }
                }
            }
            (auto_refresh_script(retry_after))
        },
        Lang::Es => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Estado del pedido" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--pending" { "Pendiente" }
                    p {
                        "Su transacci\u{f3}n " code { (tx_hash) }
                        " a\u{fa}n no se ha confirmado en Polygon."
                    }
                    p."sw-or-hint" {
                        "Verificamos su transacci\u{f3}n autom\u{e1}ticamente \u{2014} esta p\u{e1}gina "
                        "se actualizar\u{e1} en aproximadamente " (retry_after) " segundos. La p\u{e1}gina "
                        "permanece v\u{e1}lida para siempre una vez que su pago se confirme, as\u{ed} "
                        "que es seguro dejarla abierta o volver m\u{e1}s tarde."
                    }
                }
            }
            (auto_refresh_script(retry_after))
        },
    }
}

pub fn order_confirmed_markup(
    tx_hash: &str,
    license_key: &str,
    product_id: &str,
    lang: Lang,
) -> Markup {
    let download_href = format!("/order/{tx_hash}/download?product={product_id}");
    match lang {
        Lang::En => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Order status" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--confirmed" { "Confirmed" }
                    p { "Payment confirmed for " strong { (product_id) } "." }
                    div."sw-or-receipt" {
                        span."sw-or-receipt__label" { "Your receipt \u{2014} proof of ownership" }
                        span."sw-or-receipt__key" { (license_key) }
                        p."sw-or-receipt__note" {
                            "A permanent, portable record you hold \u{2014} verifiable, transferable, \
                             ours to honor but never to revoke. Keep it for your records; you do not \
                             need it to download."
                        }
                    }
                    a."sw-or-download" href=(download_href) { "Download" }
                    p."sw-or-hint" {
                        "This page is permanent \u{2014} bookmark it. Revisiting later and clicking \
                         Download again always gets you a fresh, working download link."
                    }
                }
            }
        },
        Lang::Es => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Estado del pedido" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--confirmed" { "Confirmado" }
                    p { "Pago confirmado para " strong { (product_id) } "." }
                    div."sw-or-receipt" {
                        span."sw-or-receipt__label" { "Su recibo \u{2014} comprobante de propiedad" }
                        span."sw-or-receipt__key" { (license_key) }
                        p."sw-or-receipt__note" {
                            "Un registro permanente y port\u{e1}til que usted conserva \u{2014} "
                            "verificable, transferible, que nos comprometemos a honrar pero nunca a "
                            "revocar. Consérvelo para sus registros; no lo necesita para descargar."
                        }
                    }
                    a."sw-or-download" href=(download_href) { "Descargar" }
                    p."sw-or-hint" {
                        "Esta p\u{e1}gina es permanente \u{2014} gu\u{e1}rdela en marcadores. Volver a "
                        "visitarla y hacer clic en Descargar de nuevo siempre le dar\u{e1} un enlace "
                        "de descarga nuevo y funcional."
                    }
                }
            }
        },
    }
}

pub fn order_not_found_markup(tx_hash: &str, product_id: Option<&str>, lang: Lang) -> Markup {
    let back_href = product_id
        .map(|p| lang.localize(&format!("/checkout/{p}")))
        .unwrap_or_else(|| lang.localize("/software"));
    match lang {
        Lang::En => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Order status" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--notfound" { "Not found" }
                    p { "We could not find a recognised USDC payment for " code { (tx_hash) } "." }
                    p."sw-or-hint" {
                        "Double-check the transaction hash you pasted, or confirm the payment has \
                         been sent to the correct address."
                    }
                    a."sw-or-back" href=(back_href) { "\u{2190} Back to checkout" }
                }
            }
        },
        Lang::Es => html! {
            (order_style())
            div."sw-or-wrap" {
                h1."sw-or-title" { "Estado del pedido" }
                article."sw-or-card" {
                    span."sw-or-status sw-or-status--notfound" { "No encontrado" }
                    p {
                        "No pudimos encontrar un pago reconocido en USDC para "
                        code { (tx_hash) } "."
                    }
                    p."sw-or-hint" {
                        "Verifique el hash de transacci\u{f3}n que peg\u{f3}, o confirme que el pago "
                        "se envi\u{f3} a la direcci\u{f3}n correcta."
                    }
                    a."sw-or-back" href=(back_href) { "\u{2190} Volver al pago" }
                }
            }
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pending_state_shows_retry_hint() {
        let html = order_pending_markup("0xabc", 30, Lang::En).into_string();
        assert!(html.contains("Pending"));
        assert!(html.contains("0xabc"));
        assert!(html.contains("30"));
    }

    // M3: the pending page used to instruct a manual reload with no auto-refresh at
    // all — the highest-anxiety moment in a crypto purchase deserved better.
    #[test]
    fn pending_state_auto_refreshes_via_inline_script() {
        let html = order_pending_markup("0xabc", 30, Lang::En).into_string();
        assert!(html.contains("<script>"));
        assert!(html.contains("setTimeout"));
        assert!(html.contains("location.reload()"));
        // retry_after (seconds) converted to milliseconds for setTimeout.
        assert!(html.contains("30000"));
    }

    #[test]
    fn pending_state_auto_refresh_ms_conversion_is_exact() {
        let html = order_pending_markup("0xabc", 7, Lang::En).into_string();
        assert!(html.contains("7000"));
        assert!(!html.contains("7000000"));
    }

    #[test]
    fn confirmed_state_shows_receipt_and_download_link() {
        let html = order_confirmed_markup("0xabc", "aaaa-bbbb-cccc-dddd", "os-console", Lang::En)
            .into_string();
        assert!(html.contains("Confirmed"));
        assert!(html.contains("aaaa-bbbb-cccc-dddd"));
        assert!(html.contains("os-console"));
        assert!(html.contains("href=\"/order/0xabc/download?product=os-console\""));
        assert!(html.contains("proof of ownership"));
    }

    #[test]
    fn not_found_state_links_back_to_checkout_when_product_known() {
        let html = order_not_found_markup("0xabc", Some("os-console"), Lang::En).into_string();
        assert!(html.contains("Not found"));
        assert!(html.contains("href=\"/checkout/os-console\""));
    }

    #[test]
    fn not_found_state_falls_back_to_software_page_without_product() {
        let html = order_not_found_markup("0xabc", None, Lang::En).into_string();
        assert!(html.contains("href=\"/software\""));
    }

    #[test]
    fn spanish_order_states_translate_labels_and_localize_back_links() {
        let pending = order_pending_markup("0xabc", 30, Lang::Es).into_string();
        assert!(pending.contains("Pendiente"));

        let confirmed =
            order_confirmed_markup("0xabc", "aaaa-bbbb-cccc-dddd", "os-console", Lang::Es)
                .into_string();
        assert!(confirmed.contains("Confirmado"));
        assert!(confirmed.contains("Descargar"));
        // Download link itself is not lang-prefixed (no UI to translate, machine link).
        assert!(confirmed.contains("href=\"/order/0xabc/download?product=os-console\""));

        let not_found_known =
            order_not_found_markup("0xabc", Some("os-console"), Lang::Es).into_string();
        assert!(not_found_known.contains("No encontrado"));
        assert!(not_found_known.contains("href=\"/es/checkout/os-console\""));

        let not_found_bare = order_not_found_markup("0xabc", None, Lang::Es).into_string();
        assert!(not_found_bare.contains("href=\"/es/software\""));
    }
}
