# Design round 4 — resolving the storage, ABI, presence and analysis questions

**Date:** 2026-11-26
**Ledger:** #86–#94
**Recorded in:** `docs/03`, `docs/05`, `docs/06`, `docs/07`, `docs/08`

## Context

The open items from round 3 were made concrete and answered: presence syntax and
its ABI encoding, the ABI compatibility model, the cancellation and compensation
protocols, the storage layout, and the fidelity of analysed software.

## Decisions

- **Presence syntax (#86).** `name?: T`, on fields only. `Option<T>` is value
  optionality; a missing required field is an error by default; a default applies
  only when declared.
- **Presence in the ABI (#87).** A record is schema id + presence bitmap + field
  values. `None` is a separate value tag; empty and absent are distinct; presence
  and `Option` stay separate end to end.
- **ABI compatibility (#88).** The module declares a required and supported
  range; the runtime validates. A different major is incompatible by default;
  incompatible modules are rejected, not converted; an adapter is a separate
  path; ABI versions may coexist in a Workspace.
- **Cancellation (#89).** Cooperative: a runtime token plus Capability-declared
  safe points. `await` is not automatically safe; cancellation is not rollback; an
  unresponsive Capability is bounded by a grace period.
- **Compensation (#90).** The Capability owns the semantics, the runtime owns the
  lifecycle (order, execution, tracking, recovery). Compensation is distinct from
  atomic rollback.
- **CAS hash (#91).** BLAKE3. The algorithm is in the manifest and in the object
  id; a hash match is identity, never provenance.
- **COW (#92).** Reflink first, logical COW fallback, identical snapshot
  semantics across backends.
- **Version model (#93).** A Workspace version is a manifest of hashes, committed
  by an atomic pointer swap. Version number (order) and identifier (hash) are
  separate; GC is reachability-based.
- **Analysis fidelity (#94).** Proxy → wrapper → reimplementation →
  differentially verified. Reimplementation is a promotion, not a replacement.

## Notes

- The BLAKE3 choice **changed from an earlier SHA-256 preference**: the CAS is
  the basis of versioning, manifests, snapshots, COW and GC, so throughput and
  parallelism matter more than the widest external familiarity. The algorithm is
  recorded per object, so migration stays possible.
- The presence model keeps **two absences** distinct all the way from AIR to the
  ABI. Merging them at any layer would lose the meaning that motivated the split.

## Still open

- The concrete AIR grammar (RFC 0003) and the ABI wire format (RFC 0004).
- The SQLite schema and the exact CAS directory layout.
- Per-operation tolerances, per-representation integrity and error accumulation.
- The static/runtime contract-checking split (Stage 3).
