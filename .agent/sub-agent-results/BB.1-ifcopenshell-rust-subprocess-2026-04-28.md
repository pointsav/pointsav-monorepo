---
schema: foundry-sub-agent-result-v1
brief_id: BB.1
authored: 2026-04-28
authored_by: research sub-agent (Sonnet 4.6)
authored_with: Claude Code sub-agent dispatch
target_cluster: project-bim
target_file: app-workplace-bim/src/ifc_sidecar.rs (scaffold)
subject: IfcOpenShell 0.8.5 Rust subprocess integration — CLI surface, LGPL compliance, and scaffold
---

# BB.1 — IfcOpenShell 0.8.5 Rust Subprocess Integration

**Research date:** 2026-04-28
**IfcOpenShell version targeted:** 0.8.5 (released April 2026, LGPL-3.0)
**Rust target:** Tauri 2.10 backend (tokio ^1, async commands)
**Purpose:** provide everything needed to scaffold `app-workplace-bim/src/ifc_sidecar.rs` correctly first time

---

## 1. `IfcConvert` CLI Surface

`IfcConvert` is the compiled C++ binary that ships with IfcOpenShell 0.8.5. It converts an IFC-SPF file into multiple output formats. It is the primary geometry tool the sidecar will call.

**Source:** https://docs.ifcopenshell.org/ifcconvert/usage.html (IfcOpenShell 0.8.5 official docs)

### Invocation pattern

```
IfcConvert [options] <input.ifc> <output.{glb,svg,obj,...}>
```

On Windows, `.ifc` files can be dragged onto `IfcConvert.exe`. Include/exclude filter flags must not precede the input file argument.

### Supported output formats

| Extension | Format |
|-----------|--------|
| `.glb` | glTF Binary v2.0 — primary visualization target |
| `.obj` + `.mtl` | WaveFront OBJ |
| `.dae` | Collada Digital Assets Exchange |
| `.svg` | Scalable Vector Graphics (2D floor plans) |
| `.stp` | ISO 10303 STEP |
| `.igs` | IGES |
| `.xml` | Property definitions and decomposition tree |
| `.json` | xeokit JSON format |
| `.rdb` | RocksDB key-value store |
| `.h5` | HDF Hierarchical Data Format |
| `.ttl` | RDF Turtle with Well-Known-Text geometry |
| `.ifc` | IFC-SPF (re-serialization) |

### Key CLI flags

**General / output control:**

| Flag | Description |
|------|-------------|
| `-h, --help` | Display usage |
| `--version` | Display version |
| `-v, --verbose` | Increase verbosity (use `-vv` for debug) |
| `-q, --quiet` | Suppress status output; dots to stdout/stderr |
| `-y, --yes` | Auto-confirm prompts (required for scripted invocation) |
| `--no-progress` | Suppress carriage-return progress bar |
| `--stderr-progress` | Route progress messages to stderr instead of stdout |
| `--log-format plain\|json` | Log output as plain text (default) or structured JSON |
| `--log-file PATH` | Write logs to a file instead of stderr |
| `--cache` | Enable geometry caching |

**Parallelism:**

| Flag | Description |
|------|-------------|
| `-j NUM, --threads NUM` | Parallel processing threads (default: 1); `-j 7` recommended for multi-core hosts |

**Geometry kernel and coordinates:**

| Flag | Description |
|------|-------------|
| `--kernel opencascade\|cgal\|hybrid` | Choose geometry kernel |
| `--center-model` | Center by placement origins |
| `--center-model-geometry` | Center by mesh vertex bounding box |
| `--model-offset X;Y;Z` | Translate model coordinates |
| `--model-rotation X;Y;Z;W` | Apply quaternion rotation |
| `--reorient-shells` | Orient face normals consistently (improves glTF output) |
| `--weld-vertices` | Produce manifold mesh |
| `--use-world-coords` | Fold placements into coordinates |
| `--y-up` | Set Y as up axis (default: Z) |
| `--ecef` | Write glTF in Earth-Centered Earth-Fixed coordinates |

**Filtering:**

| Flag | Description |
|------|-------------|
| `--include entities TYPE1 TYPE2` | Include only these IFC entity types |
| `--include attribute NAME VALUE` | Filter by attribute value (e.g., `attribute GlobalId abc123`) |
| `--include+ ...` | Include with full decomposition/containment hierarchy |
| `--exclude entities TYPE` | Exclude entity types |
| `--exclude+ ...` | Exclude with hierarchy |
| `--filter-file PATH` | Load filter criteria from file |

**Meshing detail:**

| Flag | Default | Description |
|------|---------|-------------|
| `--mesher-linear-deflection NUM` | 0.001 | Surface chord error |
| `--mesher-angular-deflection NUM` | 0.5 rad | Angular tolerance |
| `--circle-segments NUM` | 16 | Segments per circle |
| `--no-normals` | — | Skip normal computation |
| `--generate-uvs` | — | Create texture coordinates |

**SVG-specific:**

| Flag | Description |
|------|-------------|
| `--bounds WIDTHxHEIGHT` | Output bounding box |
| `--scale RATIO` | Scale (e.g., `1:100`) |
| `--auto-section` | Auto cross-section per storey |
| `--auto-elevation` | Auto elevations |
| `--draw-storey-heights` | Show storey height indicators |
| `--section-height NUM` | Cut plane height |
| `--print-space-names` | Include space labels |
| `--print-space-areas` | Include calculated area labels |
| `--svg-poly` | Use polygonal hidden-line algorithm |

