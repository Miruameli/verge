#!/usr/bin/env python3
"""Gate aturan struktural repository Verge.

KENAPA skrip ini ada: aturan modularisasi (150 SLOC per berkas, batas berkas
dan subfolder per folder, file header lengkap) sebelumnya hanya diukur manual
lewat skrip sekali pakai di shell percakapan, dan tidak ada artefak yang
menyimpan pengukurannya. Pada PR #35 header comment rusak tiga kali berturut
turut — `License` terhapus, field issue terduplikasi, bullet hilang — dan
tidak satu pun gate yang menangkapnya karena tidak ada gate struktur.

Berkas ini hanya orkestrasi. Aturan header ada di `header_rules.py`, aturan
kebersihan teks di `text_rules.py`, dan deteksi huruf non-Latin di
`homoglyphs.py`. Semuanya dipisah supaya berkas ini tetap di bawah 150 SLOC.

Aturan yang ditegakkan:
  1. Berkas `.rs` maksimal `MAX_SLOC` baris SLOC.
  2. Folder maksimal `MAX_FILES` berkas langsung. Batas subfolder
     `MAX_SUBDIRS`, kecuali folder root layer yang mendapat `LAYER_SUBDIRS`.
  3. Setiap berkas `.rs` punya seluruh `HEADER_FIELDS`, masing-masing tepat
     satu kali.
  4. `TODO`/`FIXME`/`HACK` selalu diikuti referensi issue, pada berkas `.rs`.
     Cakupannya sengaja hanya `.rs`. Dokumen dan roadmap boleh menyebut marker
     itu sebagai istilah, dan dokumen audit ini sendiri melakukannya.
  5. Tidak ada karakter non-Latin di luar tanda baca yang disetujui, pada
     berkas `.rs`.
  6. Tidak ada huruf non-Latin pada seluruh berkas teks ter-track. Aturan ini
     menolak huruf, bukan tipografi, dan cakupannya jauh lebih luas daripada
     aturan 5.

Metode SLOC: baris non-kosong dan bukan baris komentar, sama dengan audit di
`docs/engineering/audit/audit-kepatuhan-mandate.md`. Menghitung baris mentah
salah menilai berkas yang header dan doc comment-nya panjang.

Exit code 0 bila semua aturan terpenuhi, 1 bila ada pelanggaran, 2 bila gate
tidak bisa dijalankan karena dijalankan dari luar root repository.
"""

from __future__ import annotations

import pathlib
import sys

# Gate berjalan di checkout bersih CI dan di working tree lokal. Tanpa ini,
# mengimpor modul lokal menulis `__pycache__/` ke dalam repo setiap kali gate
# dijalankan, sehingga working tree ikut kotor oleh proses yang read-only.
sys.dont_write_bytecode = True

from header_rules import header_problems  # noqa: E402
from text_rules import (  # noqa: E402
    homoglyph_problems,
    marker_problems,
    non_ascii_problems,
    tracked_text_files,
)

MAX_SLOC = 150
MAX_FILES = 5
MAX_SUBDIRS = 5
LAYER_SUBDIRS = 10

SOURCE_ROOT = pathlib.Path("crates")

# Aturan batas folder berlaku pada kedua akar ini. `crates/` adalah kode
# produk; `.github/scripts/` adalah perkakas yang menjaga kode itu, dan
# aturan yang sama berlaku padanya.
FOLDER_ROOTS = (SOURCE_ROOT, pathlib.Path(".github/scripts"))

# Folder hasil build atau implementasi bahasa lain. Bukan bagian repo, jadi
# tidak pernah ikut dihitung.
EXCLUDED_DIRS = frozenset({"target", "dist", "__pycache__", "node_modules"})


def sloc(text: str) -> int:
    """Baris SLOC: non-kosong dan bukan baris komentar."""
    return sum(
        1
        for line in text.splitlines()
        if line.strip() and not line.strip().startswith("//")
    )


def subfolder_limit(folder: pathlib.Path) -> int:
    """Root layer (`src/<layer>`) mendapat batas subfolder yang lebih longgar.

    Sesuai tabel aturan mandat: folder root layer boleh 10 subfolder, folder
    lain 5. Tanpa pengecualian ini, `domain/` yang punya tujuh subdomain akan
    dilaporkan melanggar meskipun batas yang berlaku untuknya adalah 10.
    """
    return LAYER_SUBDIRS if folder.parent.name == "src" else MAX_SUBDIRS


def folder_problems() -> list[str]:
    """Folder yang melebihi batas berkas langsung atau subfolder.

    Kedua akar dipindai, bukan hanya `crates/`. Semula `.github/scripts/`
    dilewati, sehingga folder itu pernah mencapai tujuh berkas langsung tanpa
    gate menyadarinya. Pelanggaran itu persis kelas yang mestinya dicegat.
    """
    problems = []
    # Akar ikut dihitung. `rglob("*")` hanya memberi turunannya, sehingga tanpa
    # `{root}` folder `.github/scripts/` sendiri tidak pernah dihitung dan
    # pelanggaran tujuh berkas langsung di sana lolos tanpa terdeteksi.
    folders = {
        folder
        for root in FOLDER_ROOTS
        for folder in (root, *root.rglob("*"))
        if folder.is_dir() and folder.name not in EXCLUDED_DIRS
    }
    for folder in sorted(folders):
        files = sum(1 for path in folder.iterdir() if path.is_file())
        subdirs = sum(1 for path in folder.iterdir() if path.is_dir())
        if files > MAX_FILES:
            problems.append(f"{folder}: {files} berkas langsung, batas {MAX_FILES}")
        limit = subfolder_limit(folder)
        if subdirs > limit:
            problems.append(f"{folder}: {subdirs} subfolder, batas {limit}")
    return problems


def main() -> int:
    if not SOURCE_ROOT.is_dir():
        print(f"error: {SOURCE_ROOT} tidak ada; jalankan dari root repository")
        return 2

    sources = sorted(SOURCE_ROOT.rglob("*.rs"))
    problems: list[str] = []
    for path in sources:
        text = path.read_text(encoding="utf-8")
        count = sloc(text)
        if count > MAX_SLOC:
            problems.append(f"{path}: {count} SLOC, batas {MAX_SLOC}")
        problems += header_problems(path, text)
        problems += marker_problems(path, text)
        problems += non_ascii_problems(path, text)
    problems += folder_problems()

    # Aturan 6 berlaku pada seluruh berkas teks ter-track, bukan hanya `.rs`:
    # kontaminasi huruf di `.md` dan `.yml` lolos dari gate sebelum aturan ini
    # ada, dan tidak ada gate lain yang menyadarinya.
    tracked = tracked_text_files()
    for path in tracked:
        problems += homoglyph_problems(path, path.read_text(encoding="utf-8"))

    if problems:
        print(f"gagal: {len(problems)} pelanggaran struktur")
        for problem in problems:
            print(f"  {problem}")
        return 1

    print(
        f"struktur OK: {len(sources)} berkas, SLOC <= {MAX_SLOC}, "
        f"{MAX_FILES} berkas langsung per folder, "
        f"{len(tracked)} berkas teks bebas huruf non-Latin"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
