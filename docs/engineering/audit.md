# Audit Trail

Catatan tindakan signifikan terhadap repository ini: waktu, aksi, alasan, pelaku,
terkait issue atau PR, dampak, dan cara rollback.

## 2026-10-03 — Inisialisasi repository dari nol

| Field    | Nilai                                                          |
| -------- | -------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                     |
| Aksi     | Build ulang workspace dari kosong: engine, CLI, docs, dan CI     |
| Pelaku   | Miruameli (akun `gh` terverifikasi lewat `gh auth status`)       |
| Alasan   | Spesifikasi produk menuntut fondasi versioning native dan audit trail tanpa utang teknis |
| Terkait  | Issue #1 (Milestone 1)                                          |
| Dampak   | 40 test hijau, `clippy -D warnings` bersih, `cargo fmt` bersih  |
| Rollback | `git revert` pada commit bootstrap; tidak ada data pengguna terdampak |

## 2026-10-03 — Merge Milestone 1 dan proteksi branch

| Field    | Nilai                                                                       |
| -------- | --------------------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                                  |
| Aksi     | Merge PR #2 (squash) ke `main` dan aktifkan branch protection                |
| Pelaku   | Miruameli                                                                   |
| Alasan   | Menutup issue #1 dan menjadikan `main` branch yang hanya berubah lewat PR     |
| Terkait  | PR #2, issue #1, issue #6                                                    |
| Dampak   | `main` berisi fondasi engine; seluruh quality gate wajib hijau sebelum merge  |
| Rollback | `git revert 40bdb34`; proteksi branch dapat diubah lewat pengaturan repository |

### Catatan kepatuhan

- Status check wajib pada `main`: `format`, `lint`, `test`, `audit`, `secrets`.
  Branch harus up-to-date sebelum merge; force push dan penghapusan branch
  dinonaktifkan; proteksi berlaku juga untuk admin.
- `required_approving_review_count` bernilai 0 karena GitHub tidak mengizinkan
  self-approval. Daftar periksa self-review pada setiap PR menggantikan review dari
  orang lain sampai ada kontributor kedua.
- Commit bootstrap pertama ke `main` (`4a961d4`) adalah commit kosong tanpa kode,
  dibuat hanya supaya branch `main` ada sebelum PR dapat dibuat. Seluruh kode masuk
  lewat PR.

### Definition of done Milestone 1

- [x] 40 test lulus, `clippy -D warnings` bersih, `cargo fmt` bersih.
- [x] `cargo audit` (RustSec) bersih.
- [x] `gitleaks` bersih di pre-commit dan CI.
- [x] Smoke run CLI terbukti: `verge init` membuat layout, init kedua ditolak.
- [x] PR #2 direview dan di-merge; issue #1 tertutup.
- [x] `main` dilindungi branch protection.
- [ ] Rilis `v0.1.0` dengan binary lintas platform, checksum, dan SBOM (issue #6).
