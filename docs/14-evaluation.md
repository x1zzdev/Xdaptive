# 14 · Evaluation

Xdaptive must not remain a project that only ships a new interface. It must be
able to measure and verify the effect of its own technology. Every core claim in
this specification names how it is measured here.

A measurement MUST report the method, the input, and the result, so that another
person can reproduce it. A number without a method is not a measurement.

## 1 · Software-understanding accuracy

How accurately does the OS infer the functionality of the software it analyses?

- function-identification accuracy
- input and output inference accuracy
- dependency-analysis accuracy
- behaviour-reproduction success rate
- compatibility-prediction accuracy

## 2 · Capability composition

How accurately and efficiently are Capabilities combined?

- composition success rate
- dependency-resolution accuracy
- composition time
- proportion of unnecessary Capabilities included
- execution-failure rate

## 3 · Adaptive compiler

Compiler performance and correctness:

- compile time
- incremental-compile time
- execution performance
- memory use
- performance before and after optimisation
- type and semantic error detection rate
- incremental result equals full-compile result (a correctness check, not a
  speed check)

## 4 · Transactional adaptation

Workspace-change stability:

- change success rate
- rollback success rate
- state-preservation accuracy
- change-application time
- whether a failed change loses data
- recovery time

A change is counted as successful only if it passes **both** verification layers
of [RFC 0002](rfc/0002-stage0-execution-contract.md): numerical result
equivalence within tolerance (`eps_abs = 1e-12`, `eps_rel = 1e-9` as PoC
defaults) **and** state integrity. A single "close enough" number is not a
measurement of state preservation. Per-operation tolerances, per-representation
integrity rules, and error accumulation on large datasets are follow-ups.

## 5 · Runtime performance

Ordinary work, compared against a baseline:

- execution latency
- CPU use
- memory use
- GPU use
- power use (#57)
- process creation and teardown cost
- IPC overhead

Performance is measured **per work type** (#56): a benchmark exists for each kind
of task before a native-relative ratio is stated, and no single target ratio is
fixed in advance. The baseline MUST be named. "Faster than the previous commit"
is not a measurement; "faster than the same task done by hand in the baseline
tools" is.

## 6 · AI efficiency

How efficiently AI is used in composition:

- intent-interpretation accuracy
- AIR-generation accuracy
- rate at which bad plans are caught and blocked (this SHOULD be high; a low
  rate means the validator is weak, not that the model is good)
- number of AI calls
- token use
- total task-completion time

The efficiency of the **AI-free** path MUST also be measured, because principle 4
([15-principles.md](15-principles.md)) requires that ordinary work does not pay
for AI it does not use. The AI-free path is measured by (#45):

- the proportion of tasks completed without an AI call;
- the mean latency added per AI call;
- the energy consumed by AI inference;
- the proportion of AI-proposed changes that pass verification.

Lowering the AI-call rate is **not** a goal in itself. A task that genuinely
needs AI MUST use it; forcing it onto a non-AI path to improve the ratio is a
regression, not an optimisation.

## How a claim enters the specification

1. State the claim as a normative sentence in the owning document.
2. State the measurement here: method, input, baseline, result.
3. If the claim cannot be measured yet, mark it an open question
   ([16-open-questions.md](16-open-questions.md)) rather than asserting it.

A claim whose measurement regresses is a defect, not a trade-off.
