# 13 · First proof of concept

The first PoC is deliberately small. It tests the core hypothesis on a data
analysis task, not on a whole operating system. It does **not** try to build a
faster data engine than Polars; it tests whether the OS can safely replace a
running data-processing Capability while preserving the task.

## Goal

A user asks for a Workspace that analyses a CSV file and produces a graph, then
changes the environment. The test is whether the existing task state survives the
change and the result stays consistent.

## What the PoC validates

Every earlier decision is exercised at once:

| Decision | What the PoC checks |
|---|---|
| Capability is the atom (#64) | data processing is built from independent Capabilities |
| minimal AIR (#65) | function declaration and connection are expressed in AIR |
| hybrid execution (#20) | execution modules are generated and connected |
| three-step state policy (#66) | data-processing state survives a change |
| atomic transition (#67) | a failed change restores the previous execution |

## Initial Workspace

Only two Capabilities exist at the start:

- **CSV Reader**
- **Data Viewer**

The statistics and chart Capabilities are neither installed nor enabled.

## The two scenarios

### Scenario 1 · Extend

The user asks:

> "Analyse the correlation in this data and add a scatter plot."

The OS analyses the Workspace and computes the change: keep the CSV Reader, add a
Data Analysis Capability, connect the Chart Engine, compose the correlation
workflow, and add a scatter-plot panel. Existing data and panels are preserved.
This is a Level 2 change plus a Level 1 change
([07-execution-model.md](07-execution-model.md)).

### Scenario 2 · Swap

The user replaces the data-processing Capability with a different implementation
while the task runs. The OS:

1. reads the CSV;
2. computes the mean and the correlation;
3. replaces the data-processing Capability at runtime;
4. preserves or re-derives the existing processing state;
5. checks that the final result is consistent before and after the swap.

This scenario is the heart of the PoC. If the result differs after the swap, or
the state is lost, the hypothesis fails.

## The hypothesis under test

> Can the system safely recompose an execution environment when the task changes,
> while preserving existing software functionality and task state?

## The minimal experiment

The hypothesis is tested by one sequence (#1):

1. Declare a Capability.
2. Express its function and connections in AIR.
3. Have the Adaptive Compiler generate an execution module.
4. Run it in the runtime.
5. Replace the Capability while it is running.
6. Migrate or re-derive the state.
7. Verify that the result before and after the change is consistent and safe.

If this sequence succeeds, it is the foundation on which AI, software
understanding, dynamic UI and GPU optimisation are later attached. Nothing else
is built until it does.

## Scope of the self-built engine

The PoC builds a minimal data engine directly, with no external dependencies. It
does not build a database.

| Feature | Scope |
|---|---|
| CSV | basic parsing, headers, numeric data |
| Statistics | mean, variance, correlation |
| Chart | scatter-plot data generation |
| State | processing position, accumulated statistics |
| Change | state migration and recovery |

The CSV parser MUST support the basic CSV rules — quotes, commas and embedded
newlines. A parser that is too naive would make the experiment's result
untrustworthy.

The interface between the data engine and the OS MUST be designed so that an
external engine (Polars, Arrow) can be connected as a Capability later. In Stage
1 and beyond, the same interface is used to attach external engines, so the
project gains its own technology **and** the ecosystem's performance.

## What the PoC must not depend on

The PoC does **not** need AI. It MUST be built and demonstrated with AIR and the
Workspace runtime alone. AI is connected afterwards, in Stage 4
([12-roadmap.md](12-roadmap.md)), and the result must hold with AI removed.

## Why this shape

- It exercises all four core technologies at once: AIR (the change), the compiler
  (lowering it), the execution model (state migration), and the Workspace runtime
  (versioning and rollback).
- Its state-preservation requirement is falsifiable: either the loaded data and
  the existing panels survive and the result is consistent, or they do not.
- It is small enough to build before the surrounding system exists.

## How a change is verified

A change is not verified by "the program still runs". It MUST pass **two
independent layers**, both defined in
[RFC 0002](rfc/0002-stage0-execution-contract.md) (#66):

1. **Numerical result equivalence.** The result before and after must satisfy

   ```text
   |a - b| <= eps_abs + eps_rel * max(|a|, |b|)
   ```

   with `eps_abs = 1e-12` and `eps_rel = 1e-9` as PoC defaults. Absolute and
   relative tolerances are both used, because a relative-only test breaks near
   zero.
2. **State integrity.** The migrated state must be validated against its
   representation: bitwise equality where the representation allows it, and
   structural or semantic validation otherwise (for example, floating-point
   accumulation state).

The two layers catch different faults: a tolerance-only check can pass when a
datum was dropped but the final number happened to match; a bitwise-only check
can fail on a correct recomputation. **Both MUST pass.**

Comparison assumes a **determinism level** ([16-open-questions.md](16-open-questions.md),
#82): integer and serialization work is compared bitwise, statistics within
tolerance, and GPU or parallel work records its conditions. See
[14-evaluation.md](14-evaluation.md).

## Execution boundaries at Stage 0

The PoC runs **in-process, and only trusted, self-written code**
([RFC 0002](rfc/0002-stage0-execution-contract.md), #23). The runtime talks to an
**execution interface**, so process isolation is added in Stage 1 without
rewriting the runtime. Stage 0 MUST NOT claim that process isolation or a VM
sandbox is provided, and untrusted or model-generated code MUST NOT run through
the trusted in-process path.

AIR and the manifest **express** preconditions and postconditions, but nothing
evaluates them at Stage 0 (#18); enforcement is a Stage 3 concern. Basic input
validation — rejecting a malformed CSV, refusing an unsupported statistic — is
still mandatory, because that is what makes the PoC correct at all.

## Success criteria

1. The initial Workspace is created from an AIR description with no AI.
2. The extend change is applied as a transaction and the Workspace reaches v2.
3. The swap passes **both** verification layers: numerical equivalence within
   tolerance and state integrity.
4. The loaded data and the existing panels are preserved across both changes.
5. A deliberately failing change rolls back and leaves the Workspace at its
   previous version.

## The four core tests

Stage 0 is not measured by how many features exist. It is measured by four tests
([16-open-questions.md](16-open-questions.md), #84):

1. Can Capability A run normally?
2. Can A's state be saved and migrated to B?
3. Did B restore the migrated state correctly?
4. On failure, is A's state preserved and recovered safely?

If these four pass, the core hypothesis is verified in substance.

## Related documents

- [05-adaptive-ir.md](05-adaptive-ir.md) — the description language.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — the change
  procedure.
- [14-evaluation.md](14-evaluation.md) — how the result is measured.
