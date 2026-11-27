// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.
//! The one table that describes every command. `help`, `<command> --help`, the man page, the
//! machine-readable `describe --json` and the README command list are all generated from it, so
//! the documentation cannot drift from the commands (a test compares this table with the
//! dispatcher in `lib.rs`).
//!
//! `access` tells a person or an AI what a command may do: `read` never writes anything;
//! `append` adds rows to a register under `precedent/`; `write` creates or rewrites a file. A
//! read command is safe to run freely; the others are run at a person's instruction (SYS-ADR-07:
//! the person instructing the session is the person acting).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Access {
    Read,
    Append,
    Write,
}

impl Access {
    pub fn word(self) -> &'static str {
        match self {
            Access::Read => "read",
            Access::Append => "append",
            Access::Write => "write",
        }
    }
    pub fn ai(self) -> &'static str {
        match self {
            Access::Read => "free",
            _ => "at-a-persons-instruction",
        }
    }
}

pub struct Flag {
    pub name: &'static str,
    /// The placeholder for the value, or empty for a switch.
    pub value: &'static str,
    pub help: &'static str,
}

pub struct Cmd {
    pub name: &'static str,
    /// The customer verb this belongs to (Find, File, ...) or empty for maintenance commands.
    pub verb: &'static str,
    pub summary: &'static str,
    pub usage: &'static str,
    pub flags: &'static [Flag],
    pub example: &'static str,
    pub access: Access,
    /// What it reads or writes, in words.
    pub touches: &'static str,
}

const ROOT: Flag = Flag {
    name: "--root",
    value: "DIR",
    help: "the work folder (default: the current directory)",
};

