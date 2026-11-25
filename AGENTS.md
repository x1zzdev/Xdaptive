# AGENTS.md — Working agreement for this repository

Xdaptive is a specification-driven project. The product of this repository at
Stage 0 is the **specification** in `docs/`, plus a reference implementation that
grows crate by crate under `core/`, `adaptation/`, `execution/`, `security/`,
`desktop/`, `intelligence/`, `integrations/` and `tools/`. Everything an agent or
a human does here must make the system *more reviewable and more testable*.

Read, in order, before changing anything:

`AGENTS.md` → `docs/00-glossary.md` → `docs/01-vision.md` →
`docs/02-architecture.md` → the `docs/` file that owns the area you are touching
→ `NEXT.md` (local task queue).

## Repo layout

- `docs/` — the specification. It is the contract: `docs/05-adaptive-ir.md` is
  the authority for the IR, `docs/04-capability-model.md` for the capability and
  manifest schemas, `docs/03-workspace.md` for the workspace data model.
- `docs/vision.ko.md` — the original Korean master vision. When a decision
  changes, update the English spec **and** the master in the same change.
- `docs/rfc/` — numbered design records. A contested decision becomes an RFC.
- `core/`, `adaptation/`, `execution/`, `security/`, `desktop/`,
  `intelligence/`, `integrations/`, `tools/` — the Rust Cargo workspace. Only
  split a crate out once its interface and responsibility boundary are stable.
- `NEXT.md` / `NEXT-archive.md` — the local task queue. **Never committed**
  (see `.gitignore`).

## Ground rules

1. **Spec before implementation.** Any new IR construct, manifest field, or
   runtime behaviour goes into the owning `docs/` file first. An implementation
   that the spec does not describe is rejected, not merged and documented later.
2. **AI proposes, the OS disposes.** No code path may let a model's output reach
   the kernel, the filesystem, or user data without passing the validator and
   the transaction boundary described in `docs/09-security-model.md`. This is a
   hard architectural rule, not a guideline.
3. **One canonical form.** Never introduce a second way to express the same
   thing. When a choice is ambiguous, prefer the most explicit, most reviewable
   form and record the decision.
4. **Measurable claims.** A performance or correctness claim in the docs must
   name how it is measured (`docs/14-evaluation.md`). Do not assert speed.
5. **No emojis. No comments that restate the code.** Docs are English; the user
   writes Korean, so respond in Korean unless the user switches.

## Verification commands (run before every commit)

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

Documentation-only changes must still pass `cargo fmt --all --check` and must
not leave dangling relative links. The CI workflow (`.github/workflows/ci.yml`)
runs the same three commands on `ubuntu-latest`.

## Commit policy (most important)

**Commit continuously and autonomously, in the smallest coherent unit, as soon
as one completes. Do not wait for the user to ask.**

- One logical change = one commit. A spec change, a doc fix, and a code change
  are three commits even when they touch related files.
- "Smallest coherent unit" means the change is internally consistent and
  complete: docs and code agree, links resolve, the workspace builds, no
  half-edits.
- Conventional Commits, area-scoped, and the subject states the decision:
  `spec(ir): make Capability the composition atom`, not `update docs`.
  Allowed scopes: `spec`, `ir`, `compiler`, `runtime`, `workspace`, `security`,
  `desktop`, `cli`, `docs`, `ci`, `deps`, `release`.
- Never bundle unrelated edits, and never commit secrets.

## Before committing

- Run `git status`, `git diff`, and `git log --oneline -10`; stage only the
  intended files.
- Check consistency: no dangling links, no stale examples, no contradiction
  between two spec documents.

## Working loop

Work is driven by `/next`: it takes the top unfinished `- [ ]` item from
`NEXT.md`, implements exactly that one unit, verifies, commits, moves the
finished item to `NEXT-archive.md`, and appends newly discovered follow-ups to
the backlog section of `NEXT.md`. Keep `NEXT.md` short — its length is the token
cost of every `/next` call.
