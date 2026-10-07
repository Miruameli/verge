#!/usr/bin/env python3
r"""

KENAPA: dipisah dari `check-structure.py` karena tanggung jawabnya berbeda.
        Gate utama butuh hitungan SLOC dan batas folder; ia tidak perlu tahu
        bentuk header. Setelah pemisahan, `check-structure.py` kembali di bawah
        batas 150 SLOC tanpa perlu justifikasi tertulis.

Hanya satu bentuk header yang diterima: satu field per baris, diawali `//!`.
Bentuk ringkas — beberapa field berbagi satu baris, dipisahkan `·` — pernah
hidup berdampingan dengan bentuk kanonik di 32 berkas, lalu dinormalisasi pada
issue #41. Tanpa larangan ini, bentuk lama bisa masuk lagi tanpa gate
menyadarinya: grow yang tidak dijaga.

Yang dihitung adalah field yang MEMULAI baris, bukan kemunculan namanya di
dalam nilai. Jadi `Dependencies: \`a.rs\`, \`b.rs\`` tetap sah: koma dan
backtick di dalam nilai tidak pernah ditafsirkan sebagai pemisah field.
"""

from __future__ import annotations

import pathlib
import re

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

HEADER_SCAN_LINES = 30

FIELD_SEPARATOR = "·"

# `//!` (penanda inner-doc) yang langsung diikuti `//!` lagi berarti ganda:
# bukan satu field, melainkan dua penanda doc bertumpuk. Garis pemisah
# rustdoc `//!////` (marker diikuti empat slash) tetap sah, karena setelah
# `//!` berikutnya adalah `////`, bukan `//!`. `^//!(?://!)` hanya cocok
# pada gandaan `//!`+`//!`; `//!!` (empat karakter) tak tertangkap.
DOUBLED_MARKER = re.compile(r"^//!(?://!)")


def fields_in_line(body: str) -> list[str]:
    """Nama field yang memulai baris header.

    Baris dipecah pada `·`, lalu setiap potongan dicocokkan ke awal daftar
    field. Nilai field boleh memuat koma maupun `·`, jadi hanya potongan yang
    benar-benar diawali nama field yang dihitung.
    """
    found: list[str] = []
    for part in body.split(FIELD_SEPARATOR):
        part = part.strip()
        for field in HEADER_FIELDS:
            if part.startswith(field):
                found.append(field)
                break
    return found


def header_problems(path: pathlib.Path, text: str) -> list[str]:
    """Field header yang hilang, berulang, berbagi satu baris, atau punya
    penanda `//!` ganda (dua marker bertumpuk di satu baris)."""
    problems = []
    counts: dict[str, int] = {}
    for number, line in enumerate(text.splitlines()[:HEADER_SCAN_LINES], start=1):
        if not line.startswith("//!"):
            continue
        if DOUBLED_MARKER.match(line):
            problems.append(
                f"{path}:{number}: `//!` bertemu `//!` lagi (penanda ganda), "
                f"bukan field header; ganti jadi `//!` tunggal atau pisahkan "
                f"`//!` ke baris lain"
            )
            continue
        found = fields_in_line(line[3:])
        if len(found) > 1:
            problems.append(
                f"{path}:{number}: {len(found)} field dalam satu baris "
                f"({', '.join(found)}); header harus satu field per baris"
            )
        for field in found:
            counts[field] = counts.get(field, 0) + 1

    for field in HEADER_FIELDS:
        total = counts.get(field, 0)
        if total == 0:
            problems.append(f"{path}: header tidak punya `{field}`")
        elif total > 1:
            problems.append(f"{path}: header punya `{field}` {total} kali")
    return problems
