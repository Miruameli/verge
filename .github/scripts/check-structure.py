#!/usr/bin/env python3
"""Gate aturan struktural repository Verge.

KENAPA skrip ini ada: aturan modularisasi (150 SLOC per berkas, batas berkas
dan subfolder per folder, file header lengkap) sebelumnya hanya diukur manual
lewat skrip sekali pakai di shell percakapan, dan tidak ada artefak yang
menyimpan pengukurannya. Pada PR #35 header comment rusak tiga kali berturut
turut — `License` terhapus, field issue terduplikasi, bullet hilang — dan
tidak satu pun gate yang menangkapnya karena tidak ada gate struktur.

Aturan yang ditegakkan:
  1. Berkas `.rs` maksimal `MAX_SLOC` baris SLOC.
  2. Folder maksimal `MAX_FILES` berkas langsung. Batas subfolder
     `MAX_SUBDIRS`, kecuali folder root layer yang mendapat `LAYER_SUBDIRS`.
  3. Setiap berkas `.rs` punya `HEADER_FIELDS`, masing-masing tepat satu kali.
  4. `TODO`/`FIXME`/`HACK` selalu diikuti referensi issue.
  5. Tidak ada karakter non-Latin di luar tanda baca yang disetujui.

Dua bentuk header diterima: kanonik (satu field per baris `//!`) dan ringkas
(`//! Author: X · Created: Y`). Bentuk ringkas masih dipakai 32 berkas, jadi
menolaknya sekarang akan membuat gate tidak dapat dijalankan; normalisasi ke
bentuk kanonik dicatat terpisah di issue #40.

Metode SLOC: baris non-kosong dan bukan baris komentar, sama dengan audit di
`docs/engineering/audit/audit-kepatuhan-mandate.md`. Menghitung baris mentah
salah menilai berkas yang header dan doc comment-nya panjang.

Exit code 0 bila semua aturan terpenuhi, 1 bila ada pelanggaran.
"""

from __future__ import annotations

import pathlib
import re
import sys

MAX_SLOC = 150
MAX_FILES = 5
MAX_SUBDIRS = 5
LAYER_SUBDIRS = 10

HEADER_FIELDS = (
    "File:",
    "Deskripsi:",
    "Layer:",
    "Tanggung jawab:",
    "Author:",
    "Created:",
    "Modified:",
    "Version:",
    "License:",
    "Related issues:",
    "Related ADR:",
)

# Tanda baca non-ASCII yang sah dipakai di dokumentasi repo ini. Daftar sengaja
# sempit: apa pun di luar daftar ini adalah salah ketik, termasuk homoglif
# Cyrillic atau CJK yang lolos dari `cargo fmt` dan `cargo clippy`.
ALLOWED_NON_ASCII = frozenset("—–·…→≤─►│├└’‘“”")

MARKERS = ("TODO", "FIXME", "HACK")
SOURCE_ROOT = pathlib.Path("crates")
HEADER_SCAN_LINES = 30

# `·` memisahkan field pada header ringkas, tetapi sebuah nilai boleh memuat
# koma dan backtick. Pecah hanya bila potongan berikutnya diawali nama field.
COMPACT_SPLIT = re.compile(r"\s*·\s*(?=[A-Z][A-Za-z ]*?:)")


def sloc(text: str) -> int:
    """Baris SLOC: non-kosong dan bukan baris komentar."""
    return sum(
        1
        for line in text.splitlines()
        if line.strip() and not line.strip().startswith("//")
    )


def header_chunks(head: list[str]) -> list[str]:
    """Belah baris header ringkas menjadi potongan `Field: value`."""
    chunks: list[str] = []
    for line in head:
        body = line.strip()[3:].strip()
        chunks += [part.strip() for part in COMPACT_SPLIT.split(body) if part.strip()]
    return chunks


def header_problems(path: pathlib.Path, text: str) -> list[str]:
    """Masalah header: field yang hilang atau field yang terduplikasi."""
    chunks = header_chunks(text.splitlines()[:HEADER_SCAN_LINES])
    counts: dict[str, int] = {}
    for chunk in chunks:
        for field in HEADER_FIELDS:
            if chunk.startswith(f"//! {field}") or chunk.startswith(field):
                counts[field] = counts.get(field, 0) + 1

    problems = []
    for field in HEADER_FIELDS:
        found = counts.get(field, 0)
        if found == 0:
            problems.append(f"{path}: header tidak punya `{field}`")
        elif found > 1:
            problems.append(f"{path}: header punya `{field}` {found} kali")
    return problems


def marker_problems(path: pathlib.Path, text: str) -> list[str]:
    """TODO/FIXME/HACK tanpa referensi issue."""
    problems = []
    for number, line in enumerate(text.splitlines(), start=1):
        if not any(marker in line for marker in MARKERS):
            continue
        if not any(f"{marker}(" in line for marker in MARKERS):
            problems.append(f"{path}:{number}: marker tanpa referensi issue")
    return problems


def non_ascii_problems(path: pathlib.Path, text: str) -> list[str]:
    """Karakter di luar ASCII dan di luar tanda baca yang disetujui."""
    problems = []
    for number, line in enumerate(text.splitlines(), start=1):
        for char in line:
            if ord(char) < 128 or char in ALLOWED_NON_ASCII:
                continue
            problems.append(
                f"{path}:{number}: karakter tak terduga U+{ord(char):04X} ({char!r})"
            )
            break
    return problems


def subfolder_limit(folder: pathlib.Path) -> int:
    """Root layer (`src/<layer>`) mendapat batas subfolder yang lebih longgar.

    Sesuai tabel aturan mandat: folder root layer boleh 10 subfolder, folder
    lain 5. Tanpa pengecualian ini, `domain/` yang punya tujuh subdomain akan
    dilaporkan melanggar although batas yang berlaku untuknya adalah 10.
    """
    return LAYER_SUBDIRS if folder.parent.name == "src" else MAX_SUBDIRS


def folder_problems(root: pathlib.Path) -> list[str]:
    """Folder yang melebihi batas berkas langsung atau subfolder."""
    problems = []
    for folder in sorted({path.parent for path in root.rglob("*.rs")}):
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

    files = sorted(SOURCE_ROOT.rglob("*.rs"))
    problems: list[str] = []
    for path in files:
        text = path.read_text(encoding="utf-8")
        count = sloc(text)
        if count > MAX_SLOC:
            problems.append(f"{path}: {count} SLOC, batas {MAX_SLOC}")
        problems += header_problems(path, text)
        problems += marker_problems(path, text)
        problems += non_ascii_problems(path, text)
    problems += folder_problems(SOURCE_ROOT)

    if problems:
        print(f"gagal: {len(problems)} pelanggaran struktur")
        for problem in problems:
            print(f"  {problem}")
        return 1

    print(
        f"struktur OK: {len(files)} berkas, SLOC <= {MAX_SLOC}, "
        f"{MAX_FILES} berkas langsung per folder"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())