# ADR 004: Hybrid Layout for Nested Flex Containers

## Status

Accepted

## Context

When nesting `Flex::horizontal().wrap(true)` inside a `Flex::vertical()` inside another wrapping flex container, `egui_flex` entered an infinite layout loop (`request_discard` spam). This happened because wrapping alters height, and height alters wrapping constraints, causing the layout to never settle on a final frame. Additionally, unbounded `available_width()` queries during intrinsic sizing passes caused widgets to infinitely stretch.

## Decision

Adopted a hybrid layout strategy for complex widget assemblies:

1. Outer containers (like grids and card bodies) use `Flex::vertical()` or `Flex::horizontal()`.
2. Elements requiring text-like wrapping (e.g., button rows, badges) are delegated back to `egui`'s battle-tested `ui.horizontal_wrapped` via `flex.add_ui()`.
3. Stretchable block components (like `Alert`) explicitly take a fixed layout width from their outer constrained container (e.g., `ui.add_sized(egui::vec2(inner_width, 0.0), Alert::new(...))`) instead of querying `ui.available_width()` internally.

## Consequences

- **Pros:** Complete elimination of frame layout loops and warning spam. Highly stable layout regardless of window resizing.
- **Cons:** Internal layouts of complex showcase assemblies are slightly less "pure flex", requiring explicit sizing bounds to be passed down.
