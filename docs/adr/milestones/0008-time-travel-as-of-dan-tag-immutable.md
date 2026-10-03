# ADR-0008: Time-travel `AS OF` dan tag immutable

## Status

Accepted — diterapkan pada v0.3.0.

## Konteks

v0.2.0 sudah bisa membaca isi tabel pada satu revisi (`verge show <REVISION>`)
tetapi hanya bila pengguna mengetahui nama branch, `HEAD~N`, atau awalan hex.
Dua keunggulan produk belum terpenuhi:

1. **Time-travel query.** Menjawab "keadaan tabel pada 2026-10-01 10:00"
   memerlukan pengguna mencatat `commit id` secara manual — justru hal yang
   seharusnya diotomatisasi untuk audit dan debug.
2. **Tag.** Namespace `.verge/refs/tags/` dibuat saat `init` tetapi tidak pernah
   berisi apa pun, jadi tidak ada cara memberi nama pada titik waktu tertentu.

Tiga tantangan teknis yang harus diputuskan lebih dulu:

1. **Commit tidak berurutan pada branch yang sama.** Setelah merge, `HEAD`
   menunjuk commit merge yang timestamp-nya paling baru, tetapi commit dari
   branch yang digabung bisa bertimestamp jauh lebih lama. Memilih "commit
   pertama yang ditemukan lebih lama dari waktu yang diminta" pada penelusuran
   acak akan mengembalikan jawaban yang berbeda tergantung urutan penyimpanan.
2. **Satu waktu bisa cocok dengan beberapa commit.** Dua commit berbeda dapat
   memiliki milidetik yang sama, sehingga "paling baru" tidak selalu unik.
3. **Waktu lokal bersifat ambigu.** `2026-10-01 10:00` tanpa zona waktu berarti
   berbeda pada mesin berbeda, sehingga hasil audit tidak reproducible.

## Keputusan

### `AS OF <TIMESTAMP>` memilih commit paling baru pada rantai first-parent

`AS OF T` memilih commit pertama pada rantai `first-parent` branch aktif yang
memenuhi `timestamp_unix_ms <= T`. Karena penelusuran selalu dimulai dari
`HEAD` dan berhenti pada commit pertama yang memenuhi batas, hasilnya **tidak
bergantung pada urutan penyimpanan maupun pada branch mana yang lebih dulu
dibaca**.

Rantai `first-parent` dipilih, bukan seluruh DAG, dengan alasan: rantai
`first-parent` adalah urutan keadaan yang benar-benar pernah aktif pada branch
tersebut. Commit dari branch yang sudah digabung bukan "keadaan branch ini pada
waktu itu" — ia baru menjadi bagian dari branch pada saat commit merge dibuat.

### Tie-break ditentukan, bukan acak

Bila beberapa commit memenuhi batas dengan milidetik yang sama, dipilih yang
paling dekat dengan `HEAD`. Aturan ini dapat diulang: input yang sama pada
repository yang sama selalu menghasilkan commit yang sama.

### Hanya UTC

Bentuk yang diterima:

| Bentuk                        | Contoh                     |
| ----------------------------- | -------------------------- |
| RFC 3339 dengan offset `Z`    | `2026-10-01T10:00:00Z`     |
| RFC 3339 dengan milidetik     | `2026-10-01T10:00:00.123Z` |
| Unix milidetik didahului `@`  | `@1767225600000`           |

Offset selain nol ditolak. Konversi offset memerlukan tabel zona waktu yang
tidak dimiliki engine, dan menerima waktu lokal membuat hasil audit bergantung
pada mesin pembaca. Yang ditolak adalah zona waktu, bukan presisi: waktu dalam
milidetik sejak epoch adalah bilangan bulat yang tidak bergantung pembaca.

### Tag immutable

Tag adalah pointer seperti branch, dengan dua perbedaan:

1. `verge tag create` **menolak** nama yang sudah ada. Tag tidak pernah digeser
   diam-diam ke commit lain, karena tag yang berubah artinya berubah makna
   audit trail: laporan yang menyebut `tag q2-report` harus tetap menunjuk
   keadaan yang sama .
