# 04 · Capability model

For the OS to understand and combine software, each piece must describe what it
provides in a standard form. This document defines two things: the **Capability**
(the unit of functionality) and the **Software Manifest** (how a piece of
software declares its Capabilities and the conditions of their use).

## Capability

A **Capability** is a single executable unit of functionality. It is the atom of
composition: the Workspace is built from Capabilities, the resolver finds
Capabilities, and the compiler lowers Capabilities into execution.

Example — a data-analysis program might provide:

- CSV Reader
- Data Filter
- Statistical Analysis
- Correlation Analysis
- Chart Renderer
- Export

A Capability declares:

- a unique ID
- a description of what it does
- input and output types
- dependencies on other Capabilities
- the conditions under which it may run
- the permissions it needs
- its resource requirements
- UI extension points
- its API or IPC interface
- compatibility information

`Open question:` whether Capability is the final minimum unit, or whether a
smaller or larger concept (Module, Component) is the atom
([16-open-questions.md](16-open-questions.md), item 1). The working choice is
Capability, with **Module** as an implementation grouping and **Component** as a
deployment grouping. This is not yet settled by a test.

### Type rules

`Open question:` the common type and memory model that carries data between
Capabilities ([16-open-questions.md](16-open-questions.md), item 4). The working
direction:

- Every Capability input and output MUST have a declared type.
- Two Capabilities MAY be connected only if the producer's output type is
  assignable to the consumer's input type, under a relation defined once and
  used everywhere.
- The relation MUST be checkable statically, before execution, wherever
  possible.
- Where a Capability wraps external software whose types are not known
  statically, the type MUST be recorded in its manifest as an external type with
  a declared encoding.

## Software Manifest

A **Software Manifest** describes what a piece of software provides and the
conditions of use. The OS resolves against manifests.

A manifest contains, conceptually:

- Software ID
- name and version
- provided Capabilities
- input and output types
- dependencies
- permissions
- execution environment
- resource requirements
- extension points
- UI information
- compatibility and the range of change it supports (its adaptation levels; see
  [07-execution-model.md](07-execution-model.md))

`Open question:` the final schema ([16-open-questions.md](16-open-questions.md),
items 9 and 11).

## Software Registry

The registry stores manifests and makes them queryable. A Capability is resolved
against the registry, never against a hard-coded list.

The registry MUST record, for each manifest, its origin and an integrity
reference, so a manifest cannot be silently substituted. `Open question:` the
integrity mechanism (hash pinning, signing) is not yet chosen.

## Capability Resolver

The resolver turns a task's requirements into a set of connected Capabilities:

- find the Capabilities the task needs
- check compatibility between them
- resolve dependencies
- decide execution order
- find substitutable Capabilities
- check permissions and resources

The resolver MUST fail closed: if a required Capability cannot be found, or two
cannot be proven compatible, the resolution fails with a diagnostic rather than
proceeding on an assumption.

## How a Capability is produced

A Capability can come from three sources, in increasing order of adaptation
level:

1. **Declared** — the software already provides it and its manifest describes it.
2. **Composed** — existing Capabilities are wired into a new workflow (L2).
3. **Generated or reconstructed** — a new implementation is generated, or an
   existing program is analysed and a compatible implementation is produced
   (L3). See [08-software-understanding.md](08-software-understanding.md).

A Capability produced by a model MUST pass verification before it may run against
user data. See [09-security-model.md](09-security-model.md).

## Related documents

- [03-workspace.md](03-workspace.md) — how Capabilities bind into a Workspace.
- [05-adaptive-ir.md](05-adaptive-ir.md) — how bindings and connections are
  expressed.
- [06-adaptive-compiler.md](06-adaptive-compiler.md) — how a resolved
  composition becomes executable.
