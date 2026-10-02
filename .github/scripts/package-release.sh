#!/usr/bin/env bash
# Kemas binary rilis Verge beserta LICENSE dan README menjadi satu arsip.
#
# KENAPA: pengguna mengunduh satu berkas dan langsung bisa memverifikasi isinya
#         lewat SHA256SUMS yang terbit bersama rilis.
set -euo pipefail

TARGET="${1:?target wajib diisi}"
ARCHIVE="${2:?format arsip wajib diisi}"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
VERSION="$(sed -n 's/^version = "\(.*\)"$/\1/p' "$ROOT/Cargo.toml" | head -1)"
OUT="$ROOT/dist"
STAGE="$OUT/verge-$VERSION-$TARGET"

BIN_NAME="verge"
case "$TARGET" in
  *windows*) BIN_NAME="verge.exe" ;;
esac

mkdir -p "$STAGE"
cp "$ROOT/target/$TARGET/release/verge" "$STAGE/$BIN_NAME"
cp "$ROOT/LICENSE" "$STAGE/LICENSE"
cp "$ROOT/README.md" "$STAGE/README.md"

if [ "$ARCHIVE" = "zip" ]; then
  # Runner Windows tidak menyediakan `zip`; PowerShell adalah alat bawaannya.
  if command -v zip >/dev/null 2>&1; then
    (cd "$OUT" && zip -qr "verge-$VERSION-$TARGET.zip" "verge-$VERSION-$TARGET")
  else
    powershell -NoProfile -Command \
      "Compress-Archive -Path '$STAGE' -DestinationPath '$OUT/verge-$VERSION-$TARGET.zip'"
  fi
  rm -rf "$STAGE"
else
  (cd "$OUT" && tar -czf "verge-$VERSION-$TARGET.tar.gz" "verge-$VERSION-$TARGET")
  rm -rf "$STAGE"
fi
