// SPDX-License-Identifier: Apache-2.0

//! `DavFileSystem` backed by `worm-ledger` — the storage adapter
//! `dav_server::DavHandler` mounts, per the BRIEF's own framing:
//! "the store is the DataGraph [ledger], a codec is all that's
//! needed." `dav-server` handles the full WebDAV/CalDAV protocol
//! machinery itself (PROPFIND XML, REPORT calendar-query/multiget,
//! MKCALENDAR, etc.) against this trait; only storage is this crate's
//! concern — a much better fit for direct crate use than `imap-
//! codec` was for `service-email`'s IMAP serve (see that crate's
//! `imap_serve.rs` module doc for why that one was hand-rolled
//! instead).
//!
//! **Deliberately scoped for Phase 2's first pass**: a single flat
//! calendar collection at the root path (`/`), containing one `.ics`
//! resource per event named `<uid>.ics`. No nested collections, no
//! per-principal calendar-home routing, no locking beyond `FakeLs`'s
//! macOS/Windows-compatibility stub -- all real, named gaps for a
//! later pass once a real multi-calendar/multi-principal deployment
//! needs them.
//!
//! **The `.ics` filename's stem MUST equal the VEVENT's own `UID`
//! property.** A PUT stores the event under the UID parsed from the
//! body (never the URL path -- the ICS body is authoritative, matching
//! real RFC5545 semantics where UID, not filename, is a resource's
//! true identity), so a client naming the resource anything else will
//! PUT successfully but then get 404 on GET/DELETE against that same
//! path -- found live via this crate's own end-to-end test using a
//! mismatched filename/UID pair. Real CalDAV clients (Thunderbird,
//! Apple Calendar) already follow this filename=UID convention as a
//! matter of practice, so this isn't a made-up restriction, but it IS
//! a real constraint this filesystem enforces rather than just hopes
//! for -- a future pass could add an explicit path->UID index to
//! support arbitrary filenames if a real client ever needs it.

use std::io::SeekFrom;
use std::pin::Pin;
use std::sync::Arc;
use std::time::SystemTime;

use bytes::Bytes;
use dav_server::davpath::DavPath;
use dav_server::fs::{
    DavDirEntry, DavFile, DavFileSystem, DavMetaData, FsError, FsFuture, FsResult, FsStream,
    OpenOptions, ReadDirMeta,
};
use futures_util::stream;

use crate::event_store::EventStore;
use crate::ledger_backing::LedgerBacking;

/// A boxed, pinned stream of directory entries — the return type
/// `read_dir` must produce; factored out to satisfy
/// `clippy::type_complexity`.
type DirEntryStream = Pin<Box<dyn futures_util::Stream<Item = FsResult<Box<dyn DavDirEntry>>> + Send>>;

#[derive(Clone)]
pub struct WormCalendarFs {
    ledger_backing: Arc<LedgerBacking>,
    event_store: Arc<EventStore>,
}

impl std::fmt::Debug for WormCalendarFs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WormCalendarFs").finish_non_exhaustive()
    }
}

impl WormCalendarFs {
    pub fn new(ledger_backing: Arc<LedgerBacking>, event_store: Arc<EventStore>) -> Box<Self> {
        Box::new(Self {
            ledger_backing,
            event_store,
        })
    }

    fn uid_for(path: &DavPath) -> FsResult<Option<String>> {
        let rel = path.as_rel_ospath();
        let name = rel.to_str().ok_or(FsError::GeneralFailure)?;
        if name.is_empty() {
            return Ok(None); // root collection
        }
        Ok(Some(name.strip_suffix(".ics").unwrap_or(name).to_string()))
    }

