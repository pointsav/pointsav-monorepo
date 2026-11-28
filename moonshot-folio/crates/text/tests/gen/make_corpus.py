#!/usr/bin/env python3
# SPDX-License-Identifier: Apache-2.0
# SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
"""Build tests/data/icu_line_breaks.txt: a deterministic corpus plus ICU's line-break offsets.

ICU (PyICU) is an independent oracle for the Rust implementation. Output line format:
    <hex code points, space separated> | <break offsets as code point indices, space separated>
Offsets exclude 0 and include the end of text, exactly like ICU's iterator.
Usage: python3 -I tests/gen/make_corpus.py [locale]   (default locale: root)
"""
import sys
import icu

LOCALE = icu.Locale.getRoot() if len(sys.argv) < 2 else icu.Locale(sys.argv[1])

# Representative code points per Line_Break class (dictionary scripts SA and surrogates SG are
# excluded: ICU segments Thai/Lao/Khmer/Myanmar by dictionary, which UAX#14 does not specify).
REPS = {
    "AI": [0xA7], "AL": [0x61, 0x3B1], "B2": [0x2014], "BA": [0x7C, 0x2010, 0x9], "BB": [0xB4],
    "BK": [0xB, 0xC], "CB": [0xFFFC], "CJ": [0x3041, 0x30FC], "CL": [0x7D, 0x3001], "CM": [0x300],
    "CP": [0x29, 0x5D], "CR": [0xD], "EB": [0x1F466], "EM": [0x1F3FB], "EX": [0x21],
    "GL": [0xA0, 0x202F], "H2": [0xAC00], "H3": [0xAC01], "HL": [0x5D0], "HY": [0x2D],
    "ID": [0x4E00, 0x3042], "IN": [0x2024], "IS": [0x2C, 0x2E], "JL": [0x1100], "JT": [0x11A8],
    "JV": [0x1161], "LF": [0xA], "NL": [0x85], "NS": [0x203C, 0x3005], "NU": [0x30, 0x661],
    "OP": [0x28, 0x5B, 0x3008], "PO": [0x25], "PR": [0x24], "QU": [0x22, 0x27, 0x201C, 0x201D],
    "RI": [0x1F1E6, 0x1F1E7], "SP": [0x20], "SY": [0x2F], "WJ": [0x2060], "XX": [0x378],
    "ZW": [0x200B], "ZWJ": [0x200D],
}
# Extra: Extended_Pictographic but unassigned (LB30b) and an emoji base that is ID-like.
EXTRA = [0x1F2FF, 0x2764]

def classname(cp):
    v = icu.Char.getIntPropertyValue(cp, icu.UProperty.LINE_BREAK)
    return icu.Char.getPropertyValueName(icu.UProperty.LINE_BREAK, v, icu.UPropertyNameChoice.SHORT_PROPERTY_NAME)

for name, cps in REPS.items():
    for cp in cps:
        got = classname(cp)
        if got != name and not (name == "XX" and got == "XX"):
            print("note: U+%04X expected %s, ICU says %s" % (cp, name, got), file=sys.stderr)

bi = icu.BreakIterator.createLineInstance(LOCALE)

def breaks(cps):
    s = "".join(map(chr, cps))
    bi.setText(s)
    # Convert UTF-16 offsets to code point indices.
    pos16 = {}
    u = 0
    for i, cp in enumerate(cps):
        pos16[u] = i
        u += 2 if cp > 0xFFFF else 1
    pos16[u] = len(cps)
    return [pos16[o] for o in bi if o != 0]

flat = [cp for cps in REPS.values() for cp in cps] + EXTRA
firsts = [cps[0] for cps in REPS.values()] + EXTRA

corpus = []
for a in flat:                       # every ordered pair of representatives
    for b in flat:
        corpus.append([a, b])
MIDDLE = ["SP", "CM", "ZWJ", "ZW", "WJ", "GL", "BA", "HY", "CL", "CP", "QU", "OP", "NU", "IS", "SY",
          "AL", "RI", "ID", "EB", "EM", "PR", "PO", "HL", "B2", "CB", "JL", "JV", "JT", "IN", "EX", "NS", "BB"]
middles = [REPS[m][0] for m in MIDDLE]
for a in firsts:                     # triples: any first representative around a context-class middle
    for b in middles:
        for c in firsts:
            corpus.append([a, b, c])

HAND = [
    "a\r\nb", "a\rb", "a\nb", "a\x0bb", "a\u0085b", "a b c", "\r\n\r\n", "a b c",
    "say \"hi\" now", "“hello” world", "(a) [b] {c}", "f(x) = (1 + 2)", "a ( b ) c",
    "3,000.00", "$(1.5)", "($1.50)", "-3.5%", "1,234,567.89 USD", "US$ 12", "10 %", "3.14.15",
    "A-1", "co-operate and well-known", "a - b", "x‐y", "-abc", " -abc", "a -abc",
    "א-ב", "א‐ b", "http://example.com/a/b?c=d&e=f#g", "a/b/c", "a/א",
    "mailto:me@example.com", "path\\to\\file", "e.g. this", "wait... what?!", "a!b?c", "Hello, world.",
    "\U0001f468‍\U0001f469‍\U0001f467 family", "\U0001f44d\U0001f3fd ok", "a‍\U0001f600",
    "\U0001f1fa\U0001f1f8\U0001f1e9\U0001f1ea\U0001f1eb\U0001f1f7", "\U0001f1fa\U0001f1f8 US", "a\U0001f1fa b",
    "日本語のテキスト。「こんにちは」",
    "今日は、天気がいいですね。", "あぃーぁ", "中文 text 中文",
    "한국어 텍스트", "각", "ᄀ가", "áb", "́a", " ́a",
    "a ́b", "​a", "a​ b", "a⁠b", "a⁠ b", "￼ a ￼", "A. B", "a­b",
    "( a)", "\" a\"", "a \" b", "\" (a", "' a", "a . b", "1 . 2", "1.2.3", "a1 b2", "1a", "a1", "$a", "a%", "%a",
    "การ", "ทดสอบ", "", " ", "  a  b  ",
    "a　b", "あ　い", "—a—b", "a——b", "x․․y", "ab․cd",
]
corpus.extend([ord(c) for c in s] for s in HAND if s)

state = 12345
def rnd(n):
    global state
    state = (state * 6364136223846793005 + 1442695040888963407) & 0xFFFFFFFFFFFFFFFF
    return (state >> 33) % n

for _ in range(30000):               # deterministic pseudo-random mixes of representatives
    corpus.append([flat[rnd(len(flat))] for _ in range(2 + rnd(9))])

seen = set()
with open(sys.argv[2] if len(sys.argv) > 2 else "tests/data/icu_line_breaks.txt", "w") as f:
    for cps in corpus:
        key = tuple(cps)
        if key in seen:
            continue
        seen.add(key)
        f.write("%s|%s\n" % (" ".join("%X" % c for c in cps), " ".join(map(str, breaks(cps)))))
print(len(seen), "cases; ICU", icu.ICU_VERSION, "Unicode", icu.UNICODE_VERSION, file=sys.stderr)