pub const COMMANDS: &[Cmd] = &[
    Cmd {
        name: "find",
        verb: "Find",
        summary: "Where is this used? A cross-document glossary of defined terms, one term, one entity code, or a phrase. Wordings are grouped and shown, never flagged.",
        usage: "find [--glossary | --parties | --term T | --entity E | --text PHRASE] [--entities FILE[,FILE]] [--parties-file F] [--counts] [--show] [--show-text] [--all-versions] [--near N] [--min-docs N] [--dir D[,D]] [--root DIR]",
        flags: &[
            Flag { name: "--glossary", value: "", help: "every defined term (not party names) with the documents that define it, the documents that use it, and the number of distinct wordings (the default)" },
            Flag { name: "--entities", value: "FILE[,FILE]", help: "entity register(s), CSV, whose titles, aliases and roles are the authority for party names (absolute path or relative to the root); read only" },
            Flag { name: "--parties-file", value: "F", help: "a plain list, one name per line: +name marks a party, -name a term, # a comment (default precedent/parties.txt if it exists); read only" },
            Flag { name: "--parties", value: "", help: "the names of parties and entities (short names after a legal name, acronyms, names with an entity word), with the documents that use them" },
            Flag { name: "--term", value: "T", help: "look at one term or name (matches by containment)" },
            Flag { name: "--entity", value: "E", help: "the documents whose file name carries this entity code" },
            Flag { name: "--text", value: "PHRASE", help: "the documents containing this phrase, with unit and occurrence counts" },
            Flag { name: "--counts", value: "", help: "print counts only: no file names, terms or text" },
            Flag { name: "--show", value: "", help: "list the documents under each wording" },
            Flag { name: "--show-text", value: "", help: "print each wording's defining sentence as written, with the documents under it (prints your text: for you to run, not to share)" },
            Flag { name: "--all-versions", value: "", help: "read every version, not only the current one" },
            Flag { name: "--names", value: "", help: "with --counts, also list the document types taken from file names; off by default so counts are numbers only" },
            Flag { name: "--near", value: "N", help: "similarity (0 to 1) at which two definitions count as one wording (default 0.5)" },
            Flag { name: "--min-docs", value: "N", help: "list terms defined in at least N documents (default 2)" },
            Flag { name: "--dir", value: "D[,D]", help: "folders to read, relative to the root (default service-wip)" },
        ],
        example: "tool-precedent-core find --root /path/to/matter --glossary --show",
        access: Access::Read,
        touches: "reads the HTML versions in the named folders; writes nothing",
    },
    Cmd {
        name: "lineage-report",
        verb: "Find",
        summary: "How the files grow from each other, read from the version names (and draw rows when present): instruments, versions, authors, heads, forks, gaps. Counts only.",
        usage: "lineage-report [--dir D[,D]] [--initials JW,MW,AI] [--names] [--root DIR]",
        flags: &[
            Flag { name: "--dir", value: "D[,D]", help: "folders to read (default service-wip)" },
            Flag { name: "--initials", value: "A,B", help: "accepted author initials (default JW,MW,AI)" },
            Flag { name: "--names", value: "", help: "also list the words taken from file names (author initials, variant words, renditions, document types); off by default so the report is numbers only" },
        ],
        example: "tool-precedent-core lineage-report --root /path/to/matter --dir service-wip,outputs",
        access: Access::Read,
        touches: "lists file names and reads precedent/draws.csv if present; writes nothing",
    },
    Cmd {
        name: "language-report",
        verb: "Find",
        summary: "What language the current documents hold: headings, paragraphs, exact and near repeats, defined terms, entity codes. Counts only.",
        usage: "language-report [--dir D[,D]] [--initials JW,MW,AI] [--min-words N] [--near N] [--names] [--root DIR]",
        flags: &[
            Flag { name: "--dir", value: "D[,D]", help: "folders to read (default service-wip)" },
            Flag { name: "--min-words", value: "N", help: "smallest unit counted as a paragraph (default 12)" },
            Flag { name: "--near", value: "N", help: "similarity at which two paragraphs are near twins (default 0.5)" },
            Flag { name: "--initials", value: "A,B", help: "accepted author initials (default JW,MW,AI)" },
            Flag { name: "--names", value: "", help: "also list the words taken from file names (author initials, variant words, renditions, document types); off by default so the report is numbers only" },
        ],
        example: "tool-precedent-core language-report --root /path/to/matter",
        access: Access::Read,
        touches: "reads the current HTML versions; writes nothing",
    },
    Cmd {
        name: "catalog",
        verb: "Find",
        summary: "List the CSV registers (shards) in a folder with their columns and row counts.",
        usage: "catalog [--dir precedent] [--root DIR]",
        flags: &[Flag { name: "--dir", value: "D", help: "folder to scan (default precedent)" }],
        example: "tool-precedent-core catalog --root /path/to/matter",
        access: Access::Read,
        touches: "reads every .csv in the folder; writes nothing",
    },
    Cmd {
        name: "changes",
        verb: "Find",
        summary: "Show the change log: what text was replaced by what, in which version, under which decision.",
        usage: "changes [--instrument NAME] [--version V] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "only this instrument (the file name without the _JW suffix)" },
            Flag { name: "--version", value: "V", help: "only this version" },
            Flag { name: "--file", value: "F", help: "read another CSV (default precedent/changes.csv)" },
        ],
        example: "tool-precedent-core changes --instrument MEMO_A --version JW3",
        access: Access::Read,
        touches: "reads precedent/changes.csv; writes nothing",
    },
    Cmd {
        name: "terms",
        verb: "Find",
        summary: "Show the defined-term register: which term each document version uses, what it replaced, live or retired.",
        usage: "terms [--instrument NAME] [--term T] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "only this instrument (the file name without the _JW suffix)" },
            Flag { name: "--term", value: "T", help: "only this term" },
            Flag { name: "--file", value: "F", help: "read another CSV (default precedent/terms.csv)" },
        ],
        example: "tool-precedent-core terms --term affiliate",
        access: Access::Read,
        touches: "reads precedent/terms.csv; writes nothing",
    },
    Cmd {
        name: "draws",
        verb: "Find",
        summary: "Show the draw register: which earlier version (of this or another document) each version grew from.",
        usage: "draws [--instrument NAME] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "only this instrument (the file name without the _JW suffix)" },
            Flag { name: "--file", value: "F", help: "read another CSV (default precedent/draws.csv)" },
        ],
        example: "tool-precedent-core draws --instrument MEMO_A",
        access: Access::Read,
        touches: "reads precedent/draws.csv; writes nothing",
    },
    Cmd {
        name: "changes-add",
        verb: "File",
        summary: "Record one change: the old text, the new text, the section, a tag, and the decision it follows.",
        usage: "changes-add --instrument NAME --version V --baseline B --section S --old TEXT --new TEXT --tag TAG --decision REF [--date D] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "the instrument (the file name without the _JW suffix)" },
            Flag { name: "--version", value: "V", help: "the version the change appears in" },
            Flag { name: "--baseline", value: "B", help: "what it was compared against" },
            Flag { name: "--section", value: "S", help: "the section changed" },
            Flag { name: "--old", value: "TEXT", help: "the previous wording" },
            Flag { name: "--new", value: "TEXT", help: "the new wording" },
            Flag { name: "--tag", value: "TAG", help: "Added, Deleted or Replaced" },
            Flag { name: "--decision", value: "REF", help: "the decision reference" },
            Flag { name: "--date", value: "D", help: "ISO date" },
        ],
        example: "tool-precedent-core changes-add --instrument MEMO_A --version JW4 --baseline JW3 --section 2 --old \"may\" --new \"shall\" --tag Replaced --decision P-12",
        access: Access::Append,
        touches: "appends one row to precedent/changes.csv",
    },
    Cmd {
        name: "terms-add",
        verb: "File",
        summary: "Record that a document version uses a defined term, and which term it replaced.",
        usage: "terms-add --instrument NAME --version V --term T [--replaces T] [--status live|retired] [--date D] [--context TEXT] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "the instrument (the file name without the _JW suffix)" },
            Flag { name: "--version", value: "V", help: "the version" },
            Flag { name: "--term", value: "T", help: "the term" },
            Flag { name: "--replaces", value: "T", help: "the term it replaces" },
            Flag { name: "--status", value: "S", help: "live (default) or retired" },
            Flag { name: "--date", value: "D", help: "ISO date" },
            Flag { name: "--context", value: "TEXT", help: "the sentence the term appeared in (optional)" },
        ],
        example: "tool-precedent-core terms-add --instrument MEMO_A --version JW4 --term sponsor --replaces promoter",
        access: Access::Append,
        touches: "appends one row to precedent/terms.csv",
    },
    Cmd {
        name: "draw-add",
        verb: "File",
        summary: "Record that a version grew from an earlier one (in this or another document).",
        usage: "draw-add --instrument NAME --version V --from ID --from-version V [--date D] [--note N] [--file F] [--root DIR]",
        flags: &[
            Flag { name: "--instrument", value: "NAME", help: "the instrument (the file name without the _JW suffix)" },
            Flag { name: "--version", value: "V", help: "the new version" },
            Flag { name: "--from", value: "ID", help: "the document it grew from" },
            Flag { name: "--from-version", value: "V", help: "the version it grew from" },
            Flag { name: "--date", value: "D", help: "ISO date" },
            Flag { name: "--note", value: "N", help: "an optional one-line note" },
        ],
        example: "tool-precedent-core draw-add --instrument MEMO_A --version JW4 --from MEMO_A --from-version JW3",
        access: Access::Append,
        touches: "appends one row to precedent/draws.csv",
    },
    Cmd {
        name: "versions",
        verb: "",
        summary: "Rebuild the version index from the files present (it is an index, recomputed each time).",
        usage: "versions [--dir outputs] [--out F] [--root DIR]",
        flags: &[
            Flag { name: "--dir", value: "D", help: "folder to scan (default outputs)" },
            Flag { name: "--out", value: "F", help: "where to write (default precedent/versions.csv)" },
        ],
        example: "tool-precedent-core versions --dir service-wip",
        access: Access::Write,
        touches: "rewrites precedent/versions.csv (or --out) from a scan of the folder",
    },
    Cmd {
        name: "apply",
        verb: "",
        summary: "Bring a document's style block into line with its family token and components; --check only reports.",
        usage: "apply <file.html> [--check] [--root DIR]",
        flags: &[Flag { name: "--check", value: "", help: "report drift without changing the file" }],
        example: "tool-precedent-core apply outputs/MEMO_A_JW3.html --check",
        access: Access::Write,
        touches: "rewrites the style block of the named HTML file in place (nothing with --check)",
    },
    Cmd {
        name: "registry",
        verb: "",
        summary: "The typesetting registry: check it, list its classes, generate CSS or DTCG tokens from it, lint a document against it.",
        usage: "registry check | classes | css [--component C] [--out DIR] | dtcg [--out F] | lint <files> [--strict] [--family F] [--registry DIR] [--root DIR]",
        flags: &[
            Flag { name: "--registry", value: "DIR", help: "the registry folder (default precedent/registry)" },
            Flag { name: "--family", value: "F", help: "limit to one document family" },
            Flag { name: "--component", value: "C", help: "css: one component" },
            Flag { name: "--out", value: "PATH", help: "css/dtcg: write the output here instead of printing" },
            Flag { name: "--strict", value: "", help: "lint: treat informational findings as errors" },
        ],
        example: "tool-precedent-core registry lint outputs/MEMO_A_JW3.html --family memo",
        access: Access::Write,
        touches: "reads the registry tables; css/dtcg write a file only when --out is given",
    },
    Cmd {
        name: "shared-hash",
        verb: "",
        summary: "Print a fingerprint of each shared module, to compare two copies of the code.",
        usage: "shared-hash",
        flags: &[],
        example: "tool-precedent-core shared-hash",
        access: Access::Read,
        touches: "none",
    },
    Cmd {
        name: "help",
        verb: "",
        summary: "Show the command list, or one command in full.",
        usage: "help [COMMAND]   (also: COMMAND --help)",
        flags: &[],
        example: "tool-precedent-core help find",
        access: Access::Read,
        touches: "none",
    },
    Cmd {
        name: "describe",
        verb: "",
        summary: "Describe every command for a program or an AI: names, flags, whether it reads or writes, and when it may run.",
        usage: "describe [--json | --markdown]",
        flags: &[
            Flag { name: "--json", value: "", help: "machine-readable JSON (the default)" },
            Flag { name: "--markdown", value: "", help: "a Markdown table for the README" },
        ],
        example: "tool-precedent-core describe --json",
        access: Access::Read,
        touches: "none",
    },
    Cmd {
        name: "man",
        verb: "",
        summary: "Print the manual page (troff) for the whole tool.",
        usage: "man",
        flags: &[],
        example: "tool-precedent-core man | man -l -",
        access: Access::Read,
        touches: "none",
    },
];

