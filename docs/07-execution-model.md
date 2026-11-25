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

`Open question:` the binary and ABI contract execution modules follow
([16-open-questions.md](16-open-questions.md), item 6).

## Preserving state across a change

The hardest requirement: when a running program's structure changes, the existing
task state MUST NOT be lost.

`Open question:` which state can migrate and which must be re-derived
([16-open-questions.md](16-open-questions.md), item 7). The working direction:

- State a Capability declares as **migratable** is moved across the change.
- State that cannot migrate MUST be re-derivable from recorded inputs, or the
  change MUST be rejected before it is applied.
- A change that cannot preserve required state MUST fail the
  [transaction](10-adaptation-transactions.md), not proceed and lose data.

## Isolation

Every execution module runs isolated. The starting mechanism is Linux
`namespaces`, `seccomp` and `cgroups`. See
[09-security-model.md](09-security-model.md).

## Related documents

- [04-capability-model.md](04-capability-model.md) — the units that are composed.
- [06-adaptive-compiler.md](06-adaptive-compiler.md) — what emits the modules.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — how a change
  is applied atomically.
