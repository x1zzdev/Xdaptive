# 16 · Decision ledger

This document tracks every deferred decision and its status. A decision leaves
the open list only when it is written into the owning document as a normative
statement (or accepted as an RFC) — never because someone has an opinion.

Question numbers are **permanent identifiers**. They are not reused, so a
reference like "#22" stays stable across edits.

## Tiers

- **Tier 1 · Settled for Stage 0.** The architecture depends on it; it is
  recorded in the owning document now.
- **Tier 2 · Validate by the PoC (Stages 0–2).** The direction is chosen; the
  result must be confirmed by building it.
- **Tier 3 · Later (Stages 3–6).** Decide when the earlier stages prove the core
  hypothesis.
- **Open.** No direction yet.

## Governing note

The core research hypothesis gates everything below it:

> Can the execution structure of existing software be expressed declaratively,
> compiled to different execution modules, and recomposed at runtime while
> preserving state and integrity?

Decisions are prioritised by how much they serve that hypothesis
([13-poc.md](13-poc.md)). The biggest risk is not failing to implement a
technology — it is implementing too many at once and never testing the core
hypothesis.

---

## Tier 1 · Settled for Stage 0

| # | Decision | Owner |
|---|---|---|
| 1 | The first public milestone is **PoC success**: capability create → execute → state migration → result verification, not a demo screen. | [12-roadmap.md](12-roadmap.md), [13-poc.md](13-poc.md) |
| 3 | **DCO** for contributions (CLA only if a legal need appears). | [CONTRIBUTING.md](../CONTRIBUTING.md) |
| 4 | The trademark policy is a **draft**; legal review before registration or commercial use. | [TRADEMARK.md](../TRADEMARK.md) |
| 5 | **Monorepo** while compiler, runtime and AIR are still co-evolving. | [02-architecture.md](02-architecture.md) |
| 8 | **English spec + Korean master vision**; other translations when outside contributors appear. | [README.md](README.md) |
| 9 | RFC approvers are the **maintainers**; security, ABI and compatibility RFCs need **extra review**. | [rfc/0001-rfc-process.md](rfc/0001-rfc-process.md) |
| 10 | Start at **0.1.0**; reach **1.0.0** when the core architecture and public API stabilise. SemVer applies, with limited early compatibility. | [README.md](README.md) |
| 12 | Keep **topic-separated** docs; merge duplicates. | [README.md](README.md) |
| 13 | AIR is a **declarative DSL** that expresses structure and relationships, not executable code. | [05-adaptive-ir.md](05-adaptive-ir.md) |
| 14 | AIR has its **own type system** with numeric, string, table and tensor base types. | [04-capability-model.md](04-capability-model.md), [05-adaptive-ir.md](05-adaptive-ir.md) |
| 15 | **Nominal typing** is the default; structural compatibility is an explicit opt-in. | [04-capability-model.md](04-capability-model.md) |
| 16 | Data passing supports **value, reference and streaming**; copying GB-scale data is prohibited. | [04-capability-model.md](04-capability-model.md) |
| 17 | AIR documents carry an **explicit schema version** and a migration path. | [05-adaptive-ir.md](05-adaptive-ir.md) |
| 19 | AIR **declares** permissions and resource needs; the runtime **grants and enforces** them. | [05-adaptive-ir.md](05-adaptive-ir.md), [09-security-model.md](09-security-model.md) |
| 20 | A **minimal project bytecode VM**, starting with a verifier and a limited instruction set. | [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 22 | A **minimal project Module ABI** (value passing, error handling, one async rule) with an explicit **version field** and a compatibility check before a module runs; advanced ownership, streaming and hardware conventions are deferred. Stage 0 validates one **bytecode** module call. | [06-adaptive-compiler.md](06-adaptive-compiler.md), [07-execution-model.md](07-execution-model.md), [RFC 0002](rfc/0002-stage0-execution-contract.md) |
| 23 | Target isolation is the **process boundary**; untrusted model code runs in the **VM sandbox**. **Stage 0 is the exception: in-process, trusted code only, behind an execution interface.** gVisor/Firecracker are not early requirements. | [07-execution-model.md](07-execution-model.md), [RFC 0002](rfc/0002-stage0-execution-contract.md) |
| 26 | Module cache is **local, content-addressed**; a remote registry is later. | [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 27 | **SQLite + content-addressed storage**, separating metadata from large content. | [03-workspace.md](03-workspace.md) |
| 33 | Permissions are per-Capability and least-privilege, with **fine-grained file and network scopes**. | [09-security-model.md](09-security-model.md) |
| 36 | Verification is **static analysis + tests + sandbox execution**; formal verification is staged in for core security modules. | [09-security-model.md](09-security-model.md) |
| 37 | Secrets are **not stored in a Capability**; the runtime passes a reference and an access grant. | [09-security-model.md](09-security-model.md) |
| 38 | An **audit log** records who ran which Capability, when, and with which permissions, with traceability enough to reconstruct a run. | [09-security-model.md](09-security-model.md) |
| 44 | Prompt and personal data are **not logged**; only minimal metadata (usage, time, selected model). | [09-security-model.md](09-security-model.md) |
| 45 | The **AI-free path is preferred**; metrics measure it rather than forcing it. | [11-performance.md](11-performance.md), [14-evaluation.md](14-evaluation.md) |
| 64 | **Capability is the minimum unit of composition**; Module is an implementation grouping, Component a deployment grouping. (Appendix B-1) | [04-capability-model.md](04-capability-model.md) |
| 65 | **The Stage 0 AIR expresses only what the PoC needs**, and grows by RFC. (Appendix B-2) | [05-adaptive-ir.md](05-adaptive-ir.md) |
| 66 | **State preservation is a three-step policy** — migrate, re-derive, or reject — and the old state is preserved until the new state is verified. (Appendix B-4) | [07-execution-model.md](07-execution-model.md) |
| 67 | **Change is an atomic transition** (snapshot through rollback, atomic across the parts it touches). (Appendix B-9) | [10-adaptation-transactions.md](10-adaptation-transactions.md) |
| 68 | The **compile and runtime structure** that keeps native performance while supporting dynamic composition, including recompiling verified bytecode to native. (Appendix B-10) | [11-performance.md](11-performance.md) |
| 69 | The **form in which analysis output becomes a Software Manifest**. (Appendix B-7) | [08-software-understanding.md](08-software-understanding.md) |
| 70 | **Capability identity is implementation-language independent**; Modules in any language conform to the Module ABI. Xz integrates through an independent adapter, never a mandatory dependency. | [04-capability-model.md](04-capability-model.md), [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 71 | **Capability and Permission are distinct.** A Capability grants no Permission; permissions are declared, granted and enforced by the runtime. | [00-glossary.md](00-glossary.md), [09-security-model.md](09-security-model.md) |
| 72 | **AIR has declarative connections and bounded conditional gates (`if`/`when`), no general iteration.** Conditions are typed and structural, not arbitrary code; repetition is delegated to a Capability or a streaming engine. | [05-adaptive-ir.md](05-adaptive-ir.md) |
| 73 | **Dimensional and unit types are deferred to a future RFC.** Stage 0 uses primitive and structural types and keeps the type system extensible. | [05-adaptive-ir.md](05-adaptive-ir.md) |
| 74 | **The runtime owns the Capability state lifecycle**; a Capability exposes a versioned `get_state`/`set_state`. External resources are managed handles with explicit recovery policies. | [03-workspace.md](03-workspace.md), [07-execution-model.md](07-execution-model.md) |
| 75 | **The model discovers Capabilities through structured registry queries (tool calling).** RAG is an optional discovery aid, never an execution authority; discovery, selection, validation, authorization and execution are separate. | [02-architecture.md](02-architecture.md), [09-security-model.md](09-security-model.md) |
| 76 | **Distribution is crates.io plus GitHub Releases**, with a local Capability Registry and a language-independent Manifest. Install integrity and execution permission are verified separately. | [02-architecture.md](02-architecture.md) |
| 77 | **Generated Capability code is user-owned; the tool is Apache-2.0.** External code and model terms are respected separately. | [GENERATED-CODE-POLICY.md](../GENERATED-CODE-POLICY.md) |
| 78 | **ABI value passing is a normalized ABI plus large-data handles.** Small values pass by value; large data passes as a runtime-managed handle (a resource id, not a raw address). | [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 79 | **The VM is a stack machine with mandatory static verification.** Invalid bytecode is rejected before execution; loop and memory limits are a separate mechanism. | [06-adaptive-compiler.md](06-adaptive-compiler.md), [07-execution-model.md](07-execution-model.md) |
| 80 | **AIR separates Nullable (`Option<T>`) from Missing (field presence)** and uses **explicit namespaces**. | [05-adaptive-ir.md](05-adaptive-ir.md), [04-capability-model.md](04-capability-model.md) |
| 81 | **A swap drains gracefully first and cancels only when safe.** Running → Draining → Stopped, with a hard timeout, a declared cancel-safety, and swap rejection when neither drain nor cancel is safe. | [07-execution-model.md](07-execution-model.md), [10-adaptation-transactions.md](10-adaptation-transactions.md) |
| 82 | **Determinism is a three-level model** — bitwise (integers, serialization), numerical (tolerance), best-effort (GPU/parallel). The level is declared and **verified**, not trusted. | [14-evaluation.md](14-evaluation.md), [13-poc.md](13-poc.md) |
| 83 | **Linux is the primary target; WSL2 is allowed for development.** Isolation and process tests run on real Linux (CI). | [12-roadmap.md](12-roadmap.md) |
| 84 | **Observability is structured event tracing** (a trace id per swap); **testing is layered** (unit, integration, golden, property, failure/swap). | [14-evaluation.md](14-evaluation.md), [02-architecture.md](02-architecture.md) |

The Appendix B decisions were answered before the 63-question round and are
renumbered here (#64–#69) so that every reference has one unique number. A bare
`#N` always means a question from the 63-question round.

## Tier 2 · Validate by the PoC (Stages 0–2)

| # | Direction | Owner |
|---|---|---|
| 11 | CI grows in stages: doc-link check, markdownlint, cargo test, clippy, cargo-deny first; then cargo audit and coverage. Coverage is not a goal by itself. | [.github/workflows/ci.yml](../.github/workflows/ci.yml) |
| 18 | Compatibility considers type **plus** semantic contract, preconditions and postconditions. Stage 0 defines the **representation only**; enforcement is Stage 3 ([RFC 0002](rfc/0002-stage0-execution-contract.md)). | [04-capability-model.md](04-capability-model.md), [05-adaptive-ir.md](05-adaptive-ir.md) |
| 21 | **LLVM** is the first native backend candidate; Cranelift is compared when fast JIT matters. | [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 24 | **async/await + channels**; a full actor model is not introduced up front. | [07-execution-model.md](07-execution-model.md) |
| 25 | Incremental compilation is per Capability, **plus** dependents of a changed dependency. | [06-adaptive-compiler.md](06-adaptive-compiler.md) |
| 28 | **Copy-on-write + incremental snapshots**; compression is applied by measured trade-off. | [03-workspace.md](03-workspace.md) |
| 29 | Re-derivation alone is insufficient: some state (a consumed one-shot response, a live session) needs **selective per-engine checkpoints** and a change-rejection path. | [03-workspace.md](03-workspace.md), [07-execution-model.md](07-execution-model.md) |
| 30 | Clone shares immutable data by default and **COW**s on write; real copies only when required. | [03-workspace.md](03-workspace.md) |
| 34 | Permissions are requested before execution and re-approved on change; a **per-Workspace policy** reduces approval fatigue. | [09-security-model.md](09-security-model.md) |
| 40 | **llama.cpp** is the first local inference engine; ONNX Runtime is connected for ONNX models and hardware acceleration. | [02-architecture.md](02-architecture.md) |
| 41 | Model routing considers difficulty **plus** memory, GPU, power and privacy policy. | [02-architecture.md](02-architecture.md) |
| 42 | **JSON Schema** structured output first; validate the intermediate result, then convert to AIR. | [02-architecture.md](02-architecture.md) |
| 50 | The Slint license is checked against the actual distribution model before it is assumed settled. | [02-architecture.md](02-architecture.md) |
| 54 | Build a **policy layer over the existing OS's resource management**; do not write a kernel scheduler. | [11-performance.md](11-performance.md) |
| 55 | A **GPU backend abstraction**; Vulkan as the general candidate, CUDA/ROCm as optional backends. | [11-performance.md](11-performance.md) |
| 56 | Measure native-relative performance **per work type**; no single target ratio yet. | [14-evaluation.md](14-evaluation.md) |
| 57 | Power management is a design goal; start with **resource measurement and observability**. | [11-performance.md](11-performance.md) |

## Tier 3 · Later (Stages 3–6)

| # | Decision deferred | Owner |
|---|---|---|
| 2 | Publicising the project (GitHub-centred; HN if the result is interesting). | [12-roadmap.md](12-roadmap.md) |
| 6 | Co-maintainers, granted to those who show real contribution. | [GOVERNANCE.md](../GOVERNANCE.md) |
| 7 | Competitions and papers are pursued; **patents** only for concretely novel technology, checked before disclosure. | [GOVERNANCE.md](../GOVERNANCE.md) |
| 31 | Multiple concurrent Workspaces, with policy-based resource priority. | [03-workspace.md](03-workspace.md) |
| 32 | Multi-user and remote Workspaces, after the security model stabilises. | [03-workspace.md](03-workspace.md) |
| 35 | External registry signing (Sigstore) when a remote registry exists. | [04-capability-model.md](04-capability-model.md) |
| 39 | Model family choice (Qwen first candidate, Llama compared) — kept vendor-independent. | [02-architecture.md](02-architecture.md) |
| 43 | Remote model providers behind an abstraction; keys in the OS secret store. | [02-architecture.md](02-architecture.md) |
| 46–49 | Reverse-engineering research: name, scope (open source first), Ghidra first, and a written legal/ethical guideline. | [08-software-understanding.md](08-software-understanding.md) |
| 51–53 | Accessibility from the start, i18n (English + Korean), keyboard/mouse first, touch then voice. | [02-architecture.md](02-architecture.md) |
| 58 | aarch64 after x86_64; keep AIR, ABI and bytecode architecture-independent. | [02-architecture.md](02-architecture.md) |
| 59 | Self-hosted kernel research only **after** Stages 0–4 prove the core hypothesis on the existing kernel. | [12-roadmap.md](12-roadmap.md) |
| 60 | Stage 6 starts from a **minimal bootable system**, not a full desktop OS. | [12-roadmap.md](12-roadmap.md) |
| 61–63 | Team growth by area; `GOOD_FIRST_ISSUES` after the architecture stabilises; Discussions first, Discord only when real-time work needs it. | [GOVERNANCE.md](../GOVERNANCE.md) |

## How to close a question

1. Write the answer into the owning document as a normative statement.
2. If contested, record the reasoning in an RFC under [`rfc/`](rfc/).
3. Move it up a tier here with its number and a one-line summary.
4. A change to a principle or the security model is a separate,
   consensus-level change ([GOVERNANCE.md](../GOVERNANCE.md)).
