#!/usr/bin/env bash
set -euo pipefail
[ $# -ne 1 ] && echo "Usage: $0 <version>" && exit 1
NEW_VERSION="$1"
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
echo "$NEW_VERSION" > "$REPO_ROOT/VERSION"
# cyrius.cyml derives its version from VERSION via `${file:VERSION}` — no manifest edit needed.
# The dist bundle embeds the version in its header, so regenerate it to keep the
# CI dist-freshness gate green.
(cd "$REPO_ROOT" && cyrius distlib >/dev/null 2>&1) || echo "  (warn: 'cyrius distlib' failed — regenerate dist/sankhya.cyr manually)"
echo "Bumped to ${NEW_VERSION}. Add a CHANGELOG entry, regenerate benchmarks, then tag and push."
