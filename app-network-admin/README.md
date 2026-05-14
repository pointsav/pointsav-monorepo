# app-network-admin

Control-plane application for `os-network-admin` — the PPN mesh routing authority.

Runs on `os-network-admin`. Deployed as `route-network-admin` in the showcase tier
and `route-network-admin-1` on Laptop A in the PointSav Private Network.

Manages the WireGuard mesh configuration and publishes live mesh state at
`public/mesh-state.json`. The Network Ledger UI (`public/index.html`) provides
F-key navigation over the mesh topology.

Master WireGuard key authority resides on this node — it is never delegated to a
cloud provider.
