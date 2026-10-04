#!/usr/bin/env python3
"""Aturan file header untuk gate struktur repository Verge.

KENAPA: dipisah dari `check-structure.py` karena tanggung jawabnya berbeda.
        Gate utama butuh hitungan SLOC dan batas folder; ia tidak perlu tahu
        bentuk header ringkas maupun cara memecahnya. Setelah pemisahan,
        `check-structure.py` kembali di bawah batas 150 SLOC tanpa perlu
        justifikasi tertulis.

Dua bentuk header diterima:

- Kanonik: satu field per baris, diawali `//!`.
- Ringkas: beberapa field berbagi satu baris, dipisah `·`, misalnya
  `//! Author: X · Created: Y` dan `//! Version: 0.1.0 · License: Apache-2.0`.

Bentuk ringkas masih dipakai 32 berkas, jadi menolaknya akan membuat gate tidak
dapat dijalankan. Normalisasi ke bentuk kanonik dicatat di issue #41.

Pemecah `·` hanya memakai potongan yang diawali nama field. Nilai boleh memuat
koma dan backtick, jadi memotong pada setiap `·` merusak `Dependencies`.
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

# `·` memisahkan field pada header ringkas, tetapi sebuah nilai boleh memuat
# koma dan backtick. Pecah hanya bila potongan berikutnya diawali nama field.
COMPACT_SPLIT = re.compile(r"\s*·\s*(?=[A-Z][A-Za-z ]*?:)")


def header_chunks(head: list[str]) -> list[str]:
    """Belah baris header ringkas menjadi potongan `Field: value`."""
    chunks: list[str] = []
    for line in head:
        body = line.strip()[3:].strip()
        chunks += [part.strip() for part in COMPACT_SPLIT.split(body) if part.strip()]
    return chunks


def header_problems(path: pathlib.Path, text: str) -> list[str]:
    """Field header yang hilang, atau field yang muncul lebih dari sekali."""
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
