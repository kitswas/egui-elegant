# egui-elegant

A beautiful, minimal, and elegant UI component library for `egui`. Designed with inspiration from modern web component libraries (like oat, shadcn/ui, and daisyUI) and brought to immediate-mode Rust.

> [!NOTE]  
> This is an experimental project.

## Features

- **Rich Aesthetics**: Pre-configured colors, dark/light modes, and micro-animations.
- **Unified `Elegant` Trait**: All components implement the `Elegant` marker trait.
- **Optional `egui_flex` Integration**: When the `flex` feature is enabled, all components automatically become `FlexWidget`s and can be directly placed inside `Flex::horizontal()` or `Flex::vertical()` containers.
- **Async Theme Streaming**: Enable the `async` feature to use `ElegantTheme::stream()` for real-time reactive OS theme updates.
- **Responsive Sizing**: Sizing models similar to CSS limits (`min_width`, etc.) encoded directly into components natively for layout predictability.

## Components

Here are the components provided out-of-the-box, compatible with both native `egui` layouts and `egui_flex`.

| Component | Image                          |
| :-------- | -----------------------------: |
| Accordion | ![](docs/images/accordion.png) |
| Alert     | ![](docs/images/alert.png)     |
| Avatar    | ![](docs/images/avatar.png)    |
| Badges    | ![](docs/images/badges.png)    |
| Buttons   | ![](docs/images/buttons.png)   |
| Card      | ![](docs/images/card.png)      |
| Dropdown  | ![](docs/images/dropdown.png)  |
| Progress  | ![](docs/images/progress.png)  |
| Skeleton  | ![](docs/images/skeleton.png)  |
| Tabs      | ![](docs/images/tabs.png)      |
| Toast     | ![](docs/images/toast.png)     |

## Setup

Add the following to your `Cargo.toml`:

```toml
[dependencies]
egui = "0.35"

# Basic usage
egui-elegant = { version = "0.1" }

# Or with egui_flex integration:
egui-elegant = { version = "0.1", features = ["flex"] }

# Or with multilingual text support:
egui-elegant = { version = "0.1", features = ["noto"] }

# Or with async reactive theme streaming:
egui-elegant = { version = "0.1", features = ["async"] }
```

Initialize the theme in your eframe app:

```rust
use egui_elegant::{ElegantTheme, ThemeMode, MonaspaceFont};

// Inside app setup:
let theme = ElegantTheme::build(ThemeMode::System, MonaspaceFont::Neon);
theme.apply(&cc.egui_ctx);
```

## Architecture

The `Elegant` trait ties the library together. Leaf components (like buttons, progress bars) implement both `egui::Widget` and `Elegant`. Container components (like `Card`, `Accordion`) provide custom `.show()` closures.

```mermaid
graph TD
    A["egui-elegant (no features)"] --> B["Elegant trait\n(marker + sizing hints)"]
    B --> C["egui::Widget\n(all leaf components)"]
    
    D["egui-elegant (features = flex)"] --> B
    D --> E["egui_flex::FlexWidget\n(macro impl via Elegant)"]
    E --> F["flex.add(item(), ElegantButton::new(...))"]

    G["Card / Accordion\n(containers)"] --> H["show(&mut Ui, closure)"]
    G --> I["show_flex(&mut FlexInstance, item, closure)\n(feature-gated)"]
```

## Examples

Run the showcases to see all components in action:

```bash
# Showcase with native egui layouts
cargo run -p egui-elegant --example showcase_noflex

# Showcase with egui_flex based wrapping layouts
cargo run -p egui-elegant --example showcase --features flex

# Showcase with multilingual content
cargo run -p egui-elegant --example noto_multilingual --features noto

# Async reactive theme streaming
cargo run -p egui-elegant --example async_theme --features async
```