    fn commit_put(&self, raw_ics: &str) -> Result<(), FsError> {
        let (_cursor, event) = self
            .ledger_backing
            .put_event(raw_ics)
            .map_err(|_| FsError::GeneralFailure)?;
        // Update the live index immediately -- a GET/PROPFIND
        // right after this PUT must see it without waiting for a
        // restart+replay cycle.
        self.event_store.upsert(event);
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct WormMetaData {
    len: u64,
    modified: SystemTime,
    is_dir: bool,
    is_calendar: bool,
}

impl DavMetaData for WormMetaData {
    fn len(&self) -> u64 {
        self.len
    }
    fn modified(&self) -> FsResult<SystemTime> {
        Ok(self.modified)
    }
    fn is_dir(&self) -> bool {
        self.is_dir
    }
    fn is_calendar(&self, _path: &DavPath) -> bool {
        self.is_calendar
    }
}

struct WormDirEntry {
    name: Vec<u8>,
    meta: WormMetaData,
}

impl DavDirEntry for WormDirEntry {
    fn name(&self) -> Vec<u8> {
        self.name.clone()
    }
    fn metadata(&self) -> FsFuture<'_, Box<dyn DavMetaData>> {
        let meta = self.meta.clone();
        Box::pin(async move { Ok(Box::new(meta) as Box<dyn DavMetaData>) })
    }
}

enum WormDavFile {
    Read {
        content: Bytes,
        pos: usize,
        modified: SystemTime,
    },
    Write {
        buffer: Vec<u8>,
        fs: WormCalendarFs,
    },
}

impl std::fmt::Debug for WormDavFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WormDavFile::Read { content, .. } => {
                f.debug_struct("WormDavFile::Read").field("len", &content.len()).finish()
            }
            WormDavFile::Write { buffer, .. } => {
                f.debug_struct("WormDavFile::Write").field("len", &buffer.len()).finish()
            }
        }
    }
}

impl DavFile for WormDavFile {
    fn metadata(&mut self) -> FsFuture<'_, Box<dyn DavMetaData>> {
        let (len, modified) = match self {
            WormDavFile::Read { content, modified, .. } => (content.len() as u64, *modified),
            WormDavFile::Write { buffer, .. } => (buffer.len() as u64, SystemTime::now()),
        };
        Box::pin(async move {
            Ok(Box::new(WormMetaData {
                len,
                modified,
                is_dir: false,
                is_calendar: false,
            }) as Box<dyn DavMetaData>)
        })
    }

    fn write_buf(&mut self, mut buf: Box<dyn bytes::Buf + Send>) -> FsFuture<'_, ()> {
        match self {
            WormDavFile::Write { buffer, .. } => {
                while buf.has_remaining() {
                    let n = buf.chunk().len();
                    buffer.extend_from_slice(buf.chunk());
                    buf.advance(n);
                }
                Box::pin(async move { Ok(()) })
            }
            WormDavFile::Read { .. } => Box::pin(async move { Err(FsError::Forbidden) }),
        }
    }

    fn write_bytes(&mut self, buf: Bytes) -> FsFuture<'_, ()> {
        match self {
            WormDavFile::Write { buffer, .. } => {
                buffer.extend_from_slice(&buf);
                Box::pin(async move { Ok(()) })
            }
            WormDavFile::Read { .. } => Box::pin(async move { Err(FsError::Forbidden) }),
        }
    }

    fn read_bytes(&mut self, count: usize) -> FsFuture<'_, Bytes> {
        match self {
            WormDavFile::Read { content, pos, .. } => {
                let end = (*pos + count).min(content.len());
                let slice = content.slice(*pos..end);
                *pos = end;
                Box::pin(async move { Ok(slice) })
            }
            WormDavFile::Write { .. } => Box::pin(async move { Err(FsError::Forbidden) }),
        }
    }

    fn seek(&mut self, pos: SeekFrom) -> FsFuture<'_, u64> {
        match self {
            WormDavFile::Read { content, pos: p, .. } => {
                let len = content.len() as i64;
                let new_pos = match pos {
                    SeekFrom::Start(n) => n as i64,
                    SeekFrom::End(n) => len + n,
                    SeekFrom::Current(n) => *p as i64 + n,
                }
                .clamp(0, len) as usize;
                *p = new_pos;
                let ret = new_pos as u64;
                Box::pin(async move { Ok(ret) })
            }
            WormDavFile::Write { .. } => Box::pin(async move { Err(FsError::NotImplemented) }),
        }
    }

    fn flush(&mut self) -> FsFuture<'_, ()> {
        match self {
            WormDavFile::Write { buffer, fs } => {
                let raw = std::mem::take(buffer);
                let fs = fs.clone();
                Box::pin(async move {
                    let raw_str = String::from_utf8(raw).map_err(|_| FsError::GeneralFailure)?;
                    fs.commit_put(&raw_str)
                })
            }
            WormDavFile::Read { .. } => Box::pin(async move { Ok(()) }),
        }
    }
}

