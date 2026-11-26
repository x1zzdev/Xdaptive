# Xdaptive-process

A record of **how** Xdaptive's design was decided, kept separate from the
specification in [`docs/`](../docs/).

`docs/` states what the system is. This folder keeps the discussions, the
rejected options, and the troubleshooting that led there, so a later reader can
see not only the decision but the argument. It is a lab notebook, not a contract:
where it disagrees with `docs/`, the specification wins.

## What goes here

- **`discussions/`** — design discussions: the options weighed, the reasoning,
  the rebuttals, and the outcome. One file per round, named by date.
- **`troubleshooting/`** — problems hit while building, what was tried, and how
  they were resolved. This is where a fix's reasoning lives when it does not
  belong in the code or the specification.

## What does not go here

- Normative rules. Those go in `docs/` (spec-first).
- Task tracking. That is `NEXT.md` (local only).
- Contested decisions that need a durable, versioned record with a status. Those
  become an RFC in [`docs/rfc/`](../docs/rfc/); a discussion here can feed an RFC.

## Conventions

- One file per topic, named `YYYY-MM-DD-<topic>.md`.
- Record the reasoning and the alternatives, not just the conclusion.
- Link to the ledger entry (`docs/16-open-questions.md`, `#N`) or the owning
  document where a decision landed.
- No emojis. English for shared records; a Korean discussion may be kept as-is
  when it is the primary record.
