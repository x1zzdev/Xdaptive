# 05 · Adaptive IR (AIR)

**AIR** is the intermediate representation in which software composition and
change are expressed, verified and executed. It is the project's core
representation and the boundary between "something proposed" and "something the
OS will run".

## Why AIR exists

A model's natural-language plan or a reverse-engineering result MUST NOT be
executed directly. It is translated into AIR, checked, and only then lowered to
an execution structure. AIR is what makes the AI/OS split concrete: the model
produces AIR (or something that becomes AIR), and the OS owns everything after
that. See [09-security-model.md](09-security-model.md).

AIR is not a configuration file. It is connected to the OS's composition and
execution model, and it is the input to the [Adaptive
Compiler](06-adaptive-compiler.md).

## What AIR expresses

- software components
- activation and deactivation of Capabilities
- connections between Capabilities
- data flow
- workflow
- interface composition
- state preservation and migration
- permissions and execution conditions
- resource requirements

## Nature: a declarative DSL

AIR is a **declarative DSL** ([16-open-questions.md](16-open-questions.md), #13).
It expresses structure and relationships — components, connections, flow,
interface, permissions — and it MUST NOT be a language for writing executable
logic. Behaviour belongs to a Capability's implementation, not to AIR.

## Control flow

AIR has **declarative connections and bounded conditional gates**, and **no
general iteration** ([16-open-questions.md](16-open-questions.md), #72):

- A **connection** says which Capability's output feeds another's input.
- A **conditional gate** (`if` / `when`) says under which condition a Capability
  runs.
- **Iteration is not expressed in AIR.** Repetition is the job of a Capability
  implementation or a streaming engine, which delivers events that the
  connections and gates react to.

A gate's condition MUST be **typed and structural**, not an arbitrary expression
that runs code. A complex decision is a Capability, not a condition. This keeps
AIR from growing into a general-purpose programming language and keeps the number
of variables the core hypothesis must test small. Complex branching and workflow
orchestration are deferred to a future RFC.

## Types

AIR has its **own type system** (#14), with numeric, string, table and tensor
types provided as base types. Two rules govern compatibility:

- **Nominal typing is the default** (#15). Compatibility is decided by name and
  explicit declaration, not by shape alone. Two Capabilities may both accept a
  number, but one may mean a temperature and the other a voltage; equal structure
  is not equal meaning. Structural compatibility is an explicit opt-in.
- **Compatibility is more than type** (#18). A connection is valid only if the
  type, the semantic contract, the preconditions and the postconditions all
  agree. A connection that passes on type alone is not proven safe.

### Base types and namespaces

Stage 0 defines a base type set — integer, float, boolean, string, table and
tensor (#80). **Primitives are global** (`Int`, `Float`, `String`, …); only
**user-defined and Capability-defined types are namespaced** (`csv::Row`,
`statistics::Result`). Namespacing the primitives (`core::Int`) would put a
prefix on every expression for no benefit. Namespaces exist because Capabilities
from different sources may define the same name; a connection between two
Capabilities checks compatibility explicitly rather than relying on a name match.

### Nullable and missing

AIR separates two absences that a single `null` would merge (#80):

- **Nullable** — the value may be absent: `Option<T>`, a **value type**.
- **Missing** — the field itself may be omitted: **field presence**, a
  **record/schema-level** property, not a value type.

```text
Student {
    score:  Option<Float>,   // required field, value may be absent
    comment?: String,        // the field itself may be omitted
}
```

A recorded-but-empty score and a score that was never entered are different
facts, so the two are kept apart. But presence belongs to the record, not the
value: `Presence<T>` is **not** a value type. The presence syntax is
**`name?: T`** ([16-open-questions.md](16-open-questions.md), #86):

- `?` applies **only to a field declaration**, never to a value.
- `Option<T>` expresses value optionality; `name?` expresses field presence.
- A missing required field is an **error by default**.
- A default applies **only when it is declared explicitly**.

`Option<T>` is a **built-in type constructor**, not a user generic. General user
generics are deferred (#80 follow-up); a built-in `Option` does not require them.

### Data passing

A connection carries data in one of three modes (#16):

- **value** — small data is passed by value;
- **reference** — shared data is passed by reference;
- **streaming** — large data is streamed.

Copying GB-scale data is prohibited: the compiler MUST choose reference or
streaming for large payloads.

### Schema versioning

An AIR document carries an **explicit schema version** (#17). The compiler MUST
be able to read a previous version and migrate it forward; a document with an
unknown version is rejected rather than guessed at.

### Dimensional and unit types

Dimensional and unit types are **deferred to a future RFC**
([16-open-questions.md](16-open-questions.md), #73). Stage 0 uses primitive and
structural types only: integer, float, boolean, string, table and tensor. The
type system MUST remain extensible enough to add dimensions, unit conversion,
dimensional arithmetic and compile-time dimensional checking later. Unit
correctness is not abandoned; it is studied separately, so it does not complicate
the PoC.

## Permissions and resources

AIR **declares** the permissions and resource requirements a composition needs
(#19). It does not grant them. The runtime grants and enforces them
([09-security-model.md](09-security-model.md)), and a composition that uses a
permission it did not declare MUST fail validation.

## Interface declaration

AIR declares **which panels exist and how they relate to Capabilities** (#52). It
does not describe layout, animation or interaction logic; those belong to the UI
Composition Layer ([02-architecture.md](02-architecture.md)). AIR stays small by
not absorbing the presentation layer.

## Contracts

AIR and the manifest **express** preconditions and postconditions; nothing
evaluates them at Stage 0 ([RFC 0002](rfc/0002-stage0-execution-contract.md),
#18). Stage 0 fixes only:

- the distinction between a precondition and a postcondition;
- that a contract belongs to a Capability;
- that the representation is extensible;
- the direction of contract identification and versioning.

Contract evaluation and enforcement are deferred to Stage 3. Basic input
validation and runtime error handling are separate and remain mandatory: refusing
a malformed input is not contract enforcement, it is the minimum defence a
function needs to be correct.

## Minimal form (Stage 0)

**The Stage 0 AIR expresses only what the [PoC](13-poc.md) needs, and grows by
RFC** ([16-open-questions.md](16-open-questions.md), #65). The project
deliberately avoids designing a large DSL up front.

The initial constructs are:

- **workspace declaration** — a task environment with a goal
- **capability declaration** — bind a Capability, enable or disable it, configure
  it
- **data-flow connection** — connect one Capability's output to another's input
- **workflow connection** — declare ordering and conditions
- **interface composition** — declare panels and controls
- **permission declaration** — the permissions the composition requires
- **state migration declaration** — how existing state carries across a change

### Illustrative sketch

The following is a sketch to make the shape concrete. It is **not** a frozen
grammar; the grammar is settled by RFC before the parser is written.

```air
workspace "sales-analysis" {
    goal "Analyse sales.csv and show the data"

    capability csv.reader as reader { source = "sales.csv" }
    capability data.viewer as viewer { input = reader.rows }

    interface {
        panel "data" { viewer }
    }
}
```

A change that adds correlation analysis and a scatter plot, while keeping the
existing reader and viewer state:

```air
plan extend "sales-analysis" {
    add capability stats.correlation as corr { input = reader.rows }
    add capability chart.scatter as scatter { input = corr.result }

    interface {
        panel "correlation" { scatter }
    }

    // reader and viewer are untouched, so their state is preserved.
}
```

## Validation before execution

AIR MUST pass, in order:

1. **parse** — the document is well-formed;
2. **type check** — every connection is type-compatible
   ([04-capability-model.md](04-capability-model.md));
3. **dependency check** — every Capability and dependency resolves;
4. **permission check** — every declared permission is available and no
   undeclared permission is used;
5. **resource check** — the composition fits the Workspace's resource envelope.

Only after all five does the compiler lower AIR to an execution structure.

## Extension roadmap

Later versions add: runtime dynamic change, advanced optimisation, and
analysis-result ingestion (turning a
[Semantic Reconstruction](08-software-understanding.md) into AIR). Each addition
is an RFC, and each must keep the five validation steps total: every construct
MUST be checkable.

## Related documents

- [04-capability-model.md](04-capability-model.md) — the units AIR composes.
- [06-adaptive-compiler.md](06-adaptive-compiler.md) — what consumes AIR.
- [10-adaptation-transactions.md](10-adaptation-transactions.md) — how a change
  described in AIR is applied safely.
