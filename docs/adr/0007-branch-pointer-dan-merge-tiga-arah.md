# ADR-0007: Branch sebagai pointer dan merge tiga arah

## Status

Accepted — diterapkan bertahap. Branch (pointer O(1)) pada v0.3.0; merge tiga
arah menyusul pada rilis yang sama.

## Konteks

Pada v0.2.0 repository hanya punya satu branch: `HEAD` menunjuk `main` dan
pointer branch tidak pernah ditulis ulang. Janji produk "setiap eksperimen adalah
branch" belum dapat ditepati karena tidak ada tempat lain untuk bereksperimen.

Dua batasan yang harus dipenuhi:

1. **Branching harus O(1).** Copy data per branch membuat ukuran repository
   tumbuh sebanding dengan jumlah branch dan experiment, yang bertentangan dengan
   model content-addressed yang sudah dibangun di ADR-0002 dan ADR-0006.
2. **Merge harus dapat diaudit.** Hasil merge harus dapat dijelaskan kembali
   ke asal-usulnya: base, sisi ours, dan sisi theirs.

## Keputusan

### Branch = pointer

Branch disimpan sebagai satu berkas berisi 64 hex di `refs/heads/<nama>`.
Membuat branch hanya menulis satu berkas kecil; tidak ada blok tabel yang
disalin atau ditulis ulang. Menghapus branch menghapus pointer saja, bukan
blok, sehingga commit yang sudah dibagikan masih dapat dibaca lewat identifier
dan audit trail tetap utuh.

### Batas nama branch

Nama branch menjadi nama berkas, jadi divalidasi sebagai satu segmen path yang
aman di semua platform: bukan kosong, tidak diawali titik, tidak memuat
separator path, karakter terlarang Windows (`<>:"|?*`), karakter kontrol,
tidak diakhiri titik atau spasi, dan bukan nama device tercadang (`con`, `nul`,
`com1`, …). Modul `domain/commit/value-objects/branch_name_policy.rs` menyimpan
kebijakan ini sebagai fungsi murni agar dapat diuji tanpa `Result`.

### Merge base

Merge base adalah commit pertama yang menjadi leluhur dari kedua ujung branch,
dihitung dengan berjalan pada rantai `first-parent` keduanya. Ini mengikuti
sejarah linear yang dipakai `verge commit`; bila suatu hari commit bisa punya
dua parent dari proses non-merge, penelusuran harus diperluas ke seluruh DAG dan
kebijakan "base terdekat" perlu keputusan tersendiri.

### Konflik

Baris digabung menurut kunci.Satu sisi yang tidak berubah dari base tidak pernah
menjadi konflik. Konflik hanya terjadi bila `ours` dan `theirs` sama-sama
mengubah nilai yang sama dari nilai base yang sama. Penghapusan baris yang
diubah sisi lain dianggap konflik, bukan diam-diam diabaikan.

### Strategi resolusi

| Strategi         | Perilaku                                                          |
| ---------------- | ----------------------------------------------------------------- |
| `manual`         | mencetak daftar konflik dan keluar tanpa menulis commit            |
| `ours`           | mempertahankan nilai branch aktif                                  |
| `theirs`         | mengambil nilai branch yang digabung                               |
| `last-write-wins`| mengambil sisi dengan `timestamp_unix_ms` terbaru                  |

## Alternatif yang dipertimbangkan

- **Copy tabel per branch.** Ditolak: melanggar O(1) dan menggandakan penyimpanan.
- **Branch sebagai entri di dalam satu berkas refs.** Ditolak: satu berkas
  bersama berarti dua operasi berhadapan mengunci berkas yang sama; berkas per
  branch membuat `create` dan `delete` tidak saling ganggu.
- **Merge dua arah tanpa base.** Ditolak: tanpa base tidak mungkin membedakan
  "diubah di satu sisi" dari "diubah di kedua sisi", sehingga setiap perbedaan
  menjadi konflik palsu.
- **Delta penuh antar commit sebagai pengganti tree.** Ditolak di ADR-0006:
  delta harus diverifikasi terhadap base, dan hilangnya base merusak audit trail.

## Konsekuensi

- Operasi branch tetap O(1) terukur: membuat branch tidak menambah blok data
  sama sekali.
- `HEAD` tidak lagi hanya nama branch; `switch` menulis ulang `HEAD` secara
  atomik dengan nama berkas sementara yang tidak pernah dibaca sebagai branch.
- Branch yang dihapus tidak lagi bisa menjadi acuan nama, jadi pengguna perlu
  menyimpan identifier commit bila ingin merujuk commit tersebut nanti.
- Merge memerlukan pembacaan tiga versi tabel; conflicted merge tidak menulis
  apa pun sehingga repo tetap bisa di-recovery.

## Justifikasi

Janji "Git for your data" hanya terbukti kalau branch dan merge benar-benar
berfungsi; tanpa keduanya, fitur diff yang sudah selesai hanya bisa membandingkan
sejarah linear.

## Tanggal
## Penulis
## Review Date: 2026-12-03