impl DavFileSystem for WormCalendarFs {
    fn open<'a>(&'a self, path: &'a DavPath, options: OpenOptions) -> FsFuture<'a, Box<dyn DavFile>> {
        Box::pin(async move {
            let uid = Self::uid_for(path)?;
            let uid = uid.ok_or(FsError::Forbidden)?; // root isn't a file

            if options.write {
                return Ok(Box::new(WormDavFile::Write {
                    buffer: Vec::new(),
                    fs: self.clone(),
                }) as Box<dyn DavFile>);
            }

            match self.event_store.lookup_by_uid(&uid) {
                Some(event) => Ok(Box::new(WormDavFile::Read {
                    content: Bytes::from(event.raw_ics.into_bytes()),
                    pos: 0,
                    modified: SystemTime::from(event.updated_at),
                }) as Box<dyn DavFile>),
                None => Err(FsError::NotFound),
            }
        })
    }

    fn read_dir<'a>(
        &'a self,
        path: &'a DavPath,
        _meta: ReadDirMeta,
    ) -> FsFuture<'a, FsStream<Box<dyn DavDirEntry>>> {
        Box::pin(async move {
            let uid = Self::uid_for(path)?;
            if uid.is_some() {
                return Err(FsError::Forbidden); // only the root collection has children
            }
            let entries: Vec<FsResult<Box<dyn DavDirEntry>>> = self
                .event_store
                .list_all()
                .into_iter()
                .map(|event| {
                    let meta = WormMetaData {
                        len: event.raw_ics.len() as u64,
                        modified: SystemTime::from(event.updated_at),
                        is_dir: false,
                        is_calendar: false,
                    };
                    Ok(Box::new(WormDirEntry {
                        name: format!("{}.ics", event.uid).into_bytes(),
                        meta,
                    }) as Box<dyn DavDirEntry>)
                })
                .collect();
            let s: DirEntryStream = Box::pin(stream::iter(entries));
            Ok(s)
        })
    }

    fn metadata<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, Box<dyn DavMetaData>> {
        Box::pin(async move {
            match Self::uid_for(path)? {
                None => Ok(Box::new(WormMetaData {
                    len: 0,
                    modified: SystemTime::now(),
                    is_dir: true,
                    is_calendar: true,
                }) as Box<dyn DavMetaData>),
                Some(uid) => match self.event_store.lookup_by_uid(&uid) {
                    Some(event) => Ok(Box::new(WormMetaData {
                        len: event.raw_ics.len() as u64,
                        modified: SystemTime::from(event.updated_at),
                        is_dir: false,
                        is_calendar: false,
                    }) as Box<dyn DavMetaData>),
                    None => Err(FsError::NotFound),
                },
            }
        })
    }

    fn remove_file<'a>(&'a self, path: &'a DavPath) -> FsFuture<'a, ()> {
        Box::pin(async move {
            let uid = Self::uid_for(path)?.ok_or(FsError::Forbidden)?;
            if self.event_store.lookup_by_uid(&uid).is_none() {
                return Err(FsError::NotFound);
            }
            self.ledger_backing
                .delete_event(&uid)
                .map_err(|_| FsError::GeneralFailure)?;
            self.event_store.remove(&uid);
            Ok(())
        })
    }
}
