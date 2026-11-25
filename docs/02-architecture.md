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

**A small local model is the default execution path; remote models are an
optional, permissioned extension** ([16-open-questions.md](16-open-questions.md),
#39, #45). The following rules are settled:

- The OS MUST work offline: basic system function and core adaptation MUST NOT
  depend on a remote model.
- Personal or sensitive system data MUST NOT be sent externally by default.
- Using a remote model is decided by permission and policy.
- The system MUST NOT stop if a remote model is unavailable; it degrades to the
  local path.
- Model-generated output MUST pass the validation layer before it has any effect
  ([09-security-model.md](09-security-model.md)).
- Model selection MUST consider memory, GPU and power use, not only task
  difficulty. A task that is easy but has no resources for the local model takes
  another path.

**A model MUST NOT hold OS authority.** It proposes a plan and AIR; the OS
validates and executes.

Concrete candidates and mechanics:

- The first local model candidate is a **small Qwen-family model**, with Llama
  compared against it (#39). The system MUST NOT depend on one vendor's model.
- The first local inference engine is **llama.cpp** (#40). ONNX Runtime is
  connected for ONNX models and hardware acceleration.
- Routing considers difficulty, memory, GPU, power and the privacy policy (#41).
- Structured output is **JSON Schema first** (#42): the model's intermediate
  result is validated, then converted to AIR. The model does not emit AIR
  directly.
- Remote providers sit behind an **abstraction**; API keys live in the OS secret
  store (#43, later).
- The model discovers Capabilities through **structured registry queries**
  (tool/function calling), not memory or a prompt list; RAG is an optional
  discovery aid, never an execution authority (#75). See
  [09-security-model.md](09-security-model.md).

## L1/L2 · Isolation

Isolation is owned by [07-execution-model.md](07-execution-model.md): the process
boundary is the default, and untrusted model code runs in the VM sandbox (#23).
Stronger sandboxes are added only if needed.

## L4 · Compiler and modules

The compiler emits native or bytecode modules under a project ABI
([06-adaptive-compiler.md](06-adaptive-compiler.md), #20, #22). The Workspace
runtime's storage is SQLite plus content-addressed storage
([03-workspace.md](03-workspace.md), #27).

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
- **UI Composition Layer**
- Slint Renderer
- Interaction Manager
- Notification System
- Task Interface

The user deals with one Workspace, not a set of programs. See
[03-workspace.md](03-workspace.md) for the Workspace model.

**The UI framework MUST NOT control the OS.** The desktop UI is Slint, but it
sits behind a framework-independent **UI Composition Layer**
([16-open-questions.md](16-open-questions.md), #50, #52):

- AIR declares the UI panels and their relationship to Capabilities.
- The UI Composition Layer decides which panels are shown, and when.
- The Slint renderer draws them.

Because the composition layer is separate, Slint can be replaced without changing
the OS's core structure. egui MAY be used for developer and debug tooling (system
monitoring, internal panels), but it is not the user-facing desktop.

Accessibility is considered from the initial design, not added later (#51):
keyboard navigation, a clear focus model, and screen-reader support are
requirements. The UI MUST support multiple languages; the initial languages are
English and Korean (#51). Keyboard and mouse come first, touch follows, and voice
is an optional feature (#53). The Slint license is checked against the actual
distribution model before it is assumed settled (#50).

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
- **Architecture independence.** x86_64 is the first target (#58). AIR, the
  module ABI and the bytecode are designed to be architecture-independent, so
  aarch64 can be added later without redesigning them.
- **Distribution.** `aos` is distributed through **crates.io and GitHub
  Releases**; Capabilities are independent packages on GitHub Releases, held in a
  **local** Capability Registry, described by a **language-independent Manifest**
  (#76). A self-hosted package manager, a remote registry, signing and system
  packages are later. Install integrity and execution permission are verified
  separately ([09-security-model.md](09-security-model.md)).
