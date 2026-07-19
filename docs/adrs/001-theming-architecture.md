# ADR 001: Decoupled Theming Architecture

## Status

Accepted

## Context

Initially, the library directly manipulated `egui::Context` visuals in a highly coupled way. The theme was rigid, and supporting OS-level accents (like Windows system accent colors) required injecting platform-specific code deeply into the widget rendering logic.

## Decision

We decoupled the theming system into a central `ElegantTheme` struct and isolated the application of visuals.

- `ElegantTheme` serves as a pure data structure (tokens, colors, spacing).
- A separate `apply(ctx)` method pushes the tokens into `egui::Context::set_visuals` and `set_style`.
- Integrated `ThemeMode::System` which utilizes `winreg` to read the host OS (Windows) accent color and dynamically generates a cohesive HSL palette.

## Consequences

- **Pros:** Widgets just read `ElegantTheme::get(ctx)` without worrying about how visuals are applied. Makes supporting multiple themes (Light, Dark, System) trivial.
- **Cons:** Slightly more boilerplate when initializing the app, as the user must explicitly build and apply the theme on startup.
