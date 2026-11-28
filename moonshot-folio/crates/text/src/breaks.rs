// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! UAX#14 line-break opportunities (rules LB1 to LB31; Unicode 15.0 property data).

use crate::class::{class_of, Lb};

/// Whether a line must end at a break or merely may.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BreakKind {
    /// A hard break (LB4, LB5): a line must end here.
    Mandatory,
    /// A break opportunity: a line may end here.
    Allowed,
}

/// A line-break opportunity located before the byte at `offset`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Break {
    /// Byte offset into the text; always on a `char` boundary, never zero.
    pub offset: usize,
    /// Whether the break is mandatory or only allowed.
    pub kind: BreakKind,
}

/// The classes after which a CM or ZWJ does not attach (LB9 exclusions).
fn no_attach(c: Lb) -> bool {
    matches!(c, Lb::BK | Lb::CR | Lb::LF | Lb::NL | Lb::SP | Lb::ZW)
}

/// Running context for the break decision at each position.
#[allow(clippy::struct_excessive_bools)] // independent rule flags, not a state machine
struct State {
    /// Raw class of the previous character (`XX` at start of text).
    prev_raw: Lb,
    /// Effective class of the previous cluster after LB9/LB10 (`XX` at start).
    pe: Lb,
    /// Effective class of the cluster before that.
    pe2: Lb,
    /// Effective class of the last non-space cluster, skipping a trailing run of spaces.
    pb: Lb,
    /// LB8: previous cluster is ZW, or spaces following ZW.
    zw_sp: bool,
    /// Number of consecutive regional indicators ending at the previous cluster.
    ri_run: u32,
    /// LB20a: previous cluster is a hyphen that begins a word.
    hy_start: bool,
    /// LB15a: the last non-space cluster is an initial quotation mark in opening context.
    pi_open: bool,
    /// Nothing has been seen yet.
    sot: bool,
}

impl State {
    fn new() -> Self {
        Self {
            prev_raw: Lb::XX,
            pe: Lb::XX,
            pe2: Lb::XX,
            pb: Lb::XX,
            zw_sp: false,
            ri_run: 0,
            hy_start: false,
            pi_open: false,
            sot: true,
        }
    }

    /// Fold character `ch` of raw class `raw` into the state.
    fn advance(&mut self, raw: Lb, ch: char) {
        let attach = matches!(raw, Lb::CM | Lb::ZWJ) && !self.sot && !no_attach(self.prev_raw);
        self.prev_raw = raw;
        if attach {
            return; // LB9: X (CM | ZWJ)* is treated as X.
        }
        let cc = if matches!(raw, Lb::CM | Lb::ZWJ) {
            Lb::AL // LB10
        } else {
            raw
        };
        let start_ok = self.sot || no_attach(self.pe) || matches!(self.pe.base(), Lb::CB | Lb::GL);
        self.hy_start = start_ok && (cc == Lb::HY || (cc == Lb::BA && ch == '\u{2010}'));
        if cc == Lb::QU_PI {
            let ctx = matches!(
                self.pe.base(),
                Lb::BK | Lb::CR | Lb::LF | Lb::NL | Lb::OP | Lb::QU | Lb::GL | Lb::SP | Lb::ZW
            );
            self.pi_open = self.sot || ctx;
        } else if cc != Lb::SP {
            self.pi_open = false;
        }
        self.pb = if cc == Lb::SP && self.pe == Lb::SP {
            self.pb
        } else if cc == Lb::SP {
            self.pe
        } else {
            cc
        };
        self.zw_sp = cc == Lb::ZW || (cc == Lb::SP && self.zw_sp);
        self.ri_run = if cc == Lb::RI {
            self.ri_run.saturating_add(1)
        } else {
            0
        };
        self.pe2 = self.pe;
        self.pe = cc;
        self.sot = false;
    }
}

/// Outcome of the early rules.
enum Step {
    /// The position is settled: `Some` breaks, `None` does not.
    Done(Option<BreakKind>),
    /// Continue with the pair rules.
    Go,
}

