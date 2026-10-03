# Audit: Milestone 3 — prolly tree dan diff row-level

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
