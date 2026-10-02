#!/usr/bin/env bash
# Menghasilkan SBOM CycloneDX dari metadata dependensi Cargo.
#
# KENAPA: `cargo cyclonedx` menulis keluarannya ke lokasi yang berbeda-beda
#         antar versi (dari `target/` sampai root workspace), sehingga artefak
#         SBOM sempat hilang dari rilis. Script ini menulis ke path yang pasti.
# CATATAN: hanya memetakan paket yang dipakai untuk target tersebut, sehingga
#          SBOM bisa diaudit dan tidak memuat crate yang tidak pernah dibuild.
set -euo pipefail

TARGET="${1:?target wajib diisi}"
# OUT_DIR boleh relatif terhadap root repo atau absolut.
OUT_DIR="${2:-dist}"
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
case "$OUT_DIR" in
  /*) OUT_PATH="$OUT_DIR" ;;
  *) OUT_PATH="$ROOT/$OUT_DIR" ;;
esac

mkdir -p "$OUT_PATH"
OUT="$OUT_PATH/verge-$TARGET.cdx.json"
python3 "$ROOT/.github/scripts/sbom_from_metadata.py" "$TARGET" > "$OUT"
echo "SBOM ditulis: $OUT ($(wc -c < "$OUT") byte)"
