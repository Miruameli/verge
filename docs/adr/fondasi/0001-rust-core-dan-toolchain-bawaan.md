# ADR-0001: Rust as the core engine

## Status

Accepted

## Context

Verge needs a storage and query engine that is fast, memory-safe, developable
in-house without vendor lock-in, and able to be shipped inside a container
without a large language runtime. The product specification states "Rust core:
high performance, safe, no GC pauses".

## Decision

The engine is written in Rust using its bundled toolchain:

| Requirement          | Tool                  |
| -------------------- | --------------------- |
| Build and test       | `cargo`               |
| Code format          | `rustfmt` (bundled)   |
| Lint                 | `clippy` (bundled)    |
| Toolchain version pin | `rust-toolchain.toml` |

The Rust version is pinned to `1.82`, the crate MSRV is set to that same
version, and every crate is required to use `forbid(unsafe_code)`.

## Alternatives Considered

- **Zig** — an excellent toolchain because build, test, and format are all
  bundled. Rejected because its library ecosystem is still far thinner for
  storage and serialization needs, and much of its standard library contains
  `unsafe`.
- **Go** — mature and easy to learn. Rejected because of its garbage collector
  orientation and heap growth under long query loads.
- **C++** — comparable performance, but with far higher security risk and a much
  larger supporting tooling requirement.

## Consequences

- Fast builds, minimal native dependencies, small binaries.
- No custom scripts for the quality gate: everything uses official tooling.
- A Rust learning curve; accepted as a risk that must be acknowledged.

## Justification

Rust is a mainstream production language that provides memory safety without a
garbage collector, with bundled quality tooling and a large crate ecosystem.

## Date

2026-10-03

## Author

Miruamel
