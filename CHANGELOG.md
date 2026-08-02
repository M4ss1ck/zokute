# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-08-01

### Added
- Always-on desktop widget windows for system, memory, disk, network, CPU, clock, date, panels, and audio visualizers
- TOML configuration with validation, crash recovery, backup rotation, and unknown field preservation
- First-run onboarding wizard (theme, preset, autostart) and safe mode (`--safe-mode`)
- Profile management: create, duplicate, export, import, delete, and atomic switching
- Three built-in themes (light, dark, system), global appearance controls, and per-widget appearance overrides
- Centralized formatting for bytes, rates, temperatures, and locale-aware dates
- Panel widget that stacks child widgets vertically with configurable gap and padding
- Interactive mode with clickable actions (URL opening, clipboard copy)
- CLI and Unix socket IPC for controlling and diagnosing the application
- Native telemetry: per-interface network, per-disk I/O, AMD GPU, hardware sensors, and load averages
- Plugin system for external widget processes and the Maibuk Notes reference plugin
- Layout editor with drag, resize, snap guides, arrow key nudging, and explicit Save/Cancel transactions
- Stable monitor-relative placement using EDID and connector, robust against display changes
- Hero clock widget and repeatable date widgets with per-instance alignment, font, format, and color
- Audio visualizers (spectrum bars and ring) with per-visualizer parameters including bar count, frequency range, gain, mirror, and gradient direction
- Fullscreen detection with automatic dimming or hiding behavior
- Edit mode: per-widget size slider (25–400 %), independent width and height, zoom on diagonal drag
- Settings dialog with sidebar scroll-spy, sticky footer, draft mode, external change alerts, and error feedback
- Granular widget controls: system field reorder, disk selection and labeling, per-core CPU toggles
- Autostart with desktop-settled delay and platform compatibility guard

### Fixed
- Collector stalling after hiding all widgets
- Audio spectrum range limited to 20–22,000 Hz as specified
- Spectrum visualizer band mapping, mirroring, and gain calculation
- Zoomed dashboard clipping and edit overlay anchoring
- Layout editor panel scrolling and real coordinate display
- Clock layout changes overwriting saved widget dimensions
- Widget visibility on Cinnamon Show Desktop
- Saving geometry only for widgets actually moved during editing
- Settings dialog close and window destruction
