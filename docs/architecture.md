# Arsitektur Verge

Dokumen ini menjelaskan bagaimana kode ditata dan mengapa. Keputusan spesifik
di setiap detail ada di `docs/adr/fondasi/` (fondasi) dan `docs/adr/milestones/`
(per milestone); catatan tindakan dan bukti ada di `docs/engineering/audit/`.

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

Arah ini hasil pengukuran otomatis atas seluruh `use` di repo, bukan
kehendak. Hasil pengukuran per 2026-10-04:

| Layer  | Layer yang diimpor                                      |
| ------ | ------------------------------------------------------- |
| domain | `shared`                                                 |
| application | `domain`, `config`, `shared`                         |
| infrastructure | `domain`, `config`, `shared`                    |
| interfaces | `config`, `shared` (+ modul `cli` miliknya sendiri) |
| shared | `domain` (hanya `Digest` pada varian galat)             |
| config | `domain` (hanya `TableName`, `BlockId`, `HexText`)      |

Dua baris terakhir adalah pengecualian yang disengaja: `shared/exceptions`
memakai `Digest` sebagai isi galat dan `config/*` memvalidasi `TableName`,
`BlockId`, serta `HexText` saat memetakan path. Keduanya hanya menyentuh
value object murni tanpa I/O maupun layanan, dan memindahkannya hanya akan
menyalin tipe. Yang tetap dilarang: `domain` mengimpor `application`,
`infrastructure`, `interfaces`, atau `config` — termasuk di berkas test.
Karena itu test merge base yang memakai adapter filesystem dipindahkan ke
`crates/verge-core/tests/merge/`.

Tidak ada lingkaran dependensi modul: graf `use` di seluruh repo dianalisis
tanpa menemukan satu pun siklus. Dependensi antar crate juga sepele,
`verge-cli` hanya bergantung pada `verge-core`, dan `verge-core` tidak
bergantung pada crate lain di workspace.

`domain` tidak boleh tahu apa pun tentang filesystem, jaringan, atau format
on-disk. Semua kontrak I/O dinyatakan sebagai trait (port) yang diimplementasikan
di `infrastructure`. Inilah yang membuat backend S3/GCS bisa menyusul tanpa
menyentuh logika bisnis.

## Konvensi folder

- Folder memakai kebab-case (`value-objects/`, `file-system/`), nama modul
  memakai snake_case lewat atribut `#[path]`.
- Batas **5 berkas langsung** dan **5 subfolder** per folder adalah target, bukan
  angka mutlak: penyimpangan diterima bila setiap isi folder merupakan konteks
  terpisah yang tidak dapat digabung tanpa mengaburkan tanggung jawab. Saat ini
  `domain/` memuat tujuh konteks (commit, ident, merge, storage, table, time,
  tree) dan tetap lebih mudah dinavigasi daripada dipaksa digabung.
- Batas **150 baris per berkas** tetap mengikat; berkas yang perlu lebih besar
  dipecah per tanggung jawab (contoh: `commit_graph.rs` + `commit_history.rs`).
- Setiap folder punya `mod.rs` sebagai satu-satunya titik deklarasi modulnya.
- `docs/adr/` memakai subfolder bernomor (`fondasi/`, `milestones/`), bukan
  satu folder datar: registri bernomor tetap berurutan global (0001–0009),
  sementara subfolder menjaga tiap folder di bawah lima berkas. Pemindahan ADR
  lama ke subfolder tidak mengubah isi ADR (DILARANG ubah ADR lama tetap
  berlaku untuk isi, bukan lokasi berkas).
- `docs/engineering/audit/` memakai satu berkas per entri dengan `audit.md` sebagai
  indeks; audit tumbuh bersama waktu sehingga satu berkas datar akan cepat
  melewati batas baris.

## Peta kode

