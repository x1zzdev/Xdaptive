# 08 · Software understanding and reconstruction

To adapt software the OS did not build, it must first understand it. This
document describes the analysis layer that extends the OS's understanding of
software beyond what official APIs expose.

## Why analysis is part of the project

Relying only on a program's official API or extension mechanism limits what the
OS can use. Analysing a program's structure and behaviour lets the OS understand
software with no available source, reason about its functionality, and attempt a
compatible implementation. This extends the OS's **software understanding**, and
it is one of the project's important technical directions.

## What analysis is not

Analysing software is not the same as making it open source, and it is not the
project's defining idea. The following are **different problems** and MUST NOT be
conflated:

- binary analysis
- understanding behaviour
- recovering source code
- producing a compatible implementation
- the right to redistribute

Being able to analyse something does not grant a right to copy or redistribute
its code.

## The four layers

### 1 · Software Analyzer

Analyses structure and behaviour:

- binary analysis
- function and control-flow analysis
- data-structure inference
- UI and user-interaction analysis
- IPC and network behaviour
- dependency analysis

### 2 · Semantic Reconstruction

Turns analysis output into a model the OS can reason about:

- the purpose of a function
- its inputs and outputs
- its data structures
- dependencies between functions
- execution conditions
- state changes
- external interfaces

### 3 · Adaptation Generator

Turns a reconstructed model into something usable:

- connect an existing function
- generate a compatible implementation
- independently implement a needed function
- compose an interface and workflow
- verify each function

### 4 · Adaptive Composition Engine

Combines verified functions into a Workspace. See
[04-capability-model.md](04-capability-model.md).

## From analysis to a manifest

`Open question:` the form in which analysis output becomes a Software Manifest
([16-open-questions.md](16-open-questions.md), item 9). The working direction:
Semantic Reconstruction produces the manifest's Capability list, and each
Capability's declared type and permissions are inferred where possible and marked
as inferred where not. An inferred manifest MUST be distinguishable from a
declared one, and a Capability with inferred types MUST be verified before it
runs against user data.

## Limits

The project MUST account for these limits; it MUST NOT claim analysis is a
general solution:

- an optimised binary may lose the original source structure;
- obfuscated or encrypted programs may resist analysis;
- server-side logic cannot be understood from the client alone;
- analysing a large program fully costs a lot of time and resources;
- a wrong analysis can mean a function is misunderstood or wrongly reimplemented;
- licences and applicable law vary in what analysis and reimplementation allow.

Analysis is therefore one core technique for extending the OS's understanding,
not a universal tool for transforming any software.

## Legal and ethical rule

Before the OS analyses or reimplements a piece of software, it MUST record the
software's licence and the applicable constraints, and it MUST NOT reimplement in
a way that the licence forbids. This is a hard rule, not a preference.

## Related documents

- [04-capability-model.md](04-capability-model.md) — the manifest a reconstruction
  feeds.
- [09-security-model.md](09-security-model.md) — how an inferred Capability is
  verified before it runs.
