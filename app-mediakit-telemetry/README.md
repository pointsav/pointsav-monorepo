# ⚙️ OS-MEDIAKIT: SOVEREIGN TELEMETRY ENGINE
**Vendor:** PointSav Digital Systems™
**Protocol:** DS-ADR-06 (Zero-Cookie Architecture)
**Tier:** 1 (Core Engineering Monorepo)

---

## I. ARCHITECTURAL OVERVIEW
The Sovereign Telemetry Engine is a highly optimized, asynchronous data ingestion diode written in Rust. It provides institutional-grade website analytics without relying on third-party tracking cookies, Javascript states, or centralized silicon valley data brokers.

It operates entirely within the Customer's isolated cloud environment (Tier-2) and maps geographic routing strictly using an offline MaxMind `.mmdb` vault, ensuring absolute data sovereignty and compliance with international privacy mandates.

## II. THE JSON CONTRACT (DS-ADR-06)
The Rust daemon (`telemetry-daemon.rs`) is engineered to instantly drop any connection that does not strictly adhere to the following `POST` payload structure.

**Endpoint:** `POST /telemetry-endpoint`
**Headers:** `Content-Type: application/json`

```json
{
  "uri": "[https://woodfinegroup.com/investors](https://woodfinegroup.com/investors)",
  "timestamp": "2026-03-12T14:30:00Z",
  "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7)..."
}
```

## III. DEPENDENCY MATRIX
To guarantee air-gapped operation, all dependencies must be locally compiled.

1. **Rust Toolchain:** `cargo` (1.70+)
2. **Crates:** - `tokio` (Asynchronous runtime)
   - `warp` (HTTP web server framework)
   - `serde` / `serde_json` (Strict serialization)
   - `maxminddb` (Offline IP routing lookup)
   - `chrono` (ISO 8601 timestamp handling)
3. **Vendor Binaries:** `GeoLite2-City.mmdb` (Must reside in `../vendors-maxmind/` relative to the monorepo root).

## IV. COMPILATION INSTRUCTIONS
The daemons must be compiled specifically for the target Tier-2 cloud architecture (e.g., Linux x86_64).

```bash
# 1. Navigate to the core application
cd app-mediakit-telemetry

# 2. Compile for production release (strips debug symbols for maximum efficiency)
cargo build --release

# 3. Locate the compiled binaries
ls -la target/release/telemetry-daemon
ls -la target/release/telemetry-synthesizer
```

## V. DAEMON ORCHESTRATION (SYSTEMD)
The `telemetry-daemon` is designed to be kept alive by the Linux kernel via `systemd`. It requires strict environment variables to dictate its physical boundaries.

**Required Environment Variables:**
* `PORT`: The internal localhost port the daemon will bind to (e.g., `8081`).
* The daemon automatically detects its `WorkingDirectory` and writes its immutable log to `./assets/ledger_telemetry.csv`.

**Standard Systemd Unit Example:**
```ini
[Unit]
Description=PointSav Sovereign Telemetry Engine
After=network.target

[Service]
Type=simple
User=admin
WorkingDirectory=/opt/deployments/pointsav-fleet-deployment/media-marketing-landing
Environment="PORT=8081"
ExecStart=/opt/deployments/pointsav-fleet-deployment/media-marketing-landing/telemetry-daemon
Restart=always

[Install]
WantedBy=multi-user.target
```

## VI. LEGAL & LICENSING
Refer to the `LICENSE` file in this directory. This software is currently under a strict **Incubation Phase**. All rights are reserved by Woodfine Capital Projects Inc.
