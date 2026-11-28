// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! `fuzz` gate: seeded mutation fuzzing of the untrusted-input parsers.
//!
//! A mutant that makes a parser panic fails the gate and is printed as a reproducer.
//! Usage: `cargo xtask fuzz [--iters N] [--seed S]` (defaults: 400 mutants per seed file, seed 1).

use crate::util::{read_bytes, walk, Res, Verdict};
use moonshot_folio_codec::{
    crc32, decode_png, inflate, parse_jpeg_header, zlib_decompress, ZipArchive,
};
use moonshot_folio_core::Limits;
use moonshot_folio_font::Font;
use std::fmt::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;
use std::time::{Duration, Instant};

/// A mutant that takes longer than this fails the gate (parsers must be linear in input size).
const SLOW: Duration = Duration::from_secs(2);

/// xorshift64* generator: tiny, deterministic, dependency free.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A value in `0..n` (0 when `n` is 0).
    fn below(&mut self, n: usize) -> usize {
        let n = u64::try_from(n).unwrap_or(u64::MAX);
        usize::try_from(self.next().checked_rem(n).unwrap_or(0)).unwrap_or(0)
    }
}

/// Limits small enough that no mutant can allocate much.
fn tight() -> Limits {
    Limits::default()
        .with_max_file_bytes(1 << 20)
        .with_max_entry_bytes(1 << 20)
        .with_max_total_bytes(1 << 21)
        .with_max_image_pixels(1 << 18)
}

/// One mutation of `seed`.
fn mutate(seed: &[u8], rng: &mut Rng) -> Vec<u8> {
    let mut v = seed.to_vec();
    for _ in 0..=rng.below(4) {
        let at = rng.below(v.len());
        match rng.below(6) {
            0 => {
                if let Some(b) = v.get_mut(at) {
                    *b ^= 1_u8.wrapping_shl(u32::try_from(rng.below(8)).unwrap_or(0));
                }
            }
            1 => {
                let byte = u8::try_from(rng.below(256)).unwrap_or(0);
                if let Some(b) = v.get_mut(at) {
                    *b = byte;
                }
            }
            2 => v.truncate(at),
            3 => v.insert(at, u8::try_from(rng.below(256)).unwrap_or(0)),
            4 => {
                if at < v.len() {
                    v.remove(at);
                }
            }
            _ => {
                let end = at.saturating_add(rng.below(32)).min(v.len());
                let chunk: Vec<u8> = v.get(at..end).unwrap_or(&[]).to_vec();
                let to = rng.below(v.len().saturating_add(1));
                v.splice(to..to, chunk);
            }
        }
    }
    v
}

/// Recompute every PNG chunk CRC so a mutant gets past the checksum and reaches the decoder.
fn repair_png_crcs(v: &mut [u8]) {
    let mut pos = 8_usize;
    while let Some(head) = v.get(pos..pos.saturating_add(4)) {
        let len = usize::try_from(u32::from_be_bytes(
            <[u8; 4]>::try_from(head).unwrap_or([0; 4]),
        ))
        .unwrap_or(usize::MAX);
        let body_end = pos.saturating_add(8).saturating_add(len);
        let crc_end = body_end.saturating_add(4);
        let Some(covered) = v.get(pos.saturating_add(4)..body_end) else {
            break;
        };
        let crc = crc32(covered).to_be_bytes();
        let Some(slot) = v.get_mut(body_end..crc_end) else {
            break;
        };
        slot.copy_from_slice(&crc);
        pos = crc_end;
    }
}

