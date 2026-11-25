# 06 · Adaptive Compiler

The Adaptive Compiler turns AIR into an executable structure and is the
component accountable for what actually runs. A model generates or proposes;
the compiler is what the OS trusts.

The compiler is not merely a code generator. It analyses the composition and
execution plan the OS manages and lowers it into a structure that is safe and
efficient.

## Pipeline

The pipeline, in order:

1. **AIR Parser** — produce an AST from an AIR document.
2. **AST construction** — a structured tree of the composition and change.
3. **Static type checking** — every connection type-compatible
   ([04-capability-model.md](04-capability-model.md)).
4. **Semantic analysis** — bind names, resolve Capabilities, reject
   contradictions.
5. **Typed IR** — a checked representation with types attached.
6. **Dependency analysis** — the Capability dependency graph.
7. **Data-flow analysis** — where data is produced, transformed and consumed.
8. **Execution-plan optimisation** — order, fuse, and eliminate redundant work.
9. **Incremental compilation** — recompile only what a change touched.
10. **Code and module generation** — emit the execution modules.
11. **Hand-off** — deliver a verified execution plan to the runtime.

## Native or bytecode

Execution modules are **hybrid**: native object code and a project bytecode
path, unified by a module ABI ([16-open-questions.md](16-open-questions.md),
#20, #21). The choice is **not** made from the code's origin. It is made from the
analysis and the trust the compiler can establish:

- A Capability may be compiled **native** when static analysis, the verification
  level, and the execution permissions it needs all support it — **including a
  Capability a model generated**, once it has been analysed and proven safe.
- A Capability runs as **bytecode** when its safety cannot yet be established.
- Bytecode is **not** trusted by virtue of being bytecode. The runtime MUST still
  verify memory access, syscalls and resource use, and the VM itself is part of
  the attack surface ([09-security-model.md](09-security-model.md)).

The direction is to start safe and raise performance as trust is earned:
initially a Capability may run as bytecode, and a later compiler pass MAY
recompile a verified Capability to native. That recompilation is a **future
extension**, not a Stage 0 feature
([11-performance.md](11-performance.md), #68).

## The bytecode VM and the native backend

- The bytecode path is a **minimal project VM** (#20). It starts with a verifier
  and a limited instruction set; a complex VM is not built up front.
- The native backend's first candidate is **LLVM** (#21). Cranelift is compared
  when fast compilation and JIT matter more than peak code quality.

## The module ABI

The ABI is a **project ABI**, not C ABI as the internal standard (#22). It serves
both native and bytecode modules.

**Stage 0 defines a minimal ABI skeleton** ([RFC 0002](rfc/0002-stage0-execution-contract.md)):

- value passing (the basic rule for input and output values),
- error handling (a common success/failure result rule),
- one asynchronous execution rule (await and receive a result),
- an **explicit ABI version field**.

Deferred to later RFCs, based on implementation evidence: complex ownership and
lifetime rules, zero-copy transfer of large data, streaming-transfer details, GPU
memory sharing, and advanced native/VM call optimisation.

### ABI version compatibility

A version field alone does not make modules compatible. The runtime MUST verify
compatibility before running a module:

1. read the module's ABI version;
2. check it against the compatibility rules;
3. run it if compatible;
4. otherwise **reject explicitly**, or use a conversion layer.

Stage 0 MUST validate only that one **bytecode** module is called through this ABI
and returns a result. The native backend is not built at Stage 0.

## Incremental compilation

Recompiling the whole Workspace on every change violates principle 5
([15-principles.md](15-principles.md)). The compiler MUST:

- recompile only the Capabilities a change affects, **and the Capabilities that
  depend on a changed dependency** (#25);
- cache dependency analysis keyed by the inputs that produced it, in a **local,
  content-addressed module cache** (#26); a remote registry is a later addition;
- reuse unchanged execution modules;
- keep the incremental result identical to a full compile (checked by a test).

## The compiler is the trust boundary

Because a model may have produced the AIR, the compiler MUST NOT assume the AIR
is benign. In particular:

- It MUST reject a composition that violates its declared permissions.
- It MUST NOT emit code that can escape the isolation boundary.
- It MUST produce a verifiable execution plan; an unverifiable plan is a
  failure, not a warning.

## Related documents

- [05-adaptive-ir.md](05-adaptive-ir.md) — the input.
- [07-execution-model.md](07-execution-model.md) — the modules it emits and how
  they run.
- [11-performance.md](11-performance.md) — the performance the pipeline targets.
