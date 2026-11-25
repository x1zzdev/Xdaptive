## What changes

A short description of the change.

## Why

The problem this solves, or the decision it records.

## Authority

Which document is now the authority for the new behavior? If this change
introduces an IR construct, a manifest field, a security rule or a Workspace
behavior, link the specification update.

## Checklist

- [ ] The specification was updated before or with the implementation.
- [ ] The change keeps a model's output away from OS authority (see docs/09-security-model.md).
- [ ] `cargo fmt --all --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test --workspace` pass.
- [ ] Commits are one logical change each, and messages state the decision.