/// LB4 to LB8a and the LB9/LB10 gate.
fn early(st: &State, raw: Lb) -> Step {
    let p = st.prev_raw;
    if p == Lb::CR && raw == Lb::LF {
        return Step::Done(None); // LB5
    }
    if p.is_hard() {
        return Step::Done(Some(BreakKind::Mandatory)); // LB4, LB5
    }
    if raw.is_hard() || matches!(raw, Lb::SP | Lb::ZW) {
        return Step::Done(None); // LB6, LB7
    }
    if st.zw_sp {
        return Step::Done(Some(BreakKind::Allowed)); // LB8
    }
    if p == Lb::ZWJ || (matches!(raw, Lb::CM | Lb::ZWJ) && !no_attach(p)) {
        return Step::Done(None); // LB8a, LB9
    }
    Step::Go
}

/// LB11 to LB17: prohibitions that apply before the space rule. `after` is the class of the
/// next character after the current one, ignoring CM and ZWJ (`None` at the end of text).
fn glue_rules(st: &State, c: Lb, p: Lb, after: Option<Lb>) -> Step {
    let cb = c.base();
    let pb = st.pb.base();
    if p.base() == Lb::WJ || cb == Lb::WJ // LB11
        || p == Lb::GL // LB12
        || (cb == Lb::GL && !matches!(p, Lb::SP | Lb::BA | Lb::HY)) // LB12a
        || matches!(cb, Lb::CL | Lb::CP | Lb::EX | Lb::SY) // LB13
        || pb == Lb::OP // LB14
        || st.pi_open // LB15a
        || (c == Lb::QU_PF && after.is_none_or(closes_pf))
    // LB15b
    {
        return Step::Done(None);
    }
    if cb == Lb::IS {
        // LB15c: SP ÷ IS NU; LB15d: × IS.
        return Step::Done((p == Lb::SP && after == Some(Lb::NU)).then_some(BreakKind::Allowed));
    }
    if (matches!(pb, Lb::CL | Lb::CP) && cb == Lb::NS) // LB16
        || (pb == Lb::B2 && cb == Lb::B2)
    // LB17
    {
        return Step::Done(None);
    }
    Step::Go
}

/// LB15b: classes that may follow a final quotation mark without a break before it.
fn closes_pf(a: Lb) -> bool {
    matches!(
        a.base(),
        Lb::SP
            | Lb::GL
            | Lb::WJ
            | Lb::CL
            | Lb::QU
            | Lb::CP
            | Lb::EX
            | Lb::IS
            | Lb::SY
            | Lb::BK
            | Lb::CR
            | Lb::LF
            | Lb::NL
            | Lb::ZW
    )
}

/// LB20a, LB21 to LB22.
fn hyphen_rules(st: &State, c: Lb, p: Lb) -> bool {
    (st.hy_start && c == Lb::AL) // LB20a
        || matches!(c, Lb::BA | Lb::HY | Lb::NS) // LB21
        || p == Lb::BB // LB21
        || (matches!(p, Lb::HY | Lb::BA) && st.pe2 == Lb::HL) // LB21a
        || (p == Lb::SY && c == Lb::HL) // LB21b
        || c == Lb::IN // LB22
}

/// LB23 to LB25: prohibitions around letters, numbers and affixes.
fn number_rules(p: Lb, c: Lb) -> bool {
    let al = |x: Lb| matches!(x, Lb::AL | Lb::HL);
    let id = |x: Lb| matches!(x, Lb::ID | Lb::EB | Lb::EM);
    (al(p) && c == Lb::NU) || (p == Lb::NU && al(c)) // LB23
        || (p == Lb::PR && id(c)) || (id(p) && c == Lb::PO) // LB23a
        || (matches!(p, Lb::PR | Lb::PO) && al(c)) // LB24
        || (al(p) && matches!(c, Lb::PR | Lb::PO)) // LB24
        || (matches!(p, Lb::CL | Lb::CP | Lb::NU) && matches!(c, Lb::PO | Lb::PR)) // LB25
        || (matches!(p, Lb::PO | Lb::PR) && matches!(c, Lb::OP | Lb::NU)) // LB25
        || (matches!(p, Lb::HY | Lb::IS | Lb::NU | Lb::SY) && c == Lb::NU) // LB25
}

