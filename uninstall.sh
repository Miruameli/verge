#!/usr/bin/env sh
# Remove a Verge CLI installed by INSTALL.sh.
#
# SAFETY: a file named `verge` on your PATH may belong to something else, so
#   this script refuses to delete anything unless the file actually identifies
#   itself as Verge. It removes the binary and nothing else: repositories you
#   created with `verge init` are never touched.
#
# USAGE:
#   sh uninstall.sh
#   sh uninstall.sh --prefix /usr/local
set -eu

DEFAULT_PREFIX="${HOME}/.local"

usage() {
    cat <<EOF
Remove the Verge CLI.

Usage:
  sh uninstall.sh [--prefix <dir>]

Options:
  --prefix <dir>   Prefix the binary was installed into (default: \$HOME/.local).
  --help           Show this help.

Only the \`verge\` binary is removed. Repositories created with \`verge init\`
are left untouched.
EOF
}

die() {
    printf 'verge: %s\n' "$1" >&2
    exit 1
}

note() { printf '  %s\n' "$1"; }

main() {
    PREFIX="$DEFAULT_PREFIX"
    while [ $# -gt 0 ]; do
        case "$1" in
            --prefix)
                [ $# -ge 2 ] || die "--prefix needs a directory"
                PREFIX="$2"; shift 2 ;;
            --help|-h) usage; return 0 ;;
            *) die "unknown option: $1 (try --help)" ;;
        esac
    done

    BIN="$PREFIX/bin/verge"

    [ -e "$BIN" ] || die "no Verge binary at $BIN (try --prefix if it lives elsewhere)"

    # SAFETY: confirm the file is Verge before deleting it. A same-named binary
    #   from another tool must survive this script.
    REPORTED=$("$BIN" --version 2>/dev/null || printf '')
    case "$REPORTED" in
        verge\ *) ;;
        *) die "$BIN does not report itself as Verge (got: ${REPORTED:-nothing}).
     Refusing to delete it; remove it yourself if you are sure." ;;
    esac

    printf 'Removing %s (%s)\n' "$BIN" "$REPORTED"
    rm -f "$BIN"
    note "removed $BIN"

    # Remove the directory only when this script emptied it, so an unrelated
    # file in the same bin directory is never collateral damage.
    if [ -d "$PREFIX/bin" ] && [ -z "$(ls -A "$PREFIX/bin" 2>/dev/null)" ]; then
        rmdir "$PREFIX/bin" && note "removed empty $PREFIX/bin"
    fi

    printf '\n'
    note "your Verge repositories were not touched"
}

main "$@"
