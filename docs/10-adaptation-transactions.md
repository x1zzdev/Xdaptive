# 10 · Transactional adaptation

Changing a Workspace in place is dangerous: a failure partway through can lose
data or leave the environment unusable. Xdaptive therefore applies every change
as a **transaction**, so the Workspace is never observed in a half-changed
state.

## The procedure

1. **Snapshot** — preserve the current Workspace state.
2. **Plan** — produce the change plan (an AIR `plan`; see
   [05-adaptive-ir.md](05-adaptive-ir.md)).
3. **Validate** — check types, dependencies, permissions and compatibility.
4. **Stage** — build the change in an environment separate from the running one.
5. **Test** — verify the changed environment.
6. **Commit** — apply the successful change.
7. **Rollback** — on failure, restore the previous state.

Steps 1–5 MUST NOT affect the running Workspace. The user keeps working while the
change is prepared beside them. Only step 6 changes what the user sees.

## Atomicity across the four states

A Workspace versions its software configuration, workflow, interface and data
separately ([03-workspace.md](03-workspace.md)). A transaction MAY change more
than one of them, but the commit MUST be atomic across the parts it changes: the
user MUST NOT observe a new interface against old data, or new software against
an old workflow.

## What rollback must restore

Rollback MUST restore:

- the software configuration
- the workflow state
- the interface state
- the data state, to the extent the change touched it

If a change cannot be rolled back completely, it MUST be rejected in step 3, not
attempted.

## State migration

A change may need existing state to cross from the old composition to the new one
([07-execution-model.md](07-execution-model.md)). The transaction follows the
three-step policy there — migrate, re-derive, or reject — and MUST additionally:

- verify, after staging, that migrated state matches the pre-change state where
  the two are comparable;
- keep the old Capability alive until the new state is verified, so a failure
  returns to the existing state rather than an already-torn-down one;
- ensure a data change made during migration is not applied twice.

This is the **atomic transition**. Not every change can be atomic — a change that
touches an external system or an irreversible operation is out of scope for this
transaction and MUST be rejected if it cannot be made atomic.

## Storage

The transaction relies on the persistence mechanisms in
[03-workspace.md](03-workspace.md): immutable content-addressed objects for data,
a change log for composition history, and checkpoints for execution state. The
snapshot in step 1 and the restore in step 7 are the same mechanism seen from two
directions.

The transaction's shape is settled (snapshot through rollback, atomic across the
parts it touches); the concrete storage and transaction model is still open
([16-open-questions.md](16-open-questions.md), #67).

## Guarantees

- A committed change is atomic across the parts it touches.
- A failed change leaves the Workspace at its previous consistent version.
- No change is committed without passing test in step 5.
- A change that cannot preserve required state is rejected before it is applied.

## Related documents

- [03-workspace.md](03-workspace.md) — the state the transaction protects.
- [05-adaptive-ir.md](05-adaptive-ir.md) — the plan the transaction applies.
- [09-security-model.md](09-security-model.md) — why the boundary is a hard rule.
