// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: 2026 Woodfine Capital Projects Inc.

//! Startup settings for serving the mirror. All are off by default;
//! with none set the service serves native events only. The index is
//! loaded once at startup: a rebuild of the index is not seen until the
//! service is restarted.

use crate::annotation_log::AnnotationLog;
use crate::dav_fs::WormCalendarFs;
use crate::event_store::EventStore;
use crate::ledger_backing::LedgerBacking;
use crate::mirror_rebuild::MailboxRoot;
use crate::mirror_view::MirrorView;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::Arc;

/// Path of the index CSV written by a rebuild.
pub const MIRROR_INDEX: &str = "CALENDAR_MIRROR_INDEX";
/// Comma-separated `name=path` pairs, one per mailbox root.
pub const MIRROR_MAILBOXES: &str = "CALENDAR_MIRROR_MAILBOXES";
/// Path of the append-only annotation log; needs the two above.
pub const ANNOTATION_LOG: &str = "CALENDAR_ANNOTATION_LOG";

#[derive(Debug)]
pub enum ConfigError {
    /// `var` is needed because `because` is set.
    Missing {
        var: &'static str,
        because: &'static str,
    },
    Empty(&'static str),
    NotUnicode(&'static str),
    BadEntry {
        var: &'static str,
        entry: String,
        reason: &'static str,
    },
    DuplicateMailbox {
        var: &'static str,
        name: String,
    },
    /// A setting was parsed but what it names cannot be used.
    Unusable {
        var: &'static str,
        reason: String,
    },
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::Missing { var, because } => {
                write!(f, "{var} must be set when {because} is set")
            }
            ConfigError::Empty(var) => write!(f, "{var} is set but empty"),
            ConfigError::NotUnicode(var) => write!(f, "{var} is not valid UTF-8"),
            ConfigError::BadEntry { var, entry, reason } => {
                write!(f, "{var}: entry {entry:?}: {reason}")
            }
            ConfigError::DuplicateMailbox { var, name } => {
                write!(f, "{var}: mailbox name {name:?} appears more than once")
            }
            ConfigError::Unusable { var, reason } => write!(f, "{var}: {reason}"),
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MirrorConfig {
    pub index: PathBuf,
    pub mailboxes: Vec<MailboxRoot>,
    pub annotation_log: Option<PathBuf>,
}

impl MirrorConfig {
    /// Reads the settings from the process environment.
    pub fn from_env() -> Result<Option<Self>, ConfigError> {
        for var in [MIRROR_INDEX, MIRROR_MAILBOXES, ANNOTATION_LOG] {
            if let Err(std::env::VarError::NotUnicode(_)) = std::env::var(var) {
                return Err(ConfigError::NotUnicode(var));
            }
        }
        Self::parse(|var| std::env::var(var).ok())
    }

    /// Parses the settings from `get`, which returns a variable's
    /// value or `None` when it is not set. `Ok(None)` means none is
    /// set. A variable set to an empty value is an error, not unset.
    pub fn parse(get: impl Fn(&str) -> Option<String>) -> Result<Option<Self>, ConfigError> {
        let index = get(MIRROR_INDEX);
        let mailboxes = get(MIRROR_MAILBOXES);
        let log = get(ANNOTATION_LOG);
        for (var, value) in [
            (MIRROR_INDEX, &index),
            (MIRROR_MAILBOXES, &mailboxes),
            (ANNOTATION_LOG, &log),
        ] {
            if value.as_deref().is_some_and(|v| v.trim().is_empty()) {
                return Err(ConfigError::Empty(var));
            }
        }
        let (index, mailboxes) = match (index, mailboxes) {
            (None, None) => {
                return match log {
                    Some(_) => Err(ConfigError::Missing {
                        var: MIRROR_INDEX,
                        because: ANNOTATION_LOG,
                    }),
                    None => Ok(None),
                };
            }
            (Some(_), None) => {
                return Err(ConfigError::Missing {
                    var: MIRROR_MAILBOXES,
                    because: MIRROR_INDEX,
                })
            }
            (None, Some(_)) => {
                return Err(ConfigError::Missing {
                    var: MIRROR_INDEX,
                    because: MIRROR_MAILBOXES,
                })
            }
            (Some(i), Some(m)) => (i, m),
        };
        Ok(Some(Self {
            index: PathBuf::from(index),
            mailboxes: parse_mailboxes(&mailboxes)?,
            annotation_log: log.map(PathBuf::from),
        }))
    }

    /// Loads the index and opens the annotation log, if one is set.
    /// Every mailbox root must be an existing directory.
    pub fn load(&self) -> Result<(MirrorView, Option<AnnotationLog>), ConfigError> {
        for m in &self.mailboxes {
            if !m.root.is_dir() {
                return Err(ConfigError::Unusable {
                    var: MIRROR_MAILBOXES,
                    reason: format!(
                        "root of mailbox {:?} is not a directory: {}",
                        m.mailbox,
                        m.root.display()
                    ),
                });
            }
        }
        let view =
            MirrorView::load(&self.index, &self.mailboxes).map_err(|e| ConfigError::Unusable {
                var: MIRROR_INDEX,
                reason: format!("cannot load {}: {e}", self.index.display()),
            })?;
        let log = match &self.annotation_log {
            Some(path) => Some(AnnotationLog::open(path, &self.mailboxes).map_err(|e| {
                ConfigError::Unusable {
                    var: ANNOTATION_LOG,
                    reason: format!("cannot open {}: {e}", path.display()),
                }
            })?),
            None => None,
        };
        Ok((view, log))
    }

    /// The file system serving native events and the mirror, with the
    /// annotation log attached when one is set.
    pub fn build_fs(
        &self,
        ledger_backing: Arc<LedgerBacking>,
        event_store: Arc<EventStore>,
    ) -> Result<Box<WormCalendarFs>, ConfigError> {
        let (view, log) = self.load()?;
        let fs = WormCalendarFs::with_mirror(ledger_backing, event_store, Arc::new(view));
        Ok(match log {
            Some(log) => fs.with_annotation_log(Arc::new(log)),
            None => fs,
        })
    }
}

/// `name=path,name=path`. Whitespace around a name or a path is
/// dropped; a path cannot hold a comma. The first `=` ends the name.
fn parse_mailboxes(value: &str) -> Result<Vec<MailboxRoot>, ConfigError> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for entry in value.split(',') {
        let bad = |reason| ConfigError::BadEntry {
            var: MIRROR_MAILBOXES,
            entry: entry.to_string(),
            reason,
        };
        let (name, path) = entry
            .split_once('=')
            .ok_or_else(|| bad("expected name=path"))?;
        let (name, path) = (name.trim(), path.trim());
        if name.is_empty() {
            return Err(bad("empty mailbox name"));
        }
        if path.is_empty() {
            return Err(bad("empty path"));
        }
        if !seen.insert(name.to_string()) {
            return Err(ConfigError::DuplicateMailbox {
                var: MIRROR_MAILBOXES,
                name: name.to_string(),
            });
        }
        out.push(MailboxRoot {
            mailbox: name.to_string(),
            root: PathBuf::from(path),
        });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn parse(vars: &[(&str, &str)]) -> Result<Option<MirrorConfig>, ConfigError> {
        let vars: HashMap<String, String> = vars
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        MirrorConfig::parse(|k| vars.get(k).cloned())
    }

    fn root(name: &str, path: &str) -> MailboxRoot {
        MailboxRoot {
            mailbox: name.to_string(),
            root: PathBuf::from(path),
        }
    }

    #[test]
    fn nothing_set_is_native_only() {
        assert_eq!(parse(&[]).unwrap(), None);
    }

    #[test]
    fn valid_pairs_are_parsed_in_order() {
        let cfg = parse(&[
            (MIRROR_INDEX, "/var/idx/calendar-index.csv"),
            (MIRROR_MAILBOXES, "alice=/mirror/alice, bob = /mirror/b=ob "),
        ])
        .unwrap()
        .unwrap();
        assert_eq!(cfg.index, PathBuf::from("/var/idx/calendar-index.csv"));
        assert_eq!(
            cfg.mailboxes,
            vec![root("alice", "/mirror/alice"), root("bob", "/mirror/b=ob")]
        );
        assert_eq!(cfg.annotation_log, None);
    }

    #[test]
    fn the_annotation_log_is_kept_when_set() {
        let cfg = parse(&[
            (MIRROR_INDEX, "/i.csv"),
            (MIRROR_MAILBOXES, "a=/m/a"),
            (ANNOTATION_LOG, "/var/log/annotations.jsonl"),
        ])
        .unwrap()
        .unwrap();
        assert_eq!(
            cfg.annotation_log,
            Some(PathBuf::from("/var/log/annotations.jsonl"))
        );
    }

    #[test]
    fn a_duplicate_mailbox_name_is_refused() {
        let err = parse(&[
            (MIRROR_INDEX, "/i.csv"),
            (MIRROR_MAILBOXES, "a=/m/a,b=/m/b,a=/m/c"),
        ])
        .unwrap_err();
        assert!(
            matches!(&err, ConfigError::DuplicateMailbox { name, .. } if name == "a"),
            "{err}"
        );
        assert!(err.to_string().contains(MIRROR_MAILBOXES));
    }

    #[test]
    fn an_empty_value_is_refused_and_named() {
        for var in [MIRROR_INDEX, MIRROR_MAILBOXES, ANNOTATION_LOG] {
            let mut vars = vec![
                (MIRROR_INDEX, "/i.csv"),
                (MIRROR_MAILBOXES, "a=/m/a"),
                (ANNOTATION_LOG, "/l.jsonl"),
            ];
            for v in vars.iter_mut().filter(|v| v.0 == var) {
                v.1 = " ";
            }
            let err = parse(&vars).unwrap_err();
            assert!(matches!(err, ConfigError::Empty(v) if v == var), "{err}");
            assert!(err.to_string().contains(var));
        }
    }

    #[test]
    fn an_entry_without_equals_or_with_an_empty_side_is_refused() {
        for bad in ["a=/m/a,/m/b", "=/m/a", "a=", "a=/m/a,", "a=/m/a,,b=/m/b"] {
            let err = parse(&[(MIRROR_INDEX, "/i.csv"), (MIRROR_MAILBOXES, bad)]).unwrap_err();
            assert!(matches!(err, ConfigError::BadEntry { .. }), "{bad}: {err}");
            assert!(err.to_string().contains(MIRROR_MAILBOXES), "{err}");
        }
    }

    #[test]
    fn a_partial_set_is_refused_and_names_the_missing_variable() {
        let cases: [(&[(&str, &str)], &str); 4] = [
            (&[(MIRROR_INDEX, "/i.csv")], MIRROR_MAILBOXES),
            (&[(MIRROR_MAILBOXES, "a=/m/a")], MIRROR_INDEX),
            (&[(ANNOTATION_LOG, "/l.jsonl")], MIRROR_INDEX),
            (
                &[(MIRROR_INDEX, "/i.csv"), (ANNOTATION_LOG, "/l.jsonl")],
                MIRROR_MAILBOXES,
            ),
        ];
        for (vars, missing) in cases {
            let err = parse(vars).unwrap_err();
            assert!(
                matches!(err, ConfigError::Missing { var, .. } if var == missing),
                "{vars:?}: {err}"
            );
            assert!(err.to_string().starts_with(missing), "{err}");
        }
    }

    #[test]
    fn an_unreadable_index_fails_load_and_names_the_variable() {
        let dir = std::env::temp_dir().join(format!(
            "service-calendar-config-noindex-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let cfg = MirrorConfig {
            index: dir.join("absent.csv"),
            mailboxes: vec![MailboxRoot {
                mailbox: "a".to_string(),
                root: dir.clone(),
            }],
            annotation_log: None,
        };
        let Err(err) = cfg.load() else {
            panic!("load succeeded")
        };
        assert!(
            matches!(err, ConfigError::Unusable { var, .. } if var == MIRROR_INDEX),
            "{err}"
        );
    }

    #[test]
    fn a_missing_mailbox_root_fails_load() {
        let cfg = MirrorConfig {
            index: PathBuf::from("/nonexistent/i.csv"),
            mailboxes: vec![root("a", "/nonexistent/service-calendar-mailbox")],
            annotation_log: None,
        };
        let Err(err) = cfg.load() else {
            panic!("load succeeded")
        };
        assert!(
            matches!(err, ConfigError::Unusable { var, .. } if var == MIRROR_MAILBOXES),
            "{err}"
        );
    }
}
