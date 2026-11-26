# Design round 3 — resolving the round-2 rebuttals

**Date:** 2026-11-26
**Ledger:** #78–#84 (refined)
**Recorded in:** `docs/04`, `docs/05`, `docs/06`, `docs/07`, `docs/14`

## Context

Round 2 raised 18 rebuttals and questions. This round resolves them directly in
the specification, and notes where the earlier instinct to defer was too
conservative. The project needs technical ambition, not maximal caution: the
standard is "is this the right design", not "is this the safest thing to put
off".

## Resolutions

### 1 · ABI handle lifetime → generational, single-owner

Not "move only, sharing later". Handles are `(workspace_epoch, slot,
generation)` with a per-Workspace handle table. Invalidation bumps the
generation, so a stale handle is **detected and rejected** rather than silently
corrupting. Ownership is by move. This makes the hard case (dangling reference)
a clean error in-process, and the design already leaves room for refcount and
zero-copy handles. (Deferring to a bare "move" would have left a silent
use-after-free.)

### 2 · Verifier vs AIR types → typed bytecode, one vocabulary

Not two type systems. The compiler lowers typed AIR to bytecode carrying **AIR's
own type ids**; the verifier enforces them structurally in the same pass as the
stack and control-flow checks. So "verification passed" implies "AIR-compatible",
which is the property the design needs. The verifier is not a second nominal
system to keep in sync.

### 3 · Presence → record-level, not a value type

`Presence<T>` as a value type is dropped. `Option<T>` stays a value type; field
absence is a **record/schema-level** property (`name?: T`). This matches how
presence actually behaves and removes the marker-vs-absence ambiguity.

### 4 · Non-cancellable abuse → hard bound, reject, one exception

The runtime enforces a **hard drain bound** independent of any declaration. On
exceed, the swap is **rejected** (state preserved), not force-killed. The single
exception is a Capability declared **disposable**, which may be force-stopped
because it has no state to lose. The bound is a Workspace policy with a global
default.

### 5 · Determinism verification → verified record

Not "trust the declaration" and not "run twice every call". The harness checks in
the verification phase and records the result in a **signed verification record**;
the runtime trusts the record. A cheap spot-check is optional. This removes the
apparent contradiction between "declared" and "not trusted".

### 6 · Determinism unit → per output

The level is declared **per output** (with a Capability default), because one
Capability can mix deterministic and non-deterministic work.

### 7 · External side-effect swap → idempotent/compensable, not unsupported

Not "unsupported at Stage 0". A Capability with an external effect is swappable
when the effect is declared **idempotent** or **compensable**; otherwise the swap
is rejected. The declarations exist from the start; the PoC exercises the
no-side-effect case.

### 8 · Trace across a process → trace context in the ABI

Every call carries a trace context (trace id + span id, W3C-traceparent style), so
a trace survives a process boundary without a protocol change.

### 9 · Primitive namespacing → primitives global

`Int`, `Float`, `String` are global; only user and Capability types are
namespaced. No `core::` prefix on every expression.

### 10 · `Option<T>` and deferred generics → built-in constructor

`Option<T>` is a built-in type constructor, not a user generic. No contradiction
with deferring general generics.

### 11 · Four versions → one compatibility matrix

A single matrix over AIR schema, ABI, Capability and state schema versions,
defined in `docs/04`, states which combinations run. Without it, four independent
numbers multiply into an unmanageable space.

### 12 · Canonical disassembly → part of the VM spec

A stable textual disassembly is specified, so golden bytecode tests and review
have a fixed form.

### 13 · ADR / RFC / ledger → keep, by granularity

Kept, because they sit at different granularities: the ledger is status, the ADR
is narrative, the RFC is a proposal for a contested decision. Duplication is
avoided by not repeating prose across them.

### 14 · JSON log cost → typed schema, pluggable encoding

The trace is a typed event schema; JSON is the Stage 0 encoding and a binary
encoding is a later option. The schema does not change with the encoding.

### 15 · Verifier assurance → mitigated now, formal later

The verifier is a known risk mitigated by design (small auditable instruction
set, fuzzing, property tests); formal verification of the verifier joins the
Stage 3 roadmap with the contract engine. It is not accepted blindly.

### 16 · Resource limits → budget + wall-clock + memory

Instruction budget (deterministic, portable), wall-clock timeout (backstop for
blocked I/O), and a per-module memory ceiling.

### 17 · `Xdaptive-process/` → committed

Committed. The project's open-source principle is to develop the design process in
the open, so the reasoning is part of the record, not hidden. If it ever holds
sensitive material it can be moved to local-only, but design discussions are not
sensitive.

### 18 · Development machine → WSL2 confirmed

The current environment is WSL2 (`6.18.33.2-microsoft-standard-WSL2`, cgroup v2,
22 cores). It runs a real Linux kernel, so namespaces and cgroups work for
development, but isolation claims are still verified on real Linux CI.

## Where the earlier instinct was too conservative

- **Handles:** "move only" would have hidden a silent failure. The generational
  design is more work but is the correct one.
- **Side effects:** "unsupported at Stage 0" would have shrunk the design. The
  idempotent/compensable declaration keeps the ambition.
- **Determinism:** "trust the declaration" was a contradiction; the verified
  record is both safer and more honest.
- **Verifier:** "known risk" alone was too passive; naming the mitigation and the
  formal-verification roadmap is the right level.

## Genuinely open (not deferred out of caution)

- The concrete AIR presence syntax and grammar (an RFC, not a hand-wave).
- The concrete ABI compatibility rules in the matrix.
- Safe cancellation of in-flight asynchronous work (the mechanism, not whether).
- The storage and transaction model's concrete layout.
- The form in which analysis output becomes a Manifest.
