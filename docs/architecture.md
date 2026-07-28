# Architecture

## Overview

Zokute is a Tauri 2 application with a Rust backend and a React/TypeScript frontend. Each widget runs in its own transparent WebView window positioned on the desktop.

## Core Components

### Shared Collector

A single async task collects CPU, memory, disk, network, GPU, and sensor metrics at 1 Hz and emits them as `Stats` events to all WebView windows.

### Window/Content Split

Each widget window has a Tauri WebView that loads `index.html`. The React app (`App.tsx`) reads the window label and renders the matching widget component. Window positioning and flags are managed from Rust.

### Layout Transaction

When entering layout edit mode, the current profile is snapshotted. All drag/resize operations modify the in-memory profile. Save atomically writes config+profile; Cancel restores the snapshot.

### Plugin Trust Boundary

Plugins are external executables run as child processes. Communication is JSON over stdin/stdout. Plugins have the same system access as the Zokute process. Zokute does not sandbox, proxy, or restrict plugin network access.

### Actions

Widgets can register actions (open URI, copy text, refresh) that are executed from the Rust backend. URI validation restricts to `https`, `http`, and `file` schemes.

### Profiles

Named profile files contain the complete widget layout. The active profile is selected in the global config. Profiles are self-contained for portability.

### X11 Boundary

Window management uses GTK3 on the Rust side to set `WindowTypeHint::Desktop`, `stick()`, and `set_always_on_bottom(true)`. Cursor event passthrough is toggled per-widget.
