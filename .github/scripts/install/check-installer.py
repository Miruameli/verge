#!/usr/bin/env python3
"""Assert the installer agrees with the release pipeline about artifact names.

KENAPA: three real bugs shipped in the first draft of `INSTALL.sh`, all from the
        same cause — the installer guessed a naming convention instead of
        checking the one the release actually uses:

        1. It built `verge-v0.4.0-...` while `package-release.sh` names files
           `verge-0.4.0-...` (the version comes from `Cargo.toml`, which has no
           `v`). Every install 404'd.
        2. It matched `<hash>  <name>` in `SHA256SUMS`, but that file is built
           with `find . -maxdepth 1`, so every line carries a leading `./`.
           Verification never matched, so the installer always aborted.
        3. Its platform guard was a blacklist, so an Intel Mac passed the check
           and only failed later with a 404.

        None of these are visible from a single file. This gate compares the
        installer against the workflow and the packaging script that define the
        real convention, so the three cannot drift apart silently again.

Checks:
    - The published-target allowlist in `INSTALL.sh` equals the release matrix.
    - `INSTALL.sh` builds archive names the way `package-release.sh` does,
      including the `.zip` variant for Windows.
    - The installer reads `SHA256SUMS` in a way that tolerates the `./`
      prefix that `find` writes.
"""

from __future__ import annotations

import pathlib
import re
import sys

# The repository root is found by walking up until a `Cargo.toml` appears,
# rather than by counting `parent` hops. Counting is fragile: moving the file
# one directory deeper makes every path constant silently wrong, and the gate
# then reports "no files could be read" instead of a real problem.
def repo_root() -> pathlib.Path:
    """Direktori teratas yang memuat `Cargo.toml` workspace."""
    for candidate in [pathlib.Path(__file__).resolve(), *pathlib.Path(__file__).resolve().parents]:
        if candidate.is_dir() and (candidate / "Cargo.toml").is_file():
            return candidate
    raise AssertionError("tidak menemukan root repository (Cargo.toml)")


ROOT = repo_root()
INSTALLER = ROOT / "INSTALL.sh"
UNINSTALLER = ROOT / "uninstall.sh"
PACKAGER = ROOT / ".github" / "scripts" / "package-release.sh"
RELEASE_WORKFLOW = ROOT / ".github" / "workflows" / "release.yml"


def release_targets() -> set[str]:
    """Target triples dari matriks build pada `release.yml`."""
    text = RELEASE_WORKFLOW.read_text(encoding="utf-8")
    return set(re.findall(r"-\s*target:\s*([a-z0-9_]+-[a-z0-9\-]+)", text))


def installer_targets() -> set[str]:
    """Allowlist target di dalam fungsi `target_is_published`."""
    text = INSTALLER.read_text(encoding="utf-8")
    match = re.search(
        r"target_is_published\(\)\s*\{(.*?)\n\}", text, re.DOTALL
    )
    if match is None:
        raise AssertionError("INSTALL.sh: fungsi `target_is_published` tidak ditemukan")
    body = match.group(1)
    # Buang baris komentar, lalu ambil setiap alternatif pola `case`.
    body = re.sub(r"#[^\n]*", "", body)
    # Extract hyphenated triples rather than matching whole lines: the `case`
    # body ends its last alternative with `) return 0 ;;` and wraps lines with a
    # trailing backslash, so line-oriented matching misses real targets.
    # Anchor on the vendor segment so prose-shaped words such as
    # `content-addressed` cannot be mistaken for a target triple.
    return set(re.findall(r"\b[a-z0-9_]+-(?:unknown|apple|pc)-[a-z0-9-]+\b", body))


def main() -> int:
    problems: list[str] = []

    try:
        published = release_targets()
        allowlist = installer_targets()
    except (AssertionError, OSError) as error:
        print(f"::error::{error}")
        return 1

    if not published:
        problems.append("release.yml: tidak ada target yang terbaca dari matriks build")
    if published != allowlist:
        for missing in sorted(published - allowlist):
            problems.append(
                f"INSTALL.sh: target rilis `{missing}` tidak ada di allowlist "
                "target_is_published; pengguna platform itu akan ditolak"
            )
        for extra in sorted(allowlist - published):
            problems.append(
                f"INSTALL.sh: allowlist mentions `{extra}`, but release.yml does "
                "not build it; users on that platform would get a 404"
            )

    installer = INSTALLER.read_text(encoding="utf-8")
    packager = PACKAGER.read_text(encoding="utf-8")

    # The archive name is assembled from the bare version, and the packaging
    # script must keep deriving that version from Cargo.toml (no `v` prefix).
    # Asserting the *source* of the version is what actually protects the
    # naming; asserting a particular literal in the packager only breaks when
    # an unrelated edit renames a local variable.
    if "verge-%s-%s.tar.gz" not in installer:
        problems.append("INSTALL.sh: tar.gz archive name pattern not found")
    if "verge-%s-%s.zip" not in installer:
        problems.append("INSTALL.sh: Windows .zip archive name pattern not found")
    if 'sed -n \'s/^version = "\\(.*\\)"$/\\1/p\'' not in packager:
        problems.append(
            "package-release.sh no longer reads the version from Cargo.toml; "
            "the installer assumes artifact names carry no `v` prefix"
        )

    # SHA256SUMS is produced by `find .`, so every entry keeps a leading `./`.
    # Match on the basename so both spellings resolve.
    if "find . -maxdepth 1" not in RELEASE_WORKFLOW.read_text(encoding="utf-8"):
        problems.append(
            "release.yml: SHA256SUMS is no longer built with `find .`; "
            "recheck the manifest matching in INSTALL.sh"
        )
    if 'sub(/^\\.\\//' not in installer:
        problems.append(
            "INSTALL.sh: SHA256SUMS matching no longer tolerates the `./` prefix"
        )

    if problems:
        for problem in problems:
            print(f"::error::{problem}")
        return 1

    print(f"release matrix   : {len(published)} target")
    for target in sorted(published):
        print(f"  - {target}")
    print(f"installer agrees : allowlist identik, penamaan arsip cocok")
    print("installer OK: INSTALL.sh selaras dengan alur rilis")
    return 0


if __name__ == "__main__":
    sys.exit(main())