/// LB26 to LB29: Hangul and letters.
fn script_rules(p: Lb, c: Lb) -> bool {
    let hangul = |x: Lb| matches!(x, Lb::JL | Lb::JV | Lb::JT | Lb::H2 | Lb::H3);
    let al = |x: Lb| matches!(x, Lb::AL | Lb::HL);
    (p == Lb::JL && matches!(c, Lb::JL | Lb::JV | Lb::H2 | Lb::H3)) // LB26
        || (matches!(p, Lb::JV | Lb::H2) && matches!(c, Lb::JV | Lb::JT)) // LB26
        || (matches!(p, Lb::JT | Lb::H3) && c == Lb::JT) // LB26
        || (hangul(p) && c == Lb::PO) // LB27
        || (p == Lb::PR && hangul(c)) // LB27
        || (al(p) && al(c)) // LB28
        || (p == Lb::IS && al(c)) // LB29
}

/// LB30 to LB30b with the East Asian and emoji refinements visible (`c`, `p` unfolded).
fn extended_rules(st: &State, c: Lb, p: Lb) -> bool {
    let al = |x: Lb| matches!(x, Lb::AL | Lb::HL);
    (matches!(p, Lb::AL | Lb::HL | Lb::NU) && c == Lb::OP) // LB30
        || (p == Lb::CP && (al(c) || c == Lb::NU)) // LB30
        || (c == Lb::RI && p == Lb::RI && st.ri_run % 2 == 1) // LB30a
        || (matches!(p, Lb::EB | Lb::ID_PICT) && c == Lb::EM) // LB30b
}

/// Decide the break at one position; `raw` is the class of the character after it.
fn decide(st: &State, raw: Lb, after: Option<Lb>) -> Option<BreakKind> {
    if let Step::Done(v) = early(st, raw) {
        return v;
    }
    let cf = if matches!(raw, Lb::CM | Lb::ZWJ) {
        Lb::AL
    } else {
        raw
    };
    let pf = st.pe;
    let (c, p) = (cf.base(), pf.base());
    if let Step::Done(v) = glue_rules(st, cf, pf, after) {
        return v;
    }
    if p == Lb::SP {
        return Some(BreakKind::Allowed); // LB18
    }
    if p == Lb::QU || c == Lb::QU {
        return None; // LB19
    }
    if c == Lb::CB || p == Lb::CB {
        return Some(BreakKind::Allowed); // LB20
    }
    if hyphen_rules(st, c, p)
        || number_rules(p, c)
        || script_rules(p, c)
        || extended_rules(st, cf, pf)
    {
        return None;
    }
    Some(BreakKind::Allowed) // LB31
}

/// All line-break opportunities of `text`, in increasing byte order.
///
/// Implements UAX#14 LB1 to LB31 (including LB9/LB10, LB15a to LB15d, LB20a, LB30a and LB30b)
/// over Unicode 15.0 property data, with the default LB1 resolutions: AI, SG and XX as AL, SA
/// as CM or AL by General_Category, CJ as NS. Not implemented: LB19a and LB28a (later
/// Unicode versions). Offsets are byte offsets of the position a line may end at;
/// a break at the end of a non-empty text is always reported (LB3), as `Mandatory` when the
/// text ends in a hard line terminator and `Allowed` otherwise. Empty text has no breaks.
#[must_use]
pub fn line_breaks(text: &str) -> Vec<Break> {
    let items: Vec<(usize, char, Lb)> = text
        .char_indices()
        .map(|(o, ch)| (o, ch, class_of(ch).resolve()))
        .collect();
    // after[i]: class of the first character after i that is not CM or ZWJ.
    let mut after: Vec<Option<Lb>> = vec![None; items.len()];
    let mut next = None;
    for (slot, item) in after.iter_mut().zip(&items).rev() {
        *slot = next;
        if !matches!(item.2, Lb::CM | Lb::ZWJ) {
            next = Some(item.2);
        }
    }
    let mut out = Vec::new();
    let mut st = State::new();
    for (&(offset, ch, raw), &aft) in items.iter().zip(&after) {
        if offset > 0 {
            // `aft` is the lookahead for the character at `offset`.
            if let Some(kind) = decide(&st, raw, aft) {
                out.push(Break { offset, kind });
            }
        }
        st.advance(raw, ch);
    }
    if !text.is_empty() {
        let kind = if st.prev_raw.is_hard() {
            BreakKind::Mandatory
        } else {
            BreakKind::Allowed
        };
        out.push(Break {
            offset: text.len(),
            kind,
        });
    }
    out
}
