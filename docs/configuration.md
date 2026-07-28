# Configuration Reference

The config file is `~/.config/zokute/zokute.toml` (TOML format).

## Fields

| Field | Type | Default | Allowed | Scope | Version |
|---|---|---|---|---|---|
| `schema_version` | u32 | `3` | `3` only | global | 0.1.0 |
| `active_profile` | string | `"default"` | any profile name | global | 0.1.0 |
| `opacity` | f64 | `0.92` | `0.1`–`1.0` | global | 0.1.0 |
| `text_opacity` | f64 | `1.0` | `0.0`–`1.0` | global | 0.1.0 |
| `text_color` | hex | `"#292824"` | any hex color | global | 0.1.0 |
| `graph_color` | hex | — | any hex color | global | 0.1.0 |
| `icon_color` | hex | — | any hex color | global | 0.1.0 |
| `show_background` | bool | `true` | `true`/`false` | global | 0.1.0 |
| `theme` | string | `"light"` | `"light"`, `"dark"`, `"system"` | global | 0.1.0 |
| `accent_color` | hex | — | any hex color | global | 0.1.0 |
| `density` | string | `"compact"` | `"compact"`, `"comfortable"` | global | 0.1.0 |
| `font_scale` | f64 | `1.0` | `0.5`–`2.0` | global | 0.1.0 |
| `sans_font` | string | — | font name | global | 0.1.0 |
| `mono_font` | string | — | font name | global | 0.1.0 |
| `byte_format` | string | `"binary"` | `"binary"`, `"decimal"` | global | 0.1.0 |
| `temperature_unit` | string | `"celsius"` | `"celsius"`, `"fahrenheit"` | global | 0.1.0 |
| `locale` | string | — | locale string | global | 0.1.0 |

## Profile fields

Profiles live in `~/.config/zokute/profiles/<name>.toml`.

| Field | Type | Default | Description |
|---|---|---|---|
| `profile_schema_version` | u32 | `1` | Profile schema |
| `sections` | array | — | Widget configurations |
| `system_fields` | array | `[...]` | Enabled system info fields |
| `show_cpu_cores` | bool | `true` | Show per-core CPU |
| `disks` | array | — | Disk visibility preferences |
| `collect_interval_ms` | u64 | `1000` | Metrics collection interval |

## Examples

### Minimal

```toml
schema_version = 3
active_profile = "default"
opacity = 0.92
text_opacity = 1.0
theme = "dark"
font_scale = 1.5
```

### Performance panel

```toml
opacity = 0.85
theme = "dark"
density = "compact"
byte_format = "decimal"
```

### Gaming

```toml
opacity = 0.7
theme = "dark"
density = "compact"
show_background = false
accent_color = "#ff4444"
```

### Multi-monitor

```toml
schema_version = 3
active_profile = "multi"
opacity = 0.9
theme = "dark"
```

### Dark transparent

```toml
opacity = 0.6
theme = "dark"
text_opacity = 0.9
show_background = false
```

### Fullscreen hide

```toml
opacity = 0.92
theme = "dark"
density = "compact"
```
