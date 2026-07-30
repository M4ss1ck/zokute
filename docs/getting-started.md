# Getting Started

## First Run

On first run, Zokute opens the onboarding wizard:

1. **Theme** — Atelier Light, Dark, or System
2. **Preset** — Minimal (clock/date), System Monitor (clock, date, CPU, memory, disk, network), or Blank
3. **Autostart** — Enable or disable

After completing onboarding, your config is saved atomically. The settings window shows a summary with your config location and a button that opens the settings dialog with arranging turned on.

## Presets

### Minimal

Clock and date widgets anchored top-left on the primary monitor.

### System Monitor

Clock, date, CPU, memory, disk, and network widgets stacked vertically on the left side.

### Blank

No widgets. Opens Settings so you can add widgets manually.

## Configuration Files

- `~/.config/zokute/zokute.toml` — global configuration
- `~/.config/zokute/profiles/*.toml` — named profiles
- `~/.config/zokute/plugins/` — installed plugins
- `~/.local/state/zokute/backups/` — automatic backups
- `~/.local/state/zokute/logs/zokute.log` — application logs

## CLI

```sh
zokute status          # show running status
zokute show            # show all widgets
zokute hide            # hide all widgets
zokute toggle          # toggle widget visibility
zokute reload          # reload configuration
zokute edit            # enter layout editing mode
zokute config validate [path]  # validate config file
zokute config show              # show effective configuration
zokute diagnostics              # collect diagnostics
```