| Path                                  | Tanggung jawab                                        |
| ------------------------------------- | ------------------------------------------------------ |
| `domain/ident/value-objects/`         | Digest SHA-256 dan representasi hexnya                 |
| `domain/commit/entities/`             | Entitas `Commit` beserta pembaca fieldnya              |
| `domain/commit/codec/`                | Encoding kanonik dan decoding yang memverifikasi digest |
| `domain/commit/value-objects/`        | `CommitId`, `Ref` (branch/tag/commit)                  |
| `domain/commit/repositories/`         | `CommitGraph`, traversal sejarah, dan port commit/ref/tag |
| `domain/time/value-objects/`          | `Timestamp` UTC, kalender civil, dan parsing RFC 3339 |
| `domain/table/`                       | `TableName` dan port data kerja tabel                  |
| `domain/tree/`                        | `TableRows`, codec node, builder, reader, dan diff     |
| `domain/tree/nodes/`                  | `TreeNode` (header, daun, internal) dan codec-nya     |
| `domain/merge/`                       | Strategi resolusi (`manual`/`ours`/`theirs`/`last-write-wins`), merge base, dan gabungan baris |
| `domain/storage/ports/`               | `Store`, `BlockStoreFactory`, `MetadataWriter`         |
| `application/repository-bootstrap/`   | Use case pembuatan repository                          |
| `application/version-control/`        | `revision_target`, `revision_resolver`, `revision_walk`, `instant_commit_lookup` |
| `application/version-control/use-cases/refs/branching/` | Use case create, switch, list, delete branch |
| `application/version-control/use-cases/merging/`   | Use case merge: baca sisi, tulis commit merge          |
| `application/version-control/use-cases/refs/tagging/`   | Use case create, list, delete tag (immutable)          |
| `application/version-control/use-cases/queries/`   | Use case `query --as-of <WHEN>` (RFC 3339/`@ms`/tag/commit) |
| `application/version-control/tests/`  | Test lintas use case: instant lookup (`revision/` untuk resolusi revisi, `tagging/` untuk tag) |
| `crates/verge-core/tests/merge/`      | Test integrasi merge base memakai adapter filesystem nyata |
| `infrastructure/storage/file-system/` | `FileBlockStore`, factory, penulis metadata lokal       |
| `infrastructure/commit/file-system/refs/` | Pointer branch/tag: `FileRefPointer`, `FileTagPointer` |
| `infrastructure/commit/file-system/`  | Penyimpanan objek commit (`FileCommitRepository`)      |
| `infrastructure/table/file-system/`   | Pointer akar tree dan pembacaan sumber tabel          |
| `infrastructure/system/`              | Jam sistem untuk cap waktu commit                      |
| `config/`                             | Layout repository dan path blok                        |
| `verge-cli/interfaces/cli/`           | Dispatcher: `init`, `branch`, `merge`, `tag`, bantuan, versi |
| `verge-cli/interfaces/cli/commands/table-versioning/dispatch.rs` | Router perintah tabel: `import`, `commit`, `log`, `show`, `diff`, `query` |

## Invariant yang dijaga

1. Objek yang sudah ditulis tidak pernah berubah; nama objek = hash isinya.
2. Branch adalah pointer bergerak; tag adalah pointer immutable.
3. Commit hanya masuk graph bila seluruh parent-nya sudah ada.
4. Penulisan blok bersifat atomik: temp file + fsync + rename.
5. Bootstrap repository bersifat all-or-nothing: kegagalan apa pun menghapus
   jejak yang sudah dibuat.
6. Isi tabel hanya hidup di dalam block store; berkas di `.verge/tables/` adalah
   pointer digest akar tree, bukan data.
7. Commit yang dimuat dari disk diverifikasi ulang terhadap digest-nya, sehingga
   manipulasi byte di luar Verge terdeteksi saat pembacaan.
8. Nama tabel tervalidasi sebelum menyentuh path: allowlist `[a-z0-9_-]`, maksimal
   64 karakter, tanpa path separator.
9. Nama pointer branch dan tag tervalidasi sebelum menyentuh path: bukan kosong,
   tidak diawali titik, maksimal 255 byte, tanpa separator path, dan bukan nama
   device tercadang Windows.
10. Tag tidak pernah ditimpa: pointer ditulis dengan `create_new` sehingga dua
    proses yang membuat tag dengan nama sama tidak dapat bergantian menulis
    pointer yang sama.
11. Tag menyimpan commit id saja, bukan nama tabel. Tabel suatu tag dibaca dari
    commit yang ditunjuknya; galat karena tabel berbeda menyebut kedua nama
    tabel, bukan melaporkan referensi rusak.

## Model Parents

Commit memakai urutan parent eksplisit. Elemen pertama adalah *first parent* —
branch tempat commit dibuat — dan dipakai saat traversal log. Sisanya adalah
parent tambahan dari merge, sehingga topologi DAG dapat direkonstruksi persis.
