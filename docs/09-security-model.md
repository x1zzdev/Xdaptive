# 09 · Security model

An operating system that composes, generates and mutates running software must
be secure by construction. The central rule of the project is stated here and is
repeated in [15-principles.md](15-principles.md): **a model proposes, the OS
disposes.**

## The AI/OS split

A model produces probabilistic output. It MUST NOT be executed directly, and it
MUST NOT be given kernel authority.

### What a model does

- understand intent
- decompose the task
- search for needed Capabilities
- propose a composition plan
- generate AIR
- generate code
- propose functional changes
- interpret analysis results

### What the OS does

- validate AIR
- type check
- check dependencies
- check permissions
- check resources
- build the execution plan
- run it sandboxed
- apply the change
- roll back on failure
- preserve state

The model proposes **what** to do; the OS decides whether it can be done safely
and efficiently.

## Hard invariants

These MUST hold. A change that breaks one is rejected, regardless of benefit.

1. No model output reaches the kernel, the filesystem, the network or user data
   without passing validation ([05-adaptive-ir.md](05-adaptive-ir.md)) and the
   transaction boundary ([10-adaptation-transactions.md](10-adaptation-transactions.md)).
2. A model-generated Capability MUST NOT touch user data until it has passed
   verification and testing.
3. A Capability or Workspace MUST NOT receive permissions beyond those its
   manifest declared.
4. An execution module MUST NOT escape its isolation boundary.
5. A change that fails validation MUST NOT commit; the Workspace MUST remain at
   its previous consistent version.

## No origin-based trust

Trust is never decided by where code came from ([16-open-questions.md](16-open-questions.md),
#5, #10).

- A model-generated Capability is not automatically unsafe; it MAY be promoted to
  native execution once it has been analysed and proven safe.
- Bytecode is not automatically safe. The runtime MUST still verify memory
  access, syscalls and resource use, and the VM is part of the attack surface.

Model output therefore passes a **separate validation layer** before it has any
effect. The mechanism is still open (#10); the rule is not.

## Data locality

Personal or sensitive system data MUST NOT be sent to a remote model by default.
Using a remote model requires permission and policy, and the OS MUST keep working
without one. See [02-architecture.md](02-architecture.md).

## Permission-based execution

A model-generated program MUST NOT be able to reach the whole user system. Each
Capability and each Workspace is granted only the permissions it needs, declared
in its manifest and confirmed by the user where the permission is sensitive. The
permission check is part of AIR validation, before execution.

Permissions are **fine-grained** (#33) and least-privilege:

- file read, write, create and delete, and directory scope — not a single "file
  access" flag;
- network destination and direction — not a single "network access" flag;
- process and GPU access, each declared separately.

Permissions are requested **before execution** and re-approved when they change
(#34). To reduce approval fatigue, a **per-Workspace policy** MAY pre-authorise a
set of permissions; the policy is itself an explicit, reviewable object, not a
silent default.

## Integrity

A registry entry is verified by **content hash plus signature** (#35). Sigstore
and similar infrastructure are considered when an external registry is added
(#35, later); hash pinning applies from the start.

## Secrets

Secrets are **not stored inside a Capability** (#37). The runtime holds them in
its secret store and passes the Capability only a reference and an access grant.
A Capability MUST NOT be able to read a secret it was not granted.

## Audit

An **audit log** is a core requirement (#38). It records who ran which
Capability, when, and with which permissions, and it is detailed enough to
**reconstruct a run** after a problem — not merely a list of events. Traceability
is the goal, not log volume.

## Privacy

Prompt content and personal data MUST NOT be logged (#44). Only minimal metadata
— usage, execution time, the selected model — is recorded. This complements the
data-locality rule above.

## Isolation

Execution modules MUST run in isolated environments. The starting mechanism is
Linux `namespaces`, `seccomp` and `cgroups`. Isolation is the second line of
defence: it contains a module whose validation passed but whose behaviour is
still wrong.

## Verified change

Every change passes, in order:

- structural validation
- type checking
- dependency checking
- permission checking
- resource checking
- test execution
- apply, or roll back

This is the same list as the AIR validation steps plus the test-and-apply stage;
[10-adaptation-transactions.md](10-adaptation-transactions.md) defines the
transaction that carries it out.

Verification of generated or reconstructed code is **static analysis plus tests
plus sandbox execution** (#36). Formal verification is introduced in stages, and
only for core security modules where its cost is justified — not for every
Capability.

## State recovery

If a change fails, the Workspace MUST return to its previous consistent state.
The mechanisms are snapshots, checkpoints, change logs and state migration; see
[10-adaptation-transactions.md](10-adaptation-transactions.md) and
[03-workspace.md](03-workspace.md).

## Storage

Copying all data on every change is inefficient. The project combines immutable
objects, content-addressed storage, change logs, incremental snapshots and
checkpoints.

## What counts as a security problem

The security policy in [SECURITY.md](../SECURITY.md) lists the report classes
that matter. In short: escaping the AI/OS split, privilege escalation through a
Capability, breaking isolation, corrupting a Workspace, and supply-chain problems
in the registry. A model producing a wrong plan is expected behaviour, not a
vulnerability; the OS rejecting it is the system working.

## Related documents

- [05-adaptive-ir.md](05-adaptive-ir.md) — the validation AIR must pass.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — the commit and
  rollback boundary.
- [11-performance.md](11-performance.md) — how the fast path keeps this model
  intact.
