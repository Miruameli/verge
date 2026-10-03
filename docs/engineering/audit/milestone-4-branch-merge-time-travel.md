# Audit: Milestone 4 — branch, merge, dan time travel

## 2026-10-03 — Milestone 4: branch O(1), merge tiga arah, dan `AS OF`

| Field    | Nilai                                                                 |
| -------- | --------------------------------------------------------------------- |
| Waktu    | 2026-10-03                                                            |
| Aksi     | Merge PR #23 (branch), #24 (merge), #26 (time-travel + tag) ke `main`  |
| Pelaku   | Miruameli                                                             |
| Alasan   | Menutup issue #22 dan #25; janji "branching O(1)", "3-way merge", dan "time-travel query" belum terpenuhi tanpa ketiganya |
| Terkait  | Issue #22, #25, PR #23, #24, #26, ADR-0007, ADR-0008, ADR-0009        |
| Dampak   | `verge branch`, `verge merge`, `verge query --as-of`, dan `verge tag` bekerja; time-travel dapat dijawab tanpa mencari commit id |
| Rollback | `git revert` masing-masing commit merge; blok lama tetap terbaca karena content-addressed |

### Bukti

- 306 test lulus setelah review; `clippy -D warnings` dan `cargo fmt` bersih; CI
  hijau pada ketiga PR (`format`, `lint`, `test`, `audit`, `secrets`).
- Smoke run branch: `verge branch create eksperimen` menambah pointer tanpa
  menambah blok data; `branch switch` menulis `HEAD` lengkap.
- Smoke run merge: `verge merge eksperimen --table users --strategy ours`
  melaporkan `merged eksperimen into main at 66ed2fc47af2 (3 rows, strategy ours)`.
- Smoke run time travel: `tag create q3` → `query --as-of q3` mengembalikan isi
  saat tag dibuat; `query --as-of 2026-10-01T00:00:00Z` sebelum commit pertama
  ditolak dengan pesan yang menyebut waktu commit tertua.

### Cacat yang ditemukan saat review dan diperbaiki dalam PR yang sama

1. `looks_like_time` menganggap teks apa pun dengan karakter ke-5 `-` sebagai
   waktu, sehingga branch `2026-q1-report` yang berhasil dibuat tidak lagi bisa
   dibaca `show` maupun `query`. Hanya prefiks `@` dan awalan `YYYY-MM-DD` yang
   kini dialihkan ke parser waktu.
2. `tag create` tidak atomik: read-lalu-tulis lalu `fs::rename` yang menimpa
   pointer, sehingga dua proses dapat bergantian menggeser tag. Sekarang memakai
   `create_new`.
3. Batas 10.000 commit melaporkan batas penelusuran sebagai "oldest commit"
   padahal ada commit lebih lama. Sekarang melaporkan `SearchLimitReached`.
4. Nama pointer lebih dari 255 byte mencapai `fs::write` dan muncul sebagai
   error I/O mentah. Sekarang ditolak sebagai `InvalidName`.

### Temuan proses

- `FileTagPointer` tidak pernah punya test adapter sama sekali meski seluruh
  test use case memakai `FakeWorld`. Test adapter ditambahkan pada PR #26.
- Bukti bahwa test baru benar-benar menangkap perilaku diambil dengan mutasi
  kode sengaja: menghapus filter tabel pada `instant_commit_lookup` membuat dua
  test `instant_table_filter_tests` gagal.

### Utang teknis yang dicatat

- `verge query --as-of <TAG>` masih memberi pesan `invalid reference` bila tag
  menunjuk commit tabel lain (issue #31).
- Motor query SQL belum ada; issue #30 merumuskan lexer dan parser SELECT
  sederhana.