pub fn get(name: &str) -> Option<&'static Cmd> {
    COMMANDS.iter().find(|c| c.name == name)
}

/// The short list, one line per command.
pub fn usage() -> String {
    let names: Vec<&str> = COMMANDS.iter().map(|c| c.name).collect();
    format!("usage: tool-precedent-core <command> [options]   commands: {}   (help COMMAND for details)", names.join(" | "))
}

pub fn help_all() -> String {
    let mut o = String::from("tool-precedent-core: the shard for precedent documents. Every command takes --root DIR.\n\n");
    for c in COMMANDS {
        let verb = if c.verb.is_empty() {
            String::new()
        } else {
            format!("[{}] ", c.verb)
        };
        o.push_str(&format!(
            "  {:<15} {}{}  ({})\n",
            c.name,
            verb,
            c.summary,
            c.access.word()
        ));
    }
    o.push_str("\nread = never writes; append = adds a row to a register; write = creates or rewrites a file.\n`tool-precedent-core help COMMAND` shows one command in full.\n");
    o
}

pub fn help_one(name: &str) -> Option<String> {
    let c = get(name)?;
    let mut o = format!(
        "{}  ({})\n\n{}\n\nusage: {}\n",
        c.name,
        c.access.word(),
        c.summary,
        c.usage
    );
    if !c.verb.is_empty() {
        o.push_str(&format!("\nverb: {}\n", c.verb));
    }
    o.push_str(&format!(
        "touches: {}\nmay be run by an AI: {}\n",
        c.touches,
        c.access.ai()
    ));
    o.push_str("\noptions:\n");
    o.push_str(&format!("  {:<14} {}\n", ROOT.name, ROOT.help));
    for f in c.flags {
        let left = if f.value.is_empty() {
            f.name.to_string()
        } else {
            format!("{} {}", f.name, f.value)
        };
        o.push_str(&format!("  {:<14} {}\n", left, f.help));
    }
    o.push_str(&format!("\nexample:\n  {}\n", c.example));
    Some(o)
}

