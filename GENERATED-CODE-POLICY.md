# Generated-code policy

Xdaptive can generate Capability code with a model. This document states the
licensing position for that code ([16-open-questions.md](docs/16-open-questions.md),
#77). It is a policy, not legal advice; the legal review noted below is required
before any public release.

## The position

**Xdaptive is licensed under Apache-2.0. AI-generated Capability code is not
automatically subject to the Xdaptive project's license.** A user may choose the
terms for their generated artifacts, subject to applicable law, third-party
rights, and the terms of the models or external components they used.

Three things are distinct and MUST NOT be conflated:

| Thing | Policy |
|---|---|
| Xdaptive source code | Apache-2.0 ([LICENSE](LICENSE)) |
| AI-generated Capability code | not automatically Apache-2.0; the user chooses |
| External libraries and model outputs | their own licenses and terms apply |

## Rules

1. **Do not apply the tool's license to generated output automatically.** Xdaptive
   being Apache-2.0 does not make a generated Capability Apache-2.0.
2. **Let the user choose a license** for a generated or published Capability
   (MIT, Apache-2.0, GPL, proprietary, internal-only). Choosing a license does not
   by itself mean the user has the right to grant it; that is the user's
   responsibility to verify.
3. **Respect third-party terms.** Generated code may include external code, and a
   model's terms of use may apply. Xdaptive MUST NOT guarantee that a generated
   artifact is free of external constraints.
4. **Keep provenance.** A Capability Manifest SHOULD be able to record the
   generating tool and version, the model used (where known), the license the user
   chose, external dependencies, and the verification result. This does not prove
   a license; it makes origin and responsibility auditable later.

## What is not claimed

This policy does not assert that generated code is copyrightable, that it is
free of third-party rights, or that it is safe to run. Execution safety is a
separate matter ([SECURITY.md](SECURITY.md),
[docs/09-security-model.md](docs/09-security-model.md)).

## Follow-up

- A `THIRD-PARTY-NOTICES` file when external components are bundled.
- Legal review of generated-output copyright and per-model terms of use before a
  public release.
