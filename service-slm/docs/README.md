# Documentation

This directory holds the human-readable documentation for service-slm.

We follow the [Divio documentation system](https://documentation.divio.com/)
convention of splitting docs into four quadrants by reader intent:

- **[architecture/](./architecture/)** — *explanation*: how the service
  is designed and why. Read to understand.
- **[adr/](./adr/)** — Architecture Decision Records: the decisions that
  shape the project beyond a single release. Read to learn why something
  is the way it is.
- **[rfcs/](./rfcs/)** — forward-looking proposals for large changes.
- **[dev-guide/](./dev-guide/)** — *tutorial* + *how-to* for
  contributors. Read to start work.
- **[user-guide/](./user-guide/)** — *tutorial* + *how-to* for
  operators, community members, and customers. Read to use the service.
- **[reference/](./reference/)** — precise specifications for CLI,
  HTTP API, ledger events, configuration. Read to look something up.

The normative source of truth for architectural claims lives in
[`../specs/`](../specs/). Docs in this tree explain and operationalise
the specs but never override them.
