# Audit: Milestone 2 — versioning data tabel

## 2026-10-03 — Milestone 2: versioning data tabel

| Field    | Nilai                                                                        |
| -------- | ---------------------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                                   |
| Aksi     | Merge PR #9 ke `main`: import, commit, log, dan time-travel read             |
| Pelaku   | Miruameli                                                                    |
| Alasan   | Menutup issue #8; data tabel baru bisa di-versioning setelah ini              |
| Terkait  | Issue #8, PR #9, ADR-0005                                                    |
| Dampak   | `verge show <commit>` dapat membaca isi tabel pada commit lama               |
| Rollback | `git revert` commit merge; blok immutable yang sudah tertulis tidak rusak     |

### Bukti

- 106 test lulus: 68 unit, 18 E2E CLI, 4 integrasi, 16 doctest.
- Smoke run binary rilis: `init → import → commit → commit → log → show <lama>`
  menghasilkan isi berbeda sesuai revisinya.
- Tiga stage dengan isi identik menambah tepat satu blok di disk.
- Byte commit yang diubah di luar Verge ditolak saat dibaca
  (`VergeError::MalformedCommit`).
- CI hijau: format, lint, test, audit RustSec, dan gitleaks.

### Keputusan yang diambil

- Isi tabel disimpan sebagai satu blok content-addressed; `tables/<nama>/working`
  hanya menyimpan digest. Alasannya dan batasannya tercatat di ADR-0005.
- `Commit` membawa nama tabel agar riwayat dapat disaring per tabel.
- CLI menjadi composition root yang menyusun adapter, jam sistem, dan use case.
