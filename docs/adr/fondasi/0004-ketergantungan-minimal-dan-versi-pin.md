# ADR-0004: Ketergantungan minimal dan versi dipin

## Status

Accepted

## Konteks

Untuk database, setiap dependensi adalah bagian dari permukaan serangan dan dari
beban pemeliharaan. Spesifikasi juga menuntut proyek yang self-hostable dan bebas
vendor lock-in.

## Keputusan

1. `verge-core` hanya memakai satu dependensi eksternal: `sha2` (SHA-256 dari
   RustCrypto).
2. `verge-cli` memakai `anyhow` sebagai batas error aplikasi.
3. Versi dipin di `Cargo.toml` dan `Cargo.lock` ikut di-commit sehingga build
   bersifat reproducible.
4. `cargo audit` (RustSec advisory database) dan dependabot wajib hijau di setiap
   PR.

## Alternatif yang dipertimbangkan

- **Implementasi hashing sendiri (kurang dari 100 baris)** — menghindari
  dependensi, tetapi mengulang primitif kriptografi yang berisiko dan bertentangan
  dengan prinsip tidak membuat algoritma kriptografi sendiri.
- **BLAKE3** — lebih cepat, belum se-mapan SHA-256 untuk keperluan audit, dan
  menambah satu dependensi baru.
- **Tanpa lockfile** — build tidak reproducible dan audit dependensi mustahil.

## Konsekuensi

- Surface serangan engine tetap kecil dan mudah diaudit.
- Upgrade dependensi harus dipertimbangkan dengan sadar; dependabot membantu
  menjaga tugas itu tetap terlihat.
- Hashing dibatasi pada SHA-256; untuk keperluan integritas dan audit hal ini memadai.

## Justifikasi

Kualitas dan keamanan selalu menang atas kecepatan. Satu dependensi yang sudah
teraudit jauh lebih murah daripada beberapa dependensi yang belum tentu aman.

## Tanggal

2026-10-03

## Penulis

Miruameli
