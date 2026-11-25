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

## Minimal form (Stage 0)

**The Stage 0 AIR expresses only what the [PoC](13-poc.md) needs, and grows by
RFC** ([16-open-questions.md](16-open-questions.md), #2). The project
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
