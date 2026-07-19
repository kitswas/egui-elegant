# ADR 003: Optional Flex Layout Integration

## Status

Accepted

## Context

`egui` relies heavily on an immediate-mode, constraint-based layout engine (`ui.horizontal`, `ui.vertical`). While robust, it makes complex responsive designs (like flex-wrapping grid cards) cumbersome, requiring manual spacing and math. The user requested all showcase cards to be purely flex-based.

## Decision

Integrated the `egui_flex` crate behind an optional `flex` Cargo feature.
Instead of forcing users into `egui_flex`, components remain native `egui::Widget`s. To bridge the gap, we implemented a macro `impl_flex_widget!` that automatically implements `egui_flex::FlexWidget` for any component that implements `egui::Widget`.

## Consequences

- **Pros:** Developers can use components in standard `egui` flows seamlessly, or drop them directly into `flex.add(...)` without wrapper code.
- **Cons:** Maintaining layout consistency across both standard `egui` and `egui_flex` contexts requires careful tracking of `available_width()` and intrinsic sizing passes.
