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

**Capability is the atom of composition.** A **Module** is an implementation
grouping (how Capabilities are packaged in code), and a **Component** is a
deployment grouping (how they are isolated and shipped). Both are sub-concepts;
neither is the unit the resolver, AIR or the Workspace reason about. This is
settled ([16-open-questions.md](16-open-questions.md), #64).

### Type rules

The type system is AIR's ([05-adaptive-ir.md](05-adaptive-ir.md)). The rules that
govern a Capability's types are settled (#14, #15, #16, #18):

- Every Capability input and output MUST have a declared type.
- **Nominal typing is the default.** A connection is decided by name and explicit
  declaration, not by shape. Two Capabilities may both accept a number while
  meaning different things; equal structure is not equal meaning. Structural
  compatibility is an explicit opt-in, not the default.
- A connection is valid only when the **type, the semantic contract, the
  preconditions and the postconditions** all agree. Type agreement alone is not
  enough.
- Data passes as **value, reference or streaming**; copying GB-scale data is
  prohibited.
- Where a Capability wraps external software whose types are not known
  statically, the type MUST be recorded in its manifest as an external type with
  a declared encoding.
- The relation MUST be checkable statically, before execution, wherever
  possible.

At Stage 0 the contract part of this rule is a **declaration only**: the
structural representation of preconditions and postconditions is defined, but
enforcement is deferred to Stage 3
([RFC 0002](rfc/0002-stage0-execution-contract.md)). Type and connection
checking still apply.

## Permissions

A Capability **declares** the permissions it needs; the runtime **grants and
enforces** them ([09-security-model.md](09-security-model.md), #19, #33). The
declaration is part of the manifest, and a Capability that uses a permission it
did not declare MUST fail validation. Permissions are fine-grained — file read,
write, create and delete, directory scope, and network destination and
direction — not a single "file access" or "network access" flag.

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
