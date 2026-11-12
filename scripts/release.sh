#!/usr/bin/env bash
# Tag a Hoard release by hand. Usually you don't need this: every push to main that passes ci is tagged and
# released by .github/workflows/auto-release.yml. Releasing is only a tag; no version is committed anywhere
# (builds read it from the tag as HOARD_VERSION). Never pushes.
#
#   scripts/release.sh 1.2.0              # run the checks, then tag HEAD v1.2.0
#   scripts/release.sh 1.2.0 --dry-run    # show the notes, touch nothing
#   scripts/release.sh 1.2.0 --skip-checks
#
# Then: git push origin v1.2.0   (the tag triggers .github/workflows/release.yml)
set -euo pipefail

usage() { sed -n '2,10p' "$0" | sed 's/^# \{0,1\}//'; exit 2; }

version="${1:-}"
[[ -n "$version" && "$version" != -* ]] || usage
shift
dry_run=false
skip_checks=false
while (($#)); do
  case "$1" in
    --dry-run) dry_run=true ;;
    --skip-checks) skip_checks=true ;;
    *) usage ;;
  esac
  shift
done

cd "$(git rev-parse --show-toplevel)"
die() { echo "release: $*" >&2; exit 1; }

[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z.-]+)?$ ]] || die "'$version' isn't a semver version like 1.2.3"
tag="v$version"
last="$(git describe --tags --abbrev=0 --match 'v*' 2>/dev/null || true)"
if [[ -n "$last" ]]; then
  [[ "$(printf '%s\n%s\n' "${last#v}" "$version" | sort -V | tail -1)" == "$version" && "${last#v}" != "$version" ]] \
    || die "$version isn't newer than the last release, $last"
fi
git rev-parse -q --verify "refs/tags/$tag" >/dev/null && die "tag $tag already exists"
[[ -z "$(git status --porcelain)" ]] || die "working tree isn't clean; commit or stash first"
branch="$(git branch --show-current)"
[[ "$branch" == "main" ]] || echo "release: warning: releasing from '$branch', not main" >&2

notes="$(bash scripts/release-notes.sh HEAD)"
[[ -n "$notes" ]] || die "nothing has been committed since the last release"

echo "Releasing ${last:-(first release)} -> $tag"
echo "----- release notes -----"
echo "$notes"
echo "-------------------------"
$dry_run && { echo "(dry run: nothing changed)"; exit 0; }

if ! $skip_checks; then
  echo "Running checks (fmt, clippy, tests, web build)..."
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets --locked --quiet -- -D warnings
  cargo test --workspace --locked --quiet
  (cd web && npm run build --silent)
fi

git tag -a "$tag" -F - <<NOTES
Hoard $version

$notes
NOTES

echo
echo "Tagged $tag. Nothing was pushed. When you're ready:"
echo "  git push origin $tag"
