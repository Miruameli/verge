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
