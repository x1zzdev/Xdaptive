# Design round 2 — ABI, VM, types, async swap, determinism, environment, testing

**Date:** 2026-11-26
**Ledger:** #78–#84
**Recorded in:** `docs/06`, `docs/05`, `docs/07`, `docs/14`, `docs/12`,
`docs/13`, `docs/17`

## Context

Round 2 settled seven questions that block the next RFCs and the PoC: how values
cross the ABI, what the VM is, how AIR handles absence and names, how a running
Capability is replaced, how reproducible execution must be, where development
happens, and how the system is observed and tested.

## Decisions

1. **ABI memory (#78).** A normalized ABI for small values plus runtime-managed
   handles for large data. A handle is a resource id, not an address, so the
   interface survives a module moving out of process. A handle is not zero-copy
   by itself.
2. **VM (#79).** A stack machine with mandatory static verification. Invalid
   bytecode is rejected before execution; loop and memory limits are a separate
   mechanism.
3. **Types (#80).** Nullable (`Option<T>`) and missing (field presence) are
   distinct; explicit namespaces (`core::Int`, `csv::Row`). Dimensional types
   stay deferred (#73).
4. **Async swap (#81).** Graceful drain first, limited cancellation second.
   Running → Draining → Stopped; a hard timeout; swap rejected when neither drain
   nor cancel is safe.
5. **Determinism (#82).** Three levels — bitwise, numerical (tolerance),
   best-effort (record conditions). Declared by the Capability, verified by the
   runtime, never trusted.
6. **Environment (#83).** Linux primary; WSL2 allowed for development; isolation
   and process tests on real Linux.
7. **Observability and testing (#84).** Structured events with a trace id per
   swap; five test layers (unit, integration, golden, property, failure/swap).

## Rebuttals and questions raised (open)

These are unresolved and were not folded into the specification as decided.

- **ABI handle lifetime (#78).** The decision says a handle has an owner and
  lifetime, but not *which* rule. Reference counting, an epoch/arena, or
  ownership transfer all work; each has different failure modes. A handle that
  outlives its data is a dangling reference, and in-process this is silent. The
  lifetime rule is the hard part and is still open.
- **The verifier's type system vs AIR's (#79).** Verifying operand types requires
  the bytecode verifier to track types on the stack. AIR's typing is nominal;
  bytecode typing is structural. If the two disagree, a verified module can still
  violate the AIR contract. Is the verifier's type system the same one, or a
  second one that must be kept in sync?
- **The verifier is trusted code.** A verifier bug is a security bug. Is the
  verifier in scope for formal verification (see #36), and does Stage 0 accept an
  unverified verifier as a known risk?
- **`Presence<T>` as a value type (#80).** Field presence is usually a
  schema/mapping concern (Protobuf presence, SQL absent column vs NULL). Making
  it a value type may confuse "the field is absent" with "the value is a marker".
  Should missing-ness live at the record/schema level instead of the value type?
- **Primitives in a namespace (#80).** `core::Int` namespaces a primitive, which
  most languages do not. Are primitives global, or truly namespaced?
- **`Option<T>` and generics (#80).** Generics are deferred, but `Option<T>` is
  generic-looking. Is it built-in syntax, not a generic instantiation?
- **Cancel-safety declared by the Capability (#81).** A Capability that declares
  itself non-cancellable can block swaps. The runtime must have a hard upper bound
  independent of the declaration. Is that bound global, or per-Workspace policy?
- **Determinism verification (#82).** "Declared and verified" — verified how, and
  when? Run-twice-and-compare is plausible but expensive and not always possible
  for best-effort work. Is the level per Capability or per output/operation?
- **WSL2 and isolation (#83).** WSL2 runs a real Linux kernel, but cgroups and
  some syscalls differ. Is CI the only real-Linux environment, and is there a
  dedicated isolation test environment?
- **Structured logs (#84).** JSON is readable but costly at high event rates.
  Should the trace be a binary format with a JSON export? How does the trace id
  propagate across a process boundary in Stage 1?
- **Golden bytecode tests (#84).** These are brittle unless the bytecode has a
  canonical disassembly. Is a canonical disassembly part of the VM spec?
- **ADR vs RFC vs ledger.** Three records now exist. Is the split (proposal /
  narrative / status) clear enough, or should two merge?

## Outcome

All seven decisions are recorded as normative statements in their owning
documents, and consolidated in `docs/17-adr.md`. The rebuttals above are open and
are tracked as follow-ups in `NEXT.md`.
