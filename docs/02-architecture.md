# 02 · Architecture

Xdaptive is organised into six layers. The two lowest are borrowed from Linux;
the upper four are where the project's own technology lives. A layer MAY call
downward; it MUST NOT reach around a layer to a lower one.

## L1 · Hardware and kernel

The Linux kernel provides CPU and memory management, process management, the
filesystem, networking, device and GPU drivers, and syscalls.

The project MUST NOT depend on the kernel's internal interfaces above the
syscall boundary. The upper layers talk to L1 through a **system adapter**, so
that a different kernel can be substituted later without changing them.
Self-hosting a kernel is long-term research, not a Stage 0 goal.

## L2 · System and security runtime

The base execution and security environment.

- Process Manager, Service Manager
- IPC
- File Access Manager, Network Access Manager
- Resource Manager
- Isolation Runtime
- Capability Security System

Processes and services run independently; a Workspace composes and coordinates
them. The starting technology is Linux `namespaces`, `seccomp`, `cgroups` and
Unix domain sockets. See [09-security-model.md](09-security-model.md).

## L3 · Intelligence runtime

The layer that understands intent and produces a plan.

- Intent Understanding
- Task Planner
- Model Router
- Context Manager
- Structured Output Generator
- Output Validator
- Local Model Runtime

A small local model SHOULD be the default; larger local or remote models MAY be
used for harder tasks. **A model MUST NOT hold OS authority.** It proposes a
plan and AIR; the OS validates and executes. See
[09-security-model.md](09-security-model.md).

## L4 · Adaptive Composition Engine

The project's core technology.

- Software Registry
- Software Analyzer, Semantic Reconstruction
- Capability Resolver
- Adaptation Planner
- AIR and the Adaptive Compiler
- Workflow Composer, UI Synthesizer
- Patch Manager, Compatibility Checker
- Transactional Adaptation

It analyses software, composes a Workspace, and generates or changes
Capabilities. See [04-capability-model.md](04-capability-model.md),
[05-adaptive-ir.md](05-adaptive-ir.md) and
[10-adaptation-transactions.md](10-adaptation-transactions.md).

## L5 · Workspace runtime

The execution and lifecycle of the task environment.

- Workspace Manager
- State Manager
- Version Manager
- Checkpoint Manager
- Execution Manager
- Module Manager
- Recovery Manager

See [03-workspace.md](03-workspace.md).

## L6 · Unified experience

Where the user interacts.

- Adaptive Desktop
- Workspace Shell
- Dynamic UI
- Interaction Manager
- Notification System
- Task Interface

The user deals with one Workspace, not a set of programs. See
[03-workspace.md](03-workspace.md) for the Workspace model.

## Internal data model

The internal model is intentionally minimal. It is a conceptual sketch; concrete
types and APIs are decided during implementation, and this section is updated
when they are.

### Workspace

```rust
struct Workspace {
    id: WorkspaceId,
    goal: TaskGoal,
    version: WorkspaceVersion,
    software: SoftwareConfiguration,
    workflow: WorkflowState,
    interface: InterfaceState,
    data: DataState,
    history: VersionHistory,
}
```

### Workspace Plan

```rust
struct WorkspacePlan {
    base: WorkspaceId,
    capabilities: Vec<CapabilityBinding>,
    workflow: WorkflowGraph,
    interface: InterfaceSpec,
    permissions: CapabilitySet,
    state_migrations: Vec<StateMigration>,
}
```

### Capability Binding

```rust
struct CapabilityBinding {
    capability: CapabilityId,
    enabled: bool,
    config: Configuration,
}
```

The full schemas are owned by [03-workspace.md](03-workspace.md) and
[04-capability-model.md](04-capability-model.md).

## Cross-cutting concerns

- **IPC.** The protocol between layers is an open question
  ([16-open-questions.md](16-open-questions.md), item 13). Candidates: Unix
  domain sockets with a project protocol, or gRPC. The starting point is Unix
  domain sockets.
- **Isolation.** Every generated or composed execution module runs isolated. See
  [09-security-model.md](09-security-model.md).
- **Resources.** Allocation is made by the Intelligent Resource Runtime, never
  by a model. See [11-performance.md](11-performance.md).
- **Persistence.** Workspace state is versioned and recoverable. See
  [03-workspace.md](03-workspace.md).
