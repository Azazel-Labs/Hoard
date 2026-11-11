# Releasing Hoard

Hoard versions follow [Semantic Versioning](https://semver.org/). While the version is 0.x, a minor release
(0.2 → 0.3) may change the API, the database schema or configuration; patch releases (0.2.0 → 0.2.1) don't. The
changelog says when a release migrates the database (migrations run automatically on start, forwards only, so
back up `hoard.db` first).

What gets versioned is Hoard itself: the server, CLI, web portal and container image share one version number.
Assets inside a library have no revisions; re-importing a changed file adds it as a new asset.

## Day to day

Every user-visible change adds a line under `## [Unreleased]` in [CHANGELOG.md](CHANGELOG.md), in the
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/) groups (Added, Changed, Fixed, Removed, Security).

## Releases are automatic

When `ci` passes on a push to `main`, `.github/workflows/auto-release.yml` looks at `[Unreleased]`:

- **It has entries:** cut a release. The bump is a patch (0.8.0 → 0.8.1) unless a commit message since the last
  tag contains `#minor` (→ 0.9.0) or `#major` (→ 1.0.0). It runs `scripts/release.sh <version> --skip-checks`
  (ci already ran the checks), pushes the `Release vX.Y.Z` commit and the tag as `github-actions[bot]`, and runs the
  release workflow below.
- **It's empty:** no release; it publishes `ghcr.io/<owner>/hoard:edge` and `:sha-<short>` from that commit.

So the changelog is the switch: push work without changelog entries and you get an edge image; add an entry and the
push ships a version. Only `main`'s tip is acted on, so several quick pushes produce one release, not several.

The bot pushes to `main`, so if you add branch protection, allow `github-actions[bot]` to bypass it.

## Cutting a release by hand

Still works, for pre-releases or a specific version:

```sh
scripts/release.sh 0.3.0 --dry-run   # prints the release notes it would use
scripts/release.sh 0.3.0             # bump, date the changelog, run fmt/clippy/tests/web build, commit, tag
git push origin main v0.3.0          # the tag starts the release workflow
```

`release.sh` refuses to run with a dirty tree, an empty Unreleased section, an existing tag or a version that isn't
newer than the current one. It sets the version in `Cargo.toml` (`[workspace.package]`, inherited by every crate),
`Cargo.lock` and `web/package.json`, turns `[Unreleased]` into `[0.3.0] - <date>`, updates the compare links, and
makes an annotated `v0.3.0` tag whose message is the release notes. It never pushes.

Pre-releases work too: `scripts/release.sh 0.3.0-rc.1` produces a GitHub pre-release and doesn't move `latest`.
The next automatic release after a pre-release finalizes it (0.3.0-rc.1 → 0.3.0).

## What the release workflow publishes

`.github/workflows/release.yml`, on a `v*.*.*` tag (pushed by hand, or called by auto-release):

- checks the tag matches the `Cargo.toml` version;
- builds `hoardd` and `hoard` for Linux x86_64 and arm64, macOS arm64 and Windows x86_64, each archive bundled with
  the built web portal, plus `SHA256SUMS`;
- builds the container image natively on amd64 and arm64 runners (`.github/workflows/image.yml`) and pushes
  `ghcr.io/<owner>/hoard:<version>`, `:<major>.<minor>` and `:latest` (not for pre-releases), plus `:edge` and
  `:sha-<short>` when auto-release cut it;
- creates the GitHub release with that version's changelog section as its notes.

The running version shows on the packs page, in the logo's tooltip, in `GET /api/stats` (`version`), and in
`hoardd --version` / `hoard --version`.
