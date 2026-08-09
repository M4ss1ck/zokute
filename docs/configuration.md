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

## Widget fields (`[[sections]]`)

Every widget in a profile is one `[[sections]]` entry. Scope is `widget`
throughout; a `null` default means the global or theme value applies.

### Identity and placement

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `id` | string | — | `system`, `cpu`, `memory`, `disk`, `network`, `gpu`, `clock`, `date`, `spectrum`, `ring`, `panel`, `plugin` | 0.1.0 |
| `instance` | string | `id` | unique within the profile | 0.1.0 |
| `enabled` | bool | — | `true`/`false` | 0.1.0 |
| `show_header` | bool | `true` | `true`/`false` | 0.1.0 |
| `position` | table | — | anchored placement (see [layout-editor.md](layout-editor.md)) | 0.1.0 |
| `monitor` | usize | `0` | index into detected work areas | 0.1.0 |
| `x` / `y` | i32 | `0` | pixels from the anchor | 0.1.0 |
| `width` | u32 | — | pixels | 0.1.0 |
| `height` | u32 | — | pixels; omit to size to content | 0.1.0 |
| `scale` | f64 | `1.0` | render scale | 0.1.0 |
| `interactive` | bool | `false` | `true`/`false` (INT-002) | 0.1.0 |

### Panels (`id = "panel"`)

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `children` | array | `[]` | nested `[[sections.children]]` entries | 0.1.0 |
| `panel_gap` | u32 | `0` | pixels between children | 0.1.0 |
| `panel_padding` | u32 | `0` | pixels inside the panel | 0.1.0 |

### Plugins (`id = "plugin"`)

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `plugin_id` | string | — | must match the plugin directory name | 0.1.0 |
| `plugin_interval` | u64 | `30` | seconds, `>= 2` | 0.1.0 |
| `plugin_config` | table | — | passed through to the plugin verbatim | 0.1.0 |

### Appearance overrides

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `accent_color` | hex | — | any hex color | 0.1.0 |
| `transparent_surface` | bool | — | `true`/`false` | 0.1.0 |
| `opacity_override` | f64 | — | `0.1`–`1.0` | 0.1.0 |
| `border_visible` | bool | — | `true`/`false` | 0.1.0 |
| `radius_override` | u32 | — | pixels | 0.1.0 |
| `padding_override` | u32 | — | pixels | 0.1.0 |
| `font_scale` | f64 | — | `0.5`–`2.0` | 0.1.0 |
| `chart_colors` | array | — | hex colors | 0.1.0 |
| `color_mode` | string | — | `solid`, `gradient` | 0.1.0 |
| `color_a` / `color_b` | hex | — | any hex color; `color_b` is the gradient end | 0.1.0 |
| `gradient_direction` | string | — | `horizontal`, `vertical` | 0.1.0 |

### Clock and date

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `clock_font` | string | — | font name | 0.1.0 |
| `clock_color` | hex | — | any hex color | 0.1.0 |
| `clock_seconds` | bool | `false` | `true`/`false` | 0.1.0 |
| `clock_24h` | bool | `false` | `true`/`false` | 0.1.0 |
| `clock_ampm` | bool | `true` | `true`/`false` | 0.1.0 |
| `clock_pad` | bool | `true` | pad the leading hour digit | 0.1.0 |
| `clock_layout` | string | — | `stacked`, `inline` | 0.1.0 |
| `clock_align` | string | — | `left`, `center`, `right` | 0.1.0 |
| `timezone` | string | — | IANA name, e.g. `Europe/Madrid` | 0.1.0 |
| `date_weekday` | bool | `true` | `true`/`false` | 0.1.0 |
| `date_format` | string | — | strftime-style pattern | 0.1.0 |
| `date_color` | hex | — | any hex color | 0.1.0 |

### Visualizers (`id = "spectrum"` or `"ring"`)

Bounds are enforced on load; out-of-range values clamp to the nearest limit
(AUDIO-003).

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `viz_bar_count` | u32 | `48` | `8`–`128`; total visible bars after mirroring | 0.1.0 |
| `viz_min_hz` | f64 | `40` | `20`–`2000`; must stay below `viz_max_hz` | 0.1.0 |
| `viz_max_hz` | f64 | `16000` | `2000`–`22000` | 0.1.0 |
| `viz_gain` | f64 | `1.0` | `0.1`–`5.0`; multiplies before clamping | 0.1.0 |
| `viz_smoothing` | f64 | `0.65` | `0.0`–`0.95` | 0.1.0 |
| `viz_decay` | f64 | `0.82` | `0.0`–`0.99` | 0.1.0 |
| `viz_mirror` | bool | `false` | `true`/`false` | 0.1.0 |
| `viz_gap` | u32 | `4` | pixels between bars | 0.1.0 |
| `viz_rounded_caps` | bool | `false` | `true`/`false` | 0.1.0 |
| `viz_fps` | u32 | `30` | `30` or `60` | 0.1.0 |

`viz_mirror` is interpreted by the visualizer type. For `spectrum`, enabling
it flips the graph vertically so bars rest at the top and grow downward. For
`ring`, enabling it reflects the frequency bands across the vertical axis.

## Fullscreen policy (`[fullscreen]`)

Profile-global, not per monitor (FULL-001). Edit mode overrides it.

| Field | Type | Default | Allowed | Version |
|---|---|---|---|---|
| `behavior` | string | `"show"` | `show`, `hide`, `dim` | 0.1.0 |
| `dim_opacity` | f64 | `0.25` | `0.0`–`1.0`; used when `behavior = "dim"` | 0.1.0 |
| `enter_delay_ms` | u64 | `150` | milliseconds of enter hysteresis | 0.1.0 |
| `exit_delay_ms` | u64 | `250` | milliseconds of exit hysteresis | 0.1.0 |

## Profile fields

Profiles live in `~/.config/zokute/profiles/<name>.toml`.

| Field | Type | Default | Description |
|---|---|---|---|
| `profile_schema_version` | u32 | `1` | Profile schema |
| `sections` | array | — | Widget configurations |
| `system_fields` | array | `[...]` | Enabled system info fields |
| `show_cpu_cores` | bool | `true` | Show per-core CPU |
| `disks` | array | — | Disk visibility preferences |
| `collect_interval_ms` | u64 | `1000` | Metrics collection interval, 250–60000 ms |
| `fullscreen` | table | see above | Fullscreen policy |

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

Pairs with [`performance-panel-profile.toml`](examples/performance-panel-profile.toml).

```toml
opacity = 0.85
theme = "dark"
density = "compact"
byte_format = "decimal"
```

### Gaming

Pairs with [`gaming-profile.toml`](examples/gaming-profile.toml).

```toml
opacity = 0.7
theme = "dark"
density = "compact"
show_background = false
accent_color = "#ff4444"
```

### Multi-monitor

Pairs with [`multi-monitor-profile.toml`](examples/multi-monitor-profile.toml).

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

Global config, plus [`fullscreen-hide-profile.toml`](examples/fullscreen-hide-profile.toml)
for the policy itself:

```toml
opacity = 0.92
theme = "dark"
density = "compact"
```

### Maibuk plugin

See [`maibuk-profile.toml`](examples/maibuk-profile.toml).

### Interactive actions

See [`interactive-action-profile.toml`](examples/interactive-action-profile.toml).

Every example above is parsed by the test suite in `config_doc_tests.rs`, so
they cannot drift from the schema.
