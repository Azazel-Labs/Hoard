#!/usr/bin/env bash
# Cut a Hoard release: bump versions, date the changelog, run the checks, commit and tag. Never pushes.
#
#   scripts/release.sh 0.2.0                 # full release
#   scripts/release.sh 0.2.0 --dry-run       # show what would change, touch nothing
#   scripts/release.sh 0.2.0 -- --trailer "Reviewed-by: someone"   # extra `git commit` arguments after `--`
#
# Then: git push origin main v0.2.0   (the tag triggers .github/workflows/release.yml)
set -euo pipefail

usage() { sed -n '2,8p' "$0" | sed 's/^# \{0,1\}//'; exit 2; }

version="${1:-}"
[[ -n "$version" && "$version" != -* ]] || usage
shift
dry_run=false
skip_checks=false
commit_args=()
while (($#)); do
  case "$1" in
    --dry-run) dry_run=true ;;
    --skip-checks) skip_checks=true ;;
    --) shift; commit_args=("$@"); break ;;
    *) usage ;;
  esac
  shift
done

cd "$(git rev-parse --show-toplevel)"
die() { echo "release: $*" >&2; exit 1; }

[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]] || die "'$version' isn't a semver version like 1.2.3"
tag="v$version"
current="$(sed -n '/^\[workspace.package\]/,/^\[/ s/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
[[ -n "$current" ]] || die "couldn't read [workspace.package] version from Cargo.toml"
[[ "$(printf '%s\n%s\n' "$current" "$version" | sort -V | tail -1)" == "$version" && "$current" != "$version" ]] \
  || die "$version isn't newer than the current $current"
git rev-parse -q --verify "refs/tags/$tag" >/dev/null && die "tag $tag already exists"
[[ -z "$(git status --porcelain)" ]] || die "working tree isn't clean; commit or stash first"
branch="$(git branch --show-current)"
[[ "$branch" == "main" ]] || echo "release: warning: releasing from '$branch', not main" >&2

# The Unreleased section becomes the release notes, so it must say something.
notes="$(awk '/^## \[Unreleased\]/{on=1; next} /^## \[/{on=0} on' CHANGELOG.md | sed -e '/./,$!d' | sed -e ':a' -e '/^\n*$/{$d;N;ba' -e '}')"
[[ -n "$notes" ]] || die "CHANGELOG.md has nothing under [Unreleased]"

echo "Releasing $current -> $version"
echo "----- release notes -----"
echo "$notes"
echo "-------------------------"
$dry_run && { echo "(dry run: nothing changed)"; exit 0; }

today="$(date -u +%Y-%m-%d)"

# Versions: the Rust workspace (all crates inherit it) and the web app.
sed -i.bak "/^\[workspace.package\]/,/^\[/ s/^version = \".*\"/version = \"$version\"/" Cargo.toml && rm Cargo.toml.bak
cargo update --workspace --quiet
(cd web && npm version "$version" --no-git-tag-version --allow-same-version >/dev/null)

# Changelog: date the release, open a fresh Unreleased section, fix the compare links.
repo_url="$(sed -n 's/^\[Unreleased\]: \(.*\)\/compare\/.*/\1/p' CHANGELOG.md)"
[[ -n "$repo_url" ]] || die "couldn't find the [Unreleased] link at the bottom of CHANGELOG.md"
awk -v v="$version" -v d="$today" -v url="$repo_url" -v prev="v$current" '
  /^## \[Unreleased\]/ { print; print ""; print "## [" v "] - " d; next }
  /^\[Unreleased\]: / {
    print "[Unreleased]: " url "/compare/v" v "...HEAD"
    print "[" v "]: " url "/compare/" prev "...v" v
    next
  }
  { print }
' CHANGELOG.md > CHANGELOG.md.tmp && mv CHANGELOG.md.tmp CHANGELOG.md

if ! $skip_checks; then
  echo "Running checks (fmt, clippy, tests, web build)..."
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets --locked --quiet -- -D warnings
  cargo test --workspace --locked --quiet
  (cd web && npm run build --silent)
fi

git add Cargo.toml Cargo.lock web/package.json web/package-lock.json CHANGELOG.md
git commit --quiet -m "Release $tag" ${commit_args[@]+"${commit_args[@]}"}
git tag -a "$tag" -F - <<NOTES
Hoard $version

$notes
NOTES

echo
echo "Committed and tagged $tag. Nothing was pushed. When you're ready:"
echo "  git push origin $branch $tag"
