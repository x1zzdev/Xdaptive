# 11 · Performance

Xdaptive is not an operating system that exists to showcase AI. It MUST reach a
usable level of performance and stability. This document defines how the fast
path stays fast while the safety model
([09-security-model.md](09-security-model.md)) stays intact.

The rule is simple: **an AI call MUST NOT be in the execution path of ordinary
work.** AI is invoked when understanding or structural change is needed, not on
every operation.

## AI and the normal path are separate

- Ordinary work runs without an AI call.
- AI is invoked only when the task must be understood or its structure changed.
- A generated execution plan, once verified, is reused.
- Repeated work is not re-planned each time.

## Incremental change

The whole Workspace is never rebuilt for a change. Only what changed is analysed
and recomposed:

- incremental compilation ([06-adaptive-compiler.md](06-adaptive-compiler.md))
- dependency caching
- change-scope analysis
- partial interface recomposition
- reuse of unchanged execution modules

## Data handling

- minimise unnecessary data copies
- prefer immutable objects and references
- use change logs rather than full copies
- manage checkpoints
- track memory use
- parallelise where the task allows

## Native execution

A compiled execution plan SHOULD run in a native environment where possible. The
project MUST NOT depend on an interpreter or an AI agent for every operation.

`Open question:` the compile and runtime structure that keeps native performance
while supporting dynamic composition
([16-open-questions.md](16-open-questions.md), item 8).

## Resource management

The Intelligent Resource Runtime manages CPU, memory, GPU and processes:

- CPU and memory management
- GPU work distribution
- process priority
- analysis of a task's resource demand
- per-Workspace resource limits
- conflict management between tasks
- runtime performance monitoring
- dynamic allocation

**A model MUST NOT control system resources directly.** A model MAY describe a
task's characteristics or expected demand; the allocation decision belongs to
the verified resource manager. This is an instance of the AI/OS split
([09-security-model.md](09-security-model.md)).

The project builds a **policy layer over the existing OS's resource management**
(#54); it does not write a kernel scheduler. Adaptive scheduling policy is
researched above Linux's facilities (`cgroups`, scheduler policy), and a
self-hosted scheduler is considered only if the existing kernel cannot meet a
requirement that Stages 0–4 have actually demonstrated.

## GPU

GPU access is a **backend abstraction** (#55). Vulkan is the general candidate;
CUDA and ROCm are optional backends behind the same interface. No single vendor
API is the internal standard.

## Power

Power management is a design goal (#57), but the first step is **measurement and
observability**: resource and energy use are recorded before any policy tries to
optimise them.

## Performance is measured, not asserted

Performance MUST be demonstrated by measurement and comparison, never by
assertion. [14-evaluation.md](14-evaluation.md) defines what is measured: latency,
CPU, memory, GPU, process cost, and IPC overhead, compared against a baseline.
