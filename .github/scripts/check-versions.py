#!/usr/bin/env python3
"""Assert the CLI's `verge-core` version literal matches the workspace version.

KENAPA: `crates/verge-cli/Cargo.toml` pins `verge-core = { version = "X", ... }`
        as a literal. Bumping `[workspace.package] version` without editing that
        literal makes `cargo package`/`cargo publish` resolve the old core from
        the registry (the path dependency hides the mismatch locally), so the
        upload links the CLI against the previous core. Publishing is permanent,
        so this check runs in CI and in `publish-crate.yml` before any upload.
"""

from __future__ import annotations

import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
WORKSPACE_MANIFEST = ROOT / "Cargo.toml"
CLI_MANIFEST = ROOT / "crates" / "verge-cli" / "Cargo.toml"


def load_versions() -> tuple[str | None, str | None, str | None]:
    """Workspace version dan literal CLI, dengan diagnostik yang bisa ditindaklanjuti."""
    workspace = tomllib.loads(WORKSPACE_MANIFEST.read_text(encoding="utf-8"))
    cli = tomllib.loads(CLI_MANIFEST.read_text(encoding="utf-8"))
    try:
        expected = workspace["workspace"]["package"]["version"]
    except (KeyError, TypeError):
        return None, None, "::error::[workspace.package] version hilang di Cargo.toml"
    try:
        dep = cli["dependencies"]["verge-core"]
        actual = dep["version"]
    except (KeyError, TypeError):
        # Cacat yang memblokir ketiga tag historis: `verge-core = { path = ... }`
        # tanpa `version`. Cargo menolaknya sebelum menyentuh registry, jadi
        # pesannya harus menyebut perbaikannya, bukan sekadar KeyError.
        return expected, None, (
            "::error::verge-core tanpa field `version` di crates/verge-cli/Cargo.toml; "
            'tambah `version = "'
            + str(expected)
            + '" berdampingan dengan `path`, lalu samakan tiap bump'
        )
    if not isinstance(expected, str) or not isinstance(actual, str):
        return None, None, "::error::versi workspace atau literal CLI bukan string"
    return expected, actual, None


def main() -> int:
    expected, actual, error = load_versions()
    if error is not None:
        print(error)
        return 1
    assert expected is not None and actual is not None
    print(f"workspace version : {expected}")
    print(f"cli verge-core    : {actual}")
    if actual != expected:
        print(
            f"::error::verge-cli pins verge-core {actual} "
            f"but workspace is {expected}; bump the literal together"
        )
        return 1
    print("versions OK: cli verge-core literal matches workspace version")
    return 0


if __name__ == "__main__":
    sys.exit(main())
