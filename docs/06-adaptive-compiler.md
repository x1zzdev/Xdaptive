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

`Open question:` what kind of execution module the compiler emits — native
object code, bytecode for a project VM, or a mixture
([16-open-questions.md](16-open-questions.md), item 5). The working direction is
native execution where the Capability is available as native code, with a
project bytecode path for generated Capabilities, unified by a module ABI (item
6).

## Incremental compilation

Recompiling the whole Workspace on every change violates principle 5
([15-principles.md](15-principles.md)). The compiler MUST:

- recompile only the Capabilities a change affects;
- cache dependency analysis keyed by the inputs that produced it;
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
