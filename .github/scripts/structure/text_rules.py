#!/usr/bin/env python3
"""Aturan kebersihan teks untuk gate struktur repository Verge.

KENAPA: tiga aturan di sini saling terkait karena sama-sama menilai isi teks,
        bukan strukturnya: karakter yang tidak diharapkan, marker tanpa
        referensi issue, dan huruf non-Latin. Semuanya memindai baris per
        baris, jadi mengumpulkannya di satu modul jelas lebih mudah
        dipelihara daripada tersebar di gate utama.

Dua aturan non-ASCII sengaja terpisah dan tidak boleh digabung:

- `non_ascii_problems` memakai daftar putih ketat dan hanya berlaku pada
  berkas `.rs`. Tipografi seperti `—` dan `→` sudah ada di repo ini.
- `homoglyph_problems` memakai daftar putih huruf Latin dan berlaku pada
  SELURUH berkas teks ter-track. Yang dilarang adalah huruf, bukan tipografi.
  Detektornya ada di `homoglyphs.py`.
"""

from __future__ import annotations

import pathlib
import subprocess

from homoglyphs import first_foreign_letter

# Tanda baca non-ASCII yang sah dipakai di dokumentasi repo ini. Daftar sengaja
# sempit: apa pun di luar daftar ini adalah salah ketik.
ALLOWED_NON_ASCII = frozenset("—–·…→≤─►│├└’‘“”")

MARKERS = ("TODO", "FIXME", "HACK")


def tracked_text_files() -> list[pathlib.Path]:
    """Berkas teks di working tree. Lewati berkas biner dan berkas diabaikan.

    `--others --exclude-standard` menutup celah lokal: berkas yang baru dibuat
    belum dilacak, sehingga `git ls-files` biasa akan melewatinya justru pada
    saat paling perlu diperiksa. Di CI tidak ada berkas tak-terlacak, jadi
    hasilnya identik dengan pemanggilan biasa.
    """
    result = subprocess.run(
        ["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"],
        capture_output=True,
        text=True,
        check=True,
    )
    files = []
    for name in result.stdout.split("\0"):
        if not name:
            continue
        path = pathlib.Path(name)
        try:
            path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        files.append(path)
    return files


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


def homoglyph_problems(path: pathlib.Path, text: str) -> list[str]:
    """Huruf non-Latin, di berkas apa pun yang dilacak git."""
    problems = []
    for number, line in enumerate(text.splitlines(), start=1):
        char = first_foreign_letter(line)
        if char is not None:
            problems.append(
                f"{path}:{number}: huruf non-Latin U+{ord(char):04X} ({char!r})"
            )
    return problems
