# ADR 005: Unified Elegant Component Trait

## Status

Accepted

## Context

As the library grew to include diverse components (Buttons, Alerts, Progress bars, Skeletons), there was no unified contract for these components. Integrating them with `egui_flex` required repetitive boilerplate to define default cross-axis behaviors (e.g., some widgets naturally shrink to fit, while others like `Alert` require a minimum structural width).

## Decision

Introduced the `Elegant` trait in `src/traits.rs` as a supertrait of `egui::Widget`.
This trait serves as a central interface for component-level metadata. Currently, it provides hooks like `flex_default_min_width()` and `flex_default_min_height()`, which are automatically consumed by the `impl_flex_widget!` macro to build the correct default `FlexItem` constraints.

## Consequences

- **Pros:** Clean, centralized way to dictate component behavior. Future global component configurations (like semantic tagging, accessibility hints) can be safely added here.
- **Cons:** Requires a blanket `impl Elegant for X {}` even for components that don't need custom flex hints.
