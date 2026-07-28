# Installation

Zokute is distributed as a Debian package for Linux Mint 22+ Cinnamon on X11 (x86-64).

## Prerequisites

- Linux Mint 22 or later
- Cinnamon desktop
- X11 session (not Wayland)
- x86-64 architecture

## Install

```sh
sudo dpkg -i zokute_0.1.0_amd64.deb
```

If missing dependencies, run:

```sh
sudo apt-get install -f
```

## Autostart

Toggle autostart from the Settings window or tray menu.

## Uninstall

```sh
sudo dpkg -r io.github.m4ss1ck.zokute
```

Uninstall removes only installed application files. Your config, profiles, plugins, state, and cache are preserved in `~/.config/zokute` and `~/.local/state/zokute`.
