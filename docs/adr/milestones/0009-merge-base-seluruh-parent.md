# ADR-0009: Merge base mengikuti seluruh parent

## Status

Accepted — diterapkan pada v0.3.0 (PR #24).

## Konteks

ADR-0007 menetapkan merge base dihitung pada rantai `first-parent` kedua branch,
dengan alasan bahwa commit yang dibuat engine selalu linear.

Praktik menunjukkan asumsi itu salah begitu commit merge memiliki dua parent.
Commit hasil merge menyimpan `theirs` sebagai parent kedua, sehingga `theirs`
sudah menjadi leluhur branch aktif. Ketika branch itu digabung lagi, penelusuran
`first-parent` tidak pernah menemukan `theirs` sebagai leluhur, base kembali
menjadi commit paling awal, dan seluruh tabel dianggap berubah — merge kedua
kali menuliskan commit yang isinya tidak berbeda dari sisi `theirs` tetapi tetap
dianggap perubahan baru.

## Keputusan

Merge base dihitung pada **seluruh parent** dari kedua ujung branch: base adalah
commit terdekat yang menjadi leluhur dari `ours` maupun `theirs`, tanpa
membatasi jenis parent yang ditelusuri.

Sebagai konsekuensi, source branch yang seluruh commit-nya sudah menjadi leluhur
branch aktif ditolak dengan `AlreadyMerged` alih-alih menghasilkan commit merge
kosong.

## Alternatif yang dipertimbangkan

- **Tetap pada `first-parent`.** Ditolak: menghasilkan merge sia-sia pada merge
  kedua dan seterusnya, dan base yang terlalu awal membuat setiap baris terlihat
  berubah.
- **Menolak merge kedua secara eksplisit tanpa lewat base.** Ditolak: base yang
  benar tetap dibutuhkan bila ada commit lain yang disisipkan di antara keduanya,
  sehingga penolakan berbasis urutan commit tidak andal.
- **Menyimpan indeks leluhur per commit.** Ditolak: indeks harus diperbarui
  setiap commit, sedangkan merge base hanya dibaca saat operasi merge — biaya
  indeks tidak sebanding dengan pencarian yang jarang terjadi.

## Konsekuensi

- Merge kedua kali pada branch yang sama tidak lagi menulis commit kosong.
- `AlreadyMerged` menjadi galat yang dapat ditindaklanjuti, bukan merge senyap.
- Seluruh keputusan di ADR-0007 selain bagian "Merge base" tetap berlaku.

## Tanggal: 2026-10-03
## Penulis: Miruameli
## Review Date: 2026-12-03
