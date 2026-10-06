# Runbook: installing the Verge CLI

## Purpose

Install, verify, and remove the `verge` binary that end users get from a
GitHub release. Use this runbook when someone reports that installation
failed, or when supporting a machine where no Rust toolchain should be
required.

This covers the user-facing path only: `INSTALL.sh` and `uninstall.sh`.
Building from source is in `CONTRIBUTING.md`; publishing is in
[`publishing-to-crates-io.md`](publishing-to-crates-io.md).

## Prerequisites

- [x] The machine can reach `github.com` and `raw.githubusercontent.com`.
- [x] `curl` or `wget` for the download, and `sha256sum` or `shasum` for the
      checksum. The installer refuses to install without one of the two
      checksum tools; there is no flag to skip verification.
- [x] `tar` on Unix, `unzip` on Windows. Only the archive for your platform is
      unpacked.
- [x] A writable install prefix. `$HOME/.local` is the default and needs no
      privileges; the script never calls `sudo` and never edits a shell
      profile.

## Platform support

The release matrix publishes four targets. The installer uses a positive
allowlist of exactly these four, so anything else is refused up front with a
pointer to `cargo install verge-cli`.

- Linux x86_64: `x86_64-unknown-linux-gnu`
- Linux aarch64: `aarch64-unknown-linux-gnu`
- macOS arm64: `aarch64-apple-darwin`
- Windows x86_64: `x86_64-pc-windows-msvc`

Each asset is `verge-<version>-<target>.tar.gz`, except Windows, which is a
`.zip`. The archive name carries the bare version (`0.4.0`) while the tag
carries the `v` prefix (`v0.4.0`).

Intel Macs resolve to `x86_64-apple-darwin`, a real Rust target triple that the
release matrix does not build. The installer rejects it by name instead of
letting it reach a 404. Use `cargo install verge-cli` there.

`.github/scripts/install/check-installer.py` asserts that this allowlist stays
equal to the matrix in `.github/workflows/release.yml`, and it runs in the
`versions` CI job. When a target is added to the matrix, add it to
`target_is_published` in `INSTALL.sh` in the same change.

## Install

The default, for a user with no Rust toolchain:

```bash
curl -fsSL https://raw.githubusercontent.com/Miruameli/verge/main/INSTALL.sh | sh
```

To read the script before running it:

```bash
curl -fsSLO https://raw.githubusercontent.com/Miruameli/verge/main/INSTALL.sh
less INSTALL.sh
sh INSTALL.sh
```

Options, all verified against `sh INSTALL.sh --help`:

- `--version <ver>` installs a specific release; accepts `0.4.0` or `v0.4.0`.
- `--prefix <dir>` installs into `<dir>/bin` instead of `$HOME/.local`.
- `--dry-run` prints the plan and the detected target, and downloads nothing.
- `--check` prints the installed version and exits.
- `--help` prints usage and exits.

Pin a release and a prefix:

```bash
sh INSTALL.sh --version 0.4.0 --prefix /usr/local
```

## What the script does, and refuses to do

- Detects the target, downloads the matching release asset, verifies it against
  that release's `SHA256SUMS`, unpacks it, and copies the binary to
  `<prefix>/bin/verge` with mode 755.
- Aborts before the binary reaches the prefix when the checksum does not match
  or when the asset is not listed in `SHA256SUMS`.
- Never calls `sudo`, never edits `~/.bashrc` or `~/.zshrc`, and writes only
  inside the prefix you pass. If `<prefix>/bin` is not on `PATH`, it prints the
  exact `export` line to add and stops there.
- Checksum verification proves the download matches what the release published.
  It is not a signature and does not prove who published it.

## Verify

```bash
verge --version                 # or: sh INSTALL.sh --check
```

`verge --version` prints `verge <version>`. A successful install also prints
that line at the end of its own run; if it is missing, the binary did not start
and the prefix is wrong.

To confirm the published artifacts independently of the installer, download
the whole release. The manifest alone is not enough: `sha256sum -c` reports
every asset it cannot find, so `--pattern SHA256SUMS` would always fail.

```bash
gh release download v0.4.0 --dir /tmp/rel
cd /tmp/rel && sha256sum -c --strict SHA256SUMS
```

Each archive unpacks to `verge-<version>-<target>/` containing `verge`,
`LICENSE`, and `README.md`.

## Triage

Every message below was captured by running the scripts, except the one
marked `read from source`: this environment runs as root, which bypasses file
permissions, so that branch cannot be triggered here.

**Platform rejected before any download**

- `no release binary for this platform (Darwin x86_64 -> ...)`
  The target is outside the four-target allowlist. Install with
  `cargo install verge-cli`; publishing another target is a release change,
  not a local one.

**Download**

- `could not download <asset> (does this release include ...)`
  The release has no asset for that target. Check that the tag exists and is
  green, then confirm the matrix in `release.yml`.

**Verification; nothing has been installed at this point**

- `is not listed in the <version> SHA256SUMS`
  The manifest and the asset disagree. Treat the release as broken and do not
  work around it by hand.
- `checksum mismatch for <asset> (corrupt or tampered download); aborting`
  Corrupt transfer or a tampered asset. Re-run the installer. A second
  mismatch is an incident: do not install that binary by hand.
- `sha256sum or shasum is required to verify the download; not installing`
  No checksum tool on the machine. Install `coreutils` or `shasum`, then
  re-run.

**Install**

- `<prefix>/bin/verge exists and is not writable; pass a different --prefix`
  (`read from source`)
  The prefix belongs to another user. Re-run with a prefix you own, for
  example `--prefix "$HOME/.local"`.
- `verge is not on PATH; run the installer first`
  `--check` was used before an install. Run the installer, or put the prefix
  on `PATH`.

**Uninstall**

- `<path> does not report itself as Verge (got: ...)`
  `uninstall.sh` was pointed at a file that is not Verge. Nothing was deleted.
  Remove it yourself if you are sure.

When reporting a failure, include the output of `sh INSTALL.sh --dry-run` and
of `sh INSTALL.sh --check`, plus `uname -s` and `uname -m`. Together they
identify the target triple and the state of the prefix.

## Uninstall

```bash
curl -fsSL https://raw.githubusercontent.com/Miruameli/verge/main/uninstall.sh | sh
```

Or, from a checkout that still has the file:

```bash
sh uninstall.sh --prefix /usr/local
```

The script runs the candidate binary and refuses to delete anything that does
not report itself as Verge, so a same-named binary from another tool survives.
It removes the binary and nothing else: repositories created with `verge init`
are never touched. The `bin` directory is removed only when the script emptied
it, so an unrelated file in the same directory is never collateral damage.

## Related

- The end-user summary lives in the `Install` section of `README.md`.
- The `README.md` inside each release archive is a copy of the repository
  `README.md` at release time, so its `Install` section ships inside every
  archive from the next release onwards.
