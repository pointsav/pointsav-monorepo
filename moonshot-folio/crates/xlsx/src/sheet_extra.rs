// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Conditional formatting and data validation elements of a worksheet.

use crate::sheet::cell_ref;
use crate::styles::{Registry, StyleMap};
use crate::xml::escape_into;
use core::fmt::Write;
use moonshot_folio_core::Result;
use moonshot_folio_ir::{CmpOp, Range, Sheet, ValidationKind};

/// `A1:B2`, or `A1` for a single cell.
fn sqref(r: &Range) -> String {
    let (r0, c0) = (usize::try_from(r.row).unwrap_or(0), r.col);
    let r1 = usize::try_from(r.row.saturating_add(r.rows).saturating_sub(1)).unwrap_or(0);
    let c1 = r.col.saturating_add(r.cols).saturating_sub(1);
    if r.rows == 1 && r.cols == 1 {
        cell_ref(r0, c0)
    } else {
        format!("{}:{}", cell_ref(r0, c0), cell_ref(r1, c1))
    }
}

fn op(o: CmpOp) -> &'static str {
    match o {
        CmpOp::Gt => "greaterThan",
        CmpOp::Ge => "greaterThanOrEqual",
        CmpOp::Lt => "lessThan",
        CmpOp::Le => "lessThanOrEqual",
        CmpOp::Eq => "equal",
        CmpOp::Ne => "notEqual",
    }
}

/// `conditionalFormatting` elements, one per rule, priorities in order.
pub(crate) fn conditional(
    out: &mut String,
    s: &Sheet,
    map: &StyleMap,
    reg: &mut Registry,
) -> Result<()> {
    for (i, c) in s.cond_formats.iter().enumerate() {
        let dxf = reg.dxf_index(map, &c.role)?;
        write!(
            out,
            "<conditionalFormatting sqref=\"{}\"><cfRule type=\"cellIs\" dxfId=\"{dxf}\" priority=\"{}\" operator=\"{}\"><formula>{}</formula></cfRule></conditionalFormatting>",
            sqref(&c.range),
            i.saturating_add(1),
            op(c.op),
            c.value
        )
        .ok();
    }
    Ok(())
}

/// The `dataValidations` element, if any rule exists.
pub(crate) fn validations(out: &mut String, s: &Sheet) -> Result<()> {
    if s.validations.is_empty() {
        return Ok(());
    }
    write!(out, "<dataValidations count=\"{}\">", s.validations.len()).ok();
    for v in &s.validations {
        match &v.kind {
            ValidationKind::List(items) => {
                write!(out, "<dataValidation type=\"list\" allowBlank=\"1\" showErrorMessage=\"1\" sqref=\"{}\"><formula1>&quot;", sqref(&v.range)).ok();
                escape_into(out, &items.join(","))?;
                out.push_str("&quot;</formula1></dataValidation>");
            }
            ValidationKind::WholeNumber { min, max } => {
                write!(out, "<dataValidation type=\"whole\" operator=\"between\" allowBlank=\"1\" showErrorMessage=\"1\" sqref=\"{}\"><formula1>{min}</formula1><formula2>{max}</formula2></dataValidation>", sqref(&v.range)).ok();
            }
        }
    }
    out.push_str("</dataValidations>");
    Ok(())
}
