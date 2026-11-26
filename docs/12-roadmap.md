# 12 · Roadmap

The project does not build a complete independent operating system first. It
implements and verifies the core technology in stages, then extends the system.
Each stage has a single question it must answer before the next begins.

## The first milestone

The first public milestone is **PoC success**, and it is defined precisely
([13-poc.md](13-poc.md), #1): a Capability is created, executed, replaced at
runtime with state migration, and the result is verified — not a demo screen that
merely appears. Publicising the project (GitHub-centred, with Hacker News if the
result is interesting) follows PoC success (#2, later).

## Versioning

The project starts at **0.1.0** and reaches **1.0.0** when the core architecture
and the public API stabilise (#10). Semantic Versioning applies from 0.1.0, but
early compatibility is explicitly limited: pre-1.0, a minor version MAY break
compatibility, and the changelog says so.

## Repository shape

The compiler, runtime and AIR are co-evolving, so the project stays a **monorepo**
until their boundaries stabilise (#5). Software understanding is the one
exception: it is a separate research project
([08-software-understanding.md](08-software-understanding.md), #46).

## Development environment

**Linux is the primary target; WSL2 is allowed for development**
([16-open-questions.md](16-open-questions.md), #83). Day-to-day Rust work, builds
and unit tests may run on WSL2, but isolation and process tests MUST run on real
Linux, which CI provides:

| Work | Environment |
|---|---|
| Rust development, build, unit tests | WSL2 or Linux |
| Linux integration tests | real Linux (CI) |
| isolation and process tests | real Linux |
| hardware and GPU tests | a dedicated environment |

A result on WSL2 does not establish that namespaces, cgroups or isolation behave
correctly; those claims need real Linux. Stage 0 stays a **user-space prototype on
Linux**; self-hosted kernel work and an independent distribution do not begin
until Stage 0 is verified.

## Stage completion criteria

A stage is complete when its criteria are met, not on a date:

| Stage | Complete when |
|---|---|
| 0 | the Capability run-and-swap PoC succeeds |
| 1 | execution isolation and external modules work |
| 2 | AIR and the resolver are extended |
| 3 | the compiler and contract verification are strengthened |
| 4 | AI-driven composition works |
| 5 | the UI and user experience work |
| 6 | the system is integrated and distributable |

## Stage 0 · System specification

**Goal:** fix the project's basic concepts and execution model.

Work:

- define the overall architecture
- design the software model
- design the Workspace data model
- design the Capability model and the Software Manifest
- design the initial AIR specification
- design the security and permission model
- design the change and recovery model

Output: the documents in `docs/`, the RFC process, and this repository's
specification set.

**Status: in progress.** This is the current stage.

## Stage 1 · Adaptive runtime

**Goal:** create and change a Workspace from a declarative description, **with no
AI**.

Work:

- AIR parser and validator
- Software Registry
- Capability Resolver
- Workspace runtime
- basic Workflow Engine
- Linux system adapter
- basic state persistence and recovery

The most important goal is to prove that the designed software model actually
works without AI. If it does not, adding AI will not save it.

## Stage 2 · Software adaptation

**Goal:** change functionality and structure while preserving an existing
Workspace.

Work:

- Software Adapter
- Capability and Workflow composition
- UI adaptation
- transactional adaptation
- state migration
- Workspace versioning
- change verification and rollback

## Stage 3 · Adaptive compiler

**Goal:** build the project's own compiler that analyses AIR efficiently and
lowers it to an executable structure.

Work:

- AST and Typed IR
- static type checking
- semantic analysis
- dependency analysis
- execution-plan generation
- optimisation
- incremental compilation
- execution-module generation

## Stage 4 · Intelligence integration

**Goal:** use AI to turn user intent into a Workspace composition.

Work:

- local small-model runtime
- intent understanding
- task planning
- AIR generation
- context management
- model routing
- structured-output validation

In this stage too, a model's output MUST pass validation
([09-security-model.md](09-security-model.md)).

## Stage 5 · Unified desktop

**Goal:** build the integrated interface a general user can actually use.

Work:

- Workspace shell
- adaptive desktop
- dynamic UI
- task-state display
- the interface for changing functionality
- save and restore
- user settings

## Stage 6 · Independent OS

**Goal:** grow the system into an independent operating-system environment.

Work:

- a **minimal bootable system first**, not a full desktop OS (#60)
- boot and system services
- install image
- packaging
- updates
- hardware compatibility
- system recovery
- long-term research into a self-hosted kernel and the feasibility of migrating
  to it

**Self-hosted kernel research begins only after Stages 0–4 have proven the core
hypothesis on the existing kernel** (#59). The point is not to abandon a
self-hosted kernel; it is to first demonstrate the reason one is needed — a
requirement the existing kernel cannot meet. Until then, the runtime is a
**user-space runtime on the existing OS**.

## How the stages are judged

A stage is complete only when its goal question is answered by a test, not by a
document. The measurements that judge a stage are defined in
[14-evaluation.md](14-evaluation.md). The roadmap does not claim a stage is done
because work in it has started.