fn json_str(s: &str) -> String {
    let mut o = String::from("\"");
    for ch in s.chars() {
        match ch {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '\t' => o.push_str("\\t"),
            c if (c as u32) < 0x20 => o.push_str(&format!("\\u{:04x}", c as u32)),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

/// Machine-readable description for programs and AIs.
pub fn describe_json() -> String {
    let mut o = String::from("{\n  \"tool\": \"tool-precedent-core\",\n  \"common_flags\": [{\"name\": \"--root\", \"value\": \"DIR\", \"help\": ");
    o.push_str(&json_str(ROOT.help));
    o.push_str("}],\n  \"access\": {\"read\": \"never writes\", \"append\": \"adds rows to a register under precedent/\", \"write\": \"creates or rewrites a file\"},\n  \"ai_rule\": \"read commands may be run freely; append and write commands are run at a person's instruction (the person instructing the session is the person acting)\",\n  \"commands\": [\n");
    for (i, c) in COMMANDS.iter().enumerate() {
        o.push_str(&format!(
            "    {{\"name\": {}, \"verb\": {}, \"summary\": {}, \"usage\": {}, \"access\": {}, \"ai\": {}, \"touches\": {}, \"example\": {}, \"flags\": [",
            json_str(c.name),
            json_str(c.verb),
            json_str(c.summary),
            json_str(c.usage),
            json_str(c.access.word()),
            json_str(c.access.ai()),
            json_str(c.touches),
            json_str(c.example)
        ));
        for (j, f) in c.flags.iter().enumerate() {
            o.push_str(&format!(
                "{{\"name\": {}, \"value\": {}, \"help\": {}}}{}",
                json_str(f.name),
                json_str(f.value),
                json_str(f.help),
                if j + 1 < c.flags.len() { ", " } else { "" }
            ));
        }
        o.push_str(&format!(
            "]}}{}\n",
            if i + 1 < COMMANDS.len() { "," } else { "" }
        ));
    }
    o.push_str("  ]\n}\n");
    o
}

/// A Markdown table of the commands, for the README.
pub fn describe_markdown() -> String {
    let mut o = String::from("| Command | Verb | What it does | Access |\n|---|---|---|---|\n");
    for c in COMMANDS {
        o.push_str(&format!(
            "| `{}` | {} | {} | {} |\n",
            c.name,
            c.verb,
            c.summary,
            c.access.word()
        ));
    }
    o
}

fn roff(s: &str) -> String {
    s.replace('\\', "\\\\").replace('-', "\\-")
}

/// The manual page (troff, man(7)).
pub fn man_page() -> String {
    let mut o = String::from(".TH TOOL-PRECEDENT-CORE 1 \"\" \"tool-precedent-core\" \"User Commands\"\n.SH NAME\ntool\\-precedent\\-core \\- the shard for precedent documents\n.SH SYNOPSIS\n.B tool\\-precedent\\-core\n.I command\n[options]\n.SH DESCRIPTION\nReads and records the lineage and the language of a folder of documents. Commands marked read never write; append adds a row to a register under precedent/; write creates or rewrites a file. Every command takes\n.B \\-\\-root DIR\n(default: the current directory).\n.SH COMMANDS\n");
    for c in COMMANDS {
        o.push_str(&format!(
            ".TP\n.B {}\n{}\n.br\nUsage: {}\n.br\nAccess: {} ({}).\n",
            roff(c.name),
            roff(c.summary),
            roff(c.usage),
            c.access.word(),
            roff(c.touches)
        ));
        for f in c.flags {
            let left = if f.value.is_empty() {
                f.name.to_string()
            } else {
                format!("{} {}", f.name, f.value)
            };
            o.push_str(&format!(
                ".RS\n.TP\n.B {}\n{}\n.RE\n",
                roff(&left),
                roff(f.help)
            ));
        }
        o.push_str(&format!(".RS\nExample: {}\n.RE\n", roff(c.example)));
    }
    o.push_str(".SH FOR AN AI\nRun\n.B tool\\-precedent\\-core describe \\-\\-json\nto read every command, its flags and whether it only reads or also writes. Read commands may be run freely. Append and write commands are run at a person's instruction: the person instructing the session is the person acting.\n");
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    /// Command names the dispatcher in lib.rs actually matches.
    fn dispatched() -> BTreeSet<String> {
        let src = include_str!("lib.rs");
        let mut out = BTreeSet::new();
        for line in src.lines() {
            let l = line.trim_start();
            if line.starts_with("        \"") && line.contains("=>") {
                let head = l.split("=>").next().unwrap_or("");
                for part in head.split('|') {
                    let n = part.trim().trim_matches('"');
                    if !n.is_empty() {
                        out.insert(n.to_string());
                    }
                }
            }
        }
        out
    }

    #[test]
    fn the_table_and_the_dispatcher_name_the_same_commands() {
        let table: BTreeSet<String> = COMMANDS.iter().map(|c| c.name.to_string()).collect();
        let d = dispatched();
        // help, describe and man are answered before the dispatcher
        let extra: BTreeSet<String> = ["help", "describe", "man"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let want: BTreeSet<String> = table.difference(&extra).cloned().collect();
        assert_eq!(
            d, want,
            "every command must be in the table and every table command must be dispatched"
        );
        assert_eq!(table.len(), COMMANDS.len(), "names are unique");
    }

    #[test]
    fn every_command_has_help_and_the_outputs_are_well_formed() {
        for c in COMMANDS {
            let h = help_one(c.name).unwrap();
            assert!(h.contains(c.usage) && h.contains(c.example), "{}", c.name);
            assert!(usage().contains(c.name));
            assert!(
                c.usage.starts_with(c.name),
                "{}: usage starts with the name",
                c.name
            );
        }
        assert!(help_one("nope").is_none());
        let j = describe_json();
        assert_eq!(j.matches('{').count(), j.matches('}').count());
        assert_eq!(j.matches('[').count(), j.matches(']').count());
        assert!(
            j.contains("\"name\": \"find\"")
                && j.contains("\"ai\": \"free\"")
                && j.contains("at-a-persons-instruction")
        );
        assert!(man_page().contains(".SH COMMANDS") && man_page().contains("describe \\-\\-json"));
        assert_eq!(describe_markdown().lines().count(), COMMANDS.len() + 2);
    }

    #[test]
    fn only_read_commands_are_free_for_an_ai() {
        for c in COMMANDS {
            assert_eq!(c.access == Access::Read, c.access.ai() == "free");
        }
        for w in [
            "changes-add",
            "terms-add",
            "draw-add",
            "versions",
            "apply",
            "registry",
        ] {
            assert_ne!(get(w).unwrap().access, Access::Read, "{w} writes");
        }
        for r in [
            "find",
            "lineage-report",
            "language-report",
            "catalog",
            "changes",
            "terms",
            "draws",
            "shared-hash",
        ] {
            assert_eq!(get(r).unwrap().access, Access::Read, "{r} only reads");
        }
    }
}
