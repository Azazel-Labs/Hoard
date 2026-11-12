#!/usr/bin/env bash
# Release notes for <ref> (default HEAD): the subjects of the commits since the previous v* tag, oldest first,
# without merges, release commits or the #minor/#major/#skip-release markers. Prints nothing if there are none.
#   scripts/release-notes.sh v0.9.0
set -euo pipefail
ref="${1:-HEAD}"
prev="$(git describe --tags --abbrev=0 --match 'v*' "$ref^" 2>/dev/null || true)"
git log --reverse --no-merges --format='%s' "${prev:+$prev..}$ref" \
  | { grep -Ev '^Release v[0-9]' || true; } \
  | sed -E 's/[[:space:]]*#(minor|major|skip-release)//gI; s/^/- /'
