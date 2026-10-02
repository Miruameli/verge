# Roadmap

Target bersifat perkiraan dan dapat berubah; perubahan besar perlu ADR baru.

## M1 — Content-addressed storage dan commit graph (selesai)

- Block store immutable dengan deduplikasi dan tulis atomik.
- Commit immutable dengan identifier turunan encoding kanonik.
- Branch sebagai pointer bergerak, tag sebagai pointer immutable.
- `verge init` untuk membuat repository.
- Quality gate penuh: format, lint, test, audit dependensi, deteksi secret.

## M2 — Tree data dan snapshot tabel

- Prolly tree untuk baris tabel terurut dan deterministik.
- Pembuatan snapshot tree dari sekumpulan tabel.
- Perintah `verge commit` dan `verge log`.
- Penyimpanan dan pembacaan refs (`refs/heads`, `refs/tags`) dari disk.

## M3 — Diff

- Diff row-level dan column-level antar dua commit.
- `verge diff <ref-a>..<ref-b>` dengan output stabil yang bisa diuji.

## M4 — Merge dan time travel

- Three-way merge dengan base dari merge-base.
- Strategi resolusi: manual, ours, theirs, last-write-wins, custom resolver.
- Query `AS OF <commit | tag | timestamp>`.

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
