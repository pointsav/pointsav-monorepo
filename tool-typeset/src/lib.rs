//! tool-typeset: a zero-dependency document rendering engine. One internal
//! Document model (doc.rs), rendered by two independent backends -- HTML
//! (html.rs, for review) and PDF (pdf.rs, the final deliverable, written
//! directly rather than via HTML-to-PDF conversion). Design rationale
//! (BRIEF-tool-accounting-pro-01.md decisions 34-35, project-accounting, an
//! operator-private administrative archive): one internal Doc tree, two
//! independent backends, never HTML-to-PDF conversion, standard-14 fonts
//! only, cells carry pre-formatted strings never Money/f64 -- what keeps
//! this crate domain-agnostic.
//!
//! Canonical home per decision 37: authored fresh here in pointsav-monorepo
//! (hosted at project-source), not extracted from project-accounting's local
//! scaffold copy -- nothing from that private archive crosses into this
//! crate. Any project-* archive may path-dependency on this crate.

pub mod afm;
pub mod doc;
pub mod html;
pub mod pdf;
pub mod pdf_writer;

pub use doc::{
    Align, Block, Cell, CellContent, ColWidth, Column, Doc, PageSetup, PageSize, ParaStyle,
    Register, Row, RowKind, Span, Table,
};
pub use html::render_html;
pub use pdf::render_pdf;
