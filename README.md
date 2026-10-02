# Verge — Git for your data

Verge adalah database versioned: setiap perubahan adalah **commit**, setiap
eksperimen adalah **branch**, dan kolaborasi data diselesaikan lewat **diff** dan
**merge**. Tidak ada lagi sistem audit trail manual atau salinan database untuk
eksperimen.

```
Verge repository = Prolly tree (immutable blocks) + Commit graph (DAG)
Branch          = satu pointer ke sebuah commit, bukan salinan data
Time travel     = baca state lewat commit/tag yang sudah tercatat
```

## Status

Milestone 1 — fondasi storage dan versioning. Yang sudah berfungsi hari ini:

| Kemampuan                  | Status                                       |
| -------------------------- | -------------------------------------------- |
| Repository bootstrap       | Selesai — `verge init`                        |
| Block store content-addressed | Selesai — deduplikasi + tulis atomik        |
| Commit graph (branch/tag)  | Selesai — pointer bergerak, data tidak disalin |
| Row-level diff             | Rencana — M3                                 |
| Time-travel query          | Rencana — M4                                 |
| 3-way merge                | Rencana — M4                                 |
| SQL + ekstensi Verge       | Rencana — M5                                 |

Roadmap lengkap: [`docs/roadmap.md`](docs/roadmap.md).

## Mulai Cepat

```bash
cargo build --release
./target/release/verge init /tmp/contoh
```

Menghasilkan:

```
/tmp/contoh/.verge
├── HEAD              # ref: refs/heads/main
├── objects/          # blok immutable: objects/ab/cd/<sha256>
└── refs/
    ├── heads/        # pointer branch (movable)
    └── tags/         # pointer tag (immutable)
```

## Repository

```
crates/
├── verge-core/       # engine: domain, application, infrastructure, config
└── verge-cli/        # antarmuka command-line (`verge`)
docs/
├── architecture.md   # peta layer dan aturan ketergantungan
├── adr/              # keputusan arsitektur (tidak diubah, hanya disusul)
└── engineering/      # audit trail dan decision log
```

Arsitektur memakai 7 layer standar;dependensi hanya boleh mengalir ke dalam:

```
interfaces → application → domain
infrastructure ────────────► domain
config, shared ────────────► semua layer
```

Detail dan alasannya: [`docs/architecture.md`](docs/architecture.md).

## Prinsip Desain

1. **Immutable dan content-addressed.** Objek tidak pernah berubah setelah ditulis;
   namanya adalah hash SHA-256 dari isinya, sehingga bisa diverifikasi tanpa
   mempercayai storage.
2. **Branching O(1).** Branch adalah pointer; membuat atau berpindah branch tidak
   pernah menyalin data.
3. **Deterministik.** Encoding kanonik dijamin, sehingga konten identik selalu
   menghasilkan identifier identik — prasyarat deduplikasi dan merge yang dapat
   diverifikasi.
4. **Ketergantungan minimal.** `verge-core` hanya memakai satu dependensi eksternal
   (`sha2`). Tanpa itu, audit trail dan surface serangan ikut bertambah.

## Pengembangan

```bash
cargo fmt --all --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
gitleaks detect
```

Semua perintah di atas wajib hijau sebelum PR di-merge; CI menegakkannya.

## Kontribusi

Ikuti [`CONTRIBUTING.md`](CONTRIBUTING.md): setiap perubahan harus punya issue,
branch terpisah, dan PR yang melewati seluruh quality gate.
## Lisensi

Apache-2.0 — lihat [`LICENSE`](LICENSE).
