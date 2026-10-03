# ADR-0002: Storage immutable content-addressed

## Status

Accepted

## Konteks

Janji utama Verge adalah audit trail bawaan dan branching tanpa menyalin data.
Keduanya menuntut objek yang tidak pernah berubah dan nama yang bisa diverifikasi
tanpa mempercayai lapisan penyimpanan.

## Keputusan

1. Setiap objek storage (blok, tree, commit) immutable setelah ditulis.
2. Nama objek adalah SHA-256 dari encoding kanonik objek tersebut.
3. Blok disimpan pada layout fan-out `objects/ab/cd/<hex>` supaya satu direktori
   tidak pernah menampung ribuan entri.
4. Penulisan memakai berkas sementara, `fsync`, lalu `rename`, sehingga blok
   tidak pernah terlihat setengah tertulis.
5. Branch adalah pointer bergerak ke sebuah commit; tag adalah pointer immutable.

## Alternatif yang dipertimbangkan

- **Snapshot penuh per branch** — sederhana, tetapi menyalin data dan bertentangan
  langsung dengan klaim "branching O(1)".
- **Copy-on-write per tabel** — kuat, tetapi kompleks dan boros saat banyak branch
  aktif.
- **Object store tanpa content addressing** — butuh metadata eksternal untuk
  integritas, sehingga audit trail tidak bisa diverifikasi mandiri.

## Konsekuensi

- Identitas data dapat diverifikasi kapan saja: hash ulang isi harus menghasilkan
  nama yang sama.
- Deduplikasi otomatis untuk snapshot yang isinya tidak berubah.
- Blok yang tidak lagi terjangkau belum dihapus; pengumpulan blok berdasarkan
  reachability direncanakan terpisah setelah mesin query siap.

## Justifikasi

Content addressing adalah satu-satunya desain yang memberi audit trail,
deduplikasi, dan branching murah sekaligus, tanpa tabel metadata terpisah.

## Tanggal

2026-10-03

## Penulis

Miruameli
