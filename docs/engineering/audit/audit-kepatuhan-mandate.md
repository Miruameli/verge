# Audit: kepatuhan aturan modularisasi

## 2026-10-04 — Pengukuran dan perbaikan kepatuhan

| Field    | Nilai                                                            |
| -------- | ---------------------------------------------------------------- |
| Waktu    | 2026-10-04                                                       |
| Aksi     | Mengukur seluruh repo terhadap aturan modularisasi, lalu memperbaiki empat pelanggaran yang nyata |
| Pelaku   | Miruameli                                                        |
| Alasan   | Aturan modularisasi hanya berguna bila diukur; selama ini batas folder, import, dan arah layer hanya dijaga secara sambil lalu |
| Terkait  | Issue #33, ADR-0003 (arsitektur 7-layer), PR #34                |
| Dampak   | Tidak ada perubahan perilaku; 310 test lulus sebelum dan sesudah |
| Rollback | `git revert` commit pada PR #34; tidak ada perubahan format data |

### Cara pengukuran

Seluruh angka di bawah dihitung dari isi repo, bukan dari ingatan:

- SLOC dihitung sebagai baris yang tidak kosong dan bukan komentar (`//`,
  `///`, `//!`).
- Direct file dan direct subfolder dihitung per folder, bukan akumulasi path.
- Arah layer dihitung dari setiap pernyataan `use crate::…` dan `use super::…`
  yang dipetakan ke layer tujuannya.
- Lingkaran dependensi dicari dengan penelusuran DFS pada graf modul.

### Hasil sebelum dan sesudah

| Aturan                                                | Sebelum | Sesudah |
| ----------------------------------------------------- | ------- | ------- |
| Berkas > 150 SLOC                                      | 0       | 0       |
| Folder > 5 berkas langsung                              | 1       | 0       |
| Folder > 5 subfolder                                    | 1       | 1 (disengaja) |
| Fungsi > 50 baris                                       | 0       | 0       |
| TODO/FIXME/HACK tanpa nomor issue                       | 0       | 0       |
| Lingkaran dependensi modul                             | 0       | 0       |
| Layer `domain` mengimpor `infrastructure`             | 2       | 0       |
| Atribut `#[allow]` tanpa alasan                         | 1       | 0       |
| Test                                                    | 306     | 310     |

### Pelanggaran yang diperbaiki

1. **`application/version-control/tests/` punya enam berkas langsung.** Tiga
   test resolusi revisi dipindahkan ke `tests/revision/`, mengikuti pola yang
   sudah dipakai `tests/tagging/`.
2. **`domain/merge/tests/` mengimpor adapter filesystem.** `merge_base_tests.rs`
   dan `commit_chain_fixture.rs` memakai `FileCommitRepository` dan
   `FileBlockStore`; keduanya dipindahkan ke `crates/verge-core/tests/merge/`
   sebagai test integrasi. Ini menghapus satu-satunya inversi layer yang
   tersisa: `domain` kini hanya mengimpor `shared`.
3. **`#[allow(clippy::too_many_arguments)]` pada `merge_writer.rs`.** Sembilan
   argumen dikolapsikan menjadi tujuh tanpa atribut apa pun: header dan baris
   gabung menjadi satu `TableRows`, dan dua ujung merge diambil dari
   `MergeSides` yang sudah ada. Perilaku tidak berubah karena kedua sisi merge
   tetap menjadi parent commit dengan urutan yang sama.
4. **Dispatcher CLI mengimpor sepuluh perintah.** Enam perintah tabel kini
   ditangani router kelompok di
   `commands/table-versioning/dispatch.rs`, sehingga `cli_dispatcher.rs`
   mengimpor lima perintah kelompok dan satu router tabel. Daftar nama perintah
   tabel hidup di satu konstanta dengan fungsi `is_table_command`, sehingga
   nama yang terdaftar tidak mungkin tidak memiliki cabang `match`.
5. **Satu fungsi integrasi 57 baris.** `versioning.rs` memindahkan penyiapan
   repository, graph, dan dua branch ke helper `skenario_branch_dan_tag()`.
   Ketidaksamaan snapshot sebelum dan sesudah pemindahan dibuktikan test yang
   sama.

### Pengecualian yang tetap berlaku

- **`domain/` punya tujuh subfolder.** `commit`, `ident`, `merge`, `storage`,
  `table`, `time`, dan `tree` adalah konteks yang saling bebas; menggabungkannya
  hanya menambah kedalaman folder tanpa mengurangi jumlah konsep. Kepemilikan
  proyek menetapkan batas jumlah berkas per folder sebagai target, bukan angka
  mutlak, dan alasannya tercatat di `docs/architecture.md`.
