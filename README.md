# Verge — Git for your data

Verge adalah database versioned: setiap perubahan adalah **commit**, setiap
eksperimen adalah **branch**, dan kolaborasi data diselesaikan lewat **diff** dan
**merge**. Tidak ada lagi sistem audit trail manual atau salinan database untuk
eksperimen.

```
Verge repository = Prolly tree (immutable blocks) + Commit graph (DAG)
Branch          = satu pointer ke sebuah commit, bukan salinan data
Time travel     = baca state lewat commit, tag, atau waktu yang sudah tercatat
```

## Status

Milestone 4 — branch, merge tiga arah, dan time-travel `AS OF` selesai. Yang sudah
berfungsi hari ini:

| Kemampuan                     | Status                                              |
| ----------------------------- | --------------------------------------------------- |
| Repository bootstrap          | Selesai — `verge init`                               |
| Block store content-addressed | Selesai — deduplikasi + tulis atomik                |
| Commit graph (branch/tag)     | Selesai — pointer bergerak, data tidak disalin       |
| Commit tabel                  | Selesai — `verge import` + `verge commit`            |
| Riwayat commit                | Selesai — `verge log`                                |
| Time-travel read              | Selesai — `verge show <commit>`                      |
| Row-level diff                | Selesai — `verge diff <FROM>..<TO>`                  |
| 3-way merge                   | Selesai — `verge merge`                               |
| Query `AS OF`                 | Selesai — `verge query --as-of <WHEN>`               |
| Tag immutable                 | Selesai — `verge tag create/list/delete`             |
| SQL + ekstensi Verge          | Rencana — M5                                          |

Rilis terbaru: [`v0.3.0`](https://github.com/Miruameli/verge/releases/tag/v0.3.0) —
binary untuk linux (x86_64, aarch64), macOS (arm64), dan Windows (x86_64),
lengkap dengan `SHA256SUMS` dan SBOM CycloneDX per target.

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
verge diff 67f9a3fa6f9a39b1fafe6fa621ab246b5367c2ea2bac05dbdab7f043e82ccfde..HEAD --table users
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

Perubahan antar dua revisi dibaca per baris, tanpa memulihkan working copy:

```
+ 2 ,budi
```

`verge branch` mengelola branch tanpa menyalin data:

```
$ verge branch create eksperimen
created eksperimen at e48c017fe0c8e0dd8eda332c96bde7c7274f7b6d651e4c77c28c69c6d48954d2
$ verge branch switch eksperimen
switched main -> eksperimen
$ verge branch list
* eksperimen           e48c017fe0c8
  main                 e48c017fe0c8
```

Membuat branch tidak menambah blok data sama sekali: hanya satu berkas pointer
di `.verge/refs/heads/` yang ditulis. Menghapus branch hanya menghapus pointer,
blok dan commit tetap bisa dibaca lewat identifier-nya.

`verge merge` menggabungkan branch memakai merge tiga arah:

```
$ verge branch create eksperimen && verge branch switch eksperimen
$ # ... commit berbeda di eksperimen ...
$ verge branch switch main
$ # ... commit berbeda di main ...
$ verge merge eksperimen --table users --author ana
merged eksperimen into main at 09e000a35de0 (4 rows, strategy manual)
```

Baris yang hanya berubah di satu sisi langsung diambil dari sisi itu. Baris yang
diubah kedua sisi dengan nilai berbeda menjadi konflik; strategi `manual`
mencetak ketiga sisi dan membatalkan merge, sedangkan `ours`, `theirs`, dan
`last-write-wins` menyelesaikannya.

`verge diff` menampilkan perubahan per baris dengan urutan stabil:

```
~ 3 ,citra,surabaya -> ,citra,sidoarjo
+ 4 ,sari,medan
```

Satu baris yang berubah hanya menulis ulang daun tree yang memuat baris itu;
baris lain dan header memakai blok yang sama seperti commit sebelumnya.

`verge query --as-of` menjawab "keadaan tabel pada waktu tertentu" tanpa mencari
commit id secara manual:

```
$ verge tag create q3 --revision HEAD
tagged q3 at HEAD
$ verge query --table users --as-of q3
id,name
1,ana
$ verge query --table users --as-of 2026-10-01T10:00:00Z
id,name
1,ana
2,budi
```

Tag bersifat immutable: `verge tag create` menolak nama yang sudah ada, sehingga
laporan yang menyebut `q3` selalu menunjuk keadaan yang sama. `verge tag list`
mencetak nama, commit, dan tabel yang dimiliki setiap tag, sehingga tag tidak
perlu ditebak sebelum dipakai sebagai `--as-of`.

Layout repository yang dihasilkan:

```
.verge
├── HEAD              # ref: refs/heads/main
├── objects/          # blok immutable: objects/ab/cd/<sha256>
├── refs/
│   ├── heads/        # pointer branch (movable)
│   └── tags/         # pointer tag (immutable)
└── tables/<nama>/working   # digest akar tree, bukan datanya
```

## Repository

```
crates/
├── verge-core/       # engine: domain, application, infrastructure, config
└── verge-cli/        # antarmuka command-line (`verge`)
docs/
├── architecture.md   # peta layer dan aturan ketergantungan
├── adr/              # keputusan arsitektur per fondasi dan milestone
└── engineering/      # audit trail (satu berkas per entri) dan decision log
```

Arsitektur memakai 7 layer standar; dependensi hanya boleh mengalir ke dalam:

```
interfaces → application → domain
infrastructure ────────────► domain
config, shared ────────────► semua layer
```

Detail dan alasannya: [`docs/architecture.md`](docs/architecture.md).

## Prinsip Desain

0. **Tabel sebagai prolly tree.** Isi tabel dipisah menjadi daun berbaris;
   baris yang tidak berubah berbagi blok antar commit, sehingga perubahan kecil
   tidak menulis ulang tabel.
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
