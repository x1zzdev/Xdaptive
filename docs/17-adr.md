# 17 · Architecture Decision Record (ADR)

This document consolidates the project's settled decisions with their rationale,
so that the reasons behind the design are not lost as implementation proceeds. It
is the narrative record; [16-open-questions.md](16-open-questions.md) is the
status ledger, and [rfc/](rfc/) holds contested proposals.

## How the three records relate

| Record | Purpose |
|---|---|
| **16 · ledger** | one line per question, with a permanent number and a tier |
| **17 · ADR** (this) | the decision, its context, and its consequences |
| **rfc/** | a proposal for a contested decision, and the reasoning against it |

A number in the form `#N` refers to the ledger.

## The three axes

Everything else is built on three boundaries:

- **AIR** declares execution structure and relationships.
- **Adaptive Compiler** turns declarations into executable modules.
- **Runtime** owns execution, permission, state and replacement.

The ABI and the VM define the boundary between the compiler and the runtime; the
type system and the async policy complete the execution semantics.

## Three principles added in this round

1. **Semantics before optimization.** Zero-copy, GPU shared memory, JIT and native
   promotion are all attractive, but data ownership and result meaning MUST be
   settled before optimizing them.
2. **The runtime may refuse to run.** When state cannot migrate, a permission is
   missing, or a module is incompatible, refusing is the correct behaviour, not
   forcing execution.
3. **Stage 0 success is not a feature count.** It is the minimal path — declare,
   connect, run, replace, verify state, recover on failure — working correctly.

## Decisions by area

### Identity, scope and release

- Xdaptive / Xdaptive OS / Adaptive Computing OS; open source; Apache-2.0 (#1-#10).
- First milestone is **PoC success**, defined as a working run-and-swap, not a demo
  screen (#1). Publicising follows it (#2).
- **Monorepo** while AIR, compiler and runtime co-evolve (#5); software
  understanding is the one separate research project (#46-#49).
- Versioning: 0.1.0 toward 1.0.0, limited early compatibility (#10).
- Linux is the substrate; a self-hosted kernel waits until Stages 0–4 prove the
  need (#59). Stage 6 starts from a minimal bootable system (#60).

### AIR — representation

- A **declarative DSL**, not a language for executable logic (#13).
- Its **own type system** (#14) with **nominal typing** by default (#15).
- **Connections and bounded conditional gates (`if`/`when`); no general
  iteration** (#72). Conditions are typed and structural; repetition belongs to a
  Capability or a streaming engine.
- Data passes as **value, reference or streaming**; copying GB-scale data is
  prohibited (#16).
- **Nullable (`Option<T>`) and missing (field presence) are distinct**; explicit
  **namespaces** (#80).
- **Explicit schema version** and migration (#17).
- **Contracts are representation only at Stage 0**; enforcement is Stage 3 (#18).
- **Permissions and resources are declared** in AIR; the runtime grants and
  enforces (#19).
- Stage 0 AIR expresses **only what the PoC needs** (#65).
- Dimensional and unit types are **deferred to a future RFC** (#73).

### Capability and Manifest

- **Capability is the composition atom**; Module is implementation, Component is
  deployment (#64).
- Capability identity is **implementation-language independent** (#70); a Module
  may be any language conforming to the ABI. Xz integrates through an adapter.
- **Capability is not Permission**; a Capability grants no permission (#71).
- The **runtime owns the state lifecycle**; a Capability exposes versioned
  `get_state`/`set_state`; external resources are managed handles (#74).
- Compatibility is **type plus semantic contract plus pre/postconditions** (#18).

### Compiler, ABI and VM

- **Hybrid execution**: native and bytecode, chosen by analysis, verification and
  permissions, **not by origin** (#20, #21).
- **ABI value passing is a normalized ABI plus large-data handles** (#78); a
  handle is a runtime resource id with a defined owner and lifetime.
- **Minimal ABI skeleton with a version field**, verified before a module runs
  (#22).
- **Stack-machine VM with mandatory static verification** (#79); invalid bytecode
  is rejected before execution, and resource limits are separate.
- **Incremental compilation** per Capability plus dependents, with a local
  content-addressed cache (#25, #26).
- LLVM is the first native backend; Cranelift is compared later (#21).

### Runtime, state and swap

- **State preservation is a three-step policy**: migrate, re-derive, or reject
  (#66); old state is preserved until the new state is verified.
- **Change is an atomic transition** (#67); state integrity and numerical
  equivalence are two independent layers (RFC 0002).
- **Runtime owns the state lifecycle** (#74).
- **A swap drains gracefully first and cancels only when safe** (#81); the
  runtime enforces a hard bound and rejects rather than force.
- Storage is **SQLite plus content-addressed storage**; snapshots are COW and
  incremental; checkpoints are selective per engine (#27, #28, #29).
- Clone shares immutable data with COW (#30).
- **Stage 0 runs in-process, trusted code only, behind an execution interface**
  (RFC 0002); process isolation is Stage 1, VM sandbox later (#23).

### Security

- **AI proposes, the OS disposes** — hard invariants, no origin-based trust
  (#20, #36).
- **Fine-grained permissions** with least privilege, approval before execution
  (#33, #34).
- Integrity by **hash plus signature**; install integrity and execution
  permission are separate (#35, #76).
- Secrets are passed **by reference**, never stored in a Capability (#37).
- An **audit log** reconstructs a run; prompt and personal data are not logged
  (#38, #44).
- **The model discovers Capabilities through structured registry queries**;
  discovery, selection, validation, authorization and execution are separate
  (#75); metadata is never an instruction.

### AI

- **Local small models are the default; remote models are an optional, permissioned
  extension** (#39, #45). The OS works offline; sensitive data is not sent
  externally by default; model selection weighs memory, GPU and power.
- First candidates: Qwen-family model, llama.cpp (#39, #40).
- Structured output is **JSON Schema first**, validated then converted to AIR
  (#42).

### Performance and resources

- **AI is not in the execution path of ordinary work** (#45).
- A **policy layer over the existing kernel**, not a kernel scheduler (#54).
- **GPU backend abstraction** (Vulkan general, CUDA/ROCm optional) (#55).
- Per-work-type benchmarks; power measured (#56, #57).
- **Determinism is a three-level model** (#82), declared and verified.

### Process, environment and observability

- **English spec + Korean master**; DCO; RFCs approved by maintainers with extra
  review for security, ABI and compatibility (#8, #3, #9).
- **Linux primary; WSL2 allowed for development**; isolation tests on real Linux
  (#83).
- **Structured event tracing** with a trace id per swap; **layered testing**
  (#84).

## Consequences

- The design is deliberately conservative at the boundary and generous in the
  interior: the ABI, VM verifier and security invariants are strict; the module
  implementation language is free.
- Two independent verification layers, a versioned ABI and a verified VM cost
  more to build than a single check, a frozen ABI and an unverified VM. They are
  chosen because a single check is unsound, a frozen ABI is usually wrong, and an
  unverified VM moves the risk to runtime.

## Open problems (to be settled by test)

- ABI handle lifetime and ownership rules.
- VM verification rules (the verifier's own correctness).
- Safe cancellation of in-flight asynchronous work.
- The concrete storage and transaction model.
- Per-operation tolerances, per-representation integrity, error accumulation.
- The split between static and runtime contract checking in Stage 3.
- The form in which analysis output becomes a Software Manifest.
