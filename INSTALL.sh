#!/usr/bin/env sh
# One-command installer for the Verge CLI.
#
# WHY THIS EXISTS: the quick start used to begin with `cargo build --release`,
#   so anyone wanting to try Verge had to install a Rust toolchain first. The
#   release artifacts are already published and checksummed; this script moves
#   one onto `$PATH` without asking for a compiler.
#
# SECURITY: the checksum is verified and there is no flag to skip it. A
#   verification failure aborts before the binary reaches `$PATH`. The script
#   never calls `sudo`, never edits a shell profile, and only writes inside the
#   prefix you pass.
#
# ASSET NAMING: release tags carry a `v` prefix (`v0.4.0`) but archive filenames
#   do not (`verge-0.4.0-<target>.tar.gz`), because `package-release.sh` reads
#   the version from `Cargo.toml`, where it is `0.4.0`. Conflating the two
#   produces a 404, so the tag and the filename are derived separately.
#
# USAGE:
#   curl -fsSL https://raw.githubusercontent.com/Miruameli/verge/main/INSTALL.sh | sh
#   sh INSTALL.sh --version 0.4.0
#   sh INSTALL.sh --dry-run
#   sh INSTALL.sh --prefix /usr/local
#   sh INSTALL.sh --help
set -eu

REPO="Miruameli/verge"
DEFAULT_PREFIX="${HOME}/.local"

# Colour only when stdout is a terminal and the caller has not opted out, so
# piped or CI output stays clean and greppable.
if [ -t 1 ] && [ -z "${NO_COLOR:-}" ]; then
    BOLD="$(printf '\033[1m')"; DIM="$(printf '\033[2m')"
    GREEN="$(printf '\033[32m')"; YELLOW="$(printf '\033[33m')"
    RED="$(printf '\033[31m')"; RESET="$(printf '\033[0m')"
else
    BOLD=""; DIM=""; GREEN=""; YELLOW=""; RED=""; RESET=""
fi

usage() {
    cat <<EOF
${BOLD}Install the Verge CLI${RESET}

Usage:
  sh INSTALL.sh [options]

Options:
  --version <ver>    Release to install (default: latest).
  --prefix <dir>     Install prefix (default: \$HOME/.local).
  --dry-run          Show the plan without downloading anything.
  --check            Report the installed version, then exit.
  --help             Show this help.

Examples:
  curl -fsSL https://raw.githubusercontent.com/Miruameli/verge/main/INSTALL.sh | sh
  sh INSTALL.sh --version 0.4.0
  sh INSTALL.sh --prefix "\$HOME/.local"
EOF
}

die() {
    printf '%sverge: %s%s\n' "$RED" "$1" "$RESET" >&2
    exit 1
}

step() { printf '\n%s%s%s\n' "$BOLD" "$1" "$RESET"; }
note() { printf '  %s%s%s\n' "$DIM" "$1" "$RESET"; }
ok()   { printf '  %s✓%s %s\n' "$GREEN" "$RESET" "$1"; }
warn() { printf '  %s!%s %s\n' "$YELLOW" "$RESET" "$1"; }

# Map `uname` output onto the Rust target triples used by the release matrix.
detect_target() {
    _os=$(uname -s)
    _arch=$(uname -m)
    case "$_os" in
        Linux) _os="unknown-linux-gnu" ;;
        Darwin) _os="apple-darwin" ;;
        MINGW*|MSYS*|CYGWIN*) _os="pc-windows-msvc" ;;
        *) _os="unsupported" ;;
    esac
    case "$_arch" in
        x86_64|amd64) _arch="x86_64" ;;
        arm64|aarch64) _arch="aarch64" ;;
        *) _arch="unsupported" ;;
    esac
    # Either half being unrecognised collapses to a single `unsupported`
    # marker, so the message reads "unsupported" rather than the confusing
    # "unsupported-unknown-linux-gnu".
    if [ "$_os" = "unsupported" ] || [ "$_arch" = "unsupported" ]; then
        printf 'unsupported'
        return
    fi
    printf '%s-%s' "$_arch" "$_os"
}

# The release matrix in `.github/workflows/release.yml` publishes exactly four
# targets. This is a positive allowlist rather than a "known bad" check: an
# Intel Mac resolves to `x86_64-apple-darwin`, which is a real target triple
# the release does not build, and a blacklist would let it through to a 404.
# When a new target is added to the matrix, add it here in the same change.
target_is_published() {
    case "$1" in
        x86_64-unknown-linux-gnu|aarch64-unknown-linux-gnu|\
        aarch64-apple-darwin|x86_64-pc-windows-msvc) return 0 ;;
        *) return 1 ;;
    esac
}

fetch() {
    _url="$1"
    _out="$2"
    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$_url" -o "$_out"
    elif command -v wget >/dev/null 2>&1; then
        wget -qO "$_out" "$_url"
    else
        die "curl or wget is required to download; neither was found"
    fi
}

# Canonical version form is the bare number (`0.4.0`). Users may type either
# `0.4.0` or `v0.4.0`, and the API returns `v0.4.0`; all three collapse here so
# the tag and the asset name can never drift apart again.
resolve_version() {
    if [ -n "$VERSION" ]; then
        printf '%s' "${VERSION#v}"
        return
    fi
    _latest=$(fetch "https://api.github.com/repos/$REPO/releases/latest" - 2>/dev/null \
        | grep '"tag_name"' | head -1 | cut -d '"' -f 4) || _latest=""
    [ -n "$_latest" ] || die "could not resolve the latest release; pass --version"
    printf '%s' "${_latest#v}"
}

