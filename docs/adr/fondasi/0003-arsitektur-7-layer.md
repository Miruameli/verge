# ADR-0003: Arsitektur 7 layer

## Status

Accepted

## Konteks

Produk akan tumbuh dari engine tunggal menjadi server, SDK, dan Web UI. Tanpa
batas layer yang eksplisit, logika bisnis akan bocor ke adapter I/O dan sulit
diuji.

## Keputusan

Kode mengikuti 7 layer standar: `domain`, `application`, `infrastructure`,
`presentation`, `interfaces`, `shared`, dan `config`.

- `domain` tidak melakukan I/O; seluruh kebutuhan I/O dinyatakan sebagai port.
- `application` hanya mengorkestrasi port, sehingga dapat diuji tanpa filesystem.
- `infrastructure` mengimplementasikan port.
- `interfaces` (CLI) memanggil use case, bukan domain secara langsung.
- Folder maksimal 5 berkas langsung dan 5 subfolder; berkas maksimal 150 baris.

## Alternatif yang dipertimbangkan

- **Modul datar per crate** — cepat, tetapi batas tanggung jawab kabur seiring
  bertambahnya fitur.
- **Hexagonal architecture tanpa batas folder ketat** — arah ketergantungan sudah
  benar, tetapi folder cepat menjadi sulit dinavigasi.
- **Microservice sejak awal** — tidak relevan; yang dibutuhkan adalah batas yang
  jelas, bukan proses terpisah.

## Konsekuensi

- Unit test dapat berjalan tanpa I/O eksternal.
- Menambah backend S3 atau GCS berarti menambah implementasi port, bukan mengubah
  use case.
- Struktur folder lebih banyak; navigasi perlu disiplin, tetapi setiap berkas
  punya satu tanggung jawab yang jelas.
- Layer `presentation/` belum berisi kode karena belum ada UI; tidak ada berkas
  placeholder yang menunggu.

## Justifikasi

Modularisasi adalah pendekatan utama proyek ini. Layer eksplisit dengan batas folder
menjaga kompleksitas tetap lokal dan mudah diuji.

## Tanggal

2026-10-03

## Penulis

Miruameli
