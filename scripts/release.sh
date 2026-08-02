#!/usr/bin/env bash
set -e

info() { printf '[INFO] %s\n' "$1"; }
warn() { printf '[WARNING] %s\n' "$1" >&2; }
die() { printf '[ERROR] %s\n' "$1" >&2; exit 1; }
usage() { printf 'Usage: %s <version> [--dry-run]\n' "$0"; }

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$REPO_ROOT"
if [ -f .env ]; then
    set -a
    # The local release configuration is intentionally shell-compatible.
    source .env
    set +a
fi
source scripts/release-changelog.sh

VERSION=""
DRY_RUN=false
for arg in "$@"; do
    case "$arg" in
        --dry-run) DRY_RUN=true ;;
        -h|--help) usage; exit 0 ;;
        -*) die "Unknown option: $arg" ;;
        *) [ -z "$VERSION" ] || die "Unexpected argument: $arg"; VERSION="$arg" ;;
    esac
done
[ -n "$VERSION" ] || { usage; die "Provide a version number."; }
[[ "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "Use a semantic version such as 1.2.3."
git rev-parse --git-dir >/dev/null 2>&1 || die "Not in a Git repository."

RELEASE_BRANCH="release/v$VERSION"
TAG="v$VERSION"
LAST_TAG="$(git tag --list 'v*' --sort=-version:refname | head -n1)"
if [ -n "$LAST_TAG" ]; then
    RANGE="$LAST_TAG..HEAD"
else
    RANGE="HEAD"
    warn "No previous tag found; using the full history."
fi
info "Generating changelog from $RANGE"
CHANGELOG_SECTION="$(generate_changelog "$RANGE" "$VERSION")"
TODAY="$(date +%Y-%m-%d)"
NEW_ENTRY="## [$VERSION] - $TODAY"$'\n\n'"$CHANGELOG_SECTION"

if [ "$DRY_RUN" = true ]; then
    printf '\nVersion: %s\nBranch: %s\nTag: %s\nRange: %s\n\n%s\n' \
        "$VERSION" "$RELEASE_BRANCH" "$TAG" "$RANGE" "$NEW_ENTRY"
    warn "Dry run complete. Nothing was changed."
    exit 0
fi

[ -z "$(git status --porcelain)" ] || die "Commit or stash local changes before releasing."
CURRENT_BRANCH="$(git rev-parse --abbrev-ref HEAD)"
if [ "$CURRENT_BRANCH" != "$RELEASE_BRANCH" ]; then
    if git show-ref --verify --quiet "refs/heads/$RELEASE_BRANCH"; then
        git checkout "$RELEASE_BRANCH"
    else
        git checkout -b "$RELEASE_BRANCH"
    fi
fi

info "Updating versions to $VERSION"
npm version "$VERSION" --no-git-tag-version --allow-same-version >/dev/null
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml
sed -i "s/\"version\": \".*\"/\"version\": \"$VERSION\"/" src-tauri/tauri.conf.json
(cd src-tauri && cargo update -p zokute --precise "$VERSION" >/dev/null 2>&1 \
    || cargo check --quiet >/dev/null 2>&1)

info "Updating CHANGELOG.md"
CHANGELOG_HEADER="# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html)."
EXISTING_BODY=""
if [ -f CHANGELOG.md ]; then
    EXISTING_BODY="$(awk 'found || /^## \[/{found=1; print}' CHANGELOG.md)"
fi
TEMP_CHANGELOG="$(mktemp)"
{
    printf '%s\n\n%s\n' "$CHANGELOG_HEADER" "$NEW_ENTRY"
    [ -z "$EXISTING_BODY" ] || printf '\n%s\n' "$EXISTING_BODY"
} > "$TEMP_CHANGELOG"
mv "$TEMP_CHANGELOG" CHANGELOG.md

git add package.json src-tauri/Cargo.toml src-tauri/Cargo.lock \
    src-tauri/tauri.conf.json CHANGELOG.md
git commit -m "chore: bump version to $VERSION"
git tag -a "$TAG" -m "Release $TAG"

printf '\nBranch %s and tag %s are ready to push.\n' "$RELEASE_BRANCH" "$TAG"
read -r -p "Push them to origin? (y/N): " REPLY
if [[ "$REPLY" =~ ^[Yy]$ ]]; then
    git push -u origin "$RELEASE_BRANCH"
    git push origin "$TAG"
    info "Release $TAG is building as a draft on GitHub."
    info "Open and merge the release branch PR, then review and publish the draft."
else
    warn "Nothing was pushed; the local commit and tag remain."
fi
