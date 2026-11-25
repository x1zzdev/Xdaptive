# 16 · Open questions

This is the list of decisions the project has deliberately deferred. An open
question is not a bug: it is a decision waiting for a test that can settle it.
Each entry names the document that will own the answer.

A question leaves this list when it is answered in the owning document (or an
accepted RFC). It does not leave because someone has an opinion.

## Decisions already settled

These were open and are now fixed. They are kept here so the reasoning is not
lost.

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

## Open — software model

1. **What is the minimum unit of composition?** Capability, Module or Component?
   *Owner: [04-capability-model.md](04-capability-model.md).* The current
   working choice is **Capability**, with Module as an implementation grouping.
   Not yet settled by a test.
2. **How far must AIR express software structure and execution flow?**
   *Owner: [05-adaptive-ir.md](05-adaptive-ir.md).*
3. **How is compatibility between Capabilities from different software decided?**
   *Owner: [04-capability-model.md](04-capability-model.md).*
4. **What common type and memory model carries data between Capabilities?**
   *Owner: [04-capability-model.md](04-capability-model.md) and
   [05-adaptive-ir.md](05-adaptive-ir.md).*

## Open — compilation and execution

5. **What kind of execution module does the Adaptive Compiler emit?** Native
   object, bytecode for a project VM, or a mixture? *Owner:
   [06-adaptive-compiler.md](06-adaptive-compiler.md).*
6. **What binary and ABI contract do execution modules follow?** *Owner:
   [07-execution-model.md](07-execution-model.md).*
7. **How is existing state preserved when a running program is changed?** Which
   state can migrate, and which must be re-derived? *Owner:
   [07-execution-model.md](07-execution-model.md) and
   [10-adaptation-transactions.md](10-adaptation-transactions.md).*
8. **What compile and runtime structure keeps native performance while
   supporting dynamic composition?** *Owner: [11-performance.md](11-performance.md).*

## Open — understanding and trust

9. **In what form does analysis output become a Software Manifest?** *Owner:
   [08-software-understanding.md](08-software-understanding.md) and
   [04-capability-model.md](04-capability-model.md).*
10. **How is a model-generated Capability tested and verified before it may
    touch user data?** *Owner: [09-security-model.md](09-security-model.md).*

## Open — persistence

11. **How is a Workspace stored and recovered?** Which parts are snapshots, which
    are content-addressed, which are logs? *Owner:
    [03-workspace.md](03-workspace.md) and
    [10-adaptation-transactions.md](10-adaptation-transactions.md).*
12. **How is a Workspace change applied atomically?** The transaction model.
    *Owner: [10-adaptation-transactions.md](10-adaptation-transactions.md).*

## Open — platform

13. **What is the IPC protocol between the layers?** *Owner:
    [02-architecture.md](02-architecture.md).*
14. **What is the desktop UI framework?** Slint and others are candidates.
    *Owner: [02-architecture.md](02-architecture.md).*
15. **What local model and inference engine is the default?** llama.cpp, ONNX
    Runtime and others are candidates. *Owner: [02-architecture.md](02-architecture.md).*
16. **What is the scope of the project's own scheduler, beyond Linux's?** *Owner:
    [11-performance.md](11-performance.md).*
17. **What is the reverse-engineering analysis pipeline, end to end?** *Owner:
    [08-software-understanding.md](08-software-understanding.md).*
18. **What is the kernel strategy: stay on Linux, or eventually self-host?** The
    current decision is to stay on Linux and keep the option open. *Owner:
    [02-architecture.md](02-architecture.md) and [12-roadmap.md](12-roadmap.md).*

## How to close a question

1. Write the answer into the owning document as a normative statement.
2. If the question was contested, record the reasoning in an RFC under
   [`rfc/`](rfc/).
3. Move the entry from "Open" to "Decisions already settled" with a one-line
   summary and a link.
4. If the answer changes a principle or the security model, that is a separate,
   consensus-level change. See [GOVERNANCE.md](../GOVERNANCE.md).
