---
schema: foundry-draft-v1
artifact_type: TOPIC
language_protocol: PROSE-TOPIC
title: "Spot VM Lifecycle — Single Controller and Kill Switch Pattern"
slug: topic-sovereign-vm-lifecycle-kill-switch
language: en
status: draft
paired_with: TOPIC-sovereign-vm-lifecycle-kill-switch.es.draft.md
target_repo: content-wiki-documentation
target_path: topics/topic-sovereign-vm-lifecycle-kill-switch.md
gateway: project-editorial
bcsc_class: no-disclosure-implication
research_done_count: 2
research_suggested_count: 0
open_questions_count: 0
research_provenance: "BRIEF-slm-learning-loop.md §13 system diagram; Opus audit findings 2026-06-09 session; bin/yoyo-daily-cycle.sh; infrastructure/local-yoyo-daily.timer"
research_inline: true
created: 2026-06-09
author: totebox@project-intelligence (claude-sonnet-4-6)
---

# Spot VM Lifecycle — Single Controller and Kill Switch Pattern

When an automated pipeline depends on a preemptible or spot VM, the lifecycle of that VM
must be owned by a single controller. Two independent timers that each hold the authority
to start the VM will eventually fire at the same time, leaving the VM running between
cycles at full cost with no automated stop path. This document describes the single-controller
architecture used for the Yo-Yo batch node and the sentinel file kill switch that provides
immediate operator control.

## The two-timer problem

The Yo-Yo batch pipeline initially had two timers operating independently:

- `local-yoyo-daily.timer` — ran the daily enrichment cycle, which started and stopped the VM
- `local-corpus-threshold.timer` — checked the training corpus and started the VM if the threshold was exceeded

Both timers called `gcloud instances start`. Only the daily cycle timer called `gcloud instances stop`.
When `local-corpus-threshold.timer` fired, it could start the VM but had no path to stop it.
If the daily cycle timer did not fire shortly afterward, the VM would remain running indefinitely.

At the Yo-Yo node's cost of approximately $0.71 per hour, an uncapped start event from
the threshold timer would cost approximately $0.85 before the next daily cycle fired to
stop it — assuming the cycle fired at all. If the cycle was skipped due to a holiday or
a kill switch being active, the VM could run for 24 hours or more at a cost of
approximately $17.

## The single-controller fix

The fix is architectural: exactly one systemd unit owns the full VM lifecycle for each VM.
`local-corpus-threshold.timer` was masked (redirected to `/dev/null`), removing its
ability to start the VM. All VM lifecycle operations — start, enrich, check threshold,
optionally train, stop, verify — are now performed within a single invocation of
`yoyo-daily-cycle.sh` triggered by `local-yoyo-daily.timer`.

The corpus threshold check is now Phase 5 inside the daily cycle rather than a separate
timer. The training trigger is Phase 6. Both run while the VM is already running for
enrichment, adding no additional VM start cost.

The rule generalises: for any spot VM that performs multiple automated tasks, consolidate
all tasks into a single orchestrator script invoked by a single timer. Do not give
multiple timers start authority over the same VM.

## The sentinel file kill switch

A kill switch is a file whose presence or absence controls whether an automated process
runs. The pattern is:

```
presence of /path/to/flag-file  →  suppress the operation
absence of /path/to/flag-file   →  normal operation
```

For the Yo-Yo batch node, the kill switch file is `/srv/foundry/data/yoyo-disabled`.

The daily cycle script checks for this file as its first action (Phase 0), before issuing
any `gcloud` commands:

```bash
if [[ -e "$KILL_SWITCH" ]]; then
    log "KILL SWITCH ACTIVE — $KILL_SWITCH present; aborting all VM lifecycle"
    exit 0
fi
```

Creating the file is a one-command action that takes effect on the next timer firing:

```bash
touch /srv/foundry/data/yoyo-disabled
```

Removing the file resumes normal operation:

```bash
rm /srv/foundry/data/yoyo-disabled
```

The pattern is appropriate for any automated process where:
- The operator needs an instant brake that survives a reboot
- The suppression should be persistent across multiple timer firings until explicitly reversed
- No service restart or configuration change should be required to activate or deactivate control

An environment variable (`export SUPPRESS=true`) would not survive a reboot or a service
restart. A systemd unit mask requires root and a `daemon-reload`. The sentinel file
approach is reversible, auditable (its presence or absence is visible with `ls`), and
requires no elevated privileges to activate.

## Defense in depth: the idle monitor

The kill switch prevents starts. A separate safety layer stops a VM that is running when
it should not be. The idle monitor timer (`yoyo-idle-monitor.timer`) fires every five
minutes and checks whether the Yo-Yo batch VM has been running for more than 30 minutes
without an active inference request. If that condition is met, the monitor issues a stop
command.

The idle monitor is a backstop, not the primary controller. Its role is to bound the cost
exposure if the daily cycle fails to complete its stop sequence — for example, if the
workspace VM loses connectivity during Phase 8, or if the cycle is interrupted by a
process signal before the stop command is issued.

The combination of single-controller daily cycle, sentinel file kill switch, and idle
monitor provides three independent layers:

1. The daily cycle stops the VM as its final phase (intended path)
2. The idle monitor stops the VM if the cycle fails (first backstop)
3. The kill switch prevents the VM from starting if the operator needs to pause all
   activity (operator override at Phase 0)

## The corpus-threshold.py guard

`corpus-threshold.py` contains a `_start_trainer_vm()` function that was originally called
by the corpus threshold timer. After the timer was masked, this function was modified to
check the kill switch file before issuing any `gcloud instances start` command. This is a
defense-in-depth measure: if the function is ever called from a code path that bypasses
the daily cycle, the kill switch still takes effect.

The guard pattern:

```python
if os.path.exists(KILL_SWITCH_PATH):
    print(f"[kill switch] {KILL_SWITCH_PATH} present — VM start suppressed")
    return
```

Any script that has the authority to start a spot VM should implement this check.

## Applying the pattern

To apply single-controller + kill switch to any spot VM pipeline:

1. Identify all timers and scripts that call `gcloud instances start` for the VM.
2. Consolidate all work into a single orchestrator script. The script starts the VM,
   performs all tasks in sequence, and stops the VM as its final step.
3. Disable all other start paths (mask the timers; modify any scripts that had start
   authority to check the kill switch file instead).
4. Create the kill switch file path in a directory that survives reboots
   (e.g. `/srv/foundry/data/` or `/var/lib/`).
5. Add the kill switch check as the first statement in the orchestrator script.
6. Add an idle monitor as a cost backstop, targeting the specific VM name and zone.