**Material / naming:**

| Flag | Description |
|------|-------------|
| `--use-element-names` | Name meshes using `IfcRoot.Name` |
| `--use-element-guids` | Name meshes using `GlobalId` |
| `--use-element-step-ids` | Name meshes using STEP numeric IDs |
| `--use-element-types` | Name meshes using element types |
| `--default-material-file PATH` | External default material definitions |
| `--use-material-names` | Name materials by name, not internal ID |
| `--surface-colour` | Prioritize surface color over presentation color |
| `--layerset-first` | Assign first material layer to whole product |
| `--enable-layerset-slicing` | Slice elements by material layers |

**Boolean and validation:**

| Flag | Description |
|------|-------------|
| `--disable-boolean-result` | Skip boolean operations (clippings); faster, less accurate |
| `--exterior-only` | Export only exterior shell |
| `--validate` | Check output geometry against explicit quantities |
| `--convert-back-units` | Retain original IFC units rather than converting to metres |
| `--digits NUM` | Floating-point precision (default: 15) |

### Exit codes

| Code | Meaning |
|------|---------|
| 0 (`EXIT_SUCCESS`) | Successful conversion |
| 1 (`EXIT_FAILURE`) | Error — unrecognized option, input parse failure, unknown output extension, absent geometric elements, serializer init failure, write error, or validation failure |

**Source:** IfcOpenShell source code, IfcConvert.cpp (v0.6.0 reference, confirmed in 0.8.x docs).

### Stderr conventions

- Default: plain text progress bar on stderr, log messages on stderr.
- `--log-format json`: structured JSON log messages, suitable for machine parsing.
- `--stderr-progress`: explicitly routes progress to stderr (useful when stdout is captured for data).
- `--no-progress`: suppresses the progress bar entirely — recommended for scripted invocation.
- `--quiet`: reduces output to dots on stdout/stderr.
- Logs are buffered and written at end of execution via `write_log()`.
- With `--log-file PATH`: logs redirected to file; stderr receives only progress.

**Recommended flags for scripted Rust invocation:**

```bash
IfcConvert -y --no-progress --log-format json --log-file /tmp/ifcconvert.log \
  input.ifc output.glb
```

This combination: suppresses interactive prompts (`-y`), removes the progress bar (`--no-progress`), writes structured logs to a file (not stderr), and keeps stderr silent so Rust can capture it cleanly for error detection.

### Performance notes

