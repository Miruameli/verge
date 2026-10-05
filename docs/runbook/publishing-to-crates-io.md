# Runbook: publishing to crates.io

## Purpose

Publish `verge-core` to the public crates.io registry so users can depend on it
with `cargo add`.

This runbook covers the **backfill** of the three historical releases
(`v0.1.0`, `v0.2.0`, `v0.3.0`) that were tagged on GitHub but never published
to crates.io, and the normal process for every release after that.

Only `verge-core` is backfilled. `verge-cli` cannot be, for a reason specific to
those three tags; it is published normally from `0.4.0` onwards. See "Backfill
scope" before starting.

## Prerequisites

- [x] The release tag exists on GitHub and points at a green commit.
- [x] `CARGO_REGISTRY_TOKEN` is set as a GitHub Actions secret on the
      repository. Verify with `gh secret list`. The value is never echoed, and
      never committed.
- [x] Both crate names are free on crates.io. `verge-core` and `verge-cli`
      were verified available on 2026-10-05.
- [ ] The crates.io account owning `CARGO_REGISTRY_TOKEN` has a **verified**
      email address
      (<https://crates.io/settings/profile>). Without it every upload fails
      at the last step with `400 Bad Request: A verified email address is
      required`, after all gates have passed. Found the hard way on
      2026-10-05: run `37341155506` (`verge-core 0.1.0`) went green through
      fmt, clippy, 106 tests and the manifest check, then failed only at
      `cargo publish`. Nothing was uploaded (crates.io still 404).

## Constraints that cannot be worked around

Read these before starting. Some are properties of the registry itself, others
are properties of the manifests in this repository. The distinction matters:
registry constraints cannot be engineered away, but manifest constraints are
known before the first upload.

1. **A publish is permanent.** The version can never be overwritten and the
   uploaded code can never be deleted. The only remedy for a bad upload is
   `cargo yank`, which stops new dependency resolution but leaves existing
   lockfiles working and does not remove the files.

2. **Publish timestamps cannot be backdated.** crates.io records the moment a
   version is uploaded. The backfilled `0.1.0`, `0.2.0` and `0.3.0` will all
   show today's date on the version list, not the original tag dates of
   2026-10-03 and 2026-10-04. This is deliberate registry behaviour and there
   is no supported way to influence it.

3. **The crate name `verge` is unavailable.** It has been taken on crates.io
   since 2024-08-21 by an unrelated project and names are first-come
   first-serve with no reclaim path. The CLI publishes as `verge-cli`. The
   installed binary is still named `verge` because `[[bin]] name = "verge"` is
   a build target name and does not need registry uniqueness.

4. **Order matters within a version.** `verge-cli` depends on `verge-core` by
   version, so `verge-core` must exist on the registry before `verge-cli` can
   be packaged. Publishing the CLI first fails with
   `no matching package named 'verge-core' found`.

   This ordering rule applies from `0.4.0` onwards. It does **not** explain the
   historical tags, which fail for a different and prior reason; see "Backfill
   scope" below.

5. **`verge-cli` cannot be backfilled.** All three tags declare
   `verge-core = { path = "../verge-core" }` with no `version` field, and a
   path dependency without a version cannot be packaged. This is a property of
   those manifests, not of the registry, and no ordering of uploads avoids it.

## Content fidelity

The backfill publishes the code from each historical tag, not a reconstruction.
This was verified on 2026-10-05 for `verge-core`, which is the only crate the
backfill covers: `cargo package -p verge-core` succeeds at all three tags
(109, 141 and 216 files), and unpacking the resulting `.crate` file and
diffing its `src/` directory against `git archive <tag>/src` produces no
differences.

### What backfilling costs: page metadata

Byte-identical is a guarantee about *code*, not about *metadata*. The three
tags predate the packaging work, so their manifests declare only `description`
and `license`. Verified on 2026-10-05 by reading each tag's manifest:

| Field | `v0.1.0`-`v0.3.0` | `main` (from `0.4.0`) |
| --- | --- | --- |
| `description`, `license` | yes | yes |
| `readme` | no | yes |
| `keywords`, `categories` | no | yes |
| `homepage` | no | yes |

So the backfilled pages will have **no README, no keywords and no categories**.
Concretely: `cargo search` and `cargo add` will not surface these versions well,
because crates.io search weighs keywords and categories. The crates.io page
itself will render with a description and licence only.

This is permanent. A crates.io version cannot be re-uploaded, so full metadata
cannot be added to `0.1.0`-`0.3.0` later. The only alternative is not publishing
them at all.

**Decision: accept it.** The purpose of the backfill is that the historical
code is installable by exact version for anyone following the changelog.
Discoverability is the thing being traded away, and it is recoverable from
`0.4.0` onwards. This section exists so that the byte-identical claim above is
read as the two-sided guarantee it is.

```
v0.1.0  Packaged 109 files, 218.6 KiB (47.0 KiB compressed)
v0.2.0  Packaged 141 files, 306.0 KiB (66.5 KiB compressed)
v0.3.0  Packaged 216 files, 481.1 KiB (103.2 KiB compressed)
```

## Steps

Publishing is driven by the `publish-crate` workflow, which runs manually.
One run publishes one crate at one version, so every upload has its own
auditable run record.

For each version, run the core crate first:

1. Trigger the workflow for `verge-core` at the version being released:

   ```
   gh workflow run publish-crate.yml \
     --repo Miruameli/verge \
     -f crate=verge-core \
     -f version=0.1.0
   ```

2. Wait for it to finish. It re-runs the full gate (format, clippy, tests,
   structure) before uploading, so a green run also proves the tag is
   releasable.

3. Publish the CLI for the same version, once `verge-core` for that version is
   on the registry. The same `gh workflow run` call applies with
   `-f crate=verge-cli`. This step does not apply to `0.1.0`, `0.2.0` or
   `0.3.0`: see "Backfill scope" below.

4. Repeat steps 1 to 3 for `0.2.0`, then for `0.3.0`. For those three versions,
   step 3 is skipped.

The workflow refuses to run when the version in the manifest does not match the
version requested in the input, which prevents publishing a tarball whose
contents disagree with the number users pin.

### Backfill scope

**`verge-core` can be backfilled. `verge-cli` cannot, at any of the three
tags.** This is a manifest defect, not a registry-ordering problem.

All three tags carry:

```toml
verge-core = { path = "../verge-core" }
```

That path dependency has no `version` field, so `cargo package` refuses before
it ever reaches the network:

```
error: all dependencies must have a version specified when packaging.
```

The `version` field exists only on `main`; it was never on a tag. Verified with
`cargo package -p verge-cli` in a clean worktree at each tag: identical failure
at `v0.1.0`, `v0.2.0` and `v0.3.0`. Running the unpatched manifest with
`--offline`, where the registry cannot be contacted at all, produces the same
error, which is what rules out registry ordering as the cause.

Patching only the `version` field changes the error to
`no matching package named 'verge-core' found`, which is the registry-ordering
problem the older revision of this runbook used to name. That is a second
blocker behind the first, not the blocker itself.

**Known limitation.** After the backfill, `cargo add verge-cli --version 0.3.0`
will not resolve. `verge-cli` becomes installable from `0.4.0`, published from
`main`. The alternative — patching the manifest during publish so the CLI can be
backfilled too — would mean publishing a package whose contents differ from its
tag, so it was rejected. Every published artifact is byte-identical to its tag.

### Backfill pre-flight

Before the first run, confirm the tag can still be packaged:

```
git worktree add /tmp/verge-backfill v0.1.0
cd /tmp/verge-backfill
cargo package -p verge-core --allow-dirty
git worktree remove /tmp/verge-backfill
```

Expect `Packaged 109 files` for `v0.1.0`. A failure here means the tag is not
releasable; stop and investigate rather than retrying the upload.

## Verification

After each publish:

1. The version appears on the crate page with the expected file list.
2. The version can be resolved from a clean checkout:

   ```
   cargo add verge-core@0.1.0
   ```

3. A full local verification, from outside the repository. During the backfill
   this applies to `verge-core` only; the `verge-cli` install check starts at
   `0.4.0`, because the historical CLI versions cannot be published:

   ```
   cargo install verge-cli --version 0.4.0 --locked
   verge --version
   ```

## Rollback plan

A publish cannot be undone. The available response is:

```
cargo yank --crate verge-core --version 0.1.0
```

A yank prevents new projects from resolving that version. It does not break
existing lockfiles and does not delete anything. Use it only for a broken or
unsafe upload. For a mistake in metadata that does not break builds, leaving
the version in place and documenting it is usually better.

## Escalation

crates.io account and token management happen on the crates.io website, not
through the CLI. If the token is suspected compromised, revoke it at
`https://crates.io/settings/tokens`, create a replacement, and update the
repository secret with:

```
gh secret set CARGO_REGISTRY_TOKEN --repo Miruameli/verge < /path/to/new/token
chmod 600 /path/to/new/token
```

## Last Updated: 2026-10-05