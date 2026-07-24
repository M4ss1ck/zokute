# Zokute Agent Rules

## Maintainability Rules
1. No file over 150 lines. If one grows past that, it is doing too much, so split it by feature and not by layer.
2. No barrel files. No index.ts that re-exports. Every import states the real file it comes from.
3. No abstraction until the third repetition. Two similar blocks of code stay duplicated. Duplication is cheaper to read than indirection.
4. Types live next to their use. No central types.ts, no models.rs. A widget's type is defined in the widget's file.
5. No custom hooks used by fewer than two components. Inline the logic instead.
6. No traits with a single implementation. No generics without a second concrete case. No dependency injection.
7. Max two directory levels under src/ and src-tauri/src/. Flat beats nested.
8. Comments explain why, never what. Delete any comment that restates the code.
9. No config builders, no options structs with defaults spread over multiple files. One config struct, one TOML file, one place.
10. Reading a single widget file top to bottom must fully explain that widget. If I have to open a second file to understand what a widget does, the design is wrong.

The 150-line cap applies to authored files only; generated lockfiles and ignored generated build output are exempt.

## Add A Widget
1. Add a field to the Rust Stats struct in src-tauri/src/collect.rs.
2. Add its collection line there.
3. Mirror the field in the Stats interface in src/useStats.ts.
4. Add src/widgets/NewWidget.tsx and explicitly import/render it in src/App.tsx.

## Performance Rules
effectively zero idle CPU; no requestAnimationFrame/animation loops; no backdrop-filter; no filter/animated gradients/box-shadow on anything that moves; CSS transitions only on discrete changes <=200ms; SVG arcs/sparklines redraw only on stats arrival, never own timers; all visual values in src/theme.css custom properties; content-tight not fullscreen.