2. `verge tag delete` menghapus pointer, bukan blok, sehingga data tetap
   dapat dibaca lewat commit-id.

Tag memakai `branch_name_policy` yang sama karena keduanya menjadi nama berkas
di `refs/`.

### `--as-of` menerima tiga bentuk

Satu flag untuk tiga bentuk agar tidak ada jalur resolusi yang berbeda:

| Masukan              | Perlakuan                                     |
| -------------------- | --------------------------------------------- |
| `2026-10-01T10:00:00Z` | dipilah sebagai timestamp                    |
| `@1767225600000`     | dipilah sebagai timestamp                    |
| `refs/tags/<nama>`   | tag eksplisit, tanpa ambiguitas nama           |
| `refs/heads/<nama>`  | branch eksplisit                              |
| nama lain            | branch bila ada, tag bila tidak — ambigu bila keduanya ada |

Prefiks `refs/` tersedia justru supaya ambiguitas dapat diselesaikan secara
eksplisit oleh pengguna, bukan ditebak resolver.

### Satu tabel per pemanggilan

`--as-of` berlaku untuk satu tabel, sama seperti `verge show`. Menjawab
"keadaan semua tabel" berarti harus konsisten satu waktu di
seluruh tabel, yang berbeda masalah dan layak terpisah.

## Alternatif yang dipertimbangkan

- **Mencari seluruh DAG, bukan `first-parent`.** Ditolak: commit dari branch
  yang sudah digabung akan muncul sebagai "keadaan" pada waktu sebelum ia
  bergabung, sehingga `AS OF` melaporkan keadaan yang tidak pernah aktif pada
  branch tersebut.
- **Menyimpan indeks commit per waktu.** Ditolak: index harus diperbarui setiap
  commit dan dapat rusak, sedangkan `first-parent` sudah memberi jawaban dengan
  batas yang jelas.
- **Tag mutable dengan `tag create --force`.** Ditolak: label yang bisa berubah
  membuat laporan yang menyebut `tag q2-report` menunjuk keadaan berbeda dari
  yang pernah dibaca. Bila memang perlu, pengguna membuat tag baru.
- **Menerima offset waktu dan konversi ke UTC.** Ditolak: membutuhkan basis
  data zona waktu di dalam engine; batasannya lebih besar daripada nilai yang
  didapat untuk CLI lokal yang menyimpan waktu dalam milidetik Unix.
- **Timestamp dari `SystemTime::now()` tanpa validasi.** Ditolak: `AS OF`
  menerima input pengguna, jadi harus divalidasi sebelum dipakai. Nilai yang
  lebih besar dari `i64::MAX/1000` ditolak agar konversi ke milidetik tidak
  overflow.

## Konsekuensi

- `AS OF` menjadi deterministik sepenuhnya: input yang sama pada repository
  yang sama selalu menghasilkan commit yang sama.
- Commit dengan timestamp identik tidak dapat dibedakan lewat `AS OF`; tag
  eksplisit (`refs/tags/<nama>`) adalah jalan keluarnya, dan itu memang
  tujuannya.
- `verge tag list` perlu menampilkan commit yang ditunjuk tag supaya pengguna
  dapat memastikan tag tidak menyesatkan.
- Tag tidak bisa dipakai sebagai nama branch, jadi `refs/` prefix eksplisit
  tetap tersedia walaupun tidak wajib dipakai pada pemakaian normal.

## Amandemen ADR-0007

Bagian "Merge base" pada ADR-0007 telah superseded oleh ADR-0009: merge base
kini mengikuti seluruh parent, bukan hanya rantai `first-parent`. Keputusan itu
dicatat sebagai ADR tersendiri agar ADR-0007 tidak lagi dibaca sebagai
spesifikasi merge base yang berlaku.

## Justifikasi

Time-travel tanpa `AS OF` berarti pengguna harus mencari `commit id` secara
manual setiap kali ingin tahu keadaan lampau — dan justru pemeriksaan yang paling
sering dilakukan saat audit. Tag immutable menutup celah kedua: memberi nama
pada titik waktu tanpa risiko label berubah makna diam-diam.

## Tanggal: 2026-10-03
## Penulis: Miruameli
## Review Date: 2026-12-03