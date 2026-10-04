#!/usr/bin/env python3
"""Deteksi huruf non-Latin pada berkas teks repository Verge.

KENAPA: huruf Cyrillic yang secara visual sama dengan huruf Latin — K dengan K
        Cyrillic, E dengan E Cyrillic — adalah byte UTF-8 yang valid, jadi
        `cargo fmt`, `cargo clippy`, dan `git diff` semuanya meloloskannya.
        Damage header pada PR #35 berasal dari kelas yang sama. Modul ini
        berdiri sendiri karena cakupannya berbeda dari aturan non-ASCII di
        `check-structure.py`: di situ yang diadhocari adalah *tipografi*, di
        sini yang diadhocari adalah *huruf*.

Arah aturan dibalik menjadi daftar putih, bukan daftar blokir script.
Pengukuran atas repo ini menunjukkan nol huruf non-Latin di seluruh 307 berkas
teks ter-track, jadi daftar putih ini tidak mempersempit cakupan apa pun.
Keuntungannya, daftar blokir script harus diperluas setiap kali ada script
baru, sedangkan daftar putih huruf Latin tidak perlu disentuh lagi.

Catatan yang sama berlaku untuk docstring berkas ini: modul ini tidak boleh
memuat contoh huruf yang ia tolak, karena gate akan menandai dirinya sendiri.
Huruf yang diblokir disebut berdasarkan nama script-nya, bukan dengan menulis
contohnya.

Hanya huruf yang diperiksa. Tanda baca dari script lain bukan huruf yang
tertukar dengan huruf Latin, sehingga tidak otomatis dianggap pelanggaran.
"""

from __future__ import annotations

import unicodedata

# Huruf Latin, termasuk bentuk beraksen. Rentang Latin-1 Supplement memuat
# `×` dan `÷`, tetapi keduanya kategori simbol sehingga tidak lolos filter
# huruf pada `first_foreign_letter`.
LATIN_RANGES = (
    (0x0041, 0x005A),
    (0x0061, 0x007A),
    (0x00C0, 0x00FF),
    (0x0100, 0x017F),
    (0x0180, 0x024F),
    (0x1E00, 0x1EFF),
)


def is_latin_letter(char: str) -> bool:
    """Apakah `char` adalah huruf Latin, dengan atau tanpa aksen?"""
    if not unicodedata.category(char).startswith("L"):
        return False
    code = ord(char)
    return any(low <= code <= high for low, high in LATIN_RANGES)


def first_foreign_letter(line: str) -> str | None:
    """Huruf non-Latin pertama pada `line`, atau `None` bila tidak ada."""
    for char in line:
        if unicodedata.category(char).startswith("L") and not is_latin_letter(char):
            return char
    return None
