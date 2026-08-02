# Releasing

## Prerequisites

- Linux Mint 22+ Cinnamon X11 (x86-64)
- Rust toolchain, pnpm, Tauri CLI
- `dpkg-deb`, `lintian` (optional)

## Cut a release

Releases are prepared locally and built by GitHub Actions:

```sh
./scripts/release.sh 0.1.0 --dry-run
./scripts/release.sh 0.1.0
```

The script creates or switches to `release/v<version>`, synchronizes the version
files, generates the latest `CHANGELOG.md` entry from commits since the previous
tag, commits and tags it, then offers to push both refs. The tag build copies that
entry into the draft GitHub release notes.

After the build, merge the release branch PR and review and publish the draft.

### Optional AI changelog

Without configuration, commits are grouped into Added, Changed, and Fixed. To
rewrite them as concise user-facing notes, copy `.env.example` to `.env` and set
an OpenAI-compatible provider. `opencode` requires `OPENCODE_API_KEY` and
`CHANGELOG_AI_MODEL`; `openai` requires `OPENAI_API_KEY` and defaults to
`gpt-4o-mini`. Use `--dry-run` to preview either result without changing files.

## Build

```sh
pnpm build:deb
```

## Output

The Debian package is at `src-tauri/target/release/bundle/deb/zokute_0.1.0_amd64.deb`.

## Checksum

```sh
sha256sum src-tauri/target/release/bundle/deb/zokute_0.1.0_amd64.deb > checksums.txt
```

## Release checklist

- [ ] Frontend build succeeds (`pnpm build`)
- [ ] Rust build succeeds (`cargo build`)
- [ ] Frontend tests pass (`pnpm test:run`)
- [ ] Rust tests pass (`cargo test`)
- [ ] Debian package can be built (`pnpm build:deb`)
- [ ] Package contents verified (`scripts/check-deb.mjs`)
- [ ] Identifier is `io.github.m4ss1ck.zokute`
- [ ] Version consistent across config, docs, and package
- [ ] Fresh install test on Mint Cinnamon X11
- [ ] Previous-version upgrade test
- [ ] Uninstall preserves user data
- [ ] Wayland guard shows compatibility message
- [ ] No telemetry or trackers present
