# Changelog

Semua perubahan penting pada proyek ini dicatat di berkas ini.
Format mengikuti [Keep a Changelog](https://keepachangelog.com/id/1.1.0/),
dan versioning mengikuti [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Galat saat revisi menunjuk commit tabel lain kini memakai varian
  `CommitBelongsToOtherTable` yang menyebut revisi, tabel yang dimiliki commit,
  dan tabel yang diminta; sebelumnya `verge query`, `verge show`, `verge diff`,
  dan `verge merge` melaporkan `invalid reference`, sehingga tag yang benar
  ikut disalahartikan rusak.
- `verge tag list` mencetak kolom tabel pada setiap tag, diambil dari commit
  yang ditunjuk tag. `TagEntry` berubah dari tuple tiga elemen menjadi struct
  bernama `TagEntry { name, commit, table, summary }`.

### Fixed

- `verge show` kini punya test end-to-end untuk revisi yang menunjuk commit
  tabel lain, sejajar dengan test `query`, `diff`, dan `merge` yang sudah ada.
  Tanpa test itu, perbaikan pesan tabel pada `show` dapat kembali diam-diam.
- Dokumentasi field `CommitBelongsToOtherTable.revision` digeneralisasi:
  field itu menerima teks revisi dari pengguna (`query`, `show`, `diff`) dan
  id commit hex (`merge`), sedangkan dokumentasi lama hanya menyebut yang
  pertama sehingga bertentangan dengan pemanggilnya.

### Added

- Gate `structure` pada CI, dijalankan lewat `.github/scripts/structure/check-structure.py`,
  yang menegakkan aturan yang sebelumnya hanya diukur manual: 150 SLOC per
  berkas, 5 berkas langsung per folder, 11 field header wajib pada setiap
  berkas `.rs` (tepat satu kali masing-masing), `TODO`/`FIXME`/`HACK` tanpa
  referensi issue, serta karakter di luar daftar tanda baca yang disetujui.
  Gate terakhir menangkap kelas kerusakan yang tidak terlihat dari `cargo`:
  pada PR #35 header comment kehilangan `License`, field terduplikasi, dan
  bullet hilang tanpa satu pun gate yang gagal.

- Gate `structure` kini juga menegakkan aturan huruf non-Latin pada **seluruh**
  berkas teks ter-track, bukan hanya `crates/**/*.rs`. Huruf Cyrillic, Greek,
  Thai, CJK, dan fullwidth di `.md`, `.yml`, `.toml`, atau `.py` sebelumnya
  keluar dari gate dengan exit 0. Aturan ini memakai daftar putih huruf Latin
  sehingga tipografi sah seperti `—` dan `→` tetap diterima, dan huruf Latin
  beraksen seperti `José` juga tetap sah.
- Gate `structure` kini memindai `.github/scripts/` juga, bukan hanya
  `crates/`. Empat modul gate dipindah ke `.github/scripts/structure/` agar
  `.github/scripts/` tidak melewati batas lima berkas langsung — pelanggaran
  yang sebelumnya tidak bisa ditangkap gate karena akarnya tidak dipindai.

## [0.3.0] — 2026-10-04

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
- CLI `verge branch create|switch|list|delete`: pointer branch O(1) di
  `refs/heads/` tanpa menyalin blok, dengan validasi nama lintas platform
  (menolak karakter terlarang Windows dan nama device tercadang).
- CLI `verge merge <BRANCH> --table <NAME>`: merge tiga arah terhadap merge base
  leluhur terdekat (seluruh parent, bukan hanya first-parent), dengan strategi
  `manual` (bawaan), `ours`, `theirs`, dan `last-write-wins`; merge yang sudah
  ada ditolak dengan `AlreadyMerged`.
- ADR-0008: semantik `AS OF` pada rantai first-parent dan tag immutable.
- ADR-0009: merge base mengikuti seluruh parent; bagian "Merge base" pada
  ADR-0007 ditandai superseded.

### Changed

- `resolve_revision` menerima `Option<&TableName>` agar `--as-of <TIMESTAMP>`
  dapat diselesaikan untuk tabel tertentu; pemanggil non-waktu meneruskan `None`.
- `read_snapshot` dan `diff_tables` menerima `&dyn TagPointer` tambahan.
- `infrastructure/commit/file-system` dipisah menjadi `file-system/refs/` untuk
  pointer branch/tag dan `file-system/tests/` untuk commit store.
- Berkas dan folder yang melewati batas modularisasi dipecah per tanggung jawab:
  `use-cases/refs/` untuk pointer branch/tag, `domain/merge/rows/row_lookup.rs`,
  `merge_report_printer.rs` di CLI, dan `docs/engineering/audit/` satu berkas
  per entri.

### Fixed

- Nama branch atau tag yang tidak berbentuk tanggal tidak lagi ditolak sebagai
  timestamp: `verge show 2026-q1-report` dan `verge query --as-of 2026-q1-report`
  kembali membaca nama tersebut sebagai pointer.
- `verge tag create` tidak lagi bergantung pada pemeriksaan "sudah ada" lalu
  tulis; pointer tag dibuat dengan `create_new` sehingga dua proses tidak dapat
  bergantian menimpa tag yang sama.
- `AS OF` yang melebihi 10.000 commit kini melaporkan batas penelusuran yang
  sebenarnya (`SearchLimitReached`) alih-alih menyebut waktu commit tertua
  yang salah.
- Nama branch dan tag lebih dari 255 byte ditolak sebagai `InvalidName` sebelum
  menyentuh disk, bukan gagal sebagai error I/O.

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


[Unreleased]: https://github.com/Miruameli/verge/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/Miruameli/verge/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/Miruameli/verge/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Miruameli/verge/releases/tag/v0.1.0
