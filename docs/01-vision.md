# 01 · Vision and identity

## Identity

| Field | Value |
|---|---|
| Project name | **Xdaptive** |
| Product name | **Xdaptive OS** |
| Category | **Adaptive Computing OS** |
| Slogan | **Beyond Applications.** |
| Supporting line | **Software that adapts to your intent.** |
| Philosophy | **Adaptation is a system capability.** |
| Development | Open source |

## The thesis

Xdaptive is an open-source operating system that generates, compiles, executes
and evolves software around what the user is trying to do.

The project does **not** add AI features to an existing operating system. It
changes what an operating system manages. Today an OS runs programs that were
designed in advance, and the user bends their work to fit the program. Xdaptive
makes the OS own the **composition, execution, optimisation, change and
lifecycle of software itself**, so the program stops being the fixed boundary and
becomes something the system assembles for the task.

This inverts the relationship between people and software:

- Today: the user picks a program, learns it, and connects programs by hand.
- Xdaptive: the user states a task; the OS composes a Workspace, runs it as one
  integrated environment, and adapts it in place as the task changes.

## What the slogan means

- **Beyond Applications.** — move past the fixed boundary of the application.
  The unit is the task, and the software that serves it is composed and changed
  rather than installed and obeyed.
- **Software that adapts to your intent.** — software is configured and changed
  around the user's intent, not the other way around.
- **Adaptation is a system capability.** — adaptation is not a feature a program
  ships. It is a capability of the system: the OS understands and manages
  software composition, data flow, execution structure, functionality, interface
  and state, and changes them safely when needed.

## What Xdaptive is not

Stating the boundaries is part of the design.

- It is **not** "an OS with an AI feature". AI interprets intent and proposes
  changes; the substance of the project is the system that represents, verifies,
  executes and evolves software. See [09-security-model.md](09-security-model.md).
- It is **not** an AI that drives the user's computer. A model holds no OS
  authority. It proposes a plan; the OS validates, sandboxes and commits it, or
  rolls it back.
- It is **not** a claim that any program can be rewritten. Adaptation has three
  levels and every piece of software supports a different range; the OS must
  know the range and stay inside it. See [07-execution-model.md](07-execution-model.md).
- It is **not** a promise that reverse engineering opens every program. Being
  able to analyse a binary is not a right to copy or redistribute its code. See
  [08-software-understanding.md](08-software-understanding.md).

## Differentiation

AI coding tools and agents already generate code and automate existing software.
The difference is not that Xdaptive can produce code. It is that Xdaptive:

1. integrates the whole lifecycle **at the operating-system level**, and
2. **redesigns the representation and execution model of software** rather than
   automating a model that already exists.

To do that the project builds its own intermediate representation, compiler,
execution model, Workspace runtime, capability model and composition engine.
Reverse engineering and external software are available resources, not the
project's defining idea; even when the project uses them, the software model and
the execution framework are designed here.

## The end state

In the environment Xdaptive targets, the user describes a task. The OS
understands it, composes the software it needs, reuses existing Capabilities,
generates new ones, and optimises the environment. The user works inside one
Workspace. When requirements change, the software changes while preserving its
state.

Software is no longer a fixed product. It is part of a system that is composed,
executed and changed to realise the user's purpose — and that composition and
execution are managed by the operating system.
