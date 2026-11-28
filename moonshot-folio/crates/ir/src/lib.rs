// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-ir (L1): the document model and its validation.
//!
//! Design: BRIEF-moonshot-folio-architecture. Task F2.1.

#![forbid(unsafe_code)]

mod capability;
mod dump;
mod model;
mod validate;
mod validate_sheet;

pub use capability::{check_target, features, support, Feature, Support, Target};
pub use dump::dump;
pub use model::{
    Block, Body, BreakKind, Cell, CellValue, CmpOp, CondFormat, DefinedName, Document, FieldKind,
    Formula, Heading, Image, ImageFormat, Inline, List, Meta, PageSetup, Pagination, Para, Range,
    Row, Section, Sheet, Table, Validation, ValidationKind,
};
pub use validate::{valid_date, validate};
pub use validate_sheet::{check_formula_text, FORMULA_FUNCTIONS};
