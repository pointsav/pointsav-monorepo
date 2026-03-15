# 🌍 VENDOR DEPENDENCY | MAXMIND GEO-ROUTING

**Component:** Sovereign Offline Geographic Database
**Provider:** MaxMind (GeoLite2)
**License Compliance:** End-User must provision their own API Key.

## Engineering Logic
To maintain absolute data sovereignty (zero third-party pings), the Omni-Matrix Telemetry Engine requires a locally hosted `.mmdb` database. PointSav Digital Systems **does not** distribute the proprietary MaxMind database within this repository. 

## Provisioning Protocol
The generic end-user must execute a provisioning script with a valid MaxMind API Key to pull the payload into their local execution environment, placing it securely inside the `app-mediakit-telemetry/assets/` vault.
