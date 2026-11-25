---
rfc: 0001
title: The RFC process
status: accepted
author: x1zz
created: 2026-11-25
owning-document: docs/README.md
---

# RFC 0001 · The RFC process

## Summary

This RFC defines how Xdaptive records contested design decisions: a numbered
document under `docs/rfc/`, discussed on a pull request, and merged into the
owning specification document when accepted.

## Motivation

Xdaptive is specification-first. Most changes are routine and belong in a normal
pull request to the owning document. But some decisions — the shape of AIR, the
manifest schema, the security model, the principles — are contested, and a
decision recorded only in a commit message or a chat thread is lost. The project
needs one place where such decisions, and the reasoning against them, survive.

## Guide-level explanation

A contributor who wants to change a contested decision:

1. Opens an issue labelled `rfc` stating the problem.
2. Copies [`0000-template.md`](0000-template.md) to the next free number, e.g.
   `0002-capability-as-atom.md`, and sets `status: draft`.
3. Opens a pull request. Sets `status: proposed` when it is ready for a decision.
4. Responds to review. Maintainers decide per [GOVERNANCE.md](../../GOVERNANCE.md).
5. On acceptance, updates the owning document in the same pull request and sets
   `status: accepted`. The RFC file stays as the record.

## Reference-level explanation

- An RFC file MUST be named `NNNN-title.md`, four digits, lowercase, hyphenated.
- An RFC MUST carry the YAML front matter in the template.
- An RFC MUST name its `owning-document`.
- An RFC that is `accepted` MUST have updated its owning document in the same
  pull request; an RFC is not accepted while the documents still disagree with
  it.
- A `rejected` or `withdrawn` RFC MUST NOT be deleted; the reasoning is the
  value.
- An RFC MUST NOT change a design principle or the security model without
  maintainer consensus ([GOVERNANCE.md](../../GOVERNANCE.md)).
- RFCs are approved by the **maintainers**. An RFC that touches **security, the
  module ABI, or compatibility** requires additional review beyond the normal
  one, because those decisions are the hardest to reverse
  ([16-open-questions.md](../16-open-questions.md), #9).
- The RFC index in [`README.md`](README.md) MUST list every RFC with its status.

## Drawbacks

The process adds ceremony. For a small project, most decisions do not need it.
The mitigation is the rule that an RFC is only for contested decisions; a routine
change stays a normal pull request.

## Rationale and alternatives

- **Decisions in issues only.** Rejected: issues are for work, not for the
  durable record of a design choice, and they close.
- **Decisions in commit messages only.** Rejected: the reasoning against a
  rejected option does not survive.
- **A wiki.** Rejected: not versioned with the specification.

## Unresolved questions

None. The process can itself be changed by a later RFC.

## What this updates

Adds `docs/rfc/` to the specification set and links it from
[docs/README.md](../README.md).
