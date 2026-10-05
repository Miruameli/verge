# ADR-0003: 7-layer architecture

## Status

Accepted

## Context

The product will grow from a single engine into a server, an SDK, and a Web UI.
Without explicit layer boundaries, business logic leaks into I/O adapters and
becomes hard to test.

## Decision

The code follows 7 standard layers: `domain`, `application`, `infrastructure`,
`presentation`, `interfaces`, `shared`, and `config`.

- `domain` performs no I/O; every I/O need is expressed as a port.
- `application` only orchestrates ports, so it can be tested without a
  filesystem.
- `infrastructure` implements the ports.
- `interfaces` (CLI) calls use cases, not the domain directly.
- A folder holds at most 5 direct files and 5 subfolders; a file holds at most
  150 lines.

## Alternatives Considered

- **Flat modules per crate** — fast, but responsibility boundaries blur as
  features accumulate.
- **Hexagonal architecture without strict folder boundaries** — the dependency
  direction is already correct, but folders quickly become hard to navigate.
- **Microservices from the start** — irrelevant; what is needed is a clear
  boundary, not separate processes.

## Consequences

- Unit tests can run without external I/O.
- Adding an S3 or GCS backend means adding a port implementation, not changing
  use cases.
- The folder structure is larger; navigation requires discipline, but every
  file has one clear responsibility.
- The `presentation/` layer contains no code yet because there is no UI; no
  placeholder files are waiting.

## Justification

Modularization is this project's primary approach. Explicit layers with folder
boundaries keep complexity local and easy to test.

## Date

2026-10-03

## Author

Miruamel
