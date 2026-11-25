# 00 · Glossary

The vocabulary the rest of the specification depends on. A term is defined here
once; other documents link to it instead of redefining it.

## Intent and task

- **Intent** — what the user is trying to accomplish, expressed in natural
  language or another input modality. Intent is the input to the system, not a
  program.
- **Task** — the system's interpretation of intent: a goal with a set of
  requirements that software must satisfy. The task, not the program, is the
  unit the OS is organised around.
- **Workspace** — the persistent, integrated execution environment for one task.
  See [03-workspace.md](03-workspace.md).

## Software model

- **Capability** — a single, executable unit of functionality with declared
  inputs, outputs, dependencies and permissions. Capability is the atom of
  composition, and its identity is **implementation-language independent**. See
  [04-capability-model.md](04-capability-model.md).
- **Module** — an implementation of a Capability in some language. A Module
  conforms to the [Module ABI](07-execution-model.md) and may be written in any
  language; no language is mandatory.
- **Component** — a deployment and isolation unit that carries one or more
  Modules.
- **Permission** — authorization to access a resource or perform a restricted
  operation. A Capability **grants no Permission** by existing; permissions are
  declared, granted and enforced by the runtime
  ([09-security-model.md](09-security-model.md)).
- **Software Manifest** — the description of what a piece of software provides
  to the OS: its Capabilities, interfaces, dependencies, permissions and the
  range of adaptation it supports.
- **Software Registry** — the store of Software Manifests the OS can resolve
  against.
- **Capability Resolver** — the component that finds the Capabilities a task
  needs, checks their compatibility, resolves dependencies and orders their
  execution.
- **Adaptive Software** — software the OS can reconfigure for a task. It has
  three adaptation levels; see [07-execution-model.md](07-execution-model.md).

## Representation and compilation

- **AIR (Adaptive Intermediate Representation)** — the structured form in which
  software composition and change are expressed, verified and executed. See
  [05-adaptive-ir.md](05-adaptive-ir.md).
- **Workspace Plan** — a description of a Workspace's composition and the change
  applied to it, written in AIR.
- **Capability Binding** — the association of a Capability to a Workspace,
  including whether it is enabled and how it is configured.
- **Adaptive Compiler** — the component that parses, type-checks and lowers AIR
  into an executable structure. It, not a model, is accountable for what runs.
  See [06-adaptive-compiler.md](06-adaptive-compiler.md).

## Adaptation

- **Adaptation Level** — one of three ranges of change a piece of software
  supports: interface (L1), workflow (L2) or program (L3).
- **Adaptive Composition Engine** — the component that analyses software and
  composes a Workspace from verified Capabilities.
- **Adaptation Planner** — the part of the composition engine that computes the
  change a request requires.
- **Adaptation Plan** — the proposed change. A model MAY propose one; the OS
  MUST validate and apply it.
- **Transactional Adaptation** — applying a change atomically so a failure
  leaves the Workspace intact. See [10-adaptation-transactions.md](10-adaptation-transactions.md).
- **State Migration** — moving existing task state across a change so the user's
  work survives it.

## Understanding existing software

- **Software Analyzer** — the component that analyses a program's structure and
  behaviour (binary, control flow, data structures, UI, IPC).
- **Semantic Reconstruction** — turning analysis output into a model the OS can
  reason about: purpose, inputs, outputs, dependencies, conditions.
- **Adaptation Generator** — turning a reconstructed model into something usable:
  a connection, a compatibility implementation, or an independent implementation.

## Runtime and system

- **Persistent Workspace Engine** — the runtime that owns a Workspace's
  lifecycle, state, versions, checkpoints, recovery and cloning.
- **Checkpoint** — a recorded execution state a Workspace can be restored to.
- **Snapshot** — a recorded composition and data state, distinct from a running
  process's memory.
- **Intelligent Resource Runtime** — the resource manager that allocates CPU,
  memory, GPU and processes. A model MAY describe expected demand; the manager
  MUST make the allocation decision.
- **UI Synthesizer** — the component that builds and changes a Workspace's
  interface from its current Capabilities and task state.

## Terminology and security

Two words are easy to confuse and MUST NOT be used interchangeably.

| Term | Meaning in Xdaptive |
|---|---|
| **Capability** | a logical unit of executable functionality (what something can do) |
| **Permission** | authorization to access a resource or perform a restricted operation (what it is allowed to touch) |

The operating-system security field uses "capability" for an authorization token
(seL4, capability-based security). Xdaptive keeps **Capability** for the
functional unit and reserves **Permission** for authorization, so a reader is
never left guessing which one a sentence means. The rules:

1. Capability and Permission MUST NOT be used as synonyms.
2. Declaring a Capability MUST NOT grant any Permission.
3. The runtime checks Permissions and controls actual access.
4. Every Capability follows least privilege.

## The central invariant

- **AI/OS split** — the rule that a model's output MUST NOT reach the kernel,
  the filesystem, the network or user data without passing validation and the
  transaction boundary. A model **proposes**; the OS **disposes**. See
  [09-security-model.md](09-security-model.md).
