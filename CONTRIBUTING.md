# Contributing to Xdaptive

Thanks for taking the time to help. Xdaptive has an unusual shape — it is a
specification-first operating system project — so a few minutes here will save
you a rejected pull request later.

## What kind of project this is

Xdaptive is defined by its specification. The product of the repository is
`docs/` plus the reference implementation in the Cargo workspace. The most
valuable contributions usually make the system *more reviewable and more
testable*. A documentation change and a parser change are equally important, and
sometimes the documentation change is the one that matters.

## Ways to contribute

- Fix or clarify the specification in `docs/`.
- Propose a decision through an RFC in `docs/rfc/`.
- Report a place where the implementation and the specification disagree.
- Implement or improve a crate under `core/`, `adaptation/`, `execution/`,
  `security/`, `intelligence/`, `desktop/`, `integrations/` or `tools/`.
- Improve the test suite. Tests are how the specification stays honest.
- Report a vulnerability privately as described in [SECURITY.md](SECURITY.md).

## Before you start

Read [docs/00-glossary.md](docs/00-glossary.md) and
[docs/01-vision.md](docs/01-vision.md) first, then the document that owns the
area you want to change. Two invariants decide what is accepted:

1. **Spec before implementation.** Any new IR construct, manifest field or
   runtime behaviour goes into the owning `docs/` file first.
2. **AI proposes, the OS disposes.** No path may let a model's output reach the
   kernel, the filesystem or user data without the validator and the
   transaction boundary described in
   [docs/09-security-model.md](docs/09-security-model.md).

## Development setup

You need a current Rust toolchain. Nothing else is required to build the
workspace skeleton today.

```sh
git clone https://github.com/x1zzdev/Xdaptive.git
cd Xdaptive
cargo build --workspace
cargo test --workspace
```

The usual loop before a commit:

```sh
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --workspace
```

CI runs the same three commands on `ubuntu-latest`. Match the style of the
surrounding code rather than reformatting unrelated files.

## The two rules that matter

1. **Spec before implementation.** If a change needs a feature the specification
   does not describe, update the specification before the implementation. Never
   invent behaviour silently.
2. **One logical change per commit.** A specification gap, a documentation fix,
   and a code change are three commits, not one, even when they touch related
   files. Each commit should be internally complete: links resolve, the workspace
   builds, and tests pass.

## Commit messages

Conventional Commits, area-scoped, and the subject states the decision:

```
spec(ir): make Capability the composition atom
```

not

```
update docs
```

Allowed scopes: `spec`, `ir`, `compiler`, `runtime`, `workspace`, `security`,
`desktop`, `cli`, `docs`, `ci`, `deps`, `release`.

## Pull requests

Keep a pull request focused. If you find an unrelated problem while working,
open a separate issue or pull request instead of bundling it. In the description,
say what changes, why, and which document is now the authority for it. Reviewers
check the diff against that document, not against their own preferences.

## Reporting bugs and requesting features

Use the issue templates. A bug report is most useful with a minimal
reproduction, the command you ran, the output you expected and the output you
got, and a note on whether the specification agrees with you.

## Code of conduct

Participation is covered by [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md). Report
unacceptable behavior to ax1s@x1zz.com.

## Developer Certificate of Origin

Contributions use the **Developer Certificate of Origin (DCO)**, not a CLA
(#3). Sign off each commit:

```sh
git commit -s -m "spec(ir): ..."
```

The sign-off is a line in the commit message:

```
Signed-off-by: Your Name <you@example.com>
```

It certifies that you wrote the contribution, or have the right to submit it
under the project's license. The full text is in [DCO](DCO). A CLA is
introduced only if a legal or licensing need appears.

## License

Xdaptive is licensed under Apache-2.0. By submitting a contribution you agree
to license it under the same terms. See the [README](README.md#license) for the
exact wording.
