# Releasing

## Prerequisites

- Linux Mint 22+ Cinnamon X11 (x86-64)
- Rust toolchain, pnpm, Tauri CLI
- `dpkg-deb`, `lintian` (optional)

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
