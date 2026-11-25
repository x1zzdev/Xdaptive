# RFCs

A **Request for Comments** is how Xdaptive records a design decision that the
existing documents do not settle.

An RFC is used when:

- a decision is contested, or maintainers disagree;
- a change touches AIR, the manifest schema, the security model, or a design
  principle;
- a decision would otherwise live only in a chat thread or a commit message.

An RFC is **not** used for a routine fix. Fix the owning document and move on.

## Process

The full process is [0001-rfc-process.md](0001-rfc-process.md). In short:

1. Open an issue labelled `rfc` describing the problem.
2. Copy [0000-template.md](0000-template.md) to `NNNN-title.md` (the next free
   number, four digits, lowercase-hyphenated).
3. Open a pull request. Discussion happens on the pull request.
4. The RFC reaches one of: **accepted**, **rejected**, or **withdrawn**.
5. If accepted, the decision is merged into the owning document, and the RFC
   stays here as the record.

## Statuses

- `draft` — being written; not yet open for a decision.
- `proposed` — open for discussion.
- `accepted` — the decision is made and has been merged into the owning document.
- `rejected` — the decision is made against it; the reasoning stays here.
- `withdrawn` — the author stopped pursuing it.

## Index

| RFC | Title | Status |
|---|---|---|
| [0001](0001-rfc-process.md) | The RFC process | accepted |
