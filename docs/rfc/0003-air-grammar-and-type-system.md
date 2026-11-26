---
rfc: 0003
title: The Stage 0 AIR grammar and type system
status: accepted
author: x1zz
created: 2026-11-26
owning-document: docs/05-adaptive-ir.md
---

# RFC 0003 · The Stage 0 AIR grammar and type system

## Summary

This RFC freezes the **minimal AIR grammar** and the **AIR type system** for
Stage 0. [05-adaptive-ir.md](../05-adaptive-ir.md) already fixes the *decisions*
— AIR is a declarative DSL (#13), it has its own type system (#14), nominal
typing is the default (#15), data passes as value, reference or streaming (#16),
documents carry a schema version (#17), permissions and resources are declared
(#19), and contracts are representation-only (#18) — but it says the grammar "is
settled by RFC before the parser is written". This RFC settles it, so the parser
and the AST have one canonical shape to implement.

## Motivation

The parser, the type checker and the bytecode verifier all need the same
vocabulary. Without a frozen grammar, three implementations would each invent
their own spelling for a connection, a type reference or a gate, and the
specification would no longer be the contract. The sketch in
[05-adaptive-ir.md](../05-adaptive-ir.md) makes the shape concrete but is
explicitly "not a frozen grammar".

The grammar is also where the settled decisions become checkable. A decision such
as "nominal typing is the default" is only real if the grammar has a way to
declare a nominal type and to opt into structural compatibility; "data passing
has three modes" is only real if a connection can name its mode. This RFC closes
that gap for Stage 0.

## Guide-level explanation

An AIR document is a short declaration of **structure**, never of behaviour. It
starts with the schema version it was written against, then declares either a
**workspace** (a new task environment) or a **plan** (a change to an existing
one):

```air
air 0.1;

workspace "sales-analysis" {
    goal "Analyse sales.csv and show the data"

    capability csv.reader as reader { source = "sales.csv" }
    capability data.viewer as viewer

    connect reader.rows -> viewer.input

    interface {
        panel "data" { viewer }
    }
}
```

A change that adds correlation analysis and a scatter plot, keeping the existing
reader and viewer state:

```air
air 0.1;

plan extend "sales-analysis" {
    add capability stats.correlation as corr
    add capability chart.scatter as scatter

    connect reader.rows -> corr.input
    connect corr.result -> scatter.input

    interface {
        panel "correlation" { scatter }
    }
}
```

Four ideas carry the whole language:

- **A Capability is bound, not written.** `capability csv.reader as reader`
  binds a Capability from the registry to a local alias. AIR says which
  Capability, what it is configured with, and when it runs; it never says what
  the Capability does.
- **A connection is a port to port edge.** `connect reader.rows -> viewer.input`
  wires one Capability's output to another's input. The type of both ends is
  checked against the Capability's manifest, nominally by default.
- **A gate is a typed condition.** `capability data.viewer as viewer when
  reader.has_rows == true` says when a Capability runs. It is a single
  comparison over a port, not an expression that runs code.
- **A type is named, not shaped.** `type Temperature { celsius: Float }` is a
  nominal type: it is compatible with `Temperature` and nothing else, even if
  another type has the same fields. A `structural` declaration is the explicit
  opt-in to shape-based compatibility.

## Reference-level explanation

### 1 · Document and schema version (#17)

An AIR document MUST begin with the schema declaration:

```ebnf
document        = schema, declaration, EOF ;
schema          = "air", integer, ".", integer, ";" ;
```

The two integers are the **AIR schema version**, `major.minor`. It is the first
of the four versioned dimensions in the compatibility matrix
([04-capability-model.md](../04-capability-model.md), #85).

- A document whose **major** version the compiler does not know MUST be
  **rejected**; the compiler MUST NOT guess at an unknown grammar.
- A document with a known major and an **older minor** MUST be migrated forward
  by the compiler before validation.
- A document with a known major and a **newer minor** MUST be rejected, because
  it may use constructs the compiler cannot read.

Stage 0 fixes the version at `0.1`. The value is a document property; the
compiler's accepted range is a compatibility-matrix entry.

### 2 · Lexical elements

```ebnf
identifier      = letter, { letter | digit | "_" } ;
qualified-name  = identifier, { "::", identifier } ;
comment         = "//", { character - newline } ;
string          = '"', { escaped | character - '"' }, '"' ;
integer         = [ "-" ], digit, { digit } ;
float           = [ "-" ], digit, { digit }, ".", digit, { digit } ;
boolean         = "true" | "false" ;
literal         = string | float | integer | boolean ;
```

- A **qualified name** uses `::` for namespaces: `csv::Row`, `statistics::Result`.
- A **port reference** is `alias.port` — one dot, no namespace: `reader.rows`.
- Comments are `//` to end of line.
- Whitespace and comments are not significant except as token separators.
- A literal is a string, a float, an integer or a boolean. Stage 0 has no list or
  map literal; the representation is extensible.

### 3 · Grammar (Stage 0)

```ebnf
declaration     = workspace | plan ;

workspace       = "workspace", string, "{", { workspace-item }, "}" ;
workspace-item  = goal | type-decl | capability | connect
                | interface | permission | resource ;

goal            = "goal", string ;

capability      = "capability", qualified-name, "as", identifier,
                  [ body ], [ gate ] ;
body            = "{", { body-item, ";" }, "}" ;
body-item       = setting | precondition | postcondition ;
setting         = identifier, "=", literal ;
precondition    = "requires", assertion ;
postcondition   = "ensures", assertion ;
gate            = "when", assertion ;
assertion       = port-ref, comparator, literal ;
comparator      = "==" | "!=" | "<" | "<=" | ">" | ">=" ;
port-ref        = identifier, ".", identifier ;

connect         = "connect", port-ref, "->", port-ref, [ passing ] ;
passing         = "by", ( "value" | "reference" | "streaming" ) ;

interface       = "interface", "{", { panel }, "}" ;
panel           = "panel", string, "{", { identifier }, "}" ;

permission      = "permission", ( file-permission | net-permission ) ;
file-permission = "file", file-access, [ string ] ;
file-access     = "read" | "write" | "create" | "delete" ;
net-permission  = "network", net-direction, [ string ] ;
net-direction   = "in" | "out" | "inout" ;

resource        = "resource", resource-name, "=", number, [ unit ] ;
resource-name   = "cpu" | "memory" | "gpu" | "power" ;
number          = integer | float ;
unit            = "ms" | "s" | "MB" | "GB" | "%" | "W" ;

type-decl       = "type", qualified-name, [ "structural" ], record-body ;
record-body     = "{", { field }, "}" ;
field           = identifier, [ "?" ], ":", type-ref, [ "=", literal ], ";" ;
type-ref        = ( "Option", "<", type-ref, ">" ) | qualified-name ;

plan            = "plan", "extend", string, "{", { plan-item }, "}" ;
plan-item       = goal | type-decl | add | replace | connect
                | interface | permission | resource | migration ;
add             = "add", capability ;
replace         = "replace", identifier, "with", qualified-name,
                  [ body ], [ gate ] ;
migration       = "migrate", identifier, "->", identifier, [ "by", "re-derive" ] ;
```

Notes on the shape:

- **One canonical form.** A data-flow edge is always a `connect` statement; it is
  not also spelled as an inline `input =` setting. Settings hold scalar
  configuration only.
- **A gate belongs to a Capability.** Ordering is implied by data flow; an
  explicit control-only edge is not part of Stage 0.
- **A contract belongs to a Capability**, inside its body, as `requires` and
  `ensures`.
- **A `replace` keeps the alias.** It swaps the implementation behind an existing
  alias, which is what the swap scenario needs; it does not rename a port.
- **Removal is not in Stage 0.** `add` and `replace` cover the PoC's extend and
  swap; removing a Capability is deferred.

### 4 · Type system (#14, #15, #80, #86)

#### Base types

The base types are **global** — never namespaced:

| Type | Meaning |
|---|---|
| `Int` | a signed integer |
| `Float` | an IEEE 754 double-precision float |
| `Bool` | a boolean |
| `String` | a UTF-8 string |
| `Table` | columnar/tabular data; a **large** payload |
| `Tensor` | an n-dimensional numeric array; a **large** payload |

Namespacing the primitives (`core::Int`) would prefix every type reference for no
benefit, so primitives are global and only user-defined and Capability-defined
types are namespaced (#80).

#### Named types and nominal typing

A named type is declared with `type` and is **nominal by default** (#15):

```air
type csv::Row {
    label: String;
    value: Float;
    comment?: String;
}
```

- A nominal type `N` is compatible **only** with `N` (the same fully-qualified
  name). Two types with identical fields and different names are **not**
  compatible: one may mean a temperature and the other a voltage.
- A Capability's declared input and output types are nominal in the same sense.
- Compatibility is checked from **both** endpoints of a connection: a connection
  is valid only if each endpoint's type accepts the other. A nominal type accepts
  only its own name.

#### Structural opt-in

Structural compatibility is an **explicit opt-in**, declared with the
`structural` keyword:

```air
type Point structural {
    x: Float;
    y: Float;
}
```

- A `structural` type is compatible with any record type whose structure is
  identical: the same field names, in the same order, with the same types, the
  same presence and the same defaults.
- Because compatibility is symmetric, connecting a nominal type to a structural
  type fails unless the nominal type is itself the structural type. A connection
  between two shapes requires the opt-in on both sides.
- Stage 0 fixes the syntax and the rule; a richer structural-subtyping relation
  (width, variance) is deferred.

#### Nullable versus missing (#80, #86)

Two absences are kept apart at every layer:

- **Nullable** — a **value** may be absent: `Option<T>`, a **value type**.
  `Option` is a built-in type constructor, not a user generic.
- **Missing** — a **field** may be omitted: `name?: T`, a **record-level**
  property, not a value type.

```air
type Student {
    score: Option<Float>;   // required field, value may be absent
    comment?: String;       // the field itself may be omitted
}
```

- `?` applies only to a field declaration, never to a value.
- A missing required field (no `?` and no default) is an **error by default**.
- A default applies **only** when declared explicitly with `= literal`.
- `None` is a value tag, distinct from a field-presence flag
  ([06-adaptive-compiler.md](../06-adaptive-compiler.md), #87).

#### Deferred

Dimensional and unit types (#73) and general user generics (#80 follow-up) are
**not** in Stage 0. The grammar keeps `type-ref` recursive so a constructor like
`Option<T>` extends to future constructors without a grammar change.

### 5 · Data passing (#16)

A connection carries data in one of three modes:

| Mode | Use |
|---|---|
| `value` | small data, passed by value |
| `reference` | shared data, passed by reference |
| `streaming` | large data, streamed |

- The mode is written `by value`, `by reference` or `by streaming`.
- Omitting `by` defaults to **`value`**.
- The compiler MUST NOT copy a large payload (`Table`, `Tensor`, or any payload
  above the resource envelope) by value; it **promotes** a declared or defaulted
  `value` to `reference` or `streaming`. A declared `reference` or `streaming` is
  honored.
- The mode is part of the connection's checkable structure, so it is available to
  static checking and to the execution plan.

### 6 · Permissions and resources (#19)

AIR **declares** the permissions and resource requirements a composition needs;
the runtime **grants and enforces** them
([09-security-model.md](../09-security-model.md)). A composition that uses a
permission it did not declare MUST fail validation.

```air
permission file read "data/";
permission file write "out/";
permission network out "api.example.com";

resource cpu = 200 %;
resource memory = 512 MB;
```

- Permissions are fine-grained (#33): a **file** permission names an access mode
  (`read`, `write`, `create`, `delete`) and an optional path scope; a **network**
  permission names a direction (`in`, `out`, `inout`) and an optional
  destination.
- A resource declaration names a resource (`cpu`, `memory`, `gpu`, `power`), an
  amount and a unit.
- The declarations are the **union** of what the bound Capabilities require. The
  validator MUST check that every Capability's declared need is covered and that
  no undeclared permission is used.

### 7 · Contracts (#18)

Contracts are **representation only** at Stage 0; nothing evaluates them
([RFC 0002](0002-stage0-execution-contract.md), #18). The grammar fixes the
representation:

- A contract belongs to a Capability and lives in its body.
- `requires` is a **precondition**; `ensures` is a **postcondition**. The two are
  distinct.
- An assertion is a **typed, structural** comparison over a port, the same shape
  as a gate. An arbitrary expression is not a contract; a complex decision is a
  Capability.
- The representation is extensible, and a contract is identified and versioned
  with the Capability it belongs to (the Capability dimension of the
  compatibility matrix, #85).

### 8 · Mapping to validation

Every construct maps onto the five validation steps in
[05-adaptive-ir.md](../05-adaptive-ir.md):

| Step | What the grammar makes checkable |
|---|---|
| parse | the document is well-formed against §3 |
| type check | every `connect` endpoint is nominally compatible, or structurally so by explicit opt-in (§4) |
| dependency check | every `capability` and `replace` resolves to a registry entry; every `port-ref` names a declared port |
| permission check | every declared permission is available, and no undeclared one is used (§6) |
| resource check | the declared resources fit the Workspace envelope (§6) |

## Drawbacks

- Freezing a grammar early risks freezing a mistake. The mitigation is the
  schema-version field and the rule that the grammar grows by RFC: the version is
  exactly how a later change is made safely.
- The `body` block mixes settings and contracts. It is deliberate — both are
  per-Capability clauses — but a reader may not expect contracts next to
  configuration. The `requires`/`ensures` keywords keep them distinguishable.
- Requiring `;` inside bodies is slightly more verbose than the illustrative
  sketch. It makes the grammar unambiguous without newline sensitivity.

## Rationale and alternatives

- **Keep the sketch's inline `input = reader.rows`.** Rejected: it would give
  connections two spellings (inline setting and `connect`), violating "one
  canonical form", and it hides a type-checked edge inside untyped settings.
- **Put the gate on the connection instead of the Capability.** Rejected: a gate
  says *whether a Capability runs*, which is a Capability property. Data flow
  already implies ordering, so a separate condition on the edge adds a second
  form for one idea.
- **Boolean combinators in a condition.** Rejected for Stage 0: the specification
  says a complex decision is a Capability, not a condition. A single typed
  comparison keeps the core hypothesis small.
- **Structural compatibility on the port instead of the type.** Considered: it
  localizes the opt-in to the point of use. Rejected for Stage 0 because the
  compatibility rule is a property of the *type*, and declaring it once on the
  type keeps the connection check a pure function of the two types.
- **A quoted schema version (`air "0.1"`).** Rejected: the version is a pair of
  numbers, not an opaque string; keeping it numeric lets the compiler compare it.
- **A list/map literal.** Deferred: Stage 0 settings are scalar, and the
  representation is extensible.

## Unresolved questions

- The exact grammar for removing a Capability, and for a control-only workflow
  edge (deferred; the PoC needs neither).
- Rich structural compatibility: width, variance and subtyping rules.
- The dimensional and unit grammar (#73).
- The general user-generic grammar (#80 follow-up).
- The Stage 3 contract language: which assertions are static and which are
  runtime ([RFC 0002](0002-stage0-execution-contract.md), #18 follow-up).

## What this updates

- [05-adaptive-ir.md](../05-adaptive-ir.md) — replaces the illustrative sketch
  with the frozen grammar and type-system reference; the document remains the
  authority for AIR.
- [04-capability-model.md](../04-capability-model.md) — points the type and
  compatibility rules at the settled grammar.
- [16-open-questions.md](../16-open-questions.md) — records that #14–#17, #18 and
  #19 now have a concrete grammar.
- [17-adr.md](../17-adr.md) — adds the grammar decision to the AIR section.
- [rfc/README.md](README.md) — lists this RFC.
