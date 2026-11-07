//! tool-precedent-drawings: the drawing-capture member of the
//! `tool-precedent-*` Platform-shaped precedent substrate. Forked from
//! `tool-documents-core`'s shard-capture pattern (business-admin/
//! project-documents/tool-documents-core/catalog.rs) -- same shape (scan a
//! runtime-provided data root, group by stem, categorize), adapted for
//! drawing-type files instead of document deliverables, plus a real
//! structural-fact extraction step over FreeCAD's exported IFC output
//! (`ifc` module) that document files have no equivalent of.
//!
//! "Code in engineering, data in place": this crate never embeds a
//! compiled-in path into `business-admin/project-drawings` -- every
//! function here takes a data root as a runtime argument (or reads it from
//! an env var, with the caller supplying the default), exactly like
//! `tool-documents-core::catalog::scan(archive_root: &Path)`. The CLI
//! (`main.rs`) additionally refuses to write `--out` into any git repo that
//! has a remote configured, regardless of what that repo is named.
//!
//! Phase 0 scope (proving the mechanism, matching tool-documents-core's own
//! framing): the catalog scan/categorize pass (`catalog` module) and a
//! minimal real IFC (ISO-10303-21 STEP physical file) entity-type counter
//! (`ifc` module). Not yet built: an actual FreeCAD-to-workspace transfer
//! path (see this archive's NEXT.md) and the Draw Register's flat-file
//! shard-history layer (`tool-documents-core`'s `precedent/versions.csv`
//! equivalent) -- carried forward, not silently assumed done.

pub mod catalog;
pub mod ifc;

pub use catalog::{scan, DrawingRow};
pub use ifc::{parse_entity_counts, IfcParseError};
