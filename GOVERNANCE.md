# Governance

## The model

Xdaptive uses a maintainer model. A small group of maintainers holds merge
rights and makes the final call on what enters the specification and the
reference implementation. The model is intentionally simple while the project is
pre-1.0 and pre-MVP. It can change when the contributor base is large enough to
need it.

**The project starts with a single maintainer** (#6). Merge rights are granted to
someone who understands the core architecture and has shown real contribution —
not by count, but by judgment. Until then, the project deliberately stays small;
growing the team faster than the architecture can be understood makes review
weaker, not stronger (#61).

## Who decides what

- **Routine changes** (bug fixes, documentation, tests, small implementation
  work) are merged by any maintainer after review.
- **Specification changes** (a new IR construct, a manifest field, a security
  rule, a Workspace behaviour) need review from at least one maintainer who is
  not the author, and must update the authoritative document before or with the
  implementation. A contested specification change becomes an RFC in
  `docs/rfc/`.
- **RFCs are approved by the maintainers** (#9). An RFC that touches security,
  the module ABI, or compatibility requires **additional review** beyond the
  normal one, because those decisions are the hardest to reverse.
- **Changes to the philosophy, the security model or the license** need
  consensus among the maintainers.

## The constitution

[docs/15-principles.md](docs/15-principles.md) and
[docs/01-vision.md](docs/01-vision.md) are the closest thing this project has to
a constitution. Every change is measured against the ten design principles. When
a decision is contested and the documents do not settle it, the principles break
the tie.

## Becoming a maintainer

There is no application process. A contributor who has landed several
substantial changes, shows good judgment in review, and understands that
rejecting a change is sometimes the right answer may be invited by the existing
maintainers. The invitation is about trust and sustained involvement, not about
a contribution count.

## Conflicts of interest

A maintainer with a financial or personal stake in a decision should say so and
step back from the final call. Reviewers are expected to judge a change against
the specification, not against the author or the organization behind it.

## Research, papers and patents

Competitions and papers are pursued actively (#7). **Patents are considered only
for technology whose novelty is concretely established**, and patentability is
reviewed **before** public disclosure. A public commit is a disclosure, so the
order matters.

## Communication

The default channel is **GitHub Discussions** (#63). Real-time chat (Discord) is
introduced only when collaborative work actually needs it; Slack is considered
too heavy for an early open-source project.

## Changing this document

Governance changes are proposed as a pull request to this file and require
consensus among the maintainers. The reasoning goes in the pull request
description, because the history of the decision is part of the project record.
