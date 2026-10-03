# Changelog

Semua perubahan penting pada proyek ini dicatat di berkas ini.
Format mengikuti [Keep a Changelog](https://keepachangelog.com/id/1.1.0/),
dan versioning mengikuti [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] — 2026-10-03

### Added

- Nilai waktu `Timestamp` (RFC 3339 UTC dan `@<unix_ms>`) dengan konversi kalender
  murni tanpa dependensi eksternal; dipakai `verge query` dan `verge log`.
- CLI `verge query --table <NAME> --as-of <WHEN>`: `WHEN` dapat berupa RFC 3339
  UTC (`2026-10-01T10:00:00Z`, opsi milidetik `.123Z`), unix milidetik
  (`@1767225600000`), nama tag, `refs/tags/<nama>`, `refs/heads/<nama>`, commit
  id penuh, awalan hex 12–63, `HEAD`, atau `HEAD~N`. Offset selain nol ditolak
  agar audit tetap reproducible.
- CLI `verge tag create <NAME> [--revision <REV>]`, `verge tag list`, dan
  `verge tag delete <NAME>`; tag bersifat immutable (`TagAlreadyExists`) dan
  menunjuk satu commit di `refs/tags/` tanpa menyalin blok.
- `verge log` kini mencetak `123456789abc 2026-10-01T10:00:00.000Z author summary`
  sehingga waktu yang tercetak dapat langsung disalin ke `--as-of`.
- Port `TagPointer` dan adapter `FileTagPointer`; `revision_resolver` kini
  memakai `TagPointer` sehingga `verge show` dan `verge diff` juga menerima tag
  dan timestamp.
- ADR-0008: semantik `AS OF` pada rantai first-parent, tag immutable, dan
  amandemen penelusuran merge base ADR-0007 menjadi seluruh parent.

### Changed

- `resolve_revision` menerima `Option<&TableName>` agar `--as-of <TIMESTAMP>`
  dapat diselesaikan untuk tabel tertentu; pemanggil non-waktu meneruskan `None`.
- `read_snapshot` dan `diff_tables` menerima `&dyn TagPointer` tambahan.
- `infrastructure/commit/file-system` dipisah menjadi `file-system/refs/` untuk
  pointer branch/tag dan `file-system/tests/` untuk commit store.
-
## [0.2.0] — 2026-10-03

### Added

- CLI `verge diff <FROM>..<TO> --table <NAME>` yang mencetak perubahan per baris
  (`+` tambah, `-` hapus, `~` ubah) dan `no changes` bila tabel identik.

### Changed

- Data kerja tabel (`tables/<nama>/working`) kini menunjuk digest akar prolly
  tree, bukan blok isi tabel; `FileTableWorkspace::new` hanya menerima layout.

## [0.1.0] — 2026-10-03

Rilis pertama: binary untuk linux (x86_64, aarch64), macOS (arm64), dan Windows
(x86_64), masing-masing dengan `SHA256SUMS` dan SBOM CycloneDX.

### Added

- Subdomain `table`: `TableName` tervalidasi (allowlist `[a-z0-9_-]`, maksimal 64
  karakter) sehingga nama tabel tidak dapat keluar dari `.verge`.
- Codec commit dua arah: `commit_encoding` menulis byte kanonik, `commit_decoding`
  memverifikasi ulang digest dan menolak byte yang dimanipulasi di disk.
- Port `CommitRepository`, `RefPointer`, `TableWorkspace`, dan `TableSource`.
- Use case `stage_table`, `record_commit`, `read_history`, dan `read_snapshot`.
- Adapter filesystem: `FileCommitRepository`, `FileRefPointer`,
  `FileTableWorkspace`, `FileTableSource`, dan jam sistem `now_unix_ms`.
- CLI `verge import`, `verge commit`, `verge log`, dan `verge show` dengan
  time-travel read pada commit lama.
- ADR-0005: tabel disimpan sebagai blok content-addressed beserta batasan yang
  diterima sebelum prolly tree hadir.

### Changed

- `Commit` kini membawa nama tabel sehingga riwayat dapat disaring per tabel.
- Field accessor commit dipisah ke `commit_fields.rs` agar berkas tetap di bawah
  batas 150 baris tanpa melonggarkan visibilitas.

### Milestone 1 — fondasi storage dan versioning

Masuk lebih awal pada versi yang sama:

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

[0.2.0]: https://github.com/Miruameli/verge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Miruameli/verge/releases/tag/v0.1.0
