---
rfc: 0002
title: The Stage 0 execution contract
status: accepted
author: x1zz
created: 2026-11-25
owning-document: docs/13-poc.md
---

# RFC 0002 · The Stage 0 execution contract

## Summary

This RFC fixes what the Stage 0 execution layer commits to, so the PoC tests the
core hypothesis without pulling in the whole system: how a change is verified
(the two-layer consistency rule), how modules talk (a minimal ABI), where code
runs (in-process, trusted only, behind an execution interface), and how far
contracts go (representation only, no enforcement).

## Motivation

The Stage 0 risk is building too much at once and never testing whether the OS
can recompose a running environment while preserving state and meaning. Each of
the four boundaries below could otherwise expand without limit. Fixing them keeps
Stage 0 focused and leaves room to grow.

## Guide-level explanation

- When a Capability is replaced, success is not "the program still runs". It is
  **two independent checks passing**: the state crossed intact, and the numerical
  result is equivalent within tolerance. Either check alone can be fooled.
- Modules in Stage 0 exchange values, report success or failure, and await one
  asynchronous result, under an ABI that carries its own version.
- Stage 0 runs only trusted, self-written code, in-process. The runtime talks to
  an **execution interface**, so process isolation can be added in Stage 1
  without rewriting the runtime. No isolation is claimed for Stage 0.
- AIR and the manifest can **express** preconditions and postconditions; nothing
  evaluates them yet.

## Reference-level explanation

### 1 · Consistency rule

**PoC Consistency Rule.** State migration and recomputation MUST satisfy two
independent validation layers: state integrity validation and numerical result
equivalence. Both MUST pass.

- **Numerical equivalence** uses configurable absolute and relative tolerances,
  initially `eps_abs = 1e-12` and `eps_rel = 1e-9`, compared as

  ```text
  |a - b| <= eps_abs + eps_rel * max(|a|, |b|)
  ```

  These are PoC defaults, not a permanent standard for all data.
- **State integrity** is validated according to the state representation: bitwise
  equality where the representation permits it, and structural or semantic
  validation otherwise (for example, a floating-point accumulation state whose
  exact bits may differ).

A result that is "close enough" MUST NOT by itself be taken as proof that the
state migrated correctly. A normal recomputation difference MUST NOT by itself be
taken as failure. The two layers catch different faults.

### 2 · Module ABI

Stage 0 defines a **minimal Module ABI skeleton**: value passing, error handling,
and one asynchronous execution rule. The ABI includes an **explicit version
field**.

Deferred to later RFCs, based on implementation evidence: complex ownership and
lifetime rules, zero-copy transfer of large data, streaming-transfer details, GPU
memory sharing, and advanced native/VM call optimisation.

A version field alone does not make modules compatible. The runtime MUST verify
compatibility before running a module:

1. read the module's ABI version;
2. check it against the compatibility rules;
3. run it if compatible;
4. otherwise **reject explicitly**, or use a conversion layer.

Stage 0 MUST validate only that one **bytecode** module is called through this ABI
and returns a result. The native backend is not built yet.

### 3 · Execution isolation

Stage 0 permits **in-process execution exclusively for trusted code**. The
execution architecture MUST provide an abstraction boundary — an **execution
interface** — that lets process isolation be introduced in Stage 1 without
redesigning the runtime.

- PoC code is self-written and trusted.
- Untrusted or model-generated code MUST NOT run through the trusted in-process
  path.
- Stage 0 MUST NOT claim that process isolation or a VM sandbox is provided.

The planned progression is: Stage 0 in-process executor → Stage 1 process
executor → later, a hardened VM sandbox.

### 4 · Contracts

Stage 0 defines the **structural representation** of preconditions and
postconditions in AIR and the manifest, without enforcing them. Enforcement is
deferred to Stage 3.

Stage 0 fixes only: the pre/post distinction; that a contract belongs to a
Capability; that the representation is extensible; and the direction of contract
identification and versioning.

**Basic input validation and runtime error handling remain mandatory** for PoC
correctness. Rejecting a malformed CSV or an unsupported statistic is not
contract enforcement; it is the minimum defence each function needs to be
correct at all.

## Drawbacks

- Two-layer verification is more work than a single equality check, but a single
  check is unsound.
- A minimal ABI will change; the version field and compatibility check are the
  mitigation.
- In-process execution cannot prove anything about isolation, so Stage 0 does not
  claim to.

## Rationale and alternatives

- **Bitwise-only consistency.** Rejected: floating-point recomputation would fail
  on a correct change.
- **Tolerance-only consistency.** Rejected: a dropped datum could pass if the
  final number happened to match.
- **Full ABI now.** Rejected: an ABI fixed before implementation is usually
  wrong, and it is the hardest thing to change.
- **Process isolation now.** Rejected: it would slow Stage 0 and, on failure,
  make it unclear which hypothesis was wrong.
- **Enforce contracts now.** Rejected: the PoC does not need it, and enforcement
  belongs with the compiler.

## Unresolved questions

- Per-operation tolerances for statistics (#56 follow-up).
- Per-representation state-integrity rules (#29 follow-up).
- Error accumulation on large datasets (#28 follow-up).
- The split between static and runtime contract checking in Stage 3 (#18
  follow-up).

## What this updates

[13-poc.md](../13-poc.md) (consistency, isolation, contract note),
[07-execution-model.md](../07-execution-model.md) (execution interface),
[06-adaptive-compiler.md](../06-adaptive-compiler.md) (ABI skeleton),
[05-adaptive-ir.md](../05-adaptive-ir.md) and
[04-capability-model.md](../04-capability-model.md) (contract representation),
[14-evaluation.md](../14-evaluation.md) (tolerances), and the ledger
[16-open-questions.md](../16-open-questions.md) (#66, #22).