- **`shared/` memakai nama yang masuk daftar nama generik.** Layer 6 pada
  arsitektur 7-layer bernama `shared/`; mengganti namanya dengan `common/` atau
  `cross-cutting/` tidak menambah kejelasan. Isinya bukan tumpukan utilitas:
  hanya kernel hasil dan tipe galat.
- **`shared` dan `config` mengimpor `domain`.** `VergeError` memuat `Digest`
  sebagai isi galat; `config` memvalidasi `TableName`, `BlockId`, dan
  `HexText` saat memetakan path. Keduanya hanya menyentuh value object murni.

### Test yang ditambahkan

- `table_command_tests.rs` membuktikan setiap nama perintah tabel dikenali,
  perintah kelompok lain tidak dikenali, dan pesan galat menyebut perintah yang
  benar-benar ada. Test ini menangkap kelas bug yang nyata: nama yang
  ditambahkan ke daftar tetapi tidak memiliki cabang `match` akan berakhir
  sebagai `unknown table command`.

### Catatan proses

- Pemeriksaan manual dan review pembaca tidak menemukan pelanggaran ini;
  seluruh temuan berasal dari pengukuran. Indikator seperti "folder ini mulai banyak"
  baru terlihat setelah direktori dihitung.
- Dua dari lima pelanggaran (nomor 3 dan 4) muncul karena pengukuran, bukan
  karena kegagalan test: keduanya tidak mengubah perilaku dan seluruh test
  lulus sebelum dan sesudah.

---

## 2026-10-04 — Penegakan aturan lewat gate otomatis

| Field    | Nilai                                                                                                    |
| -------- | -------------------------------------------------------------------------------------------------------- |
| Waktu    | 2026-10-04                                                                                               |
| Aksi     | Mengubah pengukuran manual menjadi job CI `structure` yang menolak PR                                      |
| Pelaku   | Miruameli                                                                                                |
| Alasan   | Pengukuran pada audit sebelumnya hanya hidup di percakapan; tidak ada artefak yang menyimpan hasilnya        |
| Terkait  | Issue #39, Issue #41, PR #40                                                                              |
| Dampak   | Tidak ada perubahan perilaku; 313 test lulus; gate baru menambah satu status check pada `main`            |
| Rollback | Hapus job `structure` dari `.github/workflows/ci.yml`; tidak ada perubahan kode produksi                  |

### Apa yang ditegakkan

| Aturan                                                        | Batas                        |
| ------------------------------------------------------------- | ---------------------------- |
| SLOC per berkas `.rs`                                          | 150                          |
| Berkas langsung per folder                                     | 5                            |
| Subfolder per folder                                           | 5, atau 10 untuk root layer |
| Field header wajib per berkas `.rs`                             | 11, masing-masing satu kali  |
| `TODO`/`FIXME`/`HACK` tanpa referensi issue                     | 0                            |
| Karakter di luar daftar tanda baca yang disetujui               | 0                            |

### Cara memverifikasi gate-nya sendiri

Gate struktur baru diuji dengan menyuntikkan delapan kelas kerusakan ke
salinan repo, lalu menjalankan gate atas salinan itu:

| Kerusakan yang disuntikkan                    | Hasil     |
| -------------------------------------------- | --------- |
| `License:` dihapus                            | tertangkap |
| `Version:` terduplikasi                      | tertangkap |
| `Related issues:` terduplikasi               | tertangkap |
| `Related ADR:` dihapus                       | tertangkap |
| Homoglif Cyrillic pada `KENAPA`              | tertangkap |
| CJK nyasar pada komentar                      | tertangkap |
| `TODO` tanpa referensi issue                 | tertangkap |
| SLOC 155                                      | tertangkap |

Setelah kedelapan kerusakan dipulihkan, gate kembali lulus dengan exit 0.
Tanpa langkah ini, gate hanya diklaim bekerja dan tidak dibuktikan.

### Batasan yang diketahui

Terdapat **dua bentuk header** yang masih hidup berdampingan: bentuk kanonik
satu field per baris (234 berkas) dan bentuk ringkas dengan pemisah `·`
(32 berkas). Gate menerima keduanya selama bentuk ringkas masih ada;
normalisasi dicatat pada Issue #41.

Dua percobaan normalisasi otomatis pada sesi yang sama **gagal dan tidak
di-commit**: pemecah berdasarkan `·` merusak nilai yang memuat koma sehingga
baris `Dependencies` berisi `` `, ` `` alih-alih nama modul. Pelajaran yang
diambil: normalisasi header bukan pekerjaan sekali-jalan; nilai setiap field
harus diambil dari berkas itu sendiri dan diverifikasi sebelum dan sesudah.

### Catatan proses

