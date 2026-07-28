# Troubleshooting

## Widgets don't appear

1. Check if widgets are enabled in Settings
2. Run `zokute status` to verify the app is running
3. Check logs at `~/.local/state/zokute/logs/zokute.log`

## Configuration errors

If your config becomes invalid, Zokute shows a recovery screen with options:
- **Restore Previous Backup** — revert to the last good config
- **Use Defaults** — overwrite with fresh defaults
- **Validate Again** — re-check the current config

## Audio visualizers don't work

- Ensure PulseAudio is running and not suspended
- Verify the visualizer widget is enabled in Settings
- Check that no other application is exclusively capturing the monitor

## CLI commands don't respond

Run `zokute status` to verify the app is running. If the socket is missing, ensure Zokute is not running and check `$XDG_RUNTIME_DIR/zokute/control.sock`.

## Wayland

Zokute only supports X11. On Wayland, the tray and settings remain available but no HUD windows are created.

## Diagnostics

Run `zokute diagnostics` to collect a system report including platform info, monitors, profile/schema, widgets, audio lifecycle, collector interval, and recent errors.
