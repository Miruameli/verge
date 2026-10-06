#!/usr/bin/env python3
"""Konsistensi indeks entri audit dengan berkas audit yang ada.

KENAPA: PR #75 menghapus baris `crates-io-backfill.md` dari indeks dan
        seluruh gate tetap hijau. Job `audit` di CI adalah RustSec
        (cargo-audit), bukan cek dokumen, jadi tidak ada yang memeriksa
        korespondensi indeks<->berkas. Modul ini menutup celah itu dan
        berjalan di dalam `check-structure.py`, sehingga ikut job `structure`
        yang sudah terdaftar di required checks tanpa perlu job CI baru.
"""

from __future__ import annotations

import pathlib
import re
import subprocess

AUDIT_INDEX = pathlib.Path("docs/engineering/audit.md")
AUDIT_ROOT = pathlib.Path("docs/engineering/audit")

# Baris indeks berbentuk `| [`audit/...`](audit/...) | ... |`; ambil target
# tautan berekstensi `.md` saja.
INDEX_LINK = re.compile(r"\(audit/([^)]+?\.md)\)")


def audit_files() -> list[pathlib.Path]:
    """Seluruh berkas audit `.md` yang ter-track maupun baru, kecuali indeks."""
    result = subprocess.run(
        [
            "git",
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
            str(AUDIT_ROOT),
        ],
        capture_output=True,
        text=True,
        check=True,
    )
    files = []
    for name in result.stdout.split("\0"):
        if not name or not name.endswith(".md"):
            continue
        path = pathlib.Path(name)
        if path == AUDIT_INDEX:
            continue
        files.append(path)
    return files

def audit_problems() -> list[str]:
    """Berkas audit tanpa baris indeks, atau baris indeks tanpa berkas."""
    try:
        indexed = set(INDEX_LINK.findall(AUDIT_INDEX.read_text(encoding="utf-8")))
    except OSError:
        return [f"{AUDIT_INDEX}: berkas indeks tidak terbaca"]
    present = {path.relative_to(AUDIT_ROOT).as_posix() for path in audit_files()}
    problems = []
    for missing in sorted(present - indexed):
        problems.append(f"{AUDIT_INDEX}: `{missing}` tidak punya baris indeks")
    for dangling in sorted(indexed - present):
        problems.append(f"{AUDIT_INDEX}: baris indeks `{dangling}` tanpa berkas")
    return problems
