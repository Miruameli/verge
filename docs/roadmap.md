# Roadmap

Target bersifat perkiraan dan dapat berubah; perubahan besar perlu ADR baru.

## M1 — Content-addressed storage dan commit graph (selesai)

- Block store immutable dengan deduplikasi dan tulis atomik.
- Commit immutable dengan identifier turunan encoding kanonik.
- Branch sebagai pointer bergerak, tag sebagai pointer immutable.
- `verge init` untuk membuat repository.
- Quality gate penuh: format, lint, test, audit dependensi, deteksi secret.

## M2 — Versioning data tabel (selesai)

- Isi tabel disimpan sebagai blok content-addressed; pointer `tables/<nama>/working`
  hanya menyimpan digest sehingga data tidak pernah ada di dua tempat.
- Commit dan pointer branch disimpan sebagai blok yang diverifikasi ulang saat
  dibaca; byte yang dimanipulasi ditolak.
- `verge import`, `verge commit`, `verge log`, dan `verge show` untuk time-travel read.
- Batasan yang diterima dan dicatat di ADR-0005: satu commit menyimpan satu blok
  penuh per tabel; deduplikasi per baris menunggu prolly tree.

## M3 — Prolly tree dan diff (selesai)

- Prolly tree untuk baris tabel terurut dan deterministik; daun yang tidak
  berubah dipakai ulang antar commit.
- Diff row-level antar dua commit, termasuk kolom yang berubah pada baris sama.
- `verge diff <rev-a>..<rev-b>` dengan output stabil, plus resolusi `HEAD~N` dan
  awalan commit.

## M4 — Branch, merge, dan time travel (selesai)

- Branch `create`/`switch`/`list`/`delete` yang O(1) tanpa menyalin data — selesai.
- Three-way merge dengan base dari merge-base — selesai.
- Strategi resolusi: manual, ours, theirs, last-write-wins — selesai; custom
  resolver menyusul bersama API query.
- Query `AS OF <commit | tag | timestamp>` — selesai.

## M5 — Query engine

- Parser dan planner SQL standar beserta ekstensi Verge.
- WASM UDF dan Python UDF tanpa restart server.
- Executor dengan batas memori dan waktu.

## M6 — Server, SDK, dan Web UI

- Server dengan gRPC, HTTP, dan protokol wire PostgreSQL.
- SDK Python (pyo3) dan Rust.
- Web UI untuk commit graph, diff viewer, dan conflict resolver.

## Lintas milestone

- Backend object storage S3 dan GCS.
- Ownership dan garbage collection berbasis reachability.
- Rilis dengan checksum, SBOM, dan artefak lintas platform.
