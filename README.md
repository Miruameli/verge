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

Milestone 2 — data tabel sudah bisa di-versioning dan dibaca kembali pada commit
lama. Yang sudah berfungsi hari ini:

| Kemampuan                     | Status                                              |
| ----------------------------- | --------------------------------------------------- |
| Repository bootstrap          | Selesai — `verge init`                               |
| Block store content-addressed | Selesai — deduplikasi + tulis atomik                |
| Commit graph (branch/tag)     | Selesai — pointer bergerak, data tidak disalin       |
| Commit tabel                  | Selesai — `verge import` + `verge commit`            |
| Riwayat commit                | Selesai — `verge log`                                |
| Time-travel read              | Selesai — `verge show <commit>`                      |
| Row-level diff                | Rencana — M3                                          |
| 3-way merge                   | Rencana — M4                                          |
| SQL + ekstensi Verge          | Rencana — M5                                          |

Roadmap lengkap: [`docs/roadmap.md`](docs/roadmap.md).

## Mulai Cepat

```bash
cargo build --release
cd /tmp/contoh
printf 'id,name\n1,ana\n' > users.csv

verge init
verge import users.csv --table users
verge commit --table users --message "feat: seed users" --author ana

printf 'id,name\n1,ana\n2,budi\n' > users.csv
verge import users.csv --table users
verge commit --table users --message "feat: add budi" --author budi

verge log --table users
verge show HEAD --table users
```

Hasil `verge log --table users`:

```
5d2ec22025b8 budi feat: add budi
67f9a3fa6f9a ana  feat: seed users
```

Isi tabel pada commit lama tetap dapat dibaca tanpa memulihkan working copy:

```bash
verge show 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde --table users
# id,name
# 1,ana
```

Layout repository yang dihasilkan:

```
.verge
├── HEAD              # ref: refs/heads/main
├── objects/          # blok immutable: objects/ab/cd/<sha256>
├── refs/
│   ├── heads/        # pointer branch (movable)
│   └── tags/         # pointer tag (immutable)
└── tables/<nama>/working   # digest blok data kerja, bukan datanya
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
