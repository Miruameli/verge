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
- [x] Milestone 2 selesai: versioning data tabel dan time-travel read (PR #9).
- [x] Milestone 3 selesai: prolly tree, diff row-level, dan resolusi revisi (PR #19).
- [x] Rilis `v0.2.0` terbit dengan 4 binary, `SHA256SUMS`, dan 4 SBOM.
- [x] Rilis `v0.1.0` terbit dengan 4 binary lintas platform, `SHA256SUMS`, dan 4 SBOM (issue #6).

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

### Utang teknis yang dicatat

- Deduplikasi per baris dan `verge diff` menunggu prolly tree pada Milestone 3.
- Rilis `v0.1.0` masih menunggu workflow binary lintas platform, checksum, dan
  SBOM (issue #6).

## 2026-10-03 — Rilis v0.1.0

| Field    | Nilai                                                              |
| -------- | ------------------------------------------------------------------ |
| Waktu    | 2026-10-03                                                         |
| Aksi     | Terbitkan GitHub Release `v0.1.0` dari tag di `main`               |
| Pelaku   | Miruameli                                                          |
| Alasan   | Menutup issue #6; rilis tanpa binary, checksum, dan SBOM dilarang    |
| Terkait  | Issue #6, PR #11–#16, workflow `.github/workflows/release.yml`      |
| Dampak   | Empat binary lintas platform dan empat SBOM dapat diunduh terverifikasi |
| Rollback | Tag dan rilis dapat dihapus; artefak immutable tidak menyentuh data pengguna |

### Artefak yang terbit

```
SHA256SUMS
verge-0.1.0-x86_64-unknown-linux-gnu.tar.gz
verge-0.1.0-aarch64-unknown-linux-gnu.tar.gz
verge-0.1.0-aarch64-apple-darwin.tar.gz
verge-0.1.0-x86_64-pc-windows-msvc.zip
verge-<target>.cdx.json   (SBOM CycloneDX 1.5 untuk keempat target)
```

### Bukti

- `sha256sum -c --strict SHA256SUMS` lolos untuk kedelapan berkas setelah diunduh
  ulang dari GitHub Release.
- SBOM memuat 13 komponen untuk linux/macOS dan 11 untuk Windows, dengan
  `verge-core` sebagai komponen utama dan `sha2` tercatat.
- Binary `aarch64-unknown-linux-gnu` hasil rilis menjalankan alur nyata:
  `init → import → commit → log → show <commit lama>` mengembalikan isi berbeda
  sesuai revisinya.
- Empat run rilis gagal sebelum berhasil; setiap kegagalan diperbaiki di PR
  tersendiri dan tercatat di commit:
  1. linker silang aarch64 belum diarahkan (`file in wrong format`),
  2. nomor minor `0.5` tidak valid dan `zip` tidak ada di runner Windows,
  3. env linker terpasang pada langkah yang salah,
  4. lokasi keluaran SBOM tidak ditemukan sehingga SBOM hilang dari rilis.

### Catatan kepatuhan

- Tag hanya pernah menunjuk commit di `main` yang sudah hijau; perpindahan tag
  tercatat pada audit trail repo.
- Rilis terbit hanya setelah `sha256sum --check --strict` menerima seluruh
  checksum; job publish gagal bila satu saja tidak cocok.
- SBOM dibuat dari `cargo metadata --locked --filter-platform` sehingga
  dapat direproduksi dan tercatat di lockfile.

## 2026-10-03 — Milestone 3: prolly tree dan diff row-level

| Field    | Nilai                                                                 |
| -------- | --------------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                            |
| Aksi     | Merge PR #19 (prolly tree + diff) dan PR #20 (versi 0.2.0), tag `v0.2.0` |
| Pelaku   | Miruameli                                                             |
| Alasan   | Menutup issue #18 dan menghapus batasan satu blok per tabel di ADR-0005 |
| Terkait  | Issue #18, PR #19, PR #20, ADR-0006                                    |
| Dampak   | Baris yang tidak berubah berbagi blok; `verge diff` menampilkan perubahan per baris |
| Rollback | `git revert` commit merge; blok lama tetap terbaca karena content-addressed |

### Bukti

- 181 test lulus; `clippy -D warnings` dan `cargo fmt` bersih; CI hijau pada kedua PR.
- Smoke run binary: `verge diff HEAD~1..HEAD --table users` menghasilkan
  `~ 3 ,citra,surabaya -> ,citra,sidoarjo` dan `+ 4 ,sari,medan`.
- Dedup terukur: menambah satu baris menambah tepat 3 blok (daun, akar, objek
  commit); header dan daun lain memakai blok yang sama seperti commit sebelumnya.
- `verge diff HEAD..HEAD` mencetak `no changes`.

### Perubahan format yang merusak kompatibilitas data lama

| Field    | Nilai                                                             |
| -------- | ----------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                        |
| Aksi     | Mengganti snapshot blok mentah dengan prolly tree                 |
| Pelaku   | Miruameli                                                         |
| Alasan   | Dedup per baris dan diff memerlukan partisi baris                |
| Terkait  | ADR-0006, CHANGELOG 0.2.0                                         |
| Dampak   | Snapshot versi 0.1.0 tidak dapat dibaca sebagai tree              |
| Rollback | `git revert` tidak memulihkan data; tabel harus diimpor ulang      |

Tabel dari repository versi 0.1.0 harus diimpor ulang (`verge import` lalu
`verge commit`) sebelum `verge log`, `verge show`, atau `verge diff` dipakai.
Batas ini dicatat sebagai breaking change di CHANGELOG dan ADR-0006.

### Temuan yang diperbaiki saat review

1. `HEADER_MARKER` tidak ter-import pada modul decoding sehingga arm-nya menjadi
   binding pattern yang menangkap semua marker; setiap node terbaca sebagai
   header. Ditemukan oleh test, bukan oleh inspeksi.
2. Doctest `build_plan` mengharapkan dua daun untuk data yang hanya muat satu
   daun.
3. Tiga berkas melewati 150 baris dan dua folder melewati 5 berkas; dipecah
   per tanggung jawab tanpa `#[allow]`.
4. Resolusi revisi hanya menerima 64 hex padahal `verge log` mencetak 12 hex;
   ditambah resolver `HEAD~N` dan awalan dengan deteksi ambiguitas.

### Utang teknis yang dicatat

- Format tabel CSV sederhana tanpa quoting; koma di dalam nilai akan salah baca.
- Beban memori saat import sebanding dengan ukuran tabel.
- Merge tiga arah dan query SQL belum ada (M4 dan M5).
