# ADR-0010: Toolchain 1.98, language edition stays 2021

## Status

Accepted

## Context

The repository pinned Rust `1.82` in `rust-toolchain.toml` while the local
toolchain in daily use is `1.98.1`. Every local gate ran under 1.82 through the
file override, so "green locally" tested the wrong compiler. The 1.82 pin also
makes `sha2 0.11` unbuildable (it needs cargo 1.85+ for the `edition2024`
feature), which is the root cause of issue #47. Clippy 1.82 and 1.98 enforce
different lint sets, so the local gate checked the wrong rules.

## Decision

1. Pin the toolchain to `1.98` in `rust-toolchain.toml` and set crate
   `rust-version = "1.98"`.
2. Keep the language edition at `2021` (`edition = "2021"` in workspace and
   `rustfmt.toml`). Edition 2024 reserves `gen` as a keyword; staying on 2021
   avoids that churn in a slice whose purpose is the compiler upgrade.
3. Pin the same `1.98` in the five `dtolnay/rust-toolchain` steps across
   `ci.yml`, `release.yml`, and `publish-crate.yml`, and update the comments
   that name the version.

## Alternatives Considered

- **Stay on 1.82.** Rejected: it perpetuates the local-vs-real divergence and
  keeps the `sha2 0.11` upgrade impossible.
- **Upgrade to edition 2024 in the same slice.** Rejected: it mixes a language
  migration (keyword risk, style churn) into a compiler upgrade. A separate
  assessment can propose it later with its own evidence.
- **Float on stable without a pin.** Rejected: ADR-0001 requires a pinned
  toolchain so builds are reproducible; floating reintroduces "works on my
  machine".

## Consequences

- Local gates and CI run the same compiler the developer actually uses.
- `sha2 0.11` becomes buildable; issue #47 is closed as a consequence.
- ADR-0001 stays untouched (ADRs are immutable); this record supersedes only
  its version number.

## Date

2026-10-06

## Author

Miruameli
