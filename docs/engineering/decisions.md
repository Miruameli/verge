# Decision Log

Catatan keputusan yang lebih ringan daripada ADR. Keputusan yang mengubah
arsitektur dicatat penuh di `docs/adr/`.

## 2026-10-03 — Mulai dari nol dengan arsitektur 7 layer

**Keputusan:** repository dibangun ulang dari kosong dengan struktur 7 layer sejak
commit pertama, bukan ditambal setelah fitur pertama selesai.

**Alasan:** biaya memindahkan struktur layer bertambah cepat seiring waktu karena
setiap fitur baru pasti menyentuh folder yang keliru. Aturan modularisasi harus
berlaku sejak mulai.

**Alternatif:** modul datar per crate dengan konvensi nama.

**Konsekuensi:** lebih banyak berkas kecil sejak awal; navigasi harus disiplin.

## 2026-10-03 — `Result` memakai parameter error default

**Keputusan:** engine memakai `pub type Result<T, E = VergeError>`, CLI memakai
`pub type Result<T, E = anyhow::Error>`.

**Alasan:** library tetap memiliki error bertipe ketat, sedangkan batas aplikasi
membutuhkan error dinamis yang membawa context. Keduanya tetap menyediakan escape
hatch lewat parameter kedua.

**Konsekuensi:** call site library dapat menulis `Result<T>` tanpa mengulang nama
error, dan test dapat menuliskannya secara eksplisit.

## 2026-10-03 — Bootstrap repository memakai port, bukan `std::fs`

**Keputusan:** use case `initialize_repository` hanya berbicara dengan port
`MetadataWriter` dan `BlockStoreFactory`.

**Alasan:** use case dapat diuji tanpa filesystem, termasuk skenario rollback dan
kegagalan I/O yang sulit direproduksi pada disk sungguhan.

**Konsekuensi:** ada satu trait tambahan di domain, tetapi seluruh operasi I/O
tetap berada di infrastructure.
