# 📊 POINTSAV SOVEREIGN TELEMETRY SUITE

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>


**Version:** 1.2.0 (Compiled Rust Core)
**Standard:** Sovereign Data Protocol (DS-ADR-06)
**Type:** Generic Self-Hosted Analytics Infrastructure
**Vendor:** PointSav Digital Systems

## System Overview
The Sovereign Telemetry Suite is a zero-cookie, mathematically absolute intelligence ledger. It replaces third-party tracking scripts (e.g., Google Analytics) with an air-gapped, privacy-first ingestion diode. 

It captures network interactions directly at the edge, stores them in an immutable `.csv` ledger, and synthesizes the data into 8 highly detailed Institutional Brutalist Markdown matrices.

## Dual-Binary Architecture
This software is compiled natively via Rust into two distinct, memory-safe execution cores:
1. **`telemetry-daemon` (The Shield):** An asynchronous `tokio`/`warp` web server that listens for incoming JS beacons and safely appends them to the physical ledger.
2. **`omni-matrix-engine` (The Synthesizer):** A fault-tolerant calculation engine that reads the ledger, cross-references offline geographic databases, and generates the Markdown reports.

```text
app-mediakit-telemetry/
├── src/bin/
│   ├── telemetry-daemon.rs    # Edge Ingestion Source
│   └── omni-matrix-engine.rs  # Synthesis Source
├── assets/
│   ├── ledger_telemetry.csv   # The Immutable Source of Truth (Generated)
│   └── GeoLite2-City.mmdb     # The Offline Geographic Brain (User Provided)
└── outbox/
    └── REPORT_TIMESTAMP.md    # The Synthesized Financial Matrix
```

## Compilation & Deployment Mandate
This software must be compiled natively for the target deployment architecture (e.g., Linux x86_64).

```bash
# 1. Compile the binaries

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

cargo build --release

# 2. Provision the database

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

# You must place a licensed GeoLite2-City.mmdb inside the assets/ directory.

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>


# 3. Ignite the Daemon (Background Service)

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

PORT=8081 ./target/release/telemetry-daemon &

# 4. Generate the Matrix (Cron Job)

<div align="center">

[ 🇪🇸 Leer este documento en Español ](./README.es.md)

</div>

FLEET_ID="YOUR_ORG" ./target/release/omni-matrix-engine
```
