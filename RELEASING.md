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

## Cutting a release

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

## What the workflow publishes

`.github/workflows/release.yml`, on any `v*.*.*` tag:

- checks the tag matches the `Cargo.toml` version;
- builds `hoardd` and `hoard` for Linux x86_64 and arm64, macOS arm64 and Windows x86_64, each archive bundled with
  the built web portal, plus `SHA256SUMS`;
- builds the container image natively on amd64 and arm64 runners and pushes `ghcr.io/<owner>/hoard:<version>`,
  `:<major>.<minor>` and `:latest` (not for pre-releases);
- creates the GitHub release with that version's changelog section as its notes.

The running version shows on the packs page, in the logo's tooltip, in `GET /api/stats` (`version`), and in
`hoardd --version` / `hoard --version`.
