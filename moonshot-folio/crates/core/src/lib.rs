// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! moonshot-folio-core (L0): units (i64 EMU), checked arithmetic, limits, errors, Ink.
//!
//! Design: BRIEF-moonshot-folio-architecture. Implemented so far: `Limits`, `FolioError` (F1.2), `Emu`, `Ink` (F2.1).

#![forbid(unsafe_code)]

mod error;
mod limits;
mod units;

pub use error::{FolioError, Result};
pub use limits::Limits;
pub use units::{Emu, Ink, EMU_PER_INCH, EMU_PER_MM, EMU_PER_POINT, EMU_PER_TWIP};
