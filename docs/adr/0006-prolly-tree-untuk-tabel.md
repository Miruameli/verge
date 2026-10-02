# ADR-0006: Prolly tree untuk tabel

## Status

Accepted

## Konteks

Milestone 2 menyimpan isi tabel sebagai satu blok penuh per tabel (ADR-0005).
Konsekuensinya jujur tetapi tidak desirable: data yang tidak berubah tidak ditulis
dua kali, tetapi perubahan satu byte menulis ulang seluruh tabel, dan diff hanya
bisa membandingkan blok utuh sehingga tidak menghasilkan informasi perubahan
per baris.

Produk menjanjikan "row-level diff" dan "branching tanpa menyalin data".
Keduanya menuntut struktur yang membagi tabel menjadi bagian-bagian kecil yang
dapat dibandingkan dan dibagikan.

## Keputusan

1. Isi tabel diparse menjadi header dan baris; kolom pertama menjadi kunci baris.
2. Baris diurutkan menurut kunci dan kunci ganda diambil baris terakhirnya
   (`last-write-wins`), sehingga urutannya deterministik dan tidak bergantung
   pada urutan file.
3. Baris dipartisi menjadi node daun dengan batas 4 KiB; node internal menunjuk
   anak-anaknya. Akar selalu memuat node header sebagai anak pertama.
4. Identitas setiap node = SHA-256 dari encoding kanoniknya, sehingga node dengan
   isi sama selalu berbagi blok: mengubah satu baris hanya menulis ulang daun
   yang memuat baris itu.
5. `diff` membandingkan dua tabel secara merge walk pada kunci terurut, sehingga
   hasilnya lengkap dan urutannya stabil.

## Batas format tabel

Tabel dibaca sebagai CSV sederhana: baris pertama adalah header, pemisah kolom
adalah koma, dan baris wajib punya minimal dua kolom. Isi tabel yang tidak
memenuhi aturan ditolak dengan nomor baris, bukan dilewati diam-diam.

## Alternatif yang dipertimbangkan

- **Chunking blok per baris tanpa tree** — memberi sebagian dedup, tetapi tidak
  memberi struktur untuk query, merge, dan penelusuran range.
- **Delta penuh antar commit** — delta harus diverifikasi terhadap base; kalau
  base hilang, audit trail ikut rusak.
- **B-tree dengan balancing** — memberi penelusuran lebih cepat, tetapi
  pembuktian kebenaran balancing jauh lebih besar dari yang dibutuhkan sekarang.
- **Protobuf/Avro untuk baris** — menambah dependensi dan kompatibilitas versi
  skema; format kanonik sendiri sudah cukup.

## Konsekuensi

- Baris yang tidak berubah tidak ditulis ulang; blok daun dipakai ulang lintas
  commit. Batas ini menghapus utang teknis yang tercatat di ADR-0005.
- `verge diff` menghasilkan perubahan per baris dengan urutan stabil.
- Snapshot yang sudah ditulis pada versi 0.1.0 berupa blok mentah dan **tidak**
  dapat dibaca sebagai tree; `verge show` dan `verge log` pada repository lama
  harus diimpor ulang. Ini adalah perubahan format yang tercatat di CHANGELOG.
- Beban memori saat commit tetap sebanding dengan ukuran tabel karena seluruh
  tabel diurai di memori sebelum dipartisi.

## Justifikasi

Kebutuhan

- dedup di tingkat baris agar perubahan kecil tidak menulis ulang tabel,
- diff per baris yang dapat diverifikasi,
- format yang dapat dibaca ulang tanpa state eksternal.

Solusi ini memenuhi ketiganya dengan objek yang sama (blok content-addressed)
dan tanpa menambah konsep penyimpanan baru.

## Tanggal

2026-10-03

## Penulis

Miruameli

## Review Date

2026-11-03
