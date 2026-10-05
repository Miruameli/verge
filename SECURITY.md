# Security Policy

## Reporting a vulnerability

Report a vulnerability through **GitHub Security Advisory** on this repository
(`Security` → `Report a vulnerability`), not through a public issue. Minimum
report contents:

- A description of the problem and its impact.
- Minimal reproduction steps.
- The affected version or commit.
- A suggested fix, if you have one.

We guarantee a reply within 7 business days and will coordinate the fix
schedule with the reporter before publication.

## Severity and time targets

| Severity | Fix target |
| -------- | ---------- |
| Critical | 24 hours   |
| High     | 72 hours   |
| Medium   | 1 week     |
| Low      | 1 month    |

## Principles upheld in this project

- **Immutable storage.** An object that has been written never changes, and its
  identifier is the SHA-256 hash of its content, so tampering can be detected.
- **A branch is a pointer.** Switching branches never copies or rewrites
  historical data.
- **No secrets in the repository.** `gitleaks` runs in pre-commit and in CI.
- **Dependencies are audited automatically.** `cargo audit` and dependabot keep
  new advisories handled quickly.
- **`unsafe` is forbidden** across all engine crates (`forbid(unsafe_code)`).

## Public incident reports

An incident that has been fixed and affected user data is published as a
blameless post-mortem in `docs/engineering/audit.md`.