- Commit gate struktur sempat mendarat di branch lokal `chore/header-konsisten`
  yang dibuat untuk percobaan normalisasi header yang dibatalkan, bukan di
  branch `ci/structure-gate`. Akibatnya PR #40 sempat tidak menampilkan
  perubahan sama sekali. Perbaikannya: PR #38 di-merge lebih dulu, commit gate
  di-cherry-pick ke atas `main`, lalu branch fitur di-force-push dengan
  `--force-with-lease`. `--force-push` hanya dipakai pada branch fitur; `main`
  tidak pernah di-force-push.
- Dua percobaan normalisasi header otomatis gagal dan di-revert lewat
  `git checkout -- crates` sebelum sempat ter-commit. Penyebabnya pemecah
  berdasarkan `·` merusak nilai yang memuat koma. Normalisasi header bukan
  pekerjaan sekali-jalan; ia perlu pemeriksaan nilai sebelum dan sesudah,
  seperti yang tercatat pada Batasan yang diketahui di atas.

---

## 2026-10-04 — Perluasan cakupan gate ke seluruh berkas ter-track

| Field    | Nilai                                                                                          |
| -------- | ---------------------------------------------------------------------------------------------- |
| Waktu    | 2026-10-04                                                                                     |
| Aksi     | Menutup dua celah cakupan gate: huruf non-Latin di luar `crates/`, dan batas folder di luar `crates/` |
| Pelaku   | Miruameli                                                                                      |
| Alasan   | Gate ada untuk menegakkan aturan, tetapi dua aturan tidak berlaku di luar `crates/` sehingga hanya separuh repo yang diawasi |
| Terkait  | Issue #43, PR #44                                                                              |
| Dampak   | Tidak ada perubahan perilaku produk; 313 test lulus; gate memindai 310 berkas teks ter-track   |
| Rollback | Kembalikan `git checkout` pada PR #44; tidak ada perubahan format data                        |

### Dua celah yang ditemukan, dan cara membuktikannya

| Celah                                                        | Cara dibuktikan                                              |
| ------------------------------------------------------------ | ------------------------------------------------------------ |
| Aturan non-ASCII hanya memindai `crates/**/*.rs`             | Satu baris berkarakter Cyrillic disisuntik ke `docs/roadmap.md`, gate tetap keluar exit 0 |
| Aturan batas folder hanya memindai `crates/`                 | Empat modul gate dikembalikan ke root `.github/scripts/`, folder mencapai tujuh berkas langsung, gate tetap exit 0 |

Keduanya ditutup pada PR #44. Huruf non-Latin kini diperiksa pada seluruh
berkas teks ter-track memakai daftar putih huruf Latin; batas folder kini juga
mempakai `.github/scripts/` sebagai akar, dan akarnya sendiri ikut dihitung
karena `rglob("*")` hanya mengembalikan turunan.

### Kesalahan yang hampir lolos sebagai "lulus"

| Kesalahan                                                                                  | Akibatnya                                                                 |
| ------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------- |
| Probe menyuntik teks ASCII `U+041A`, bukan huruf Cyrillic                                   | Gate "lulus" tanpa menguji apa pun                                        |
| Probe memakai `git checkout -- .` yang memulihkan dari indeks, bukan `HEAD`                 | Damage satu kasus bocor ke kasus berikutnya; tiga kasus melaporkan lokasi salah |
| Berkas `text_rules.py` sendiri mengandung dua huruf Hangul, dan belum dilacak git         | Gate meloloskannya justru pada saat gate baru dibuat                      |
| Perluasan batas folder versi pertama tidak menghitung akar                                  | Tujuh berkas di `.github/scripts` tetap lolos                              |

Empat di antaranya adalah cacat pada alat verifikasi, bukan pada kode yang
diperiksanya. Semuanya ditemukan karena probe dijalankan dan hasilnya dibaca,
bukan karena gate_membersih_checkpoint berjalan. Pelajaran yang diambil: gate
yang baru dibuat wajib diuji dengan menyuntikkan kerusakan, dan probe itu sendiri
wajib diperiksa apakah ia benar-benar menyuntik apa yang diklaim.

### Konsekuensi yang diterapkan

- `tracked_text_files` memakai `--others --exclude-standard`, sehingga berkas
  baru yang belum dilacak ikut diperiksa.
- `sys.dont_write_bytecode` diset di gate agar proses read-only tidak menulis
  `__pycache__/` ke dalam repo.
- `check-structure.py` dipisah menjadi empat modul di `.github/scripts/structure/`;
  berkas utama sempat naik ke 184 SLOC dan kini 115.
- Empat modul tersebut dipindah agar `.github/scripts/` tidak melewati batas
  lima berkas langsung.
