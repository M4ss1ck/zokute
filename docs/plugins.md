# Plugins

Zokute supports external plugin binaries that communicate via a JSON protocol.

## Plugin Protocol

Plugins are executables in `~/.config/zokute/plugins/<plugin_id>/`. Zokute calls the plugin with `--serve` and communicates over stdin/stdout:

- **Input:** JSON lines from Zokute:
  ```json
  {"command":"start","config":{},"interval":30}
  {"command":"sample"}
  {"command":"stop"}
  ```
- **Output:** JSON lines from plugin:
  ```json
  {"type":"content","html":"<div>...</div>"}
  {"type":"error","message":"..."}
  ```

## Security

Plugins run as child processes and are not sandboxed. Only install plugins you trust. Plugin network access is owned by the plugin process — Zokute does not proxy or restrict it.

## Installation

Place the plugin binary in `~/.config/zokute/plugins/<plugin_id>/` and ensure it is executable. The plugin manifest (`manifest.toml`) is optional but recommended.
