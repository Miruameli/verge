# Kebijakan Keamanan

## Melaporkan kerentanan

Laporkan kerentanan lewat **GitHub Security Advisory** pada repository ini
(`Security` → `Report a vulnerability`), bukan issue publik. Isi laporan minimal:

- Deskripsi masalah dan dampaknya.
- Langkah reproduksi yang minimal.
- Versi atau commit yang terdampak.
- Saran perbaikan bila Anda punya.

Kami menjamin balasan dalam 7 hari kerja dan akan mengoordinasikan waktu
perbaikan bersama pelapor sebelum publikasi.

## Klasifikasi dan target waktu

| Severity | Target perbaikan |
| -------- | ----------------- |
| Critical | 24 jam           |
| High     | 72 jam           |
| Medium   | 1 minggu         |
| Low      | 1 bulan          |

## Prinsip yang dijaga di proyek ini

- **Storage immutable.** Objek yang sudah ditulis tidak pernah berubah, dan
  identifier-nya adalah hash SHA-256 dari isinya sehingga manipulasi dapat
  dideteksi.
- **Branch adalah pointer.** Berpindah branch tidak pernah menyalin atau menulis
  ulang data historis.
- **Tidak ada secret di repository.** `gitleaks` berjalan di pre-commit dan CI.
- **Ketergantungan diaudit otomatis.** `cargo audit` dan dependabot menjaga
  advisory baru ditangani cepat.
- **`unsafe` dilarang** di seluruh crate engine (`forbid(unsafe_code)`).

## Laporan insiden publik

Insiden yang sudah diperbaiki dan berdampak pada data pengguna dipublikasikan
sebagai post-mortem blameless di `docs/engineering/audit.md`.
