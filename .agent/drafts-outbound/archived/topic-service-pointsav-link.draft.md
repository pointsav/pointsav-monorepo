---
schema: foundry-draft-v1
state: draft
originating_cluster: project-infrastructure
target_repo: content-wiki-documentation
target_path: architecture/
target_filename: service-pointsav-link.md
audience: technical operators and engineers evaluating or deploying PointSav fleets
bcsc_class: no-disclosure-implication
language_protocol: PROSE-TOPIC
authored: 2026-05-20
authored_by: project-infrastructure@claude-code
authored_with: claude-sonnet-4-6
references:
  - architecture/diode-standard.md
  - architecture/machine-based-auth.md
  - systems/os-network-admin.md
  - systems/infrastructure-os.md
notes_for_editor: >
  New topic — no existing file at this path. service-pointsav-link is described in
  diode-standard.md §"The adapter" (4-property table: default/activated/failure/code-path)
  and §"The Universal Standard". This topic provides the dedicated deep-dive. Content is
  derived entirely from the four listed sources; no new architecture is introduced.
  Article frontmatter to add on commit: title "service-pointsav-link",
  category "architecture", status "active", quality "review",
  cites [diode-standard, machine-based-auth, os-network-admin, infrastructure-os].
research_done_count: 4
research_suggested_count: 2
open_questions_count: 0
research_provenance: >
  Four source documents read directly from the current working tree:
  (1) architecture/diode-standard.md — adapter property table (default not installed,
  activated hot-plugged single command, failure mode clean severance, code path inside
  adapter not kernel), Universal Standard section;
  (2) architecture/machine-based-auth.md — pairing types (ADMIN/INPUT/USER/INTERFACE),
  hardware-bound keys, revocation mechanics that service-pointsav-link participates in;
  (3) systems/os-network-admin.md — the Authority side that issues commands to Subjects
  connected via service-pointsav-link;
  (4) systems/infrastructure-os.md — os-infrastructure is a Subject; service-pointsav-link
  is the adapter that brings it under fleet management.
research_inline: true
---

# service-pointsav-link

`service-pointsav-link` is the hot-pluggable adapter that connects an `os-*` Subject node to a PointSav fleet. It is the sole code responsible for translating authority commands — issued by `os-network-admin` and delivered via the [[ppn-command-protocol|PPN Command Protocol]] — into Subject-executable operations. The adapter ships as the `pointsav-protocol` package. Its most important property is its default state: it is not installed. A Subject with no `service-pointsav-link` has no concept of phoning home, receiving commands, or participating in fleet management.

## The four properties

| Property | Behaviour |
|---|---|
| Default state | Not installed; the Subject has no concept of phoning home |
| Activated state | Hot-plugged by the operator with a single command; brings the Subject under fleet management |
| Failure mode | If the adapter crashes, the link severs cleanly; the Subject continues running standalone; the fleet management surface goes dark |
| Code path | Diode policy lives inside the adapter, not the OS kernel — the policy can be updated without touching the rest of the system |

## Default state: no phone-home capability

A freshly booted `os-infrastructure` node does not have `service-pointsav-link` installed. This is not a configuration choice — it is an architectural invariant. The Subject OS contains no code that would allow it to initiate contact with any authority. There is no `ssh` client, no peer-to-peer routing table, and no RPC initiator. The Subject is structurally incapable of phoning home.

This property holds across the entire `os-*` family. Every Subject — `os-totebox`, `os-mediakit`, `os-privategit`, `os-infrastructure`, and `os-network-admin` when acting as a Subject — begins in a state where fleet management is absent. Bringing a node under management is always an explicit operator action, never an automatic one.

## Activation: hot-plugging the adapter

An operator activates `service-pointsav-link` on a Subject with a single command issued from `os-network-admin`. The activation sequence installs the `pointsav-protocol` package, registers the node's fiduciary keypair with the fleet's pairing registry as a Subject entry, and opens the adapter's single inbound command channel.

After activation, the Subject becomes addressable by the fleet's command broadcast. The adapter's inbound channel listens on the PPN mesh and accepts only packets whose operation code is within the permitted set defined by the fleet's [[diode-standard|Diode Standard]] policy. All other traffic is discarded at the adapter boundary.

## Failure mode: clean severance

If `service-pointsav-link` crashes or is deliberately uninstalled, the Subject does not fail. It continues running its workloads — serving content, executing computations, maintaining its local state — as if fleet management had never been present. The fleet management surface goes dark: `os-network-admin` loses visibility of the Subject and cannot issue commands to it. But the Subject's own services are unaffected.

This failure mode is by design. A Subject that depends on the adapter for normal operation would be permanently coupled to the control plane — any control-plane outage would cascade to the Subject's workloads. By decoupling Subject operation from adapter presence, `service-pointsav-link` ensures that a control-plane failure is an observability loss, not a service outage.

## Policy in the adapter, not the kernel

The [[diode-standard|Diode Standard]] policy — which command flows are permitted, what operations are allowed from which authorities, what telemetry is emitted — lives inside `service-pointsav-link`, not in the Subject OS kernel. This separation has a practical consequence: updating fleet policy requires updating the adapter, not rebuilding or rebooting the operating system.

A Subject OS release and a Diode policy update are independent deployments. An operator can tighten or loosen fleet policy by pushing a new `pointsav-protocol` package version to active Subjects without touching any kernel code.

## The Universal Standard

`service-pointsav-link` is not a feature of a specific operating system. The same `pointsav-protocol` package, with different policy bindings, is the adapter between any pair of PointSav OS instances that need to communicate. An `os-totebox` Subject connected to an `os-console` Authority uses the same adapter code path as an `os-infrastructure` Subject connected to `os-network-admin`.

This uniform standard is what makes a complex fleet auditable. Every connection looks the same at the protocol layer. An auditor examining any pair of connected nodes sees the same adapter shape — the same default-off behaviour, the same activation sequence, the same clean-severance guarantee.

## See also

- [[diode-standard]] — the authority hierarchy and traffic rules the adapter enforces
- [[os-network-admin]] — the Authority that activates and commands Subjects via the adapter
- [[infrastructure-os]] — an os-infrastructure Subject that uses this adapter to join a fleet
- [[ppn-command-protocol]] — the 16-byte binary wire format the adapter receives
- [[machine-based-auth]] — the fiduciary keypairs that authenticate the adapter's command channel

---

## Research trail

### Done

1. Read `architecture/diode-standard.md` — §"The adapter" (4-property table: default not installed, activated hot-plugged single command, failure mode clean severance, code path in adapter not kernel); §"The Universal Standard" (same package different policy bindings; uniform standard makes fleet auditable); §"Why this matters" (lateral movement prevention)
2. Read `architecture/machine-based-auth.md` — pairing types (ADMIN/INPUT/USER/INTERFACE); hardware-bound Noise Protocol keys; revocation by severing pairing entry; service-pointsav-link participates in the pairing registry on activation
3. Read `systems/os-network-admin.md` — Authority that activates Subjects via the adapter; pairing registry management; Diode enforcement rules propagation; fleet claims via Genesis Protocol
4. Read `systems/infrastructure-os.md` — os-infrastructure is a Subject; Genesis Protocol creates the ADMIN pairing that precedes activation; WireGuard mesh the adapter's inbound channel runs over

### Suggested (not done this session)

1. Read `service-pointsav-link/` source if present in the monorepo — would confirm the exact activation API, the permitted operation code set for os-infrastructure Subjects, and the current policy binding format
2. Read `architecture/deployment-patterns.md` if it exists — would contain examples of activation sequences across the six canonical fleet configurations

### Open questions

*(none)*
