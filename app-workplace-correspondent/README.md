# app-workplace-correspondent

Native macOS contacts client (working name: Correspondent). Status: first slice only.

`crates/correspondent-core` is a read-only Apache-2.0 reader for the `service-people` and
`service-business` registries. It reads CSV folders in place and never writes or copies data.
Fixtures contain fictional records only.

Not built yet: the person-to-business link, alias search, interaction history, and the app itself.
