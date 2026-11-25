# Security Policy

## Supported versions

Xdaptive is pre-1.0 and pre-MVP. Only the latest commit on `main` is supported.
There are no maintenance branches and no backports.

## Reporting a vulnerability

Report vulnerabilities privately. Do not open a public issue.

- Email ax1s@x1zz.com, or
- Use GitHub's private vulnerability reporting on the repository's Security tab.

Include what you can of the following:

- The affected component and version or commit.
- A minimal reproduction: an AIR document, a manifest, or the command you ran.
- The behavior you expected and the behavior you got.
- Why you believe it is a security problem and not a correctness bug.

You will get an acknowledgement within a few days. If the report is valid, we
will agree on a fix and a disclosure date before any public write-up. Credit is
given unless you ask to stay anonymous.

## What counts as security

Xdaptive composes, compiles and executes software, and it lets a model propose
changes to a running environment. The reports that matter most:

- **Escaping the AI/OS split.** Any path where a model's output reaches the
  kernel, the filesystem, the network or user data without passing validation
  and the transaction boundary.
- **Privilege escalation through a Capability.** A Capability or Workspace that
  gains permissions beyond those it declared in its manifest.
- **Breaking isolation.** Escaping the sandbox (`namespaces`, `seccomp`,
  `cgroups`) from a generated or composed execution module.
- **Corrupting a Workspace.** A change that commits despite failing validation,
  or a rollback that loses state it promised to preserve.
- **Supply-chain problems** in the Software Registry: a manifest or Capability
  pack that is trusted without integrity verification.

A divergence between the implementation and the specification that a reviewer
could not detect is in scope, because the project's guarantee is that such
divergence is caught.

## What does not count

- Crashes on malformed input that produce a diagnostic instead of memory
  unsafety.
- Denial of service from processing a large or adversarial document on your own
  machine.
- Issues that require an already-compromised toolchain or a modified kernel.
- Missing hardening that is not part of a stated guarantee.
- A model producing a wrong plan. The design assumes it will; the OS is required
  to reject it, and a rejected bad plan is not a vulnerability.

When in doubt, report it. It is better to close a report as not-a-vulnerability
than to miss a real one.

## Safe harbor

Good-faith research on your own systems is welcome. We will not pursue legal
action for research that follows this policy, avoids privacy violations and
service disruption, and gives us reasonable time to fix the issue before
disclosure.
