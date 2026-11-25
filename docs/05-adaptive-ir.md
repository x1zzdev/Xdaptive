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

## Minimal form (Stage 0)

`Open question:` how far AIR must express structure and flow
([16-open-questions.md](16-open-questions.md), item 2). The project deliberately
avoids designing a large DSL up front. The first version supports only what the
[PoC](13-poc.md) needs, and grows by RFC.

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
