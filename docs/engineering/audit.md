# Audit Trail

Catatan tindakan signifikan terhadap repository ini: waktu, aksi, alasan, pelaku,
terkait issue atau PR, dampak, dan cara rollback.

Setiap entri hidup di berkas sendiri supaya folder tidak menumpuk dan setiap
entri mudah ditelusuri. Indeks ini adalah pintu masuk tunggal; entri baru
menambah berkas di `audit/` dan satu baris tabel di bawah.

## Indeks entri

| Entri                                  | Isi                                                       |
| [`audit/fondasi-m1.md`](audit/fondasi-m1.md) | Bootstrap repository, merge Milestone 1, proteksi branch |
| [`audit/milestone-2-versioning-tabel.md`](audit/milestone-2-versioning-tabel.md) | Import, commit, log, dan time-travel read           |
| [`audit/milestone-3-prolly-tree-diff.md`](audit/milestone-3-prolly-tree-diff.md) | Prolly tree, `verge diff`, dan versi 0.2.0         |
| [`audit/milestone-4-branch-merge-time-travel.md`](audit/milestone-4-branch-merge-time-travel.md) | Branch O(1), merge tiga arah, `AS OF`, dan tag immutable |
| [`audit/rilis/v0.1.0.md`](audit/rilis/v0.1.0.md) | Terbitan pertama, empat binary, checksum, SBOM           |
| [`audit/rilis/v0.3.0.md`](audit/rilis/v0.3.0.md) | Terbitan Milestone 4 beserta pembuktiannya                |

## Aturan penulisan entri

- Setiap entri menyebut waktu, aksi, pelaku, alasan, issue/PR terkait, dampak,
  bukti, dan rollback.
- Bukti outperkosa: jumlah test, hasil quality gate, dan smoke run binary.
  Klaim tanpa bukti tidak ditulis sebagai fakta.
- Entri hanya ditambah, tidak ditulis ulang: kesalahan dikoreksi pada entri
  baru yang menyebut entri lama.