/// Run every parser that applies to `kind` on `data`.
fn exercise(kind: &str, data: &[u8]) {
    let limits = tight();
    match kind {
        "zip" => {
            if let Ok(z) = ZipArchive::parse(data, &limits) {
                for e in z.entries() {
                    let _ = z.read(e);
                }
            }
        }
        "inflate" => {
            let _ = inflate(data, 1 << 20);
        }
        "zlib" => {
            let _ = zlib_decompress(data, 1 << 20);
        }
        "png" => {
            if let Ok(img) = decode_png(data, &limits) {
                let _ = img.to_rgba8(&limits);
            }
        }
        "ttf" => {
            if let Ok(f) = Font::parse(data, &limits) {
                for c in [
                    ' ',
                    'A',
                    'z',
                    '\u{E9}',
                    '\u{20AC}',
                    '\u{FFFF}',
                    '\u{10FFFF}',
                ] {
                    let _ = f.glyph_for(c);
                }
                for g in 0..f.num_glyphs().min(64) {
                    let _ = (f.advance(g), f.glyph_data(g));
                }
                let _ = (
                    f.fs_type(),
                    f.italic_angle_fixed(),
                    f.is_fixed_pitch(),
                    f.postscript_name(),
                );
                let _ = moonshot_folio_font::check_embedding(&f, true);
                let _ = moonshot_folio_font::map_text(
                    &f,
                    "A B\u{E9}",
                    moonshot_folio_font::MissingGlyph::Notdef,
                );
                let gids: Vec<u16> = (0..f.num_glyphs().min(8)).collect();
                if let Ok(out) = moonshot_folio_font::subset(&f, &gids, &limits) {
                    let _ = Font::parse(&out, &limits);
                }
            }
        }
        _ => {
            let _ = parse_jpeg_header(data, &limits);
        }
    }
}

/// Lowercase hex of the first `n` bytes.
fn hex_prefix(b: &[u8], n: usize) -> String {
    let mut s = String::new();
    for x in b.iter().take(n) {
        let _ = write!(s, "{x:02x}");
    }
    s
}

/// Parse `--iters` and `--seed` from the arguments.
fn options(args: &[String]) -> (usize, u64) {
    let get = |flag: &str| {
        args.iter()
            .position(|a| a == flag)
            .and_then(|i| args.get(i.saturating_add(1)))
    };
    let iters = get("--iters").and_then(|v| v.parse().ok()).unwrap_or(400);
    let seed = get("--seed").and_then(|v| v.parse().ok()).unwrap_or(1);
    (iters, seed)
}

/// Run the gate.
pub(crate) fn run(root: &Path, args: &[String]) -> Res<Verdict> {
    let (iters, seed) = options(args);
    let corpus = root.join("assurance/fuzz-corpus");
    let mut files = walk(&corpus)?;
    files.retain(|p| p.extension().is_some_and(|e| e == "bin"));
    if files.is_empty() {
        return Ok(Verdict::Na("no seed corpus".to_string()));
    }
    std::panic::set_hook(Box::new(|_| {}));
    let mut failures = Vec::new();
    let mut execs = 0_usize;
    for file in &files {
        let kind = file
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let bytes = read_bytes(file)?;
        let mut rng = Rng(seed.wrapping_add(u64::try_from(bytes.len()).unwrap_or(0)) | 1);
        for i in 0..iters {
            let mut mutant = mutate(&bytes, &mut rng);
            if kind == "png" && rng.below(2) == 0 {
                repair_png_crcs(&mut mutant);
            }
            let started = Instant::now();
            execs = execs.saturating_add(1);
            let panicked = catch_unwind(AssertUnwindSafe(|| exercise(kind, &mutant))).is_err();
            let slow = started.elapsed() > SLOW;
            if panicked || slow {
                let hex = hex_prefix(&mutant, 256);
                let what = if panicked { "panic" } else { "slow input" };
                failures.push(format!(
                    "{what}: {} seed {seed} iter {i} len {}: {hex}",
                    file.display(),
                    mutant.len()
                ));
                break;
            }
        }
    }
    let _ = std::panic::take_hook();
    if failures.is_empty() {
        return Ok(Verdict::Pass(format!(
            "{execs} mutants of {} seeds, seed {seed}, no panic",
            files.len()
        )));
    }
    Ok(Verdict::Fail(failures))
}