# Archive filenames carry the bare version; the release tag re-adds the `v`.
archive_name() {
    case "$2" in
        *windows*) printf 'verge-%s-%s.zip' "$1" "$2" ;;
        *) printf 'verge-%s-%s.tar.gz' "$1" "$2" ;;
    esac
}

main() {
    VERSION=""
    PREFIX="$DEFAULT_PREFIX"
    DRY_RUN=0

    while [ $# -gt 0 ]; do
        case "$1" in
            --version)
                [ $# -ge 2 ] || die "--version needs a value"
                VERSION="$2"; shift 2 ;;
            --prefix)
                [ $# -ge 2 ] || die "--prefix needs a directory"
                PREFIX="$2"; shift 2 ;;
            --check)
                command -v verge >/dev/null 2>&1 \
                    || die "verge is not on PATH; run the installer first"
                verge --version
                return 0 ;;
            --dry-run) DRY_RUN=1; shift ;;
            --help|-h) usage; return 0 ;;
            *) die "unknown option: $1 (try --help)" ;;
        esac
    done

    TARGET=$(detect_target)
    if ! target_is_published "$TARGET"; then
        die "no release binary for this platform ($(uname -s) $(uname -m) -> $TARGET).
     Verge publishes binaries for linux x86_64/aarch64, macOS arm64, and
     Windows x86_64. Install from crates.io instead:
       cargo install verge-cli"
    fi

    step "Detecting platform"
    note "system   $(uname -s) $(uname -m)"
    note "target   $TARGET"
    note "prefix   $PREFIX/bin"

    VER=$(resolve_version)
    ARCHIVE=$(archive_name "$VER" "$TARGET")

    if [ "$DRY_RUN" -eq 1 ]; then
        step "Plan (dry run, nothing downloaded)"
        note "version    $VER"
        note "download   $ARCHIVE"
        note "install    $PREFIX/bin/verge"
        note "checksum   verified against the release SHA256SUMS"
        return 0
    fi

    WORK=$(mktemp -d 2>/dev/null) || die "could not create a temporary directory"
    trap 'rm -rf "$WORK"' EXIT INT TERM

    step "Downloading $VER"
    # The release tag re-adds the `v`; the asset name keeps the bare version.
    BASE="https://github.com/$REPO/releases/download/v$VER"
    fetch "$BASE/$ARCHIVE" "$WORK/$ARCHIVE" \
        || die "could not download $ARCHIVE (does this release include $TARGET?)"
    fetch "$BASE/SHA256SUMS" "$WORK/SHA256SUMS" || die "could not download SHA256SUMS"
    note "$ARCHIVE"

    step "Verifying checksum"
    # SECURITY: this checks the download against the release server's own
    #   SHA256SUMS. It proves the download matches what was published; it is
    #   not a signature and does not prove who published it.
    # The release manifest is built with `find . -maxdepth 1`, so its lines
    # read `<hash>  ./verge-...` with a leading `./`. Match the basename so
    # both `./name` and `name` lines resolve.
    LINE=$(awk -v want="$ARCHIVE" '{n=$2; sub(/^\.\//, "", n); if (n == want) print}' "$WORK/SHA256SUMS" | head -1)
    [ -n "$LINE" ] || die "$ARCHIVE is not listed in the $VER SHA256SUMS"
    WANT=$(printf '%s' "$LINE" | awk '{print $1}')
    note "expected  $LINE"
    if command -v sha256sum >/dev/null 2>&1; then
        GOT=$(sha256sum "$WORK/$ARCHIVE" | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        GOT=$(shasum -a 256 "$WORK/$ARCHIVE" | awk '{print $1}')
    else
        die "sha256sum or shasum is required to verify the download; not installing"
    fi
    [ "$WANT" = "$GOT" ] \
        || die "checksum mismatch for $ARCHIVE (corrupt or tampered download); aborting"
    ok "sha256 matches SHA256SUMS"

    step "Installing"
    case "$ARCHIVE" in
        *.zip)
            command -v unzip >/dev/null 2>&1 \
                || die "unzip is required to unpack a Windows archive"
            unzip -q "$WORK/$ARCHIVE" -d "$WORK" ;;
        *)
            tar -xzf "$WORK/$ARCHIVE" -C "$WORK" ;;
    esac
    BIN=$(find "$WORK" -type f \( -name verge -o -name verge.exe \) | head -1)
    [ -n "$BIN" ] || die "no verge binary found inside the archive"

    DEST="$PREFIX/bin"
    mkdir -p "$DEST" || die "could not create $DEST"
    if [ -e "$DEST/verge" ] && [ ! -w "$DEST" ]; then
        die "$DEST/verge exists and is not writable; pass a different --prefix"
    fi
    cp "$BIN" "$DEST/verge"
    chmod 755 "$DEST/verge"
    ok "$DEST/verge"

    step "Done"
    if "$DEST/verge" --version 2>/dev/null; then
        :
    else
        warn "installed; run \`verge --help\` to get started"
    fi

    case ":$PATH:" in
        *":$DEST:"*) ;;
        *)
            printf '\n'
            warn "that directory is not on your PATH yet"
            note "export PATH=\"$DEST:\$PATH\""
            note "add it to ~/.bashrc or ~/.zshrc to make it permanent"
            ;;
    esac
    printf '\n'
    note "next: mkdir -p /tmp/demo && cd /tmp/demo && verge init"
}

main "$@"
