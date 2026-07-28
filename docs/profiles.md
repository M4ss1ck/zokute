# Profiles

Profiles are named configurations that contain widget layout, enabled sections, system fields, and disk preferences. They live in `~/.config/zokute/profiles/<name>.toml`.

## Commands

Create or switch profiles from Settings. CLI commands are planned.

## Switching

- Set `active_profile` in `zokute.toml` to switch profiles
- Export profiles as JSON for sharing:
  ```sh
  # Via IPC (future)
  ```

## Example

```toml
profile_schema_version = 1
show_cpu_cores = true
system_fields = ["os", "host", "kernel", "uptime"]
collect_interval_ms = 1000

[[sections]]
id = "cpu"
instance = "cpu"
enabled = true
width = 360
scale = 1.0
```

## Portability

Profiles store positions anchored to monitor identities (connector + EDID hash), so layouts follow monitors across reboots and reconnections.
