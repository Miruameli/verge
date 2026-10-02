#!/usr/bin/env python3
"""Ubah `cargo metadata` menjadi SBOM CycloneDX 1.5 (JSON).

KENAPA: output `cargo cyclonedx` berpindah-pindah antar versi sehingga SBOM
        sempat hilang dari rilis; konversi di sini memakai path dan bentuk
        keluaran yang tetap.
"""

import hashlib
import json
import pathlib
import subprocess
import sys

# Versi tool dan format mengikuti spesifikasi CycloneDX 1.5.
SPEC_VERSION = "1.5"
TOOL_NAME = "verge-sbom"
TOOL_VERSION = "1.0.0"

# Lockfile dibaca agar serial number berubah bila dependensi berubah.
LOCKFILE = ""


def root_component(metadata: dict) -> dict:
    """Ambil crate terluar di workspace sebagai komponen utama SBOM.

    `workspace_members` memuat path dan nama; path member pertama adalah crate
    yang tidak dependedensi oleh crate lain di workspace ini.
    """
    members = metadata.get("workspace_members") or []
    if not members:
        return {"type": "application", "name": "verge", "version": "0.0.0"}
    # Bentuk member adalah `path+file:///.../crates/verge-core#0.1.0`.
    location, _, version = members[0].rpartition("#")
    name = location.rstrip("/").rsplit("/", 1)[-1] or "verge"
    return {"type": "application", "name": name, "version": version or "0.0.0"}


def serial_number(target: str) -> str:
    """Bentuk serial number UUID yang stabil dari target dan lockfile.

    KENAPA: CycloneDX mensyaratkan serial number berbentuk UUID; nilainya harus
            tetap untuk metadata yang sama agar SBOM dapat dibandingkan.
    """
    digest = hashlib.sha256(f"{target}|{LOCKFILE}".encode()).hexdigest()
    return f"urn:uuid:{digest[0:8]}-{digest[8:12]}-{digest[12:16]}-{digest[16:20]}-{digest[20:32]}"


def main() -> None:
    """Cetak SBOM untuk target yang diberikan sebagai argumen pertama."""
    if len(sys.argv) != 2:
        print("usage: sbom_from_metadata.py <target>", file=sys.stderr)
        raise SystemExit(2)
    target = sys.argv[1]
    global LOCKFILE  # noqa: PLW0603 - hanya dibaca setelah diinisialisasi.
    lock = pathlib.Path(__file__).resolve().parents[2] / "Cargo.lock"
    LOCKFILE = hashlib.sha256(lock.read_bytes()).hexdigest()[:16]
    metadata = json.loads(
        subprocess.run(
            [
                "cargo",
                "metadata",
                "--locked",
                "--format-version",
                "1",
                "--filter-platform",
                target,
            ],
            check=True,
            capture_output=True,
            text=True,
        ).stdout
    )
    components = sorted(
        (
            {
                "type": "library",
                "name": package["name"],
                "version": package["version"],
                "purl": f"pkg:cargo/{package['name']}@{package['version']}",
                "licenses": (package.get("license") and [{"license": {"id": package["license"]}}])
                or [],
            }
            for package in metadata["packages"]
        ),
        key=lambda item: (item["name"], item["version"]),
    )
    document = {
        "bomFormat": "CycloneDX",
        "specVersion": SPEC_VERSION,
        "serialNumber": serial_number(target),
        "version": 1,
        "metadata": {
            "component": root_component(metadata),
            "tools": {
                "components": [{"type": "application", "name": TOOL_NAME, "version": TOOL_VERSION}]
            },
        },
        "components": components,
    }
    json.dump(document, sys.stdout, indent=2, sort_keys=True)
    sys.stdout.write("\n")


if __name__ == "__main__":
    main()