IfcConvert is fast on files under 20 MB. Files around 100 MB can take minutes. For large files: use `-j <N>` where N is core count minus one; note that multithreading has had geometry correctness issues in some earlier versions (tracked in issue #1508). Validate multithreaded output against sequential for a given model size before relying on it in production.

---

## 2. `ifctester` CLI Surface

`ifctester` is a pure Python module (no compiled binary). It validates an IFC model against an IDS (Information Delivery Specification) file.

**Source:** https://docs.ifcopenshell.org/ifctester.html; source confirmed at `src/ifctester/ifctester/__main__.py` in the IfcOpenShell master branch.

### Invocation

```
python -m ifctester [-h] [-r REPORTER] [--no-color] [--excel-safe] [-o OUTPUT] <ids> <ifc>
```

### Flags

| Flag | Description |
|------|-------------|
| `ids` | Path to the IDS file (required positional) |
| `ifc` | Path to the IFC file (required positional) |
| `-r, --reporter REPORTER` | Output reporter: `Console` (default), `Txt`, `Json`, `Html`, `Ods`, `Bcf` |
| `--no-color` | Disable ANSI color in Console output |
| `--excel-safe` | Export ODS in Excel-compatible mode |
| `-o, --output PATH` | Output file path (required for non-Console reporters) |

### Output format by reporter

| Reporter | Output |
|----------|--------|
| `Console` | Colored text to stdout (human-readable) |
| `Txt` | Plain text file |
| `Json` | JSON file at `-o PATH` |
| `Html` | HTML report |
| `Ods` | ODS spreadsheet |
| `Bcf` | BCF 2.1 issue file |

### Exit codes

**Critical caveat:** As of the current source, `__main__.py` contains no explicit `sys.exit()` calls. The module exits 0 on successful completion and raises an unhandled Python exception (exit code 1) on load failure. Validation failures (IDS requirements not met) do NOT produce a non-zero exit code — the process exits 0 with the failure details embedded in the reporter output.

**Implication for Rust:** Do not rely on exit code to detect validation failures. Instead:

- Use `-r Json -o /tmp/result.json` and parse the JSON output for the `status` or failed-requirement count.
- Alternatively, pipe Console output and grep for the word "FAIL" or inspect `Html` output.

**Known bug:** GitHub issue #4526 documents that the Json reporter produced an empty `{}` output in some versions. Verify against the installed version. The recommended approach is to use `Html` or `Txt` for human review and parse the `Json` reporter output programmatically only after confirming it is non-empty.

### Examples

```bash
# Console output (human review)
python -m ifctester requirements.ids model.ifc

# JSON report for machine parsing
python -m ifctester requirements.ids model.ifc -r Json -o /tmp/ids-result.json

# BCF issue file for design coordination
python -m ifctester requirements.ids model.ifc -r Bcf -o /tmp/issues.bcf
```

### Timing print

The `__main__.py` source prints timing to stdout: `"Finished loading: X.XX"` and `"Finished validating: X.XX"`. Rust code parsing stdout must tolerate these lines before the actual report output when not using a file-output reporter.

---

## 3. `ifccsv` CLI Surface

`ifccsv` exports IFC attribute data to CSV/ODS/XLSX and can re-import edited sheets back to IFC. The COBie workflow in the cluster uses the related `ifcfm` module.

**Source:** https://docs.ifcopenshell.org/ifccsv.html; https://docs.ifcopenshell.org/ifcfm.html

### `ifccsv` invocation

```
python -m ifccsv [-h] -i IFC [-s SPREADSHEET] [-f FORMAT] [-d DELIMITER]
                 [-n NULL] [-e EMPTY] [--bool_true BOOL_TRUE]
                 [--bool_false BOOL_FALSE] [--concat CONCAT]
                 [-q QUERY] [-a ATTRIBUTES [ATTRIBUTES ...]]
                 [--headers HEADERS [HEADERS ...]]
                 [--sort SORT [SORT ...]] [--order ORDER [ORDER ...]]
                 [--export] [--import]
```

### `ifccsv` flags

| Flag | Description |
|------|-------------|
| `-i IFC` | IFC input file (required) |
| `-s SPREADSHEET` | Output spreadsheet path |
| `-f FORMAT` | Format: `csv`, `ods`, `xlsx` |
| `-d DELIMITER` | CSV delimiter (default: comma) |
| `-n NULL` | Null representation (default: `N/A`) |
| `-e EMPTY` | Empty string representation (default: `-`) |
| `--bool_true` | True value (default: `YES`) |
| `--bool_false` | False value (default: `NO`) |
| `--concat` | List concatenation separator (default: `', '`) |
| `-q QUERY` | IFC query selector e.g. `IfcProduct`, `IfcWall` |
| `-a ATTRIBUTES` | Space-separated attributes using IfcQuery syntax: `class`, `Name`, `Pset_Foo.Bar` |
| `--headers` | Human-readable column names, one per attribute |
| `--sort` | Sort attribute(s) |
| `--order` | Sort direction `ASC` or `DESC` per attribute |
| `--export` | Export IFC to spreadsheet |
| `--import` | Import spreadsheet back to IFC |

### COBie export via `ifcfm`

For US federal COBie mandate compliance, use `ifcfm` rather than `ifccsv` directly.

```
python -m ifcfm [-h] [-p PRESET] -i IFC [-s SPREADSHEET] [-f FORMAT]
                [-d DELIMITER] [-n NULL] [-e EMPTY]
                [--bool_true BOOL_TRUE] [--bool_false BOOL_FALSE]
```

| Flag | Description |
|------|-------------|
| `-i IFC` | IFC file (required) |
| `-p PRESET` | FM standard preset (required for COBie) |
| `-s SPREADSHEET` | Output file (defaults to `output.ods`) |
| `-f FORMAT` | `csv`, `ods`, `xlsx` |

**Available presets:**

| Preset | Standard |
|--------|----------|
| `cobie24` | COBie 2.4 (US federal mandate) |
| `cobie3` | COBie 3.0 |
| `aohbsem` | AOH-BSEM |
| `basic` | Vanilla IFC, specification-agnostic |

### COBie export example

```bash
# Export COBie 2.4 as ODS (default)
python -m ifcfm -i model.ifc -p cobie24 -s output.ods

# Export COBie 2.4 as XLSX for US federal submission
python -m ifcfm -i model.ifc -p cobie24 -f xlsx -s cobie-export.xlsx
```

### Exit codes

No explicit `sys.exit()` calls in the v0.6.0 source; same pattern as ifctester. Exit 0 on success, unhandled exception (exit 1) on failure. No non-zero exit on partial data extraction failures.

---

## 4. Python Module Invocation Pattern

When features are needed beyond what the CLI binaries expose — property graph traversal, spatial queries, IFC entity inspection, custom attribute extraction — the pattern is to spawn a Python script that uses `ifcopenshell` as a library and writes structured JSON to stdout.

### Venv expectation

IfcOpenShell is not a standard library module. The Rust sidecar must invoke the Python interpreter from a known virtualenv or conda environment where `ifcopenshell` is installed.

**Installation distinction (critical):**
- `pip install ifcopenshell` installs the Python library but does NOT install the `IfcConvert` compiled binary.
- `conda install -c ifcopenshell -c conda-forge ifcopenshell` installs both the Python library AND `IfcConvert`.
- The compiled CLI binaries (`IfcConvert`, `ifctester`, `ifccsv`, `ifcfm`) are Python modules or separate native binaries depending on installation route.

**Locating the interpreter:** The Rust sidecar should resolve the Python executable path at startup via a configured environment variable (`IFCOPENSHELL_PYTHON`) or by probing known locations (`$HOME/.venv/bim/bin/python3`, `$(which python3)`).

### Canonical invocation pattern

```bash
# Invoke a script that writes JSON to stdout, diagnostics to stderr
/path/to/venv/bin/python3 /path/to/script.py \
  --input /tmp/input.ifc \
  --guid IfcBuildingStorey_123abc
```

Or inline for simple queries:

```bash
/path/to/venv/bin/python3 -c "
import sys, json, ifcopenshell
ifc = ifcopenshell.open(sys.argv[1])
storey = ifc.by_type('IfcBuildingStorey')
result = [{'name': s.Name, 'elevation': s.Elevation} for s in storey]
print(json.dumps(result))
" /tmp/input.ifc
```

### JSON-over-stdout convention

- The Python script writes exactly one JSON value (object or array) to stdout on success.
- All diagnostic messages (timing, warnings, progress) go to stderr — never stdout.
- On error, the script exits non-zero and writes a JSON error object to stdout:
  `{"error": "description", "type": "ErrorClassName"}` — or writes nothing to stdout and a human-readable message to stderr.
- Rust reads stdout until EOF, then parses. If exit code is non-zero, treat stdout content as unreliable and read stderr for the error message.

### Error reporting discipline

```python
import sys
import json
import traceback

try:
    # ... main logic ...
    print(json.dumps(result))
except Exception as e:
    print(json.dumps({"error": str(e), "type": type(e).__name__}), file=sys.stdout)
    traceback.print_exc(file=sys.stderr)
    sys.exit(1)
```

This discipline ensures the Rust caller always has a parseable JSON object on stdout regardless of outcome.

### Supported IFC serializations readable by ifcopenshell

IFC-SPF (`.ifc`), IFC-JSON, IFC-XML (`.ifcXML`), IFC-HDF5 (`.ifcHDF`). The Rust sidecar should always pass `.ifc` (IFC-SPF) as input unless specifically testing other serializations.

---

## 5. Rust Subprocess Best Practices

### `std::process::Command` vs `tokio::process::Command`

| Aspect | `std::process::Command` | `tokio::process::Command` |
|--------|-------------------------|---------------------------|
| Blocking | Blocks the calling thread | Async; yields to Tokio executor |
| Use in Tauri | Acceptable only in dedicated thread | Preferred — native to Tauri's tokio runtime |
| API | `.output()` (sync) | `.output()` (async), `.spawn()` |
| Tauri async command | Will block Tauri handler thread | Correct choice |

**Verdict:** Use `tokio::process::Command` throughout. Tauri 2.10 ships `tokio ^1` as a dependency; the Tauri `async_runtime` module re-exports tokio primitives. Spawning `std::process::Command` in a Tauri `#[command]` handler blocks the async executor thread.

Alternatively: use Tauri's built-in `tauri_plugin_shell` sidecar API for bundled binaries (see Section 7 for Tauri-specific notes).

### Handling large IFC files (50 MB+)

**Do not pass large IFC files over stdin.** IfcConvert and the Python ifcopenshell module both expect a file path, not a stdin stream. Pass a file path directly:

```rust
Command::new("IfcConvert")
    .arg(input_path)   // path to existing .ifc on disk
    .arg(output_path)  // path where .glb will be written
    .arg("-y")
    .arg("--no-progress")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
```

When the input is an in-memory IFC buffer (e.g., downloaded from a HTTPS source), write it to a `NamedTempFile` first:

```rust
use tempfile::NamedTempFile;

let mut tmp = NamedTempFile::new()?;
tmp.write_all(&ifc_bytes)?;
tmp.flush()?;
// Keep the file alive during subprocess execution
let (tmp_file, tmp_path) = tmp.keep()?;  // prevent drop-cleanup
let output = Command::new("IfcConvert")
    .arg(&tmp_path)
    .arg(&output_path)
    // ...
    .output()
    .await?;
// tmp_file drops here, OS cleans up
```

Use `NamedTempFile::keep()` (returns `(File, PathBuf)`) rather than `path()` to prevent TOCTOU races where a temp-file cleaner removes the file between your `.path()` call and the subprocess opening it. The returned `File` keeps the file descriptor open, preventing deletion on Linux until it is dropped.

Output files (`.glb`, `.svg`) should be written to a deterministic output path that the Rust code specifies, not a temp file, so the caller knows where to find them after the subprocess exits.

### Capturing stderr without deadlock

The standard deadlock scenario: both stdout and stderr fill their OS pipe buffers and the parent blocks waiting to read one while the child blocks waiting to write the other.

**Safe pattern using `wait_with_output()`:**

```rust
let child = Command::new("IfcConvert")
    .arg(input)
    .arg(output)
    .arg("-y")
    .arg("--no-progress")
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()?;

let output = child.wait_with_output().await?;
// output.stdout, output.stderr, output.status all available
```

`wait_with_output()` closes stdin automatically before waiting and drains both stdout and stderr into memory concurrently. This is the standard deadlock-safe pattern for subprocesses that produce bounded output. For IfcConvert with `--no-progress`, stderr output is small (log messages); stdout is empty. Memory pressure is not a concern.

**When IfcConvert output is large (xml/json/rdb):** write output to a file path rather than capturing stdout. Never pipe multi-MB binary output through stdout.

### Timeout and cancellation

`tokio::process::Command` does not provide a built-in timeout. Use `tokio::time::timeout`:

```rust
use tokio::time::{timeout, Duration};

let result = timeout(Duration::from_secs(300), child.wait_with_output()).await;
match result {
    Ok(Ok(output)) => { /* success */ }
    Ok(Err(io_err)) => { /* process error */ }
    Err(_elapsed) => {
        // Timeout — kill the child
        child.kill().await?;
        return Err(IfcSidecarError::Timeout);
    }
}
```

`child.kill()` sends `SIGKILL` on Unix (equivalent to `kill -9`). It also awaits process reaping, so no zombie processes.

For cooperative cancellation from a Tauri frontend: use a `tokio_util::CancellationToken` stored in `tauri::State<Mutex<Option<CancellationToken>>>`:

```rust
tokio::select! {
    result = child.wait_with_output() => { /* handle */ }
    _ = cancel_token.cancelled() => {
        child.kill().await?;
        return Err(IfcSidecarError::Cancelled);
    }
}
```

### Subprocess pool reuse

IfcConvert and the Python ifcopenshell interpreters do not support a long-running server mode or a keep-alive protocol. Each invocation starts fresh: IFC file is loaded, processed, output written, process exits. There is no connection pool pattern applicable.

For concurrency control (limit to N simultaneous conversions on a multi-core host), use a `tokio::sync::Semaphore`:

```rust
static CONVERSION_SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(4));

async fn ifc_to_gltf_guarded(input: &Path, output: &Path) -> Result<(), IfcSidecarError> {
    let _permit = CONVERSION_SEMAPHORE.acquire().await.unwrap();
    ifc_to_gltf_inner(input, output).await
}
```

This caps simultaneous IfcConvert child processes at 4, preventing memory exhaustion on large models.

---

## 6. LGPL-3.0 Compliance via Subprocess

### The legal pattern

The IfcOpenShell repository is licensed under LGPL-3.0. The cluster's calling Rust binary is EUPL-1.2. The two licenses are compatible when IfcOpenShell is invoked as a separate executable process rather than linked into the Rust binary.

**LGPL-3.0 text, Section 0 (Additional Definitions):**

> "Application" means any work that makes use of an interface provided by the Library, but which is not otherwise based on the Library.
> "Combined Work" means a work produced by combining or linking an Application with the Library.

**LGPL-3.0 text, Section 4 (Combined Works):**

> You may convey a Combined Work under terms of your choice that, taken together, effectively do not restrict modification of the portions of the Library contained in the Combined Work and reverse engineering for debugging such modifications, if you also do each of the following: (a) Give prominent notice with each copy of the Combined Work that the Library is used in it and that the Library and its use are covered by this License. (b) Accompany the Combined Work with a copy of the GNU GPL and this license document.

This is the obligation that applies when you produce a **Combined Work** — i.e., when you link or embed the library.

**The subprocess pattern avoids Combined Work status.** The FSF FAQ states:

> "By contrast, pipes, sockets and command-line arguments are communication mechanisms normally used between two separate programs. So when they are used for communication, the modules normally are separate programs."

When the Rust binary invokes `IfcConvert` as a child process communicating via file paths and exit codes, the two programs are not linked. The Rust binary does not contain IfcOpenShell code. No Combined Work is produced.

### What this means in practice

| Action | LGPL-3.0 status |
|--------|----------------|
| Link `libIfcGeom.so` into Rust binary | Combined Work — Section 4 obligations apply |
| Statically link IfcOpenShell into Rust binary | Combined Work — Section 4 obligations + must provide object files for relinking |
| Invoke `IfcConvert` as subprocess via file paths | Separate programs — no Combined Work; no LGPL obligations on calling code |
| Bundle `IfcConvert` binary with the application installer | Must provide `IfcConvert` source code or reference to upstream LGPL source |
| Use `python -m ifctester` via subprocess | Same analysis as IfcConvert: separate process, no Combined Work |

### Distribution obligations when bundling

If the application installer ships `IfcConvert` alongside the Rust binary (as a Tauri external binary / sidecar), LGPL-3.0 requires:

1. A prominent notice that IfcOpenShell (LGPL-3.0) is bundled.
2. A copy of the LGPL-3.0 license text in the distribution.
3. Either: ship IfcOpenShell source code, or provide a written offer to supply it, or include a URL to the upstream source at `https://github.com/IfcOpenShell/IfcOpenShell`.

These obligations are on the distribution, not on the Rust source code itself. The Rust source remains EUPL-1.2 without modification.

### Caveat: "intimate communication" test

The FSF FAQ adds: "But if the semantics of the communication are intimate enough, exchanging complex internal data structures, that too could be a basis to consider the two parts as combined into a larger program."

The subprocess pattern as designed — file path in, file path out, exit code — communicates only file system artifacts. No IfcOpenShell internal data structures are serialized across the process boundary. This is the maximally safe pattern. Avoid patterns that deserialize IfcOpenShell-internal binary formats across the boundary (e.g., the `.rdb` RocksDB format) for clarity of separation, even though the legal risk of doing so is low.

---

## 7. Recommended Rust Crate Dependencies

These are the crates required to scaffold `ifc_sidecar.rs` in a Tauri 2.10 backend.

```toml
[dependencies]
# Async runtime — already a Tauri dependency; do not re-declare unless needed for features
tokio = { version = "1", features = ["process", "time", "sync", "fs", "io-util"] }

# Temp file creation for large IFC buffers passed as in-memory data
tempfile = "3"

# JSON serialization for Python subprocess results and structured log parsing
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Error handling
thiserror = "2"

# Lazy static initialization for the conversion semaphore
once_cell = "1"  # or use std::sync::OnceLock if Rust >= 1.70

# Tauri shell plugin (for bundled sidecar binaries via tauri-plugin-shell)
# Only if using Tauri's native sidecar API rather than raw tokio::process
tauri-plugin-shell = "2"
```

**Version notes:**
- `tokio 1.x` is correct; Tauri 2.10.3 pins `tokio ^1`.
- `tempfile 3.x` is stable and widely used; no major breaking changes expected.
- `serde_json 1.x` and `serde 1.x` are stable.
- `thiserror 2.x` (released late 2024, stable API).
- `once_cell 1.x` or `std::sync::OnceLock` for the semaphore singleton; either works.

**What to avoid:**
- Do not add `subprocess` (hniksic) or `rust-subprocess` crates — `tokio::process::Command` is sufficient and avoids a sync/async mismatch.
- Do not add `tokio-serde-json` — it targets stream framing (TCP), not one-shot subprocess capture.

---

## 8. Sample Code Skeleton

This is a scaffold for `app-workplace-bim/src/ifc_sidecar.rs`. It is not production-ready — it is structured to demonstrate the correct approach so a Task Claude can build from it.

```rust
//! ifc_sidecar.rs — IfcOpenShell subprocess integration
//!
//! Invokes IfcConvert (C++ binary) and Python ifctester/ifccsv modules
//! as separate child processes. IfcOpenShell remains LGPL-3.0; this
//! module is EUPL-1.2. The subprocess boundary satisfies the LGPL
//! "separate programs" test.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tempfile::NamedTempFile;
use thiserror::Error;
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;

// ── Error type ─────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum IfcSidecarError {
    #[error("subprocess I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("IfcConvert exited with code {code}: {stderr}")]
    ConvertFailed { code: i32, stderr: String },

    #[error("IfcConvert produced no output at {0}")]
    NoOutput(PathBuf),

    #[error("Python subprocess exited with code {code}: {stderr}")]
    PythonFailed { code: i32, stderr: String },

    #[error("JSON parse error from Python subprocess: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("subprocess timed out after {0}s")]
    Timeout(u64),

    #[error("subprocess was cancelled")]
    Cancelled,

    #[error("tempfile error: {0}")]
    TempFile(#[from] tempfile::PersistError),
}

// ── Configuration ──────────────────────────────────────────────────────────

/// Resolved at runtime from env or config.
pub struct IfcSidecarConfig {
    /// Path to the IfcConvert binary.
    pub ifcconvert_bin: PathBuf,
    /// Path to the Python interpreter in the IfcOpenShell venv.
    pub python_bin: PathBuf,
    /// Timeout for geometry conversion (seconds). Large files need 5+ min.
    pub convert_timeout_secs: u64,
    /// Timeout for IDS validation (seconds).
    pub validate_timeout_secs: u64,
    /// Threads to pass to IfcConvert -j flag.
    pub threads: usize,
}

impl Default for IfcSidecarConfig {
    fn default() -> Self {
        Self {
            ifcconvert_bin: PathBuf::from(
                std::env::var("IFCCONVERT_BIN")
                    .unwrap_or_else(|_| "IfcConvert".to_string()),
            ),
            python_bin: PathBuf::from(
                std::env::var("IFCOPENSHELL_PYTHON")
                    .unwrap_or_else(|_| "python3".to_string()),
            ),
            convert_timeout_secs: 600,   // 10 min for large models
            validate_timeout_secs: 120,
            threads: num_cpus::get().saturating_sub(1).max(1),
        }
    }
}

// ── Concurrency guard ──────────────────────────────────────────────────────

/// Limit simultaneous IfcConvert processes to avoid OOM on large models.
static CONVERSION_SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(4));

// ── Core function ──────────────────────────────────────────────────────────

/// Convert an IFC-SPF file to glTF binary (.glb).
///
/// Returns the path to the output `.glb` file on success.
/// `output_path` must have a `.glb` extension; the file is created by
/// IfcConvert and must not exist beforehand (or IfcConvert will overwrite).
pub async fn ifc_to_gltf(
    input_path: &Path,
    output_path: &Path,
    config: &IfcSidecarConfig,
) -> Result<PathBuf, IfcSidecarError> {
    // Acquire semaphore slot before spawning.
    let _permit = CONVERSION_SEMAPHORE.acquire().await.unwrap();

    let child = Command::new(&config.ifcconvert_bin)
        // Auto-confirm prompts (required for scripted use).
        .arg("-y")
        // Suppress progress bar (prevents carriage-return noise on stderr).
        .arg("--no-progress")
        // Structured JSON log — parseable if needed; written to stderr here.
        .arg("--log-format")
        .arg("json")
        // Multi-threading.
        .arg("-j")
        .arg(config.threads.to_string())
        // Ensure face normals are consistent for glTF viewers.
        .arg("--reorient-shells")
        // Input and output paths.
        .arg(input_path)
        .arg(output_path)
        // Capture stdout (expected empty) and stderr (log messages).
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = timeout(
        Duration::from_secs(config.convert_timeout_secs),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| IfcSidecarError::Timeout(config.convert_timeout_secs))??;

    let exit_code = output.status.code().unwrap_or(-1);
    if exit_code != 0 {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(IfcSidecarError::ConvertFailed {
            code: exit_code,
            stderr,
        });
    }

    // Verify output file was produced.
    if !output_path.exists() {
        return Err(IfcSidecarError::NoOutput(output_path.to_owned()));
    }

    Ok(output_path.to_owned())
}

// ── IFC-to-glTF from in-memory buffer ─────────────────────────────────────

/// Variant for when the IFC content is already in memory (e.g., downloaded).
/// Writes to a temp file first to avoid stdin pipe issues with large files.
pub async fn ifc_bytes_to_gltf(
    ifc_bytes: &[u8],
    output_path: &Path,
    config: &IfcSidecarConfig,
) -> Result<PathBuf, IfcSidecarError> {
    use std::io::Write;

    // Write to temp file; keep() prevents drop-cleanup while subprocess runs.
    let mut tmp = NamedTempFile::new()?;
    tmp.write_all(ifc_bytes)?;
    tmp.flush()?;
    let (_tmp_file, tmp_path) = tmp.keep().map_err(|e| IfcSidecarError::TempFile(e))?;

    let result = ifc_to_gltf(&tmp_path, output_path, config).await;

    // Clean up temp file regardless of outcome.
    let _ = std::fs::remove_file(&tmp_path);

    result
}

// ── IDS validation via ifctester ───────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct IdsValidationResult {
    /// Parsed from ifctester Json reporter output.
    /// NOTE: ifctester exit code is always 0 on validation failure;
    /// use this struct to determine pass/fail.
    pub passed: Option<bool>,
    pub total: Option<u32>,
    pub failed: Option<u32>,
    // Additional fields vary by ifctester version; use serde Value for robustness.
    #[serde(flatten)]
    pub raw: serde_json::Value,
}

/// Validate an IFC model against an IDS file.
///
/// Returns parsed JSON from ifctester's Json reporter.
/// WARNING: ifctester exits 0 even on validation failure; check
/// `result.failed` to determine if requirements were met.
pub async fn validate_ids(
    ifc_path: &Path,
    ids_path: &Path,
    config: &IfcSidecarConfig,
) -> Result<IdsValidationResult, IfcSidecarError> {
    use tempfile::tempdir;

    let out_dir = tempdir()?;
    let result_path = out_dir.path().join("result.json");

    let child = Command::new(&config.python_bin)
        .args(["-m", "ifctester"])
        .arg(ids_path)
        .arg(ifc_path)
        .arg("-r")
        .arg("Json")
        .arg("-o")
        .arg(&result_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = timeout(
        Duration::from_secs(config.validate_timeout_secs),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| IfcSidecarError::Timeout(config.validate_timeout_secs))??;

    let exit_code = output.status.code().unwrap_or(-1);
    if exit_code != 0 {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(IfcSidecarError::PythonFailed {
            code: exit_code,
            stderr,
        });
    }

    // Read JSON result file.
    let json_bytes = tokio::fs::read(&result_path).await?;
    let result: IdsValidationResult = serde_json::from_slice(&json_bytes)?;
    Ok(result)
}

// ── Python module invocation helper ───────────────────────────────────────

/// Run an arbitrary Python script that writes a single JSON value to stdout.
/// Script must print one JSON object on success, or
/// {"error": "...", "type": "..."} + exit(1) on failure.
pub async fn run_python_json_script<T: for<'de> Deserialize<'de>>(
    python_bin: &Path,
    script_args: &[&str],
    timeout_secs: u64,
) -> Result<T, IfcSidecarError> {
    let child = Command::new(python_bin)
        .args(script_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = timeout(
        Duration::from_secs(timeout_secs),
        child.wait_with_output(),
    )
    .await
    .map_err(|_| IfcSidecarError::Timeout(timeout_secs))??;

    let exit_code = output.status.code().unwrap_or(-1);
    if exit_code != 0 {
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        return Err(IfcSidecarError::PythonFailed {
            code: exit_code,
            stderr,
        });
    }

    let result: T = serde_json::from_slice(&output.stdout)?;
    Ok(result)
}
```

**Notes on this skeleton:**
- `num_cpus` is not listed in Section 7 — either add it as a dependency (`num_cpus = "1"`) or hard-code the thread count.
- `IdsValidationResult` uses `#[serde(flatten)] raw: serde_json::Value` as a safety net because the ifctester JSON schema is not formally published and has changed between versions.
- The `ifc_bytes_to_gltf` function manually cleans up the temp file after the subprocess exits. Using `NamedTempFile::keep()` prevents races but requires explicit cleanup.
- Tauri sidecar API (`tauri_plugin_shell`) is an alternative to raw `tokio::process::Command` for the IfcConvert binary. The Tauri sidecar approach handles binary bundling and architecture-suffix naming automatically but requires declaring `IfcConvert` in `tauri.conf.json`'s `bundle.externalBin`. For `python3` (which is not bundled), use `tokio::process::Command` directly.

---

## 9. Open Questions and Risks

1. **ifctester exit code on validation failure (HIGH RISK).** Confirmed by source inspection: `__main__.py` has no `sys.exit()` call differentiated by pass/fail. Exit code is 0 on both validation success and validation failure. The cluster must parse JSON reporter output, not exit code, to detect IDS non-compliance. This is a known design limitation; a GitHub issue (#4526) also documented a bug where the Json reporter produced `{}` (empty). Verify the Json reporter output is non-empty before deploying; consider using Html reporter and a regex parse as a fallback.

2. **ifctester `--excel-safe` flag documentation gap.** The flag is documented but its exact effect on ODS output structure is not described in official docs. If the `service-codes` module needs to consume ODS output programmatically, test this flag empirically before relying on it.

3. **IfcConvert multithreading geometry correctness.** GitHub issue #1508 notes that `-j N` (multi-threading) has caused geometry errors in some model configurations. The performance gain is real but the correctness tradeoff should be tested against the specific IFC models the cluster will encounter. Default to `-j 1` for production until tested; add `-j` as a configurable option.

4. **IfcConvert vs `python -m ifcopenshell` for geometry.** `IfcConvert` is a compiled binary and requires separate installation (Conda or manual download). The Python API does not expose all the same geometry export paths. Ensure the deployment environment has `IfcConvert` available, not just the `ifcopenshell` Python package.

5. **Tauri sidecar architecture-suffix requirement.** If bundling IfcConvert inside the Tauri app via `tauri-plugin-shell`, each target architecture requires a separate renamed binary: `IfcConvert-x86_64-unknown-linux-gnu`, `IfcConvert-aarch64-apple-darwin`, etc. IfcOpenShell releases pre-built binaries for major platforms; verify availability for all target architectures before committing to this bundling approach.

6. **IfcConvert version pinning.** The `BIM_Buildable Architecture.md` document specifies IfcOpenShell 0.8.5. The CLI flag surface described here is based on 0.8.5 docs and 0.6.0 source (used as primary source for exit code and logging implementation). Verify `IfcConvert --version` output in the deployment environment matches 0.8.5 before relying on the `--log-format json` flag, which may behave differently in older installed versions.

7. **Large file timeout selection.** The skeleton defaults to a 600-second (10-minute) timeout for geometry conversion. Field evidence from GitHub issue #1508 indicates that a 100 MB IFC file can take "really long time" but no specific benchmark is documented. For the cluster's expected file sizes (property-manager context, typical 10–50 MB), a 5-minute timeout is likely adequate. Expose this as a configurable parameter rather than hard-coding.

8. **Python venv discovery in Tauri packaging.** The Rust sidecar resolves Python via `IFCOPENSHELL_PYTHON` env var or `which python3`. In a Tauri-bundled application, the user's system Python may not have `ifcopenshell` installed. The installer or onboarding flow must provision the venv, and the app must record the venv path. This is an installer design question, not a code question — but it must be resolved before the sidecar can function on end-user machines.

9. **COBie 3.0 vs COBie 2.4 preset stability.** `ifcfm` ships with `cobie3` as a preset, but COBie 3.0 is not yet widely mandated by US federal agencies (COBie 2.4 is the current US mandate). Verify the `cobie3` preset produces output that downstream US federal systems can ingest, or default to `cobie24`.

10. **ifctester `ifc` argument is positional and optional in docs but required in practice.** The docs state `ifc` is "optional" (the module can be used for IDS authoring without an IFC), but for validation it is required. The CLI will raise an error if omitted. Not a risk, but worth noting to avoid confusing help-text parsing.

---

## 10. Sources

- [IfcConvert Usage — IfcOpenShell 0.8.5 documentation](https://docs.ifcopenshell.org/ifcconvert/usage.html)
- [IfcTester — IfcOpenShell 0.8.4 documentation](https://docs.ifcopenshell.org/ifctester.html)
- [IfcCSV — IfcOpenShell 0.8.5 documentation](https://docs.ifcopenshell.org/ifccsv.html)
- [IfcFM — IfcOpenShell 0.8.4 documentation](https://docs.ifcopenshell.org/ifcfm.html)
- [IfcOpenShell Installation — 0.8.5](https://docs.ifcopenshell.org/ifcopenshell-python/installation.html)
- [IfcOpenShell GitHub repository](https://github.com/IfcOpenShell/IfcOpenShell)
- [ifctester `__main__.py` source (master branch)](https://github.com/IfcOpenShell/IfcOpenShell/blob/master/src/ifctester/ifctester/__main__.py)
- [IfcConvert performance issue #1508](https://github.com/IfcOpenShell/IfcOpenShell/issues/1508)
- [ifctester Json reporter bug #4526](https://github.com/IfcOpenShell/IfcOpenShell/issues/4526)
- [IfcConvert DeepWiki CLI reference](https://deepwiki.com/IfcOpenShell/IfcOpenShell/3-ifcconvert)
- [tokio::process::Command docs](https://docs.rs/tokio/latest/tokio/process/struct.Command.html)
- [tokio::process::Child docs](https://docs.rs/tokio/latest/tokio/process/struct.Child.html)
- [tempfile crate — NamedTempFile](https://docs.rs/tempfile/latest/tempfile/struct.NamedTempFile.html)
- [Tauri v2 Sidecar documentation](https://v2.tauri.app/develop/sidecar/)
- [GNU LGPL v3.0 full text](https://www.gnu.org/licenses/lgpl-3.0.en.html)
- [GNU Licenses FAQ — pipes and separate programs](https://www.gnu.org/licenses/gpl-faq.en.html)
- [LGPL and Dynamic Linking — licensecheck.io](https://licensecheck.io/blog/lgpl-dynamic-linking)
- [Tauri 2.10.3 on crates.io](https://docs.rs/crate/tauri/latest)
- [CancellationToken in tokio-util](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
- [Rust tokio task cancellation patterns (cybernetist.com, 2024)](https://cybernetist.com/2024/04/19/rust-tokio-task-cancellation-patterns/)
