# ADR-0005: Tabel sebagai blok content-addressed dan pointer on-disk

## Status

Accepted

## Konteks

Milestone 1 membuktikan blok immutable dan commit graph, tetapi belum ada data
yang bisa di-versioning. Pemanggilan `verge show <commit>` pada commit lama hanya
bisa dibuktikan kalau isi tabel benar-benar tersimpan dan dapat dibaca kembali
dari disk.

Pertanyaan utamanya adalah di mana data tabel hidup: sebagai berkas kerja biasa
di luar block store, atau sebagai objek immutable di dalam block store.

## Keputusan

1. Isi tabel disimpan sebagai satu blok content-addressed di `objects/`.
2. Berkas `tables/<nama>/working` hanya berisi digest blok kerja (64 karakter
   hex + newline). Berkas ini pointer, bukan data.
3. `Commit` juga disimpan sebagai blok; byte yang disimpan adalah encoding
   kanonik yang sama dengan byte yang di-hash menjadi `CommitId`, sehingga
   pembaca dapat memverifikasi integritas setiap kali memuat commit.
4. Pointer branch (`refs/heads/<nama>`) berisi `CommitId` dengan format yang
   sama; `HEAD` menunjuk nama branch.
5. CLI adalah composition root: ia menyusun adapter filesystem, jam sistem,
   dan use case, lalu meneruskannya sebagai dependency.

## Batasan yang diterima

- Satu commit menyimpan satu blok penuh untuk tabel tersebut. Dua commit dengan
  isi identik memakai blok yang sama (tanpa duplikasi), tetapi satu perubahan
  sekecil satu byte menulis ulang seluruh blok tabel.
- Konsekuensinya, `diff` dan `merge` berbasis baris belum mungkin; keduanya
  bergantung pada struktur prolly tree yang direncanakan pada Milestone 3.
- Ukuran tabel dibatasi oleh memori proses saat commit dibuat.

## Alternatif yang dipertimbangkan

- **Prolly tree sekarang juga** — memberi dedup di tingkat baris, tetapi
  membutuhkan struktur tree dan algoritma sisip yang belum diuji. Membangunnya
  tanpa bukti kebenaran berisiko tinggi.
- **Data tabel di luar block store** — menggandakan penyimpanan, memutus
  content addressing, dan membuat working copy bisa berbeda dari yang tercatat.
- **Database eksternal untuk tabel** — memindahkan sumber kebenaran ke luar
  repository dan menutup audit trail.

## Konsekuensi

- `verge log` dan `verge show` dapat dibuktikan dengan data nyata.
- Data yang tidak berubah tidak pernah ditulis dua kali.
- Commit yang byte-nya dimanipulasi di disk ditolak saat dibaca.
- Beban memori saat commit sebanding dengan ukuran tabel; prolly tree pada
  Milestone 3 menghapus batasan ini.

## Justifikasi

Kebutuhan

- audit trail yang dapat diverifikasi,
- time-travel read pada commit lama,
- dedup tanpa duplikasi fisik.

Solusi ini memenuhi ketiganya dengan mekanisme yang sama seperti blok biasa,
tanpa menambah konsep baru, dan batasannya terdokumentasi apa adanya.

## Tanggal

2026-10-03

## Penulis

Miruameli

## Review Date

2026-11-03
