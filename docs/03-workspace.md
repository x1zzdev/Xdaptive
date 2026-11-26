# 03 · Workspace

A **Workspace** is the persistent, integrated execution environment for one
task. It is not a screen and not a container that holds several programs. It is
an independent execution environment and the unit of state management.

The user sees one Workspace. Underneath, several engines, services, processes and
modules MAY run independently; the Workspace manages them as one.

## Model

A Workspace has six parts. They version independently, so changing the interface
does not force a new version of the data.

### Identity

- a unique ID
- the goal of the user's task
- creation time
- the current version
- the task's state

### Software configuration

- the engines and services attached
- the Capabilities that are enabled
- each component's configuration
- the connections between pieces of software

### Workflow state

- the current step
- dependencies between steps
- execution status and progress
- failure and retry information

### Interface state

- layout, panels and tool placement
- user preferences
- the interface version

### Data state

- input data, intermediate results and final results
- data references and change history

### Version history

- the Workspace's change history
- restore points
- composition-change records
- checkpoints of execution state

## Lifecycle

A Workspace is an independent object the OS manages:

1. **Create** — make the task environment.
2. **Initialize** — configure the needed Capabilities and engines.
3. **Execute** — run the task.
4. **Extend** — add functionality.
5. **Adapt** — change structure and interface.
6. **Persist** — save state.
7. **Restore** — recover a previous state.
8. **Clone** — duplicate the environment.
9. **Terminate** — end the environment.

## State versus running memory

A Workspace's **saved state** and a running process's **memory state** are
different things. Saving configuration and a task specification does not
automatically restore every process's execution state. A Workspace MUST therefore
manage, per engine: checkpoints, execution logs, and re-runnable task
specifications. What cannot be restored MUST be re-derivable or explicitly
reported as lost.

## Independent versioning

Software configuration, workflow, interface and data are versioned separately. A
UI change SHOULD NOT create a new data version, and a data change SHOULD NOT
force a new interface version. A Workspace version is a tuple of these parts,
not a single counter. See [10-adaptation-transactions.md](10-adaptation-transactions.md)
for how a change is applied across them.

## Persistence strategy

Copying all data on every change is not acceptable. The storage is **SQLite plus
content-addressed storage** (#27), separating metadata from large content, and it
follows a Git-like **immutable manifest** model (#93):

- **Content addressing uses BLAKE3** (#91). The algorithm is recorded in the
  manifest, an object id includes its algorithm, algorithms are never mixed in
  one id space, and a hash match proves content identity only — **never
  provenance or trust**.
- **A Workspace version is a manifest of hashes.** The Workspace points at a
  manifest hash; the manifest lists the hashes of its Capabilities, AIR source,
  configuration and dependencies. A change writes new blobs, builds a new
  manifest, and **atomically swaps the pointer**; the previous version is
  untouched. Rollback references the previous manifest.
- **The version number and the version identifier are separate**: the manifest
  hash is the content's identity, a revision number is the change order, and a
  parent hash links to the previous version. Two identical contents share a
  manifest hash, and different Workspaces MAY share CAS objects.
- **Snapshots are copy-on-write** (#28): **reflink where the filesystem supports
  it, logical COW otherwise** (#92). Both backends MUST give the same snapshot
  semantics; a reflink failure falls back safely to logical COW, and an existing
  snapshot stays immutable.
- **GC is reachability-based**, not reference-count-only: unreachable blobs are
  reclaimed by tracing from the Workspace roots.
- **A metadata store (SQLite)** indexes IDs, versions and references.
- **Checkpoints are selective and per-engine** (#29). Where an engine can provide
  one, it is recorded; where it cannot, a re-run specification is recorded; where
  neither preserves required state, the change is rejected
  ([07-execution-model.md](07-execution-model.md)).

The storage layout MUST make the guarantee in principle 7 testable: after a
change, the Workspace's task state is preserved and recoverable.

## Cloning and concurrency

- **Clone** shares immutable data by default and copies on write (COW) (#30). A
  real copy happens only when required.
- A Workspace MUST NOT share mutable state with another unless a Capability
  declares it.
- **Multiple concurrent Workspaces** are a long-term goal, with policy-based
  resource priority (#31). Early on, the system is **single-user and local**
  (#32); multi-user and remote Workspaces arrive after the security model
  stabilises.

## Guarantees

- A Workspace change MUST NOT silently destroy task state.
- A failed change MUST leave the Workspace at its previous consistent version.
- A Workspace MUST be reconstructable from its recorded history alone.
- Two Workspaces MUST NOT share mutable state unless a Capability declares it.

## Related documents

- [02-architecture.md](02-architecture.md) — where the Workspace runtime sits.
- [04-capability-model.md](04-capability-model.md) — what a Workspace is composed
  of.
- [05-adaptive-ir.md](05-adaptive-ir.md) — how composition and change are
  expressed.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — how a change
  is applied safely.
