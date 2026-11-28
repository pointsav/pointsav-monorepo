// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Numbered indirect objects, the classic cross-reference table and the trailer.

use core::fmt::Write;
use moonshot_folio_codec::sha256;
use moonshot_folio_core::{FolioError, Result};

/// Collects indirect objects. Ids are handed out first so objects can refer forward.
#[derive(Debug, Default)]
pub(crate) struct Objects {
    bodies: Vec<Option<Vec<u8>>>,
}

impl Objects {
    /// Reserve the next object number.
    pub(crate) fn alloc(&mut self) -> Result<usize> {
        if self.bodies.len() >= 8_000_000 {
            return Err(FolioError::Limit("object count"));
        }
        self.bodies.push(None);
        Ok(self.bodies.len())
    }

    /// Define the body of a reserved object.
    pub(crate) fn set(&mut self, id: usize, body: Vec<u8>) -> Result<()> {
        let slot = id
            .checked_sub(1)
            .and_then(|i| self.bodies.get_mut(i))
            .ok_or(FolioError::Invalid("unknown object id"))?;
        *slot = Some(body);
        Ok(())
    }

    /// Allocate and define in one step.
    pub(crate) fn add(&mut self, body: Vec<u8>) -> Result<usize> {
        let id = self.alloc()?;
        self.set(id, body)?;
        Ok(id)
    }

    /// Write the file: header, objects, xref table and trailer with a content-derived `/ID`.
    pub(crate) fn serialize(&self, root: usize, info: usize, max: usize) -> Result<Vec<u8>> {
        let mut out = b"%PDF-1.7\n%\xE2\xE3\xCF\xD3\n".to_vec();
        let mut offsets = Vec::with_capacity(self.bodies.len());
        for (i, body) in self.bodies.iter().enumerate() {
            let body = body
                .as_ref()
                .ok_or(FolioError::Malformed("object reserved but never defined"))?;
            offsets.push(out.len());
            out.extend_from_slice(format!("{} 0 obj\n", i.saturating_add(1)).as_bytes());
            out.extend_from_slice(body);
            out.extend_from_slice(b"\nendobj\n");
            if out.len() > max {
                return Err(FolioError::Limit("output size"));
            }
        }
        let digest = sha256(&out);
        let mut id = String::new();
        for b in digest.iter().take(16) {
            write!(id, "{b:02X}").ok();
        }
        let xref_at = out.len();
        let n = self.bodies.len().saturating_add(1);
        out.extend_from_slice(format!("xref\n0 {n}\n0000000000 65535 f \n").as_bytes());
        for off in offsets {
            out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
        }
        out.extend_from_slice(
            format!(
                "trailer\n<< /Size {n} /Root {root} 0 R /Info {info} 0 R /ID [<{id}> <{id}>] >>\nstartxref\n{xref_at}\n%%EOF\n"
            )
            .as_bytes(),
        );
        Ok(out)
    }
}

/// A stream object: the dictionary entries (without delimiters) plus `/Length` and the data.
pub(crate) fn stream(dict: &str, data: &[u8]) -> Vec<u8> {
    let mut out = format!("<< {dict} /Length {} >>\nstream\n", data.len()).into_bytes();
    out.extend_from_slice(data);
    out.extend_from_slice(b"\nendstream");
    out
}
