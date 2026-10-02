# Changelog

Semua perubahan penting pada proyek ini dicatat di berkas ini.
Format mengikuti [Keep a Changelog](https://keepachangelog.com/id/1.1.0/),
dan versioning mengikuti [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] — 2026-10-03

### Added

- Workspace Rust dua crate: `verge-core` (engine) dan `verge-cli` (binary `verge`).
- Content-addressed block store (`FileBlockStore`) dengan deduplikasi, layout
  fan-out dua tingkat, dan penulisan atomik (temp file + fsync + rename).
- Commit immutable dengan encoding kanonik ber-length-prefix sehingga identifier
  diturunkan deterministik dari isi commit.
- Commit graph dengan branch (pointer bergerak), tag (pointer immutable),
  ancestry check pada diamond, dan traversal first-parent.
- Use case `initialize_repository` dengan rollback bila langkah pembuatan gagal.
- CLI `verge init`, `--help`, dan `--version`.
- Arsitektur 7 layer dengan port di domain dan implementasi di infrastructure.
- Quality gate: `rustfmt`, `clippy` (pedantic, `-D warnings`), `cargo test`,
  `gitleaks`, `cargo audit`, dan dependabot.

[Unreleased]: https://github.com/Miruameli/verge/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Miruameli/verge/releases/tag/v0.1.0
