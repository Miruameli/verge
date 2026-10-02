# Arsitektur Verge

Dokumen ini menjelaskan bagaimana kode ditata dan mengapa. Keputusan spesifik
di-each detail ada di `docs/adr/`.

## Layer

```
Layer 1  domain/          Entitas, value object, port (tanpa I/O)
Layer 2  application/     Use case yang mengorkestrasi port
Layer 3  infrastructure/  Implementasi port (filesystem lokal)
Layer 4  presentation/    (milestone berikutnya: HTTP/Web UI)
Layer 5  interfaces/      Entry point: CLI, jobs, event handler
Layer 6  shared/          Kernel dan exception lintas layer
Layer 7  config/          Konstanta layout dan konfigurasi
```

Aturan ketergantungan — satu arah, tanpa lingkaran:

```
interfaces  ──► application ──► domain
infrastructure ──────────────► domain
config, shared ──────────────► semua layer
domain ──► shared (hanya kernel & exception)
```

`domain` tidak boleh tahu-menahu tentang filesystem, jaringan, atau format
on-disk. Semua kontrak I/O dinyatakan sebagai trait (port) yang diimplementasikan
di `infrastructure`. Inilah yang membuat backend S3/GCS bisa menyusul tanpa
menyentuh logika bisnis.

## Konvensi folder

- Folder memakai kebab-case (`value-objects/`, `file-system/`), nama modul
  memakai snake_case lewat atribut `#[path]`.
- Maksimal **5 berkas langsung** dan **5 subfolder** per folder; layer root
  diizinkan 5 berkas dan 10 subfolder.
- Maksimal **150 baris per berkas**; berkas yang perlu lebih besar dipecah per
  tanggung jawab (contoh: `commit_graph.rs` + `commit_history.rs`).
- Setiap folder punya `mod.rs` sebagai satu-satunya titik deklarasi modulnya.

## Peta kode

| Path                                  | Tanggung jawab                                   |
| ------------------------------------- | ------------------------------------------------- |
| `domain/ident/value-objects/`         | Digest SHA-256 dan representasi hexnya            |
| `domain/commit/entities/`             | Entitas `Commit` dan encoding kanonik             |
| `domain/commit/value-objects/`        | `CommitId`, `Ref` (branch/tag/commit)             |
| `domain/commit/repositories/`         | `CommitGraph` dan traversal sejarah               |
| `domain/storage/ports/`               | `Store`, `BlockStoreFactory`, `MetadataWriter`    |
| `application/repository-bootstrap/`  | Use case pembuatan repository                     |
| `infrastructure/storage/file-system/` | `FileBlockStore`, factory, penulis metadata lokal  |
| `config/`                             | Layout repository dan path blok                   |
| `verge-cli/interfaces/cli/`           | Dispatcher perintah dan perintah `init`           |

## Invariant yang dijaga

1. Objek yang sudah ditulis tidak pernah berubah; nama objek = hash isinya.
2. Branch adalah pointer bergerak; tag adalah pointer immutable.
3. Commit hanya masuk graph bila seluruh parent-nya sudah ada.
4. Penulisan blok bersifat atomik: temp file + fsync + rename.
5. Bootstrap repository bersifat all-or-nothing: kegagalan apa pun menghapus
   jejak yang sudah dibuat.

## Model Parents

Commit memakai urutan parent eksplisit. Elemen pertama adalah *first parent* —
branch tempat commit dibuat — dan dipakai saat traversal log. Sisanya adalah
parent tambahan dari merge, sehingga topologi DAG dapat direkonstruksi persis.
