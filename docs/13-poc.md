# 13 · First proof of concept

The first PoC is deliberately small. It tests the core hypothesis on a data
analysis task, not on a whole operating system.

## Goal

A user asks for a Workspace that analyses a CSV file and produces a graph. Then
the user adds a new analysis capability. The test is whether the existing task
state survives the change.

## Initial Workspace

Only two Capabilities exist at the start:

- **CSV Reader**
- **Data Viewer**

The Chart Engine is neither installed nor enabled.

## User request

> "Analyse the correlation in this data and add a scatter plot."

## Adaptive composition

The OS analyses the existing Workspace and computes the change it needs:

- keep the existing CSV Reader;
- add a Data Analysis Capability;
- connect the Chart Engine;
- compose the correlation workflow;
- add a scatter-plot UI panel;
- preserve the existing data and state.

This is a Level 2 change (workflow adaptation) plus a Level 1 change (interface
adaptation), as defined in [07-execution-model.md](07-execution-model.md). The
change is applied as a transaction
([10-adaptation-transactions.md](10-adaptation-transactions.md)).

## Workspace v2

If the change succeeds, the Workspace moves to a new version. The user keeps
using the existing environment and gains the new capability. The data the user
had loaded is still there, and the existing panels are unchanged.

## The hypothesis under test

> Can the system safely recompose an execution environment when the task changes,
> while preserving existing software functionality and task state?

## What the PoC must not depend on

The PoC does **not** need AI. It MUST be built and demonstrated with AIR and the
Workspace runtime alone. AI is connected afterwards, in Stage 4
([12-roadmap.md](12-roadmap.md)), and the PoC's result must hold with AI removed.

## Why this shape

- It exercises all four core technologies at once: AIR (the change), the
  compiler (lowering it), the execution model (state migration), and the
  Workspace runtime (versioning and rollback).
- Its state-preservation requirement is falsifiable: either the loaded data and
  the existing panels survive, or they do not.
- It is small enough to build before the surrounding system exists.

## Success criteria

1. The initial Workspace is created from an AIR description with no AI.
2. The change is applied as a transaction and the Workspace reaches v2.
3. The loaded data and the existing panels are preserved.
4. A deliberately failing change rolls back and leaves the Workspace at v1.

## Related documents

- [05-adaptive-ir.md](05-adaptive-ir.md) — the description language.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — the change
  procedure.
- [14-evaluation.md](14-evaluation.md) — how the result is measured.
