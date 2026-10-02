# Kontribusi ke Verge

Terima kasih sudah membaca. Dokumen ini menjelaskan alur kerja yang wajib diikuti.

## Prasyarat

- Rust toolchain sesuai `rust-toolchain.toml` (dipasang lewat `rustup`).
- `gitleaks` untuk deteksi secret lokal.
- `gh` CLI untuk seluruh operasi GitHub.

## Alur kerja wajib

1. **Issue lebih dulu.** Setiap perubahan kode dimulai dari issue yang menjelaskan
   masalah, dampak, dan prioritas. PR tanpa closing issue akan ditolak.
2. **Branch terpisah** dari `main`:

   | Jenis    | Prefix      | Contoh                   |
   | -------- | ----------- | ------------------------ |
   | Fitur    | `feat/`     | `feat/prolly-tree`       |
   | Perbaikan | `fix/`     | `fix/atomic-block-write` |
   | Refaktor | `refactor/` | `refactor/commit-graph`  |
   | Docs     | `docs/`     | `docs/architecture`      |
   | Chores   | `chore/`    | `chore/dependabot`       |

3. **Commit Conventional Commits** dengan subject imperatif ≤72 karakter:

   ```text
   feat(storage): tulis blok secara atomik dengan rename

   Blok yang belum selesai tertulis bisa dibaca sebagai data rusak.
   Rename setelah fsync membuat blok hanya terlihat utuh.
   ```

4. **Quality gate lokal** — semuanya wajib hijau:

   ```bash
   cargo fmt --all --check
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all
   gitleaks detect
   ```

5. **Self-review** sebelum membuka PR: baca ulang diff seolah-olah Anda orang lain,
   periksa kode mati, duplikasi, dan test yang hanya menguji wiring.
6. **PR** berisi Problem, Approach, Alternatives, Risks/Rollback, dan Verification.
   Sertakan `Closes #<issue>`.
7. **Merge** hanya setelah CI hijau dan checklist PR terpenuhi.

## Aturan kualitas kode

- Maksimal **150 baris per berkas**, **5 berkas langsung per folder**, dan
  **5 subfolder per folder**. Struktur layer dijelaskan di
  [`docs/architecture.md`](docs/architecture.md).
- Setiap berkas punya header comment: `File`, `Deskripsi`, `Layer`,
  `Tanggung jawab`, `Author`, `Created`, `Modified`, `Version`, `License`,
  `Dependencies`, `Related issues`, `Related ADR`.
- Setiap fungsi publik, struct, dan trait punya doc comment yang menyebut argumen,
  return, error, dan contoh bila relevan.
- `TODO`/`FIXME`/`HACK` wajib menyertakan nomor issue; tanpa itu PR ditolak.
- Kode tidak boleh melewati layer secara langsung: presentation menyentuh
  application, application dan infrastructure menyentuh domain, tidak sebaliknya.

## Test

- Unit test untuk logika domain, integration test untuk jalur storage end-to-end,
  dan E2E test untuk perilaku CLI lewat binary sungguhan.
- Test wajib deterministik: tanpa ketergantungan jaringan, jam sistem, atau urutan
  eksekusi. Direktori sementara memakai nama unik per proses.

## ADR

Keputusan arsitektur yang signifikan ditulis sebagai ADR baru di `docs/adr/`.
ADR lama tidak pernah diedit; koreksi dibuat sebagai ADR baru yang menimpanya.

## Pelaporan kerentanan

Jangan membuka issue publik untuk kerentanan. Ikuti [`SECURITY.md`](SECURITY.md).
