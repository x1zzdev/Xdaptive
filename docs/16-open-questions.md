# 16 · Open questions

This is the list of decisions the project has deliberately deferred. An open
question is not a bug: it is a decision waiting for a test that can settle it.
Each entry names the document that will own the answer.

Question numbers are **permanent identifiers**, not positions. When a question is
settled it moves to "Decisions already settled" and its number is not reused, so
references like "item 7" stay stable across edits.

## Decisions already settled

Kept here so the reasoning is not lost.

- The project name is **Xdaptive**, the product is **Xdaptive OS**, the category
  is **Adaptive Computing OS**. See [01-vision.md](01-vision.md).
- Development is open source. See [CONTRIBUTING.md](../CONTRIBUTING.md).
- The starting point is an AI-native OS, but AI holds no OS authority. See
  [09-security-model.md](09-security-model.md).
- The user-facing unit is one integrated Workspace, which evolves while the task
  runs. See [03-workspace.md](03-workspace.md).
- Core technology is built here: IR, compiler, runtime, resource management,
  composition. See [15-principles.md](15-principles.md).
- Linux is the starting substrate; kernel independence is a design constraint.
  See [02-architecture.md](02-architecture.md).
- **(#1) Capability is the minimum unit of composition.** Module is an
  implementation grouping; Component is a deployment grouping. See
  [04-capability-model.md](04-capability-model.md).
- **(#2) The Stage 0 AIR expresses only what the PoC needs**, and grows by RFC.
  See [05-adaptive-ir.md](05-adaptive-ir.md).
- **(#5) Execution modules are hybrid: native plus a project bytecode path.** The
  choice between them is decided by static analysis, verification level and
  execution permissions — **not** by the code's origin. See
  [06-adaptive-compiler.md](06-adaptive-compiler.md).
- **(#6 direction) The module ABI is fixed from the start.** The concrete
  contract (ownership, data passing, error handling, async rules) is still open
  below. See [07-execution-model.md](07-execution-model.md).
- **(#7) State preservation is a three-step policy:** migration, then
  re-derivation, then reject the change. Old state is preserved until new state
  is verified. See [07-execution-model.md](07-execution-model.md).
- **(#12 direction) Change is an atomic transition.** The concrete transaction
  model is still open below. See
  [10-adaptation-transactions.md](10-adaptation-transactions.md).
- **(#14) The desktop UI is Slint, behind a framework-independent UI Composition
  Layer.** egui MAY be used for developer/debug tooling. See
  [02-architecture.md](02-architecture.md).
- **(#17) Software understanding is a separate research project.** The main
  repository does not carry binary analysis; integration is evaluated after
  Stages 1–2. See [08-software-understanding.md](08-software-understanding.md).
- **(#15 direction) Local small models are the default; remote models are an
  optional, permissioned extension.** See [02-architecture.md](02-architecture.md).
- **(#18) The kernel strategy is to stay on Linux and keep self-hosting open.**
  See [12-roadmap.md](12-roadmap.md).

## Open — software model

3. **How is compatibility between Capabilities from different software decided?**
   *Owner: [04-capability-model.md](04-capability-model.md).*
4. **What common type and memory model carries data between Capabilities?**
   *Owner: [04-capability-model.md](04-capability-model.md) and
   [05-adaptive-ir.md](05-adaptive-ir.md).* Related to #6.

## Open — compilation and execution

6. **What is the concrete module ABI?** Ownership transfer, data passing, error
   handling and async-execution rules across the native/bytecode boundary. The
   direction (a stable ABI from the start) is settled; the contract is not.
   *Owner: [07-execution-model.md](07-execution-model.md).*
8. **What compile and runtime structure keeps native performance while
   supporting dynamic composition?** Includes the future direction of
   recompiling verified bytecode Capabilities to native. *Owner:
   [11-performance.md](11-performance.md).*

## Open — understanding and trust

9. **In what form does analysis output become a Software Manifest?** *Owner:
   [08-software-understanding.md](08-software-understanding.md) and
   [04-capability-model.md](04-capability-model.md).*
10. **How is a model-generated Capability tested and verified before it may
    touch user data?** The direction (a separate validation layer, and no
    origin-based trust) is settled; the mechanism is not. *Owner:
    [09-security-model.md](09-security-model.md).*

## Open — persistence

11. **How is a Workspace stored and recovered?** Which parts are snapshots, which
    are content-addressed, which are logs? *Owner:
    [03-workspace.md](03-workspace.md) and
    [10-adaptation-transactions.md](10-adaptation-transactions.md).*
12. **What is the concrete transaction model** behind the atomic transition?
    *Owner: [10-adaptation-transactions.md](10-adaptation-transactions.md).*

## Open — platform

13. **What is the IPC protocol between the layers?** *Owner:
    [02-architecture.md](02-architecture.md).*
15. **What concrete local model and inference engine is the default?** The
    direction (local-first) is settled; the engine is not. *Owner:
    [02-architecture.md](02-architecture.md).*
16. **What is the scope of the project's own scheduler, beyond Linux's?** *Owner:
    [11-performance.md](11-performance.md).*

## How to close a question

1. Write the answer into the owning document as a normative statement.
2. If the question was contested, record the reasoning in an RFC under
   [`rfc/`](rfc/).
3. Move the entry to "Decisions already settled" with its number, a one-line
   summary and a link.
4. If the answer changes a principle or the security model, that is a separate,
   consensus-level change. See [GOVERNANCE.md](../GOVERNANCE.md).
