# 15 · Design principles

These ten principles are the filter for every change. When a decision is
contested and the documents do not settle it, the principles break the tie. A
proposal that violates one of them is rejected unless the principle itself is
changed first.

1. **Task-centric.** Organise the system around the user's task, not around
   programs. The task, not the application, is the unit of computing.

2. **Adaptive software.** Treat software as something that can be composed and
   changed for a task, not as a fixed finished product. Know and respect each
   piece's adaptation level.

3. **Unified experience.** However many engines, services, processes and modules
   run underneath, the user sees one integrated Workspace.

4. **Native performance.** Ordinary work MUST run without an AI call in the
   execution path. AI is invoked when understanding or structural change is
   needed, not on every operation.

5. **Incremental adaptation.** Never rebuild the whole environment to apply a
   change. Analyse and recompose only what changed.

6. **Safe execution.** Never trust a model's plan or code directly. The OS MUST
   validate, sandbox and control it, and MUST roll back on failure.

7. **Persistent state.** A Workspace's state and change history MUST be
   preserved and recoverable, and a change MUST NOT silently destroy task state.

8. **Kernel independence.** Start on Linux, but design the core execution model
   and the upper technology so they do not depend on one kernel.

9. **Original technology.** Do not stop at combining external tools. Build the
   project's own IR, compiler, runtime, resource management and composition
   technology.

10. **Measurable engineering.** Every core claim MUST be verifiable by a real
    implementation and a measurement. Do not assert speed or correctness
    without naming how it is measured. See
    [14-evaluation.md](14-evaluation.md).

## The safety-first corollary

> Change only when the change can be made safely. Never trade data integrity for
> performance or adaptability.

This is principles 6 and 7 stated as one rule. The OS is not one that can always
change; it is one that changes only when it can do so safely. A change that would
lose task state, or that cannot be made atomic, is rejected rather than
attempted. See [07-execution-model.md](07-execution-model.md) and
[10-adaptation-transactions.md](10-adaptation-transactions.md).

## How the principles interact

Two of them are load-bearing and can conflict with the others:

- **Safe execution (6)** overrides **native performance (4)** when the two
  disagree. A fast path that lets unvalidated model output reach user data is
  not a fast path; it is a defect.
- **Measurable engineering (10)** overrides **original technology (9)** as a
  claim. Building something new is only valuable if it can be shown to work.

The AI/OS split in [09-security-model.md](09-security-model.md) is principle 6
made concrete. If a change cannot be described without weakening that split, the
change is wrong.
