# 07 · Execution model

Xdaptive treats a program not as a fixed executable but as an execution unit
that can be composed and changed. The project does not rewrite a program from
scratch each time; it preserves as much of the running state and functionality as
possible and adds or replaces only what a change requires.

## Adaptation levels

Software supports a different range of change. The OS MUST know the range and
stay inside it.

### Level 1 · Interface adaptation

Change the user interface to fit the task:

- rearrange panels, remove unneeded elements, surface needed tools
- create controls suited to the task
- merge several functions into one interface

### Level 2 · Workflow adaptation

Connect existing functionality and change the execution flow:

- pass data between functions
- change execution order
- connect several engines
- automate task steps
- conditional execution and result handling

### Level 3 · Program adaptation

Change or extend the program's behaviour and functionality itself:

- generate new functionality
- change how an existing function behaves
- restructure internal modules
- reimplement a compatible function
- optimise execution structure and data handling

Not every program supports Level 3. A Capability's manifest MUST declare the
highest level it supports, and the OS MUST NOT attempt a change beyond it.

## Approaching different kinds of software

Different software needs different strategies:

| Software type | Approach |
|---|---|
| Software built for Xdaptive | direct recomposition via internal structure and extension API |
| Open-source software | plug-ins, API, source modification, module recomposition |
| External proprietary software | official API, IPC, accessibility interfaces, automation |
| Analysable binaries | reverse engineering and behaviour analysis |
| Software with limited support | wrap as an external program or independent service |

The point is that the OS does not treat every program the same way. It must
understand each piece's structure and use an appropriate method.

## Dynamic composition

The execution architecture treats a program as a set of modules that can be
composed and changed at runtime. Core research targets:

- dynamic module composition
- per-function start and stop
- structural change while running
- dependency management
- state migration
- inter-module communication
- incremental update
- execution isolation

The module ABI is fixed from the start; Stage 0 defines a minimal skeleton with a
version field ([16-open-questions.md](16-open-questions.md), #22). See
[06-adaptive-compiler.md](06-adaptive-compiler.md) for what the ABI MUST define.

## Preserving state across a change

The hardest requirement: when a running program's structure changes, the existing
task state MUST NOT be lost. State preservation is a **three-step policy**; the
first step that applies is used ([16-open-questions.md](16-open-questions.md),
#66).

1. **State migration.** Move the existing state directly into the new
   Capability.
2. **State re-derivation.** Reconstruct the state from the original inputs or
   from stored data.
3. **Reject the change.** If neither method can preserve the state safely, the
   change is refused before it is applied.

The policy deliberately does not force one state model on every Capability: a
file system and an AI inference engine hold state of a completely different
nature, and each MUST be free to declare the method that fits it.

Re-derivation alone is not sufficient (#29). State such as a consumed one-shot
external response or a live network session cannot be reconstructed by
re-running. Such state requires a **selective per-engine checkpoint**; if neither
a checkpoint nor re-derivation can preserve it, the change is rejected.

The **runtime owns the state lifecycle** ([16-open-questions.md](16-open-questions.md),
#74). A Capability exposes a versioned `get_state`/`set_state`; the runtime
decides when to save, where to keep, and when to restore, and does not need to
understand the state's internal meaning. External resources — GPU memory, a
database connection, a network socket — are held as **managed handles** with an
explicit recovery policy. See [04-capability-model.md](04-capability-model.md).

### Atomic transition

The policy depends on an **atomic transition**, the same idea as the transaction
in [10-adaptation-transactions.md](10-adaptation-transactions.md):

- The existing state is preserved until the new state is verified.
- If migration fails, the system MUST return to the existing state; the old
  Capability MUST NOT have been torn down first.
- A data change made during migration MUST NOT be applied twice.

Not every change can be seamless. Changes that touch an external system or an
irreversible operation need a separate policy, and a change that cannot be made
atomic MUST be rejected rather than attempted.

### The governing rule

> Change only when the change can be made safely. Never trade data integrity for
> performance or adaptability.

This is principle 6 ([15-principles.md](15-principles.md)) applied to state: the
OS is not one that can always change, but one that changes only when it can do so
safely.

## Isolation

Every execution module runs isolated. The target isolation is the **process
boundary**, with untrusted, model-generated code in the **VM sandbox**
([16-open-questions.md](16-open-questions.md), #23). The starting mechanism on
Linux is `namespaces`, `seccomp` and `cgroups`. Stronger sandboxes (gVisor,
Firecracker) are not early requirements; they are added if the process and VM
boundaries prove insufficient. See [09-security-model.md](09-security-model.md).

**Stage 0 is an exception, by design.** It runs **in-process, and only trusted,
self-written code** ([RFC 0002](rfc/0002-stage0-execution-contract.md)). The
runtime MUST talk to an **execution interface** so that isolation can be
introduced later without redesigning the runtime:

- Stage 0 — in-process executor (trusted code only)
- Stage 1 — process executor
- later — hardened VM sandbox

Untrusted or model-generated code MUST NOT run through the trusted in-process
path, and Stage 0 MUST NOT claim that process isolation or a VM sandbox exists.

## Concurrency

Concurrency is **async/await with channel-based communication**
([16-open-questions.md](16-open-questions.md), #24). A full actor model is not
introduced up front; channels carry data between Capabilities and modules, and
the module ABI ([06-adaptive-compiler.md](06-adaptive-compiler.md)) defines the
asynchronous rules across a module boundary.

## Related documents

- [04-capability-model.md](04-capability-model.md) — the units that are composed.
- [06-adaptive-compiler.md](06-adaptive-compiler.md) — what emits the modules.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — how a change
  is applied atomically.
