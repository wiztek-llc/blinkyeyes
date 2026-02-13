#!/usr/bin/env bash
set -euo pipefail

# Usage: ./scripts/release.sh <version>
# Example: ./scripts/release.sh 0.2.0
#
# Bumps version in package.json and tauri.conf.json,
# moves Unreleased changelog entries under the new version,
# commits, tags, and pushes.

VERSION="${1:-}"

if [[ -z "$VERSION" ]]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

# Strip leading 'v' if provided (e.g. v0.2.0 -> 0.2.0)
VERSION="${VERSION#v}"

# Validate semver format
if ! [[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
  echo "Error: version must be semver (e.g. 1.2.3)"
  exit 1
fi

TAG="v${VERSION}"

# Check for clean working tree
if ! git diff --quiet || ! git diff --cached --quiet; then
  echo "Error: working tree is not clean. Commit or stash changes first."
  exit 1
fi

# Check tag doesn't already exist
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "Error: tag $TAG already exists"
  exit 1
fi

ROOT="$(git rev-parse --show-toplevel)"

echo "Releasing $TAG ..."

# 1. Bump version in package.json
sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" "$ROOT/package.json"

# 2. Bump version in tauri.conf.json
sed -i "s/\"version\": \".*\"/\"version\": \"${VERSION}\"/" "$ROOT/src-tauri/tauri.conf.json"

# 3. Update CHANGELOG.md — add version heading with date below [Unreleased]
TODAY="$(date +%Y-%m-%d)"
sed -i "s/^## \[Unreleased\]/## [Unreleased]\n\n## [${VERSION}] - ${TODAY}/" "$ROOT/CHANGELOG.md"

# 4. Commit, tag, push
git add "$ROOT/package.json" "$ROOT/src-tauri/tauri.conf.json" "$ROOT/CHANGELOG.md"
git commit -m "chore: release v${VERSION}"
git tag "$TAG"

echo ""
echo "Done! Created commit and tag $TAG."
echo "Run 'git push && git push --tags' to trigger the release build."
