# The Xdaptive specification

This directory is the **contract**. The implementation is judged against it; a
change that the specification does not describe is not merged. When the
implementation and a document here disagree, the document is right until the
document is changed on purpose.

## How to read it

Start with the [glossary](00-glossary.md) — every other document depends on its
vocabulary — then read in order. Each document owns one area and links to the
others rather than repeating them.

| Document | Owns |
|---|---|
| [00-glossary.md](00-glossary.md) | The vocabulary |
| [01-vision.md](01-vision.md) | Identity, thesis, differentiation |
| [02-architecture.md](02-architecture.md) | The six layers and the internal data model |
| [03-workspace.md](03-workspace.md) | Workspace model, lifecycle, persistence |
| [04-capability-model.md](04-capability-model.md) | Capability and Software Manifest |
| [05-adaptive-ir.md](05-adaptive-ir.md) | AIR — the intermediate representation |
| [06-adaptive-compiler.md](06-adaptive-compiler.md) | The compiler pipeline |
| [07-execution-model.md](07-execution-model.md) | Adaptation levels and runtime mutation |
| [08-software-understanding.md](08-software-understanding.md) | Analysis and reconstruction |
| [09-security-model.md](09-security-model.md) | The AI/OS split, permissions, isolation |
| [10-adaptation-transactions.md](10-adaptation-transactions.md) | Safe, atomic change |
| [11-performance.md](11-performance.md) | Native paths and incremental work |
| [12-roadmap.md](12-roadmap.md) | Stages 0–6 |
| [13-poc.md](13-poc.md) | The first proof of concept |
| [14-evaluation.md](14-evaluation.md) | What gets measured |
| [15-principles.md](15-principles.md) | The ten design principles |
| [16-open-questions.md](16-open-questions.md) | Decisions still open |

[vision.ko.md](vision.ko.md) is the original Korean master vision. It is the
source this specification was derived from; when a decision changes, update the
English document **and** the master in the same change.

## Normative language

The words **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT** and **MAY** are
used as defined in [RFC 2119][rfc2119]. A sentence without one of these words is
descriptive context, not a requirement.

An **open question** is marked inline as `Open question:` and listed in
[16-open-questions.md](16-open-questions.md). An unresolved open question is not
a bug; it is a decision the project has deliberately deferred until it can be
settled by a test.

## Language and versioning

The specification is **English**, with the original **Korean master vision** at
[vision.ko.md](vision.ko.md) (#8). Other translations (Japanese, Chinese) are
added once outside contributors appear.

The specification has no version number of its own yet; the project starts at
**0.1.0** and reaches **1.0.0** when the core architecture and public API
stabilise (#10). Until then, a minor release MAY break compatibility, and the
changelog records it.

## Changing the specification

1. A change to an owning document is a normal pull request.
2. A contested change — one where the documents do not settle the question, or
   where maintainers disagree — becomes an RFC under [`rfc/`](rfc/).
3. An RFC that is accepted is merged into the owning document and kept in
   `rfc/` as the record of the decision.

See [`rfc/0001-rfc-process.md`](rfc/0001-rfc-process.md) for the process.

[rfc2119]: https://www.rfc-editor.org/rfc/rfc2119